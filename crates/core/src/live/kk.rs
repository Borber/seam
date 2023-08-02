use std::collections::HashMap;

use async_trait::async_trait;
use regex::Regex;

use crate::{
    common::CLIENT,
    error::{Result, SeamError},
    util::{hash2header, parse_url},
};

use super::{Live, Node};

const URL: &str = "https://sgapi.kktv8.com/roomApi/room/roomVideoBitrate?roomId=";

/// kk直播
///
/// https://www.kktv5.com/
pub struct Client;

#[async_trait]
impl Live for Client {
    async fn get(&self, rid: &str, headers: &Option<HashMap<String, String>>) -> Result<Node> {
        let text = CLIENT
            .get(format!("{URL}{rid}"))
            .headers(hash2header(headers))
            .send()
            .await?
            .text()
            .await?;
        // TODO 需要额外请求获取title
        let re = Regex::new(r"http[\s\S]*?flv").unwrap();
        match re.captures(&text) {
            Some(cap) => {
                let urls = vec![parse_url(cap.get(0).unwrap().as_str().to_string())];
                Ok(Node {
                    rid: rid.to_owned(),
                    title: "kk".to_owned(),
                    urls,
                })
            }
            None => Err(SeamError::None),
        }
    }
}

#[cfg(test)]
macros::gen_test!(521);
