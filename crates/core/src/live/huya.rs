use std::collections::HashMap;

use async_trait::async_trait;
use regex::Regex;

use crate::{
    common::CLIENT,
    error::{Result, SeamError},
    util::{hash2header, parse_url},
};

use super::{Live, Node};

const URL: &str = "https://www.huya.com/";

/// 虎牙直播
///
/// https://huya.com/
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
        let re = Regex::new(r"stream:([\s\S]*)window.TT_LIVE_TIMING")?;
        let stream = match re.captures(&text) {
            Some(caps) => {
                caps.get(1)
                    .ok_or(SeamError::None)?
                    .as_str()
                    .rsplit_once('}')
                    .ok_or(SeamError::None)?
                    .0
            }
            None => return Err(SeamError::None),
        };
        let re = Regex::new(r#"class="host-title" title="[\s\S]*?>([\s\S]*?)</h1>"#)?;
        let title = match re.captures(&text) {
            Some(caps) => caps.get(1).ok_or(SeamError::None)?.as_str(),
            None => "huya",
        };
        let json: serde_json::Value = serde_json::from_str(stream)?;
        let mut urls = vec![];
        match json["data"][0]["gameStreamInfoList"]
            .as_array()
            .ok_or(SeamError::None)?
        {
            list if list.is_empty() => return Err(SeamError::None),
            list => {
                for cdn in list {
                    urls.push(parse_url(format!(
                        "{}/{}.flv?{}",
                        cdn["sFlvUrl"].as_str().ok_or(SeamError::None)?,
                        cdn["sStreamName"].as_str().ok_or(SeamError::None)?,
                        cdn["sFlvAntiCode"].as_str().ok_or(SeamError::None)?
                    )));
                    urls.push(parse_url(format!(
                        "{}/{}.m3u8?{}",
                        cdn["sHlsUrl"].as_str().ok_or(SeamError::None)?,
                        cdn["sStreamName"].as_str().ok_or(SeamError::None)?,
                        cdn["sHlsAntiCode"].as_str().ok_or(SeamError::None)?
                    )));
                }
            }
        }
        Ok(Node {
            rid: rid.to_owned(),
            title: title.to_owned(),
            urls,
        })
    }
}

#[cfg(test)]
macros::gen_test!(880201);
