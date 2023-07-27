use std::collections::HashMap;

use async_trait::async_trait;

use crate::{
    common::CLIENT,
    error::{Result, SeamError},
    util::{hash2header, parse_url},
};

use super::{Live, Node};

const URL: &str = "https://webapi.busi.inke.cn/web/live_share_pc?uid=";

/// 映客直播
///
/// https://www.inke.cn/
pub struct Client;

#[async_trait]
impl Live for Client {
    async fn get(&self, rid: &str, headers: &Option<HashMap<String, String>>) -> Result<Node> {
        let json: serde_json::Value = CLIENT
            .get(format!("{URL}{rid}"))
            .headers(hash2header(headers))
            .send()
            .await?
            .json()
            .await?;

        match &json["data"]["status"].as_i64() {
            Some(1) => {
                let title = json["data"]["live_name"]
                    .as_str()
                    .unwrap_or("inke")
                    .to_string();
                let mut urls = vec![];
                for s in ["stream_addr", "hls_stream_addr", "rtmp_stream_addr"] {
                    if !json["data"]["live_addr"][0][s].is_null() {
                        urls.push(parse_url(
                            json["data"]["live_addr"][0][s]
                                .as_str()
                                .unwrap()
                                .to_string(),
                        ));
                    }
                }

                Ok(Node {
                    rid: rid.to_owned(),
                    title,
                    urls,
                })
            }
            _ => Err(SeamError::None),
        }
    }
}

#[cfg(test)]
macros::gen_test!(713935849);
