use std::collections::HashMap;

use crate::model::Detail;
use crate::{common::CLIENT, default_danmu_client, model::ShowType};

use crate::util::{do_js, md5, parse_url};
use anyhow::{Ok, Result};
use chrono::prelude::*;
use regex::Regex;
use serde_json::Value;

const URL: &str = "https://www.douyu.com/";
const M_URL: &str = "https://m.douyu.com/";
const PLAY_URL: &str = "https://www.douyu.com/lapi/live/getH5Play/";
const CDN_1: &str = "http://akm-tct.douyucdn.cn/live/";
const CDN_2: &str = "http://ws-tct.douyucdn.cn/live/";
const DID: &str = "10000000000000000000000000001501";

default_danmu_client!(Douyu);

/// 斗鱼直播
///
/// https://www.douyu.com/
pub async fn get(rid: &str) -> Result<ShowType> {
    let rid = match real_rid(rid).await {
        Some(rid) => rid,
        None => return Ok(ShowType::Error("直播间不存在".to_string())),
    };
    let json = douyu_do_js(&rid).await?;
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

/// 获取真实房间号
async fn real_rid(rid: &str) -> Option<String> {
    let resp = CLIENT.get(format!("{M_URL}{rid}")).send().await.unwrap();
    let text = resp.text().await.unwrap();
    let re = Regex::new(r#"rid":(\d{1,8}),"vipId"#).unwrap();
    re.captures(&text)
        .map(|caps| caps.get(1).unwrap().as_str().to_owned())
}

/// TODO: 更改为移动版 减少资源消耗
async fn douyu_do_js(rid: &str) -> Result<Value> {
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
    let res = do_js(&func).await;

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
    let sign = do_js(&res).await;
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
    Ok(json)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_get_url() {
        println!("{}", get("33").await.unwrap());
    }
}
