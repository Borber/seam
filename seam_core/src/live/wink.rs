use std::collections::HashMap;

const URL: &str = "https://api.winktv.co.kr/v1/live/play";

use async_trait::async_trait;

use crate::{
    common::CLIENT,
    error::{Result, SeamError},
    util::parse_url,
};

use super::{Live, Node};

/// winktv
///
/// https://www.winktv.co.kr/
pub struct Client;

#[async_trait]
impl Live for Client {
    async fn get(&self, rid: &str) -> Result<Node> {
        let mut form = HashMap::new();
        form.insert("action", "watch");
        form.insert("userId", rid);
        let json: serde_json::Value = CLIENT.post(URL).form(&form).send().await?.json().await?;
        match &json["PlayList"] {
            serde_json::Value::Null => Err(SeamError::None),
            list => {
                let mut urls = vec![];
                for item in ["hls", "hls2", "hls3", "rtmp"] {
                    if list.get(item).is_some() {
                        urls.push(parse_url(
                            list[item][0]["url"].as_str().unwrap().to_string(),
                        ));
                    }
                }
                Ok(Node {
                    rid: rid.to_owned(),
                    title: "wink".to_owned(),
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
    async fn test_panda() {
        let cli = Client;
        match cli.get("csp1208").await {
            Ok(node) => println!("{}", node.json()),
            Err(e) => println!("{e}"),
        }
    }
}
