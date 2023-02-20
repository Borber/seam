use crate::{common::CLIENT, error::Result, Client};
use async_trait::async_trait;

pub struct CcStatusClient {}

const URL: &str = "https://cc.163.com/";

#[async_trait]
impl Client for CcStatusClient {
    async fn status(rid: &str) -> Result<bool> {
        let text = CLIENT
            .get(format!("{URL}{rid}"))
            .send()
            .await
            .unwrap()
            .text()
            .await
            .unwrap();
        Ok(text.find("quickplay").is_some())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_get_url() -> Result<()> {
        println!("{}", CcStatusClient::status("361433").await?);
        Ok(())
    }
}
