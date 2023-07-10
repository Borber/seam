use async_trait::async_trait;

use crate::{
    common::CLIENT,
    error::{Result, SeamError},
    util::parse_url,
};

use super::{Live, Node};

const URL: &str = "https://now.qq.com/cgi-bin/now/web/room/get_live_room_url?platform=8&room_id=";

/// NOW直播
///
/// https://now.qq.com/
pub struct Client;

#[async_trait]
impl Live for Client {
    async fn get(&self, rid: &str) -> Result<Node> {
        let json: serde_json::Value = CLIENT
            .get(format!("{URL}{rid}"))
            .send()
            .await?
            .json()
            .await?;
        match &json["result"]["is_on_live"].as_bool().unwrap() {
            true => {
                let mut urls = vec![];
                for f in ["raw_flv_url", "raw_hls_url", "raw_rtmp_url"] {
                    if let Some(url) = json["result"][f].as_str() {
                        urls.push(parse_url(url.to_string()));
                    }
                }
                Ok(Node {
                    rid: rid.to_owned(),
                    title: "now".to_owned(),
                    urls,
                })
            }
            false => Err(SeamError::None),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_get_url() {
        let cli = Client;
        match cli.get("1347547853").await {
            Ok(node) => println!("{}", node.json()),
            Err(e) => println!("{e}"),
        }
    }
}
