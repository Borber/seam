use crate::{
    common::CLIENT,
    default_danmu_client,
    model::{Detail, ShowType},
    util::parse_url,
};

use crate::common::USER_AGENT;
use anyhow::{Ok, Result};
use regex::Regex;
use reqwest::header::HeaderMap;
use serde_json::Value;
use urlencoding::decode;

const URL: &str = "https://live.douyin.com/";

default_danmu_client!(Douyin);

/// 抖音直播
///
/// https://live.douyin.com/
pub async fn get(rid: &str) -> Result<ShowType> {
    let mut header_map = HeaderMap::new();
    header_map.insert("user-agent", USER_AGENT.parse()?);
    let resp = CLIENT
        .get(format!("{URL}{rid}"))
        .headers(header_map.clone())
        .send()
        .await?;
    header_map.insert("cookie", resp.headers().get("set-cookie").unwrap().clone());
    let resp = CLIENT
        .get(format!("{URL}{rid}"))
        .headers(header_map)
        .send()
        .await?;
    let resp_text = resp.text().await?;

    let re = Regex::new(r#"<script id="RENDER_DATA" type="application/json">([\s\S]*?)</script>"#)?;
    let re1 = Regex::new(r#""live-room-name">([\s\S]*?)</h1>"#)?;
    let json = decode(re.captures(&resp_text).unwrap().get(1).unwrap().as_str())?;
    let json: serde_json::Value = serde_json::from_str(&json)?;

    let room_info = &json["app"]["initialState"]["roomStore"]["roomInfo"];
    let title = room_info["room"]["title"].as_str().unwrap().to_string();
    match room_info["anchor"] {
        serde_json::Value::Null => Ok(ShowType::Error("直播间不存在".to_string())),
        _ => match &room_info["room"]["stream_url"] {
            Value::Null => Ok(ShowType::Off),
            stream_url => {
                let nodes = vec![
                    parse_url(douyin_trim_value(
                        stream_url["flv_pull_url"]["FULL_HD1"].as_str().unwrap(),
                    )),
                    parse_url(douyin_trim_value(
                        stream_url["hls_pull_url"].as_str().unwrap(),
                    )),
                ];
                Ok(ShowType::On(Detail::new(title, nodes)))
            }
        },
    }
}

/// 去除抖音third链接的多余参数
/// 目前遇到了 third 链接去除清晰度参数无法播放的情况, 所以删除去除逻辑
fn douyin_trim_value(url: &str) -> String {
    match url.contains("third") {
        true => {
            let url = url.split_once('?').unwrap().0;
            let re = Regex::new(r#"_[\s\S]*?\."#).unwrap();
            match re.captures(url) {
                Some(c) => url.replace(c.get(0).unwrap().as_str(), "."),
                None => url.to_owned(),
            }
        }
        false => url.to_owned(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_get_url() {
        println!("{}", get("93912444706").await.unwrap());
    }
}
