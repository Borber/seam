use crate::{common::CLIENT, error::Result, StatusTrait};
use async_trait::async_trait;

pub struct Status;

const URL: &str = "https://cc.163.com/";

#[async_trait]
impl StatusTrait for Status {
    async fn status(rid: &str) -> Result<bool> {
        let text = CLIENT
            .get(format!("{URL}{rid}"))
            .send()
            .await
            .unwrap()
            .text()
            .await
            .unwrap();
        Ok(text.contains("quickplay"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_status() {
        match Status::status("361433").await {
            Ok(true) => println!("已开播"),
            _ => println!("未开播"),
        }
    }
}
