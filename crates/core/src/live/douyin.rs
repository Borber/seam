use std::collections::HashMap;

use async_trait::async_trait;
use reqwest::header::HeaderValue;
use serde_json::Value;

use crate::{
    common::CLIENT,
    error::{Result, SeamError},
    util::{hash2header, parse_url},
};

use super::{Live, Node};

const URL: &str = "https://live.douyin.com/";
const ENTER_URL: &str =
    "https://live.douyin.com/webcast/room/web/enter/?aid=6383&live_id=1&device_platform=web&language=zh-CN&enter_from=web_live&cookie_enabled=true&screen_width=1536&screen_height=864&browser_language=zh-CN&browser_platform=Win32&browser_name=Chrome&browser_version=94.0.4606.81&room_id_str=&enter_source=&web_rid=";

/// 抖音直播
///
/// https://live.douyin.com/
pub struct Client;

#[async_trait]
impl Live for Client {
    /// `headers`: cookie 必须， 但不需要是登录状态
    async fn get(&self, rid: &str, headers: Option<HashMap<String, String>>) -> Result<Node> {
        let mut headers = hash2header(headers);
        headers.append("referer", HeaderValue::from_static(URL));
        // 通过网页内容获取直播地址
        let json = CLIENT
            .get(format!("{ENTER_URL}{rid}"))
            .headers(headers)
            .send()
            .await?
            .json::<Value>()
            .await?;

        let data = &json["data"]["data"][0];

        let status = &data["status"];

        if status.as_i64().unwrap_or(0) != 2 {
            return Err(SeamError::None);
        }

        let title = data["title"].as_str().unwrap_or("获取失败").to_string();
        let cover = data["cover"]["url_list"][0]
            .as_str()
            .unwrap_or("")
            .to_string();
        let anchor = data["owner"]["nickname"].as_str().unwrap_or("").to_string();
        let head = data["owner"]["avatar_thumb"]["url_list"][0]
            .as_str()
            .unwrap_or("")
            .to_string();

        let stream_data = data["stream_url"]["live_core_sdk_data"]["pull_data"]["stream_data"]
            .as_str()
            .ok_or(SeamError::NeedFix("stream_data"))?;

        let new_json = serde_json::from_str::<Value>(stream_data)?;
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
            cover,
            anchor,
            head,
            urls,
        })
    }
}

#[cfg(test)]
macros::gen_test!(7274955926023686967);
