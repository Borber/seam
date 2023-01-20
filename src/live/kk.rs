use anyhow::{Ok, Result};
use regex::Regex;

use crate::{
    common::CLIENT,
    default_danmu_client,
    model::{Detail, ShowType},
    util::parse_url,
};

const URL: &str = "https://sgapi.kktv8.com/roomApi/room/roomVideoBitrate?roomId=";

default_danmu_client!(Kk);

/// kk直播
///
/// https://www.kktv5.com/
pub async fn get(rid: &str) -> Result<ShowType> {
    let text = CLIENT
        .get(format!("{URL}{rid}"))
        .send()
        .await?
        .text()
        .await?;
    let re = Regex::new(r#"http[\s\S]*?flv"#).unwrap();
    match re.captures(&text) {
        Some(cap) => {
            let nodes = vec![parse_url(cap.get(0).unwrap().as_str().to_string())];
            Ok(ShowType::On(Detail::new("kk".to_owned(), nodes)))
        }
        None => Ok(ShowType::Off),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_get_url() {
        println!("{}", get("157079155").await.unwrap());
    }
}
