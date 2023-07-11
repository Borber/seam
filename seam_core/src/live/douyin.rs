use async_trait::async_trait;
use regex::Regex;
use reqwest::header::HeaderMap;
use serde_json::Value;
use urlencoding::decode;

use crate::{
    common::{CLIENT, USER_AGENT},
    error::{Result, SeamError},
    util::parse_url,
};

use super::{Live, Node};

const URL: &str = "https://live.douyin.com/";

/// 抖音直播
///
/// https://live.douyin.com/
pub struct Client;

#[async_trait]
impl Live for Client {
    async fn get(rid: &str) -> Result<Node> {
        let mut header_map = HeaderMap::new();
        // 更新 cookie
        header_map.insert("user-agent", USER_AGENT.parse()?);
        let resp = CLIENT
            .get(format!("{URL}{rid}"))
            .headers(header_map.clone())
            .send()
            .await?;
        header_map.insert("cookie", resp.headers().get("set-cookie").unwrap().clone());
        // 通过网页内容获取直播地址
        let resp = CLIENT
            .get(format!("{URL}{rid}"))
            .headers(header_map)
            .send()
            .await?;
        let resp_text = resp.text().await?;

        let re =
            Regex::new(r#"<script id="RENDER_DATA" type="application/json">([\s\S]*?)</script>"#)?;
        let json = decode(re.captures(&resp_text).unwrap().get(1).unwrap().as_str())?;
        let json: serde_json::Value = serde_json::from_str(&json)?;

        let room_info = &json["app"]["initialState"]["roomStore"]["roomInfo"];
        match room_info["anchor"] {
            // 主播不存在
            serde_json::Value::Null => Err(SeamError::None),
            _ => match &room_info["room"]["stream_url"] {
                // 未开播
                Value::Null => Err(SeamError::None),
                stream_url => {
                    let title = room_info["room"]["title"]
                        .as_str()
                        .unwrap_or("douyin")
                        .to_string();
                    // 返回最高清晰度的直播地址 flv 和 hls
                    let urls = vec![
                        parse_url(
                            stream_url["flv_pull_url"]["FULL_HD1"]
                                .as_str()
                                .unwrap()
                                .to_owned(),
                        ),
                        parse_url(
                            stream_url["hls_pull_url_map"]["FULL_HD1"]
                                .as_str()
                                .unwrap()
                                .to_owned(),
                        ),
                    ];
                    Ok(Node {
                        rid: rid.to_string(),
                        title,
                        urls,
                    })
                }
            },
        }
    }
}

#[cfg(test)]
macros::gen_test!(5893162289);
