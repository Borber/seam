use std::collections::HashMap;

const URL: &str = "https://api.winktv.co.kr/v1/live/play";

use async_trait::async_trait;

use crate::{
    common::CLIENT,
    error::{Result, SeamError},
    util::{hash2header, parse_url},
};

use super::{Live, Node};

/// winktv
///
/// https://www.winktv.co.kr/
pub struct Client;

#[async_trait]
impl Live for Client {
    async fn get(&self, rid: &str, headers: Option<HashMap<String, String>>) -> Result<Node> {
        let mut form = HashMap::new();
        form.insert("action", "watch");
        form.insert("userId", rid);
        let json: serde_json::Value = CLIENT
            .post(URL)
            .form(&form)
            .headers(hash2header(headers))
            .send()
            .await?
            .json()
            .await?;
        match &json["PlayList"] {
            serde_json::Value::Null => Err(SeamError::None),
            list => {
                let mut urls = vec![];
                for item in ["hls", "hls2", "hls3", "rtmp"] {
                    if list.get(item).is_some() {
                        urls.push(parse_url(
                            list[item][0]["url"]
                                .as_str()
                                .ok_or(SeamError::NeedFix("url"))?
                                .to_string(),
                        ));
                    }
                }

                let title = match &json["media"]["title"] {
                    serde_json::Value::Null => "获取失败",
                    title => title.as_str().unwrap_or("获取失败"),
                };

                Ok(Node {
                    rid: rid.to_owned(),
                    title: title.to_owned(),
                    cover: "".to_owned(),
                    anchor: "".to_owned(),
                    head: "".to_owned(),
                    urls,
                })
            }
        }
    }
}

#[cfg(test)]
macros::gen_test!(roeunlove);
