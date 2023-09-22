use std::collections::HashMap;

use async_trait::async_trait;
use regex::Regex;

use crate::{
    common::CLIENT,
    error::{Result, SeamError},
    util::{hash2header, parse_url},
};

use super::{Live, Node};

const URL: &str = "https://live.kuaishou.com/u/";

/// 快手直播
///
/// https://live.kuaishou.com/
pub struct Client;

#[async_trait]
impl Live for Client {
    // TODO 说明所需 cookie
    async fn get(&self, rid: &str, headers: Option<HashMap<String, String>>) -> Result<Node> {
        let text = CLIENT
            .get(format!("{URL}{rid}"))
            .headers(hash2header(headers))
            .header("User-Agent", "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/117.0.0.0 Safari/537.36 Edg/117.0.2045.31")
            .send()
            .await?
            .text()
            .await?;
        let re = Regex::new(r"<script>window.__INITIAL_STATE__=([\s\S]*?);\(function")?;
        let stream = match re.captures(&text) {
            Some(caps) => caps.get(1).ok_or(SeamError::NeedFix("stream"))?.as_str(),
            None => {
                return Err(SeamError::NeedFix("stream none"));
            }
        };
        let json: serde_json::Value = serde_json::from_str(stream)?;

        let title = json["liveroom"]["playList"][0]["liveStream"]["caption"]
            .as_str()
            .unwrap_or("获取失败")
            .to_owned();

        let cover = json["liveroom"]["playList"][0]["liveStream"]["poster"]
            .as_str()
            .unwrap_or("")
            .to_owned();

        let head = json["liveroom"]["playList"][0]["author"]["avatar"]
            .as_str()
            .unwrap_or("")
            .to_owned();

        let anchor = json["liveroom"]["playList"][0]["author"]["name"]
            .as_str()
            .unwrap_or("获取失败")
            .to_owned();

        match &json["liveroom"]["playList"][0]["liveStream"]["playUrls"][0]["adaptationSet"]
            ["representation"]
        {
            serde_json::Value::Null => Err(SeamError::None),
            reps => {
                let list = reps.as_array().ok_or(SeamError::NeedFix("list"))?;
                let url = list[list.len() - 1]["url"]
                    .as_str()
                    .ok_or(SeamError::NeedFix("url"))?;
                let urls = vec![parse_url(url.to_string())];
                Ok(Node {
                    rid: rid.to_owned(),
                    title,
                    cover,
                    anchor,
                    head,
                    urls,
                })
            }
        }
    }
}

#[cfg(test)]
macros::gen_test!(Bd20210915);
