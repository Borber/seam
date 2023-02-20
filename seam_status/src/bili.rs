use crate::{common::CLIENT, error::Result, Client};
use async_trait::async_trait;

pub struct BiliStatusClient {}

const URL: &str = "https://api.live.bilibili.com/room/v1/Room/room_init";

#[async_trait]
impl Client for BiliStatusClient {
    async fn status(rid: &str) -> Result<bool> {
        let resp: serde_json::Value = CLIENT
            .get(URL)
            .query(&[("id", rid)])
            .send()
            .await?
            .json()
            .await?;
        match resp["data"]["live_status"].as_i64() {
            Some(1) => Ok(true),
            _ => Ok(false),
        }
    }
}
