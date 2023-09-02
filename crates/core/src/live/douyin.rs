use std::collections::HashMap;

use async_trait::async_trait;
use regex::Regex;
use serde_json::Value;
use urlencoding::decode;

use crate::{
    common::CLIENT,
    error::{Result, SeamError},
    util::{hash2header, parse_url},
};

use super::{Live, Node};

const URL: &str = "https://live.douyin.com/";

/// 抖音直播
///
/// https://live.douyin.com/
pub struct Client;

#[async_trait]
impl Live for Client {
    /// `headers`: cookie 必须， 但不需要是登录状态
    async fn get(&self, rid: &str, headers: Option<HashMap<String, String>>) -> Result<Node> {
        // 通过网页内容获取直播地址
        let resp = CLIENT
            .get(format!("{URL}{rid}"))
            .headers(hash2header(headers))
            .send()
            .await?;
        let resp_text = resp.text().await?;

        let re =
            Regex::new(r#"<script id="RENDER_DATA" type="application/json">([\s\S]*?)</script>"#)?;
        let json = decode(
            re.captures(&resp_text)
                .ok_or(SeamError::NeedFix("captures"))?
                .get(1)
                .ok_or(SeamError::NeedFix("json"))?
                .as_str(),
        )?;
        let json = serde_json::from_str::<Value>(&json)?;

        let room_info = &json["app"]["initialState"]["roomStore"]["roomInfo"];
        match room_info["anchor"] {
            // TODO 主播不存在 这种需要额外判断吗?
            serde_json::Value::Null => Err(SeamError::None),
            _ => match &room_info["room"]["stream_url"] {
                // 未开播
                Value::Null => Err(SeamError::None),
                stream_url => {
                    let title = room_info["room"]["title"]
                        .as_str()
                        .unwrap_or("douyin")
                        .to_string();

                    // 获取 json str
                    let json_str = stream_url["live_core_sdk_data"]["pull_data"]["stream_data"]
                        .as_str()
                        .ok_or(SeamError::NeedFix("stream_data"))?;

                    let new_json = serde_json::from_str::<Value>(json_str)?;
                    // 返回最高清晰度的直播地址 flv 和 hls
                    let urls = vec![
                        parse_url(
                            new_json["data"]["origin"]["main"]["flv"]
                                .as_str()
                                .ok_or(SeamError::NeedFix("flv_pull_url"))?
                                .to_owned(),
                        ),
                        parse_url(
                            new_json["data"]["origin"]["main"]["hls"]
                                .as_str()
                                .ok_or(SeamError::NeedFix("hls_pull_url_map"))?
                                .to_owned(),
                        ),
                    ];
                    Ok(Node {
                        rid: rid.to_string(),
                        title,
                        cover: "".to_owned(),
                        anchor: "".to_owned(),
                        head: "".to_owned(),
                        urls,
                    })
                }
            },
        }
    }
}

#[cfg(test)]
macros::gen_test!(5893162289);
