use crate::{
    common::CLIENT,
    default_danmu_client,
    model::{Detail, ShowType},
    util::parse_url,
};

use anyhow::{Ok, Result};
use std::collections::HashMap;

const URL: &str = "https://www.173.com/room/getVieoUrl";

default_danmu_client!(Yqs);

/// 艺气山直播
///
/// https://www.173.com/
pub async fn get(rid: &str) -> Result<ShowType> {
    let mut params = HashMap::new();
    params.insert("roomId", rid);
    let resp: serde_json::Value = CLIENT.post(URL).form(&params).send().await?.json().await?;
    let data = &resp["data"];
    match data["status"].to_string().parse::<usize>()? {
        2 => {
            let nodes = vec![parse_url(data["url"].to_string())];
            Ok(ShowType::On(Detail::new("yqs".to_owned(), nodes)))
        }
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
