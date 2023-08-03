use std::collections::HashMap;

use async_trait::async_trait;

use crate::{
    common::CLIENT,
    error::{Result, SeamError},
    util::{hash2header, parse_url},
};

use super::{Live, Node};

const URL: &str = "https://api.flextv.co.kr/api/channels/rid/stream?option=all";

/// flextv
///
/// https://www.flextv.co.kr/
pub struct Client;

#[async_trait]
impl Live for Client {
    async fn get(&self, rid: &str, headers: &Option<&HashMap<String, String>>) -> Result<Node> {
        let json: serde_json::Value = CLIENT
            .get(URL.replace("rid", rid))
            .headers(hash2header(headers))
            .send()
            .await?
            .json()
            .await?;
        match &json["sources"][0]["url"] {
            serde_json::Value::Null => Err(SeamError::None),
            url => {
                let urls = vec![parse_url(url.as_str().unwrap().to_string())];
                Ok(Node {
                    rid: rid.to_owned(),
                    title: "flex".to_owned(),
                    urls,
                })
            }
        }
    }
}

#[cfg(test)]
macros::gen_test!(399291);
