use crate::{
    common::CLIENT,
    default_danmu_client,
    model::{Detail, ShowType},
    util::parse_url,
};

use anyhow::{Ok, Result};
use regex::Regex;
use serde_json::Value;
use urlencoding::decode;

const URL: &str = "https://live.douyin.com/";

default_danmu_client!(Douyin);

/// 抖音直播
///
/// https://live.douyin.com/
pub async fn get(rid: &str) -> Result<ShowType> {
    let resp = CLIENT
        .get(format!("{URL}{rid}"))
        .header("cookie", "__ac_nonce=063b59ce0001243a9217f;")
        .send()
        .await?;
    let resp_text = resp.text().await?;
    let re = Regex::new(r#"<script id="RENDER_DATA" type="application/json">([\s\S]*?)</script>"#)?;
    let json = decode(re.captures(&resp_text).unwrap().get(1).unwrap().as_str())?;
    let json: serde_json::Value = serde_json::from_str(&json)?;
    let room_info = &json["app"]["initialState"]["roomStore"]["roomInfo"];
    match room_info["anchor"] {
        serde_json::Value::Null => Ok(ShowType::Error("直播间不存在".to_string())),
        _ => match &room_info["room"]["stream_url"] {
            Value::Null => Ok(ShowType::Off),
            stream_url => {
                let nodes = vec![
                    parse_url(douyin_trim_value(&stream_url["flv_pull_url"]["FULL_HD1"])),
                    parse_url(douyin_trim_value(&stream_url["hls_pull_url"])),
                ];
                Ok(ShowType::On(Detail::new("douyin".to_owned(), nodes)))
            }
        },
    }
}

/// 去除抖音返回链接的多余引号和暂时无用的参数简化链接
fn douyin_trim_value(v: &Value) -> String {
    v.to_string()
        .trim_matches('"')
        .split_once('?')
        .unwrap()
        .0
        .to_owned()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_get_url() {
        println!("{}", get("228619203678").await.unwrap());
    }
}
