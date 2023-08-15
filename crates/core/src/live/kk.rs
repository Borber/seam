use std::collections::HashMap;

use async_trait::async_trait;
use regex::Regex;

use crate::{
    common::CLIENT,
    error::{Result, SeamError},
    util::{hash2header, parse_url},
};

use super::{Live, Node};

const URL: &str = "https://www.kktv5.com/show/";

/// kk直播
///
/// https://www.kktv5.com/
pub struct Client;

// TODO 简化后半部分逻辑, 仅判断开播与标题, 最后拼接node
#[async_trait]
impl Live for Client {
    async fn get(&self, rid: &str, headers: Option<HashMap<String, String>>) -> Result<Node> {
        let text = CLIENT
            .get(format!("{URL}{rid}"))
            .headers(hash2header(headers))
            .send()
            .await?
            .text()
            .await?;

        let re = Regex::new(r"window.acotor_simple_info = ([\s\S]*?);")?;
        let re2 = Regex::new(r"var __actor_info__ = ([\s\S]*?) var")?;

        let node1 = match re.captures(&text) {
            Some(cap) => {
                let json = cap.get(1).ok_or(SeamError::NeedFix("captures"))?.as_str();
                let json = serde_json::from_str::<serde_json::Value>(json)?;

                let title = json["roomTheme"].as_str().unwrap_or("获取失败").to_owned();

                let live = json["liveType"]
                    .as_i64()
                    .ok_or(SeamError::NeedFix("liveType"))?;

                match live {
                    1 => {
                        let urls = vec![parse_url(format!(
                            "https://pull.kktv8.com/livekktv/{}.flv",
                            rid
                        ))];
                        Some(Node {
                            rid: rid.to_owned(),
                            title,
                            urls,
                        })
                    }
                    _ => None,
                }
            }
            None => None,
        };
        let node2 = match re2.captures(&text) {
            Some(cap) => {
                let json = cap.get(1).ok_or(SeamError::NeedFix("captures"))?.as_str();
                let json = serde_json::from_str::<serde_json::Value>(json)?;

                let title = json["roomTheme"].as_str().unwrap_or("获取失败").to_owned();

                let live = json["liveType"]
                    .as_i64()
                    .ok_or(SeamError::NeedFix("liveType"))?;

                match live {
                    1 => {
                        let urls = vec![parse_url(format!(
                            "https://pull.kktv8.com/livekktv/{}.flv",
                            rid
                        ))];
                        Some(Node {
                            rid: rid.to_owned(),
                            title,
                            urls,
                        })
                    }
                    _ => None,
                }
            }
            None => None,
        };
        if let Some(node) = node1 {
            return Ok(node);
        }
        if let Some(node) = node2 {
            return Ok(node);
        }
        Err(SeamError::None)
    }
}

#[cfg(test)]
macros::gen_test!(521);
