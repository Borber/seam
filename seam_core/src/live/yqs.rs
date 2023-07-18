use async_trait::async_trait;

use crate::{
    common::CLIENT,
    error::{Result, SeamError},
    util::{hash2header, parse_url},
};

use std::collections::HashMap;

use super::{Live, Node};

const URL: &str = "https://www.173.com/room/getVieoUrl";

/// 艺气山直播
///
/// https://www.173.com/
pub struct Client;

#[async_trait]
impl Live for Client {
    async fn get(&self, rid: &str, headers: Option<HashMap<String, String>>) -> Result<Node> {
        let mut params = HashMap::new();
        params.insert("roomId", rid);
        let resp: serde_json::Value = CLIENT
            .post(URL)
            .form(&params)
            .headers(hash2header(headers))
            .send()
            .await?
            .json()
            .await?;
        let data = &resp["data"];
        match data["status"].as_i64() {
            Some(2) => {
                let urls = vec![parse_url(data["url"].as_str().unwrap().to_owned())];
                Ok(Node {
                    rid: rid.to_owned(),
                    title: "yqs".to_owned(),
                    urls,
                })
            }
            _ => Err(SeamError::None),
        }
    }
}

#[cfg(test)]
macros::gen_test!(96);
