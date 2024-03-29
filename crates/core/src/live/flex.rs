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
    async fn get(&self, rid: &str, headers: Option<HashMap<String, String>>) -> Result<Node> {
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
                let urls = vec![parse_url(
                    url.as_str().ok_or(SeamError::NeedFix("url"))?.to_string(),
                )];

                let title = match &json["stream"]["title"] {
                    serde_json::Value::Null => "获取失败",
                    title => title.as_str().unwrap_or("获取失败"),
                };

                Ok(Node {
                    rid: rid.to_owned(),
                    title: title.to_owned(),
                    cover: "".to_owned(),
                    anchor: "".to_owned(),
                    head: "".to_owned(),
                    urls,
                })
            }
        }
    }
}

#[cfg(test)]
macros::gen_test!(437149);
