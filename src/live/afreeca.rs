use std::collections::HashMap;

use anyhow::{Ok, Result};
use regex::Regex;

use crate::{
    common::CLIENT,
    default_danmu_client,
    model::{Detail, ShowType},
    util::parse_url,
};

const URL: &str = "https://play.afreecatv.com/";
const PLAY_URL: &str = "https://live.afreecatv.com/afreeca/player_live_api.php?bjid=";
const CDN: &str = "https://live-global-cdn-v02.afreecatv.com/live-stmc-32/auth_playlist.m3u8?aid=";

default_danmu_client!(Afreeca);

/// afreecatv直播
///
/// https://www.afreecatv.com/
pub async fn get(rid: &str) -> Result<ShowType> {
    let text = CLIENT
        .get(format!("{URL}{rid}"))
        .send()
        .await?
        .text()
        .await?;
    let re = Regex::new(r#"var nBroadNo = ([0-9]{9})"#).unwrap();
    let bno = match re.captures(&text) {
        Some(rap) => rap.get(1).unwrap().as_str(),
        None => {
            return Ok(ShowType::Error("未找到直播间".to_string()));
        }
    };
    let mut form = HashMap::new();
    form.insert("bid", rid);
    form.insert("bno", bno);
    form.insert("mode", "landing");
    form.insert("player_type", "html5");
    form.insert("stream_type", "common");
    form.insert("from_api", "0");
    form.insert("type", "aid");
    form.insert("quality", "original");
    let json: serde_json::Value = CLIENT
        .post(format!("{PLAY_URL}{rid}"))
        .form(&form)
        .send()
        .await?
        .json()
        .await?;
    let nodes = vec![parse_url(format!(
        "{CDN}{}",
        json["CHANNEL"]["AID"].as_str().unwrap()
    ))];
    Ok(ShowType::On(Detail::new("afreeca".to_owned(), nodes)))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_get() {
        println!("{}", get("dasl8121").await.unwrap());
    }
}
