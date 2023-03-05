use async_trait::async_trait;
use regex::Regex;
use reqwest::header::HeaderMap;

use crate::{
    common::{CLIENT, USER_AGENT},
    error::{Result, SeamError},
    util::parse_url,
};

use super::{Live, Node};

const URL: &str = "https://live.kuaishou.com/u/";

/// 快手直播
///
/// https://live.kuaishou.com/
pub struct Ks;

#[async_trait]
impl Live for Ks {
    async fn get(rid: &str) -> Result<Node> {
        let mut header_map = HeaderMap::new();
        header_map.insert("user-agent", USER_AGENT.parse()?);
        // TODO 需要保存cookie 避免快速请求
        let resp = CLIENT
            .get(format!("{URL}{rid}"))
            .headers(header_map.clone())
            .send()
            .await?;
        let cookie = resp
            .headers()
            .get_all("set-cookie")
            .iter()
            .map(|x| x.to_str().unwrap().to_string())
            .collect::<Vec<String>>()
            .join("; ")
            .to_string();
        header_map.insert("cookie", cookie.parse()?);
        let text = CLIENT
            .get(format!("{URL}{rid}"))
            .send()
            .await?
            .text()
            .await?;
        let re = Regex::new(r#"<script>window.__INITIAL_STATE__=([\s\S]*?);\(function"#).unwrap();
        let stream = match re.captures(&text) {
            Some(caps) => caps.get(1).unwrap().as_str(),
            None => {
                return Err(SeamError::None);
            }
        };
        let json: serde_json::Value = serde_json::from_str(stream).unwrap();
        // TODO 更改其他逻辑 多用Null
        match &json["liveroom"]["liveStream"]["playUrls"][0]["adaptationSet"]["representation"] {
            serde_json::Value::Null => Err(SeamError::None),
            reps => {
                let list = reps.as_array().unwrap();
                let url = list[list.len() - 1]["url"].as_str().unwrap();
                let urls = vec![parse_url(url.to_string())];
                // TODO 实现标题获取
                Ok(Node {
                    rid: rid.to_owned(),
                    title: "kuaishou".to_owned(),
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
    async fn test_kuaishou() {
        match Ks::get("3xgexgpig9gwwi2").await {
            Ok(node) => {
                println!("{}", node.json())
            }
            _ => println!("未开播"),
        }
    }
}
