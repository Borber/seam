use std::collections::HashMap;

use async_trait::async_trait;

use crate::{
    common::CLIENT,
    error::{Result, SeamError},
    util::{hash2header, parse_url},
};

use super::{Live, Node};

const ROOM_URL: &str =
    "https://now.qq.com/cgi-bin/now/web/room/get_live_room_url?platform=8&room_id=";
const URL: &str = "https://now.qq.com/pcweb/story.html?roomid=";
/// NOW直播
///
/// https://now.qq.com/
pub struct Client;

#[async_trait]
impl Live for Client {
    async fn get(&self, rid: &str, headers: Option<HashMap<String, String>>) -> Result<Node> {
        let json: serde_json::Value = CLIENT
            .get(format!("{ROOM_URL}{rid}"))
            .headers(hash2header(headers.clone()))
            .send()
            .await?
            .json()
            .await?;

        match &json["result"]["is_on_live"]
            .as_bool()
            .ok_or(SeamError::NeedFix("result"))?
        {
            true => {
                let mut urls = vec![];
                for f in ["raw_flv_url", "raw_hls_url", "raw_rtmp_url"] {
                    if let Some(url) = json["result"][f].as_str() {
                        urls.push(parse_url(url.to_string()));
                    }
                }
                let title = get_title(rid, headers)
                    .await
                    .unwrap_or("获取失败".to_owned());
                Ok(Node {
                    rid: rid.to_owned(),
                    title,
                    urls,
                })
            }
            false => Err(SeamError::None),
        }
    }
}

async fn get_title(rid: &str, headers: Option<HashMap<String, String>>) -> Result<String> {
    let json = CLIENT
        .get(format!("{URL}{rid}"))
        .headers(hash2header(headers))
        .send()
        .await?
        .text()
        .await?;

    let re = regex::Regex::new(r#""anchorName":"([\s\S]*?)""#).unwrap();

    match re.captures(&json) {
        Some(caps) => {
            let title = caps
                .get(1)
                .ok_or(SeamError::NeedFix("captures"))?
                .as_str()
                .to_owned();
            Ok(title)
        }
        None => Err(SeamError::NeedFix("title")),
    }
}

#[cfg(test)]
macros::gen_test!(1351697153);
