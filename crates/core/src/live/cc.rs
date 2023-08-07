use std::collections::HashMap;

use super::{Live, Node};
use crate::{
    common::CLIENT,
    error::{Result, SeamError},
    util::{hash2header, parse_url},
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
    async fn get(&self, rid: &str, headers: &Option<&HashMap<String, String>>) -> Result<Node> {
        let text = CLIENT
            .get(format!("{URL}{rid}"))
            .headers(hash2header(headers))
            .send()
            .await?
            .text()
            .await?;
        let re = Regex::new(
            r#"<script id="__NEXT_DATA__" type="application/json" crossorigin="anonymous">([\s\S]*?)</script>"#,
        )?;
        let json = match re.captures(&text) {
            Some(rap) => rap.get(1).ok_or(SeamError::None)?.as_str(),
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
            .unwrap_or("获取失败")
            .to_owned();
        let mut urls = vec![];
        for vbr in ["blueray", "ultra", "high", "standard"] {
            if resolution[vbr] != serde_json::Value::Null {
                if resolution[vbr]["cdn"]["ali"] != serde_json::Value::Null {
                    urls.push(parse_url(
                        resolution[vbr]["cdn"]["ali"]
                            .as_str()
                            .ok_or(SeamError::None)?
                            .to_string(),
                    ));
                }
                if resolution[vbr]["cdn"]["ks"] != serde_json::Value::Null {
                    urls.push(parse_url(
                        resolution[vbr]["cdn"]["ks"]
                            .as_str()
                            .ok_or(SeamError::None)?
                            .to_string(),
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
