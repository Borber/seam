use std::collections::HashMap;

use crate::{
    common::CLIENT,
    error::{Result, SeamError},
    util::{eval, get_plugin_path, parse_url},
};

use async_trait::async_trait;
use chrono::prelude::*;
use md5::{Digest, Md5};
use regex::Regex;

use super::{Live, Node};

const URL: &str = "https://www.douyu.com/";
const M_URL: &str = "https://m.douyu.com/";
const PLAY_URL: &str = "https://www.douyu.com/lapi/live/getH5Play/";
const PLAY_URL_M: &str = "https://m.douyu.com/api/room/ratestream";
const CDN_1: &str = "http://hlstct.douyucdn2.cn/dyliveflv1a/";
const CDN_2: &str = "http://hdltctwk.douyucdn2.cn/live/";
const DID: &str = "10000000000000000000000000001501";

/// 斗鱼直播
///
/// https://www.douyu.com/
pub struct Client;

#[async_trait]
impl Live for Client {
    async fn get(&self, rid: &str) -> Result<Node> {
        let plugin = get_plugin_path();
        if !plugin.exists() {
            return Err(SeamError::Plugin("缺少插件:请前往 https://github.com/Borber/Jin/releases/latest 下载对应平台的 jin 可执行文件并解压到 seam 同级目录.\nMissing plugin: Please go to https://github.com/Borber/Jin/releases/latest to download the jin executable for the corresponding platform and extract it to the same level as seam.".to_string()));
        }
        let rid = match real_rid(rid).await {
            Some(rid) => rid,
            None => return Err(SeamError::None),
        };
        douyu_do_js(&rid).await
    }
}

