use super::{Live, Node};
use crate::{
    common::CLIENT,
    error::{Result, SeamError},
    util::parse_url,
};
use async_trait::async_trait;
use regex::Regex;

const URL: &str = "https://cc.163.com/";

/// 网易CC直播
///
/// https://cc.163.com/
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
        let re = Regex::new(r#"<script id="__NEXT_DATA__" type="application/json" crossorigin="anonymous">([\s\S]*?)</script>"#).unwrap();
        let json = match re.captures(&text) {
            Some(rap) => rap.get(1).unwrap().as_str(),
            None => {
                return Err(SeamError::None);
            }
        };
        let json: serde_json::Value = serde_json::from_str(json)?;
        let resolution = match &json["props"]["pageProps"]["roomInfoInitData"]["live"]["quickplay"]
            ["resolution"]
        {
            serde_json::Value::Null => return Err(SeamError::None),
            v => v,
        };
        let title = json["props"]["pageProps"]["roomInfoInitData"]["live"]["title"]
            .as_str()
            .unwrap()
            .to_owned();
        let mut urls = vec![];
        for vbr in ["blueray", "ultra", "high", "standard"] {
            if resolution[vbr] != serde_json::Value::Null {
                if resolution[vbr]["cdn"]["ali"] != serde_json::Value::Null {
                    urls.push(parse_url(
                        resolution[vbr]["cdn"]["ali"].as_str().unwrap().to_string(),
                    ));
                }
                if resolution[vbr]["cdn"]["ks"] != serde_json::Value::Null {
                    urls.push(parse_url(
                        resolution[vbr]["cdn"]["ks"].as_str().unwrap().to_string(),
                    ));
                }
                break;
            }
        }
        Ok(Node {
            rid: rid.to_owned(),
            title,
            urls,
        })
    }
}

#[cfg(test)]
macros::gen_test!(361433);
