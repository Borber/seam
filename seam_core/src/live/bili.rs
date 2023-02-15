use std::collections::HashMap;

use anyhow::{Ok, Result};
use async_trait::async_trait;

use crate::live::{Live, Node, Status};
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
pub type Bili = Node;

impl Bili {
    pub async fn new(rid: &str) -> Option<Bili> {
        match Bili::status(rid, true).await {
            Status::On(Some(m)) => Some(Self {
                rid: m.get("rid").unwrap().to_owned(),
                title: "".to_string(),
                urls: vec![],
            }),
            _ => None,
        }
    }

    pub fn json(&self) -> String {
        serde_json::to_string_pretty(&self).unwrap_or("序列化失败".to_owned())
    }
}

#[async_trait]
impl Live for Bili {
    async fn get(mut self) -> Self {
        let mut stream_info = get_bili_stream_info(&self.rid, 10000).await.unwrap();
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
            stream_info = get_bili_stream_info(&self.rid, max).await.unwrap();
        }
        for obj in stream_info.as_array().unwrap() {
            for format in obj["format"].as_array().unwrap() {
                for codec in format["codec"].as_array().unwrap() {
                    let base_url = codec["base_url"].as_str().unwrap();
                    for url_info in codec["url_info"].as_array().unwrap() {
                        let host = url_info["host"].as_str().unwrap();
                        let extra = url_info["extra"].as_str().unwrap();
                        self.urls
                            .push(parse_url(format!("{host}{base_url}{extra}")));
                    }
                }
            }
        }

        let json: serde_json::Value = CLIENT
            .get(INFO_URL)
            .query(&[("room_id", &self.rid)])
            .send()
            .await
            .unwrap()
            .json()
            .await
            .unwrap();
        self.title = json["data"]["title"].as_str().unwrap().to_owned();
        self.clone()
    }

    //短id和真实房间号均可用以检测状态
    async fn status(rid: &str, ext: bool) -> Status {
        let resp: serde_json::Value = CLIENT
            .get(INIT_URL)
            .query(&[("id", rid)])
            .send()
            .await
            .unwrap()
            .json()
            .await
            .unwrap();
        let s = match resp["data"]["live_status"].as_i64() {
            Some(1) => true,
            _ => false,
        };
        if s == true && ext == true {
            let mut m = HashMap::new();
            m.insert(
                "rid".to_owned(),
                resp["data"]["room_id"].as_u64().unwrap().to_string(),
            );
            return Status::On(Some(m));
        }
        Status::On(None)
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
        let live = Bili::new("6").await.unwrap().get().await;
        println!("{}", serde_json::to_string_pretty(&live).unwrap());
    }
}
