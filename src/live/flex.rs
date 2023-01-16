use anyhow::{Ok, Result};

use crate::{
    common::CLIENT,
    model::{Node, ShowType},
};

const URL: &str = "https://api.flextv.co.kr/api/channels/rid/stream?option=all";

/// flextv
///
/// https://www.flextv.co.kr/
pub async fn get(rid: &str) -> Result<ShowType> {
    let json: serde_json::Value = CLIENT
        .get(URL.replace("rid", rid))
        .send()
        .await?
        .json()
        .await?;
    match &json["sources"][0]["url"] {
        serde_json::Value::Null => return Ok(ShowType::Off),
        url => Ok(ShowType::On(vec![Node {
            rate: "原画".to_string(),
            url: url.as_str().unwrap().to_string(),
        }])),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::util::match_show_type;

    #[tokio::test]
    async fn test_flex() {
        match_show_type(get("399291").await.unwrap());
    }
}
