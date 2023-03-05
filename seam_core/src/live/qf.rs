use async_trait::async_trait;
use regex::Regex;

use crate::{
    common::CLIENT,
    error::{Result, SeamError},
    util::parse_url,
};

use super::{Live, Node};

const URL: &str = "https://qf.56.com/";

/// 千帆直播
///
/// https://qf.56.com/
pub struct Qf;

#[async_trait]
impl Live for Qf {
    async fn get(rid: &str) -> Result<Node> {
        let text = CLIENT
            .get(format!("{URL}{rid}"))
            .send()
            .await?
            .text()
            .await?;
        let re = Regex::new(r#"flvUrl:'([\s\S]*?)'"#).unwrap();
        // TODO title
        match re.captures(&text) {
            Some(cap) => {
                let urls = vec![parse_url(cap.get(1).unwrap().as_str().to_string())];
                Ok(Node {
                    rid: rid.to_owned(),
                    title: "qf".to_owned(),
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
        match Qf::get("520006").await {
            Ok(node) => {
                println!("{}", node.json())
            }
            _ => println!("未开播"),
        }
    }
}
