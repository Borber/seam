use async_trait::async_trait;

use crate::{
    common::CLIENT,
    error::{Result, SeamError},
    util::parse_url,
};

use super::{Live, Node};

const URL: &str = "https://api.flextv.co.kr/api/channels/rid/stream?option=all";

/// flextv
///
/// https://www.flextv.co.kr/
pub struct Flex;

#[async_trait]
impl Live for Flex {
    async fn get(rid: &str) -> Result<Node> {
        let json: serde_json::Value = CLIENT
            .get(URL.replace("rid", rid))
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
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_flex() {
        match Flex::get("399291").await {
            Ok(node) => println!("{}", node.json()),
            Err(e) => println!("{e}"),
        }
    }
}
