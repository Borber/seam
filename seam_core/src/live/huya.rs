use async_trait::async_trait;
use regex::Regex;

use crate::{
    common::CLIENT,
    error::{Result, SeamError},
    util::parse_url,
};

use super::{Live, Node};

const URL: &str = "https://www.huya.com/";

/// 虎牙直播
///
/// https://huya.com/
pub struct Client;

#[async_trait]
impl Live for Client {
    async fn get(&self, rid: &str) -> Result<Node> {
        let text = CLIENT
            .get(format!("{URL}{rid}"))
            .send()
            .await?
            .text()
            .await?;
        let re = Regex::new(r"stream:([\s\S]*)window.TT_LIVE_TIMING").unwrap();
        let stream = match re.captures(&text) {
            Some(caps) => caps.get(1).unwrap().as_str().rsplit_once('}').unwrap().0,
            None => return Err(SeamError::None),
        };
        let re = Regex::new(r#"class="host-title" title="[\s\S]*?>([\s\S]*?)</h1>"#).unwrap();
        let title = match re.captures(&text) {
            Some(caps) => caps.get(1).unwrap().as_str(),
            None => "huya",
        };
        let json: serde_json::Value = serde_json::from_str(stream).unwrap();
        let mut urls = vec![];
        match json["data"][0]["gameStreamInfoList"].as_array().unwrap() {
            list if list.is_empty() => return Err(SeamError::None),
            list => {
                for cdn in list {
                    urls.push(parse_url(format!(
                        "{}/{}.flv?{}",
                        cdn["sFlvUrl"].as_str().unwrap(),
                        cdn["sStreamName"].as_str().unwrap(),
                        cdn["sFlvAntiCode"].as_str().unwrap()
                    )));
                    urls.push(parse_url(format!(
                        "{}/{}.m3u8?{}",
                        cdn["sHlsUrl"].as_str().unwrap(),
                        cdn["sStreamName"].as_str().unwrap(),
                        cdn["sHlsAntiCode"].as_str().unwrap()
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
