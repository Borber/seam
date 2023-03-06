use async_trait::async_trait;

use crate::{
    common::CLIENT,
    error::{Result, SeamError},
    util::parse_url,
};

use std::collections::HashMap;

use super::{Live, Node};

const URL: &str = "https://www.173.com/room/getVieoUrl";

/// 艺气山直播
///
/// https://www.173.com/
pub struct Yqs;

#[async_trait]
impl Live for Yqs {
    async fn get(rid: &str) -> Result<Node> {
        let mut params = HashMap::new();
        params.insert("roomId", rid);
        let resp: serde_json::Value = CLIENT.post(URL).form(&params).send().await?.json().await?;
        let data = &resp["data"];
        match data["status"].to_string().parse::<usize>()? {
            2 => {
                let urls = vec![parse_url(data["url"].to_string())];
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
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_get_url() {
        match Yqs::get("100000").await {
            Ok(node) => println!("{}", node.json()),
            Err(e) => println!("{e}"),
        }
    }
}