/// 获取真实房间号
async fn real_rid(rid: &str) -> Option<String> {
    let resp = CLIENT.get(format!("{M_URL}{rid}")).send().await.unwrap();
    let text = resp.text().await.unwrap();
    let re = Regex::new(r#"rid":(\d{1,8}),"vipId"#).unwrap();
    re.captures(&text)
        .map(|caps| caps.get(1).unwrap().as_str().to_owned())
}

// TODO 简化代码 今天状态不行, 改天再说
#[allow(dead_code)]
async fn douyu_do_js_pc(rid: &str) -> Result<Node> {
    // 构造时间戳
    let binding = Local::now().timestamp_millis().to_string();
    let dt = &binding.as_str()[0..10];

    // 获取指定直播间的首页源代码, 认证的sign和直播间是绑定的
    let text = CLIENT
        .get(format!("{URL}{rid}"))
        .send()
        .await?
        .text()
        .await?;

    // 正则匹配固定位置的js代码
    let re = Regex::new(r#"<script type="text/javascript">([\s\S]*?)</script>"#).unwrap();
    let mut func = String::new();
    let mut v = "";
    for cap in re.captures_iter(&text) {
        let script = cap.get(1).unwrap().as_str();
        let re2 = Regex::new("\"([0-9]{12})\"").unwrap();
        match re2.captures(script) {
            Some(t_cap) => {
                v = t_cap.get(1).unwrap().as_str();
                func = script.to_owned();
            }
            None => continue,
        }
    }

    // 将eval运行字符串更改为直接返回字符串
    let re3 = Regex::new(r"eval\(strc\)[\s\S]*?\)").unwrap();
    let func = re3.replace_all(&func, "strc").to_string();
    let func = format!("{func}ub98484234(0,0,0)");
    // println!("{func}");

    // 获取eval实际运行的字符串
    let res = eval(&func).await;

    // 构建函数, 替换数值
    let res = res.replace("(function", "let ccc = function");
    let res = res.replace(
        "rt;})",
        format!("rt;}}; ccc({rid}, \"{DID}\", {dt})").as_str(),
    );

    // 替换md5值避免js依赖
    let cb = format!("{rid}{DID}{dt}{v}");
    let rb = {
        let mut h = Md5::new();
        h.update(cb);
        hex::encode(h.finalize())
    };
    let res = res.replace(
        "CryptoJS.MD5(cb).toString();",
        format!("\"{}\";", &rb).as_str(),
    );

    // 运行js获取签名值
    let sign = eval(&res).await;
    let sign = sign.rsplit_once('=').unwrap().1;

    let mut params = HashMap::new();
    params.insert("v", v);
    params.insert("did", DID);
    params.insert("tt", dt);
    params.insert("sign", sign);

    let json: serde_json::Value = CLIENT
        .post(format!("{PLAY_URL}{rid}"))
        .form(&params)
        .send()
        .await?
        .json()
        .await?;
    match json["error"].as_i64().unwrap() {
        0 => {
            let key = json["data"]["rtmp_live"].as_str().unwrap();
            let key = key.split_once('.').unwrap().0;
            let key = match key.split_once('_') {
                Some((k, _)) => k,
                None => key,
            };
            let nodes = vec![
                parse_url(format!("{CDN_1}{key}.flv")),
                parse_url(format!("{CDN_2}{key}.flv")),
            ];
            Ok(Node {
                rid: rid.to_owned(),
                title: "douyu".to_owned(),
                urls: nodes,
            })
        }
        _ => Err(SeamError::None),
    }
}

async fn douyu_do_js(rid: &str) -> Result<Node> {
    // 构造时间戳
    let binding = Local::now().timestamp_millis().to_string();
    let dt = &binding.as_str()[0..10];

    // 获取指定直播间的首页源代码, 认证的sign和直播间是绑定的
    let text = CLIENT
        .get(format!("{M_URL}{rid}"))
        .send()
        .await?
        .text()
        .await?;

    let re = Regex::new(r#"roomName":"([\s\S]*?)""#).unwrap();
    let title = re
        .captures(&text)
        .unwrap()
        .get(1)
        .unwrap()
        .as_str()
        .to_owned();

    // 正则匹配固定位置的js代码
    let re = Regex::new(r"(function ub98484234.*)\s(var.*)").unwrap();
    let func = re
        .captures(&text)
        .unwrap()
        .get(0)
        .unwrap()
        .as_str()
        .to_owned();
    let re = Regex::new(r#"eval.*;}"#).unwrap();
    let func = re.replace(&func, "strc;}");
    let func = format!("{func}ub98484234(0,0,0)");
    // 获取eval实际运行的字符串
    let res = eval(&func).await;

    // 构建函数, 替换数值
    let res = res.replace("(function", "let ccc = function");
    let res = res.replace("rt;})", &format!("rt;}};ccc({rid}, \"{DID}\", {dt});"));
    let re = Regex::new(r#"v=([0-9]{12})"#).unwrap();
    let v = re.captures(&res).unwrap().get(1).unwrap().as_str();

    // 替换md5值避免js依赖
    let cb = format!("{rid}{DID}{dt}{v}");
    let rb = {
        let mut h = Md5::new();
        h.update(cb);
        hex::encode(h.finalize())
    };
    let res = res.replace(
        "CryptoJS.MD5(cb).toString();",
        format!("\"{}\";", &rb).as_str(),
    );
    // println!("{}", res);
    // 运行js获取签名值
    let res = res.trim().trim_matches('"');
    let sign = eval(res).await;
    let sign = sign.trim().trim_matches('"');
    let sign = sign.rsplit_once('=').unwrap().1;

    let mut params = HashMap::new();
    params.insert("v", v);
    params.insert("did", DID);
    params.insert("tt", dt);
    params.insert("sign", sign);
    params.insert("rid", rid);

    let json: serde_json::Value = CLIENT
        .post(PLAY_URL_M)
        .form(&params)
        .send()
        .await?
        .json()
        .await?;
    // println!("{:?}", json);
    match json["code"].as_i64().unwrap() {
        0 => {
            let url_origin = json["data"]["url"].as_str().unwrap();
            let key = url_origin.rsplit_once('/').unwrap().1;
            let urls = vec![
                parse_url(url_origin.to_owned()),
                parse_url(format!("{CDN_1}{key}").replace(".m3u8", ".flv")),
                parse_url(format!("{CDN_2}{key}").replace(".m3u8", ".flv")),
            ];
            Ok(Node {
                rid: rid.to_owned(),
                title,
                urls,
            })
        }
        _ => Err(SeamError::None),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_get_url() {
        let cli = Client;
        match cli.get("5040227").await {
            Ok(node) => println!("{}", node.json()),
            Err(e) => println!("{e}"),
        }
    }
}
