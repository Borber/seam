use crate::{common::CLIENT, model::ShowType, util::parse_url};

use anyhow::{Ok, Result};
use std::collections::HashMap;

const URL: &str = "https://www.173.com/room/getVieoUrl";

/// 艺气山直播
///
/// https://www.173.com/
pub async fn get(rid: &str) -> Result<ShowType> {
    let mut params = HashMap::new();
    params.insert("roomId", rid);
    let resp: serde_json::Value = CLIENT.post(URL).form(&params).send().await?.json().await?;
    let data = &resp["data"];
    match data["status"].to_string().parse::<usize>()? {
        2 => Ok(ShowType::On(vec![parse_url(data["url"].to_string())])),
        _ => Ok(ShowType::Off),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_get_url() {
        println!("{}", get("96").await.unwrap());
    }
}
