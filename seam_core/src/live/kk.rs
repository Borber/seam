use async_trait::async_trait;
use regex::Regex;

use crate::{
    common::CLIENT,
    error::{Result, SeamError},
    util::parse_url,
};

use super::{Live, Node};

const URL: &str = "https://sgapi.kktv8.com/roomApi/room/roomVideoBitrate?roomId=";

/// kk直播
///
/// https://www.kktv5.com/
pub struct Client;

#[async_trait]
impl Live for Client {
    async fn get(rid: &str) -> Result<Node> {
        let text = CLIENT
            .get(format!("{URL}{rid}"))
            .send()
            .await?
            .text()
            .await?;
        let re = Regex::new(r#"http[\s\S]*?flv"#).unwrap();
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
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_get_url() {
        match Client::get("521").await {
            Ok(node) => println!("{}", node.json()),
            Err(e) => println!("{e}"),
        }
    }
}
