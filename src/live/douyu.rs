use std::collections::HashMap;

use crate::model::Detail;
use crate::{common::CLIENT, default_danmu_client, model::ShowType};

use crate::util::{js, get_plugin_path, md5, parse_url};
use anyhow::{Ok, Result};
use chrono::prelude::*;
use regex::Regex;

const URL: &str = "https://www.douyu.com/";
const M_URL: &str = "https://m.douyu.com/";
const PLAY_URL: &str = "https://www.douyu.com/lapi/live/getH5Play/";
const PLAY_URL_M: &str = "https://m.douyu.com/api/room/ratestream";
const CDN_1: &str = "http://hw-tct.douyucdn.cn/live/";
const CDN_2: &str = "http://hdltc1.douyucdn.cn/live/";
const DID: &str = "10000000000000000000000000001501";

default_danmu_client!(Douyu);

/// 斗鱼直播
///
/// https://www.douyu.com/
pub async fn get(rid: &str) -> Result<ShowType> {
    let plugin = get_plugin_path();
    if !plugin.exists() {
        return Ok(ShowType::Error("缺少插件:请前往 https://github.com/Borber/Jin/releases/latest 下载对应平台的 jin 可执行文件并解压到 seam 同级目录.\nMissing plugin: Please go to https://github.com/Borber/Jin/releases/latest to download the jin executable for the corresponding platform and extract it to the same level as seam.".to_string()));
    }
    let rid = match real_rid(rid).await {
        Some(rid) => rid,
        None => return Ok(ShowType::Error("直播间不存在".to_string())),
    };
    douyu_do_js(&rid).await
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
async fn douyu_do_js_pc(rid: &str) -> Result<ShowType> {
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
    let res = js(&func).await;

    // 构建函数, 替换数值
    let res = res.replace("(function", "let ccc = function");
    let res = res.replace(
        "rt;})",
        format!("rt;}}; ccc({rid}, \"{DID}\", {dt})").as_str(),
    );

    // 替换md5值避免js依赖
    let cb = format!("{rid}{DID}{dt}{v}");
    let rb = md5(cb.as_bytes());
    let res = res.replace(
        "CryptoJS.MD5(cb).toString();",
        format!("\"{}\";", &rb).as_str(),
    );

    // 运行js获取签名值
    let sign = js(&res).await;
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
            Ok(ShowType::On(Detail::new("douyu".to_owned(), nodes)))
        }
        _ => Ok(ShowType::Off),
    }
}

async fn douyu_do_js(rid: &str) -> Result<ShowType> {
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
    let res = js(&func).await;

    // 构建函数, 替换数值
    let res = res.replace("(function", "let ccc = function");
    let res = res.replace("rt;})", &format!("rt;}};ccc({rid}, \"{DID}\", {dt});"));
    let re = Regex::new(r#"v=([0-9]{12})"#).unwrap();
    let v = re.captures(&res).unwrap().get(1).unwrap().as_str();

    // 替换md5值避免js依赖
    let cb = format!("{rid}{DID}{dt}{v}");
    let rb = md5(cb.as_bytes());
    let res = res.replace(
        "CryptoJS.MD5(cb).toString();",
        format!("\"{}\";", &rb).as_str(),
    );
    // println!("{}", res);
    // 运行js获取签名值
    let sign = js(&res).await;
    // println!("{}", sign);
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
    match json["code"].as_i64().unwrap() {
        0 => {
            let key = json["data"]["url"].as_str().unwrap();
            let key = key.split_once(".m3u8").unwrap().0;
            let key = key.rsplit_once('/').unwrap().1;
            let key = match key.split_once('_') {
                Some((k, _)) => k,
                None => key,
            };
            let nodes = vec![
                parse_url(format!("{CDN_1}{key}.flv")),
                parse_url(format!("{CDN_2}{key}.flv")),
            ];
            Ok(ShowType::On(Detail::new(title, nodes)))
        }
        _ => Ok(ShowType::Off),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_get_url() {
        println!("{}", get("221869").await.unwrap());
    }
}
