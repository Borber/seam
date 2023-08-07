use std::collections::HashMap;

use async_trait::async_trait;
use regex::Regex;

use crate::{
    common::CLIENT,
    error::{Result, SeamError},
    util::{hash2header, parse_url},
};

use super::{Live, Node};

const URL: &str = "https://qf.56.com/";

/// 千帆直播
///
/// https://qf.56.com/
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
        let re_title = Regex::new(r"nickName: '(.+)'")?;
        let title = match re_title.captures(&text) {
            Some(cap) => cap
                .get(1)
                .ok_or(SeamError::NeedFix("title"))?
                .as_str()
                .to_owned(),
            None => "qf".to_owned(),
        };
        let re = Regex::new(r"flvUrl:'([\s\S]*?)'")?;
        match re.captures(&text) {
            Some(cap) => {
                let urls = vec![parse_url(
                    cap.get(1)
                        .ok_or(SeamError::NeedFix("captures"))?
                        .as_str()
                        .to_string(),
                )];
                Ok(Node {
                    rid: rid.to_owned(),
                    title,
                    urls,
                })
            }
            None => Err(SeamError::None),
        }
    }
}

#[cfg(test)]
macros::gen_test!(520006);
