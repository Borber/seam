use anyhow::{Ok, Result};

use crate::{
    common::CLIENT,
    default_danmu_client,
    model::{Detail, ShowType},
    util::parse_url,
};

const URL: &str = "https://api.flextv.co.kr/api/channels/rid/stream?option=all";

default_danmu_client!(Flex);

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
        serde_json::Value::Null => Ok(ShowType::Off),
        url => {
            let nodes = vec![parse_url(url.as_str().unwrap().to_string())];
            Ok(ShowType::On(Detail::new("flex".to_owned(), nodes)))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_flex() {
        println!("{}", get("399291").await.unwrap());
    }
}
