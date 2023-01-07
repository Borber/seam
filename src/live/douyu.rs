use std::collections::HashMap;

use crate::modle::Node;
use crate::{common::CLIENT, modle::ShowType};

use crate::util::md5;
use anyhow::{Ok, Result};
use chrono::prelude::*;
use regex::Regex;
use reqwest::header::HeaderMap;

// const URL: &str = "https://www.douyu.com/";
const M_URL: &str = "https://m.douyu.com/";
const PLAY_URL: &str = "https://playweb.douyucdn.cn/lapi/live/hlsH5Preview/";
const CDN_1: &str = "http://akm-tct.douyucdn.cn/live/";
const CDN_2: &str = "http://ws-tct.douyucdn.cn/live/";
const DID: &str = "10000000000000000000000000001501";

/// 斗鱼直播
///
/// https://www.douyu.com/
pub async fn get(rid: &str) -> Result<ShowType> {
    let dt = Local::now().timestamp_millis().to_string();
    let rid = match real_rid(rid).await {
        Some(rid) => rid,
        None => return Ok(ShowType::Error("直播间不存在".to_string())),
    };
    let key = match get_pre(&rid, dt).await {
        Some(key) => key,
        None => return Ok(ShowType::Off),
    };
    Ok(ShowType::On(vec![
        Node {
            rate: "超清1".to_string(),
            url: format!("{CDN_1}{key}.flv"),
        },
        Node {
            rate: "超清2".to_string(),
            url: format!("{CDN_2}{key}.flv"),
        },
    ]))
}

async fn real_rid(rid: &str) -> Option<String> {
    let resp = CLIENT.get(format!("{M_URL}{rid}")).send().await.unwrap();
    let text = resp.text().await.unwrap();
    let re = Regex::new(r#"rid":(\d{1,8}),"vipId"#).unwrap();
    re.captures(&text)
        .map(|caps| caps.get(1).unwrap().as_str().to_owned())
}
async fn get_pre(rid: &str, dt: String) -> Option<String> {
    let url = format!("{PLAY_URL}{rid}");
    let auth = md5(format!("{rid}{dt}").as_bytes());

    let mut headers = HeaderMap::new();
    headers.insert("rid", rid.parse().unwrap());
    headers.insert("time", dt.parse().unwrap());
    headers.insert("auth", auth.parse().unwrap());

    let mut params = HashMap::new();
    params.insert("rid", rid);
    params.insert("did", DID);

    let resp: serde_json::Value = CLIENT
        .post(url)
        .headers(headers)
        .form(&params)
        .send()
        .await
        .unwrap()
        .json()
        .await
        .unwrap();

        // 因为前面以及判断过直播间是否存在, 所以现在直接判断是否开播, 不做 102, 104 状态码的区分
    match resp["error"].to_string().parse::<usize>().unwrap() {
        0 => {
            let rtmp_live = resp["data"]["rtmp_live"].to_string();
            let key = rtmp_live.trim_matches('"').split_once('.').unwrap().0;
            match key.split_once('_') {
                Some((k, _)) => Some(k.to_owned()),
                None => Some(key.to_owned()),
            }
        }
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use crate::live::douyu::get;
    use crate::util::match_show_type;

    #[tokio::test]
    async fn test_get_url() {
        match_show_type(get("10726615").await.unwrap());
    }
}
