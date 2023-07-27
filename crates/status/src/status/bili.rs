use crate::{common::CLIENT, error::Result, StatusTrait};
use async_trait::async_trait;

pub struct Status;

const URL: &str = "https://api.live.bilibili.com/room/v1/Room/room_init";

#[async_trait]
impl StatusTrait for Status {
    async fn status(rid: &str) -> Result<bool> {
        let resp: serde_json::Value = CLIENT
            .get(URL)
            .query(&[("id", rid)])
            .send()
            .await?
            .json()
            .await?;
        println!("{:#?}", resp);
        Ok(resp["data"]["live_status"].as_i64() == Some(1))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_status() {
        match Status::status("6").await {
            Ok(true) => println!("已开播"),
            _ => println!("未开播"),
        }
    }
}
