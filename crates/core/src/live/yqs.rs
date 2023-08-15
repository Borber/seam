use async_trait::async_trait;
use regex::Regex;

use crate::{
    common::CLIENT,
    error::{Result, SeamError},
    util::{hash2header, parse_url},
};

use std::collections::HashMap;

use super::{Live, Node};

const URL: &str = "https://www.173.com/";
const ROOM_URL: &str = "https://www.173.com/room/getVieoUrl";

/// 艺气山直播
///
/// https://www.173.com/
pub struct Client;

#[async_trait]
impl Live for Client {
    async fn get(&self, rid: &str, headers: Option<HashMap<String, String>>) -> Result<Node> {
        let mut params = HashMap::new();
        params.insert("roomId", rid);
        let resp: serde_json::Value = CLIENT
            .post(ROOM_URL)
            .form(&params)
            .headers(hash2header(headers.clone()))
            .send()
            .await?
            .json()
            .await?;
        let data = &resp["data"];
        match data["status"].as_i64() {
            Some(2) => {
                let urls = vec![parse_url(
                    data["url"]
                        .as_str()
                        .ok_or(SeamError::NeedFix("url"))?
                        .to_owned(),
                )];
                let title = match get_title(rid, headers).await {
                    Ok(title) => title,
                    Err(_) => "获取失败".to_owned(),
                };
                Ok(Node {
                    rid: rid.to_owned(),
                    title,
                    urls,
                })
            }
            _ => Err(SeamError::None),
        }
    }
}

// TODO 主播名和ID 需要 websocket 获取 就很离谱, 目前先获取标题
// TODO 异步同时请求
async fn get_title(rid: &str, headers: Option<HashMap<String, String>>) -> Result<String> {
    let resp = CLIENT
        .post(format!("{}{}", URL, rid))
        .headers(hash2header(headers))
        .send()
        .await?
        .text()
        .await?;

    let re = Regex::new(r"var room = JSON\.parse\('([\s\S]*?)'\);")?;

    let caps = re.captures(&resp).ok_or(SeamError::None)?;
    let data = caps.get(1).ok_or(SeamError::None)?.as_str();

    let data: serde_json::Value = serde_json::from_str(data)?;
    let title = data["name"].as_str().ok_or(SeamError::None)?;

    Ok(title.to_owned())
}

#[cfg(test)]
macros::gen_test!(96);
