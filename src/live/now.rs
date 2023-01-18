use anyhow::{Ok, Result};

use crate::{model::ShowType, common::CLIENT, util::parse_url};

const URL: &str = "https://now.qq.com/cgi-bin/now/web/room/get_live_room_url?platform=8&room_id=";

/// NOW直播
///
/// https://now.qq.com/
pub async fn get(rid: &str) -> Result<ShowType> {
    let json: serde_json::Value = CLIENT.get(format!("{URL}{rid}")).send().await?.json().await?;
    match &json["result"]["is_on_live"].as_bool().unwrap() {
        true => {
            let mut urls = vec![];
            for f in ["raw_flv_url", "raw_hls_url", "raw_rtmp_url"] {
                if let Some(url) = json["result"][f].as_str() {
                    urls.push(parse_url(url.to_string()));
                }
            }
            Ok(ShowType::On(urls))
        },
        false => Ok(ShowType::Off)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_get_url() {
        println!("{}", get("1347547853").await.unwrap());
    }
}