use std::collections::HashMap;

use async_trait::async_trait;
use regex::Regex;

use crate::{
    common::CLIENT,
    error::{Result, SeamError},
    util::{hash2header, parse_url},
};

use super::{Live, Node};

const URL: &str = "https://play.afreecatv.com/";
const PLAY_URL: &str = "https://live.afreecatv.com/afreeca/player_live_api.php?bjid=";
const CDN: &str = "https://live-global-cdn-v02.afreecatv.com/live-stmc-32/auth_playlist.m3u8?aid=";

/// afreecatv直播
///
/// https://www.afreecatv.com/
pub struct Client;

#[async_trait]
impl Live for Client {
    async fn get(&self, rid: &str, headers: &Option<&HashMap<String, String>>) -> Result<Node> {
        let text = CLIENT
            .get(format!("{URL}{rid}"))
            .headers(hash2header(headers))
            .send()
            .await?
            .text()
            .await?;
        let re = Regex::new(r#"var nBroadNo = ([0-9]{9})"#).unwrap();
        let bno = match re.captures(&text) {
            Some(rap) => rap.get(1).unwrap().as_str(),
            None => {
                return Err(SeamError::None);
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
        let urls = vec![parse_url(format!(
            "{CDN}{}",
            json["CHANNEL"]["AID"].as_str().unwrap()
        ))];
        Ok(Node {
            rid: rid.to_owned(),
            title: "afreeca".to_owned(),
            urls,
        })
    }
}

#[cfg(test)]
macros::gen_test!(dasl8121);
