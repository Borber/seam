use crate::{
    common::CLIENT,
    model::{Node, ShowType},
};

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
        2 => Ok(ShowType::On(vec![Node {
            rate: "".to_string(),
            url: data["url"].to_string(),
        }])),
        _ => Ok(ShowType::Off),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::util::match_show_type;

    #[tokio::test]
    async fn test_get_url() {
        match_show_type(get("96").await.unwrap());
    }
}
