use std::collections::HashMap;

use async_trait::async_trait;
use regex::Regex;

use crate::{
    common::CLIENT,
    error::{Result, SeamError},
    util::{hash2header, parse_url},
};

use super::{Live, Node};

const URL: &str = "https://live.kuaishou.com/u/";

/// 快手直播
///
/// https://live.kuaishou.com/
pub struct Client;

#[async_trait]
impl Live for Client {
    // TODO 说明所需 cookie
    async fn get(&self, rid: &str, headers: &Option<&HashMap<String, String>>) -> Result<Node> {
        // TODO 提取到 cookie 中
        // let mut header_map = HeaderMap::new();
        // header_map.insert("user-agent", USER_AGENT.parse()?);
        // TODO 需要保存cookie 避免快速请求
        // let resp = CLIENT
        //     .get(format!("{URL}{rid}"))
        //     .headers(header_map.clone())
        //     .send()
        //     .await?;
        // let cookie = resp
        //     .headers()
        //     .get_all("set-cookie")
        //     .iter()
        //     .map(|x| x.to_str().unwrap().to_string())
        //     .collect::<Vec<String>>()
        //     .join("; ")
        //     .to_string();
        // header_map.insert("cookie", cookie.parse()?);
        let text = CLIENT
            .get(format!("{URL}{rid}"))
            .headers(hash2header(headers))
            .send()
            .await?
            .text()
            .await?;
        let re = Regex::new(r"<script>window.__INITIAL_STATE__=([\s\S]*?);\(function")?;
        let stream = match re.captures(&text) {
            Some(caps) => caps.get(1).ok_or(SeamError::NeedFix("stream"))?.as_str(),
            None => {
                return Err(SeamError::NeedFix("stream none"));
            }
        };
        let json: serde_json::Value = serde_json::from_str(stream)?;
        // TODO 更改其他逻辑 多用Null
        // TODO 此处是否有未覆盖到的情况
        match &json["liveroom"]["liveStream"]["playUrls"][0]["adaptationSet"]["representation"] {
            serde_json::Value::Null => Err(SeamError::None),
            reps => {
                let list = reps.as_array().ok_or(SeamError::NeedFix("list"))?;
                let url = list[list.len() - 1]["url"]
                    .as_str()
                    .ok_or(SeamError::NeedFix("url"))?;
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
macros::gen_test!(3xgexgpig9gwwi2);
