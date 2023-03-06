use async_trait::async_trait;

use super::{Live, Node};
use crate::error::{Result, SeamError};
use crate::{
    common::{CLIENT, USER_AGENT},
    util::parse_url,
};

const INIT_URL: &str = "https://api.live.bilibili.com/room/v1/Room/room_init";
const INFO_URL: &str = "https://api.live.bilibili.com/room/v1/Room/get_info";
const URL: &str = "https://api.live.bilibili.com/xlive/web-room/v2/index/getRoomPlayInfo";

/// bilibili直播
///
/// https://live.bilibili.com/
pub struct Bili;

#[async_trait]
impl Live for Bili {
    async fn get(rid: &str) -> Result<Node> {
        let resp: serde_json::Value = CLIENT
            .get(INIT_URL)
            .query(&[("id", rid)])
            .send()
            .await?
            .json()
            .await?;
        // 获取真实房间号
        let rid = match resp["data"]["live_status"].as_i64() {
            Some(1) => resp["data"]["room_id"].as_u64().unwrap().to_string(),
            _ => return Err(SeamError::None),
        };
        let mut stream_info = get_bili_stream_info(&rid, 10000).await?;
        let max = stream_info
            .as_array()
            .unwrap()
            .iter()
            .map(|data| {
                data["format"][0]["codec"][0]["accept_qn"]
                    .as_array()
                    .unwrap()
                    .iter()
                    .map(|item| item.as_u64().unwrap())
                    .max()
                    .unwrap()
            })
            .max()
            .unwrap();
        if max != 10000 {
            stream_info = get_bili_stream_info(&rid, max).await?;
        }
        let mut urls = vec![];
        for obj in stream_info.as_array().unwrap() {
            for format in obj["format"].as_array().unwrap() {
                for codec in format["codec"].as_array().unwrap() {
                    let base_url = codec["base_url"].as_str().unwrap();
                    for url_info in codec["url_info"].as_array().unwrap() {
                        let host = url_info["host"].as_str().unwrap();
                        let extra = url_info["extra"].as_str().unwrap();
                        urls.push(parse_url(format!("{host}{base_url}{extra}")));
                    }
                }
            }
        }

        let json: serde_json::Value = CLIENT
            .get(INFO_URL)
            .query(&[("room_id", &rid)])
            .send()
            .await?
            .json()
            .await?;
        let title = json["data"]["title"].as_str().unwrap().to_owned();
        Ok(Node { rid, title, urls })
    }
}

/// 通过真实房间号获取直播源信息
pub async fn get_bili_stream_info(rid: &str, qn: u64) -> Result<serde_json::Value> {
    Ok(CLIENT
        .get(URL)
        .header("User-Agent", USER_AGENT)
        .query(&[
            ("room_id", rid),
            ("protocol", "0,1"),
            ("format", "0,1,2"),
            ("codec", "0,1"),
            ("qn", qn.to_string().as_str()),
            ("platform", "h5"),
            ("ptype", "8"),
        ])
        .send()
        .await?
        .json::<serde_json::Value>()
        .await?["data"]["playurl_info"]["playurl"]["stream"]
        .to_owned())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_get_url() {
        match Bili::get("6").await {
            Ok(node) => println!("{}", node.json()),
            Err(e) => println!("{e}"),
        }
    }
}
