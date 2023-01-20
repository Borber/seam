use anyhow::{Ok, Result};
use regex::Regex;

use crate::{
    common::CLIENT,
    default_danmu_client,
    model::{Detail, ShowType},
    util::parse_url,
};

const URL: &str = "https://qf.56.com/";

default_danmu_client!(Qf);

/// 千帆直播
///
/// https://www.kktv5.com/
pub async fn get(rid: &str) -> Result<ShowType> {
    let text = CLIENT
        .get(format!("{URL}{rid}"))
        .send()
        .await?
        .text()
        .await?;
    let re = Regex::new(r#"flvUrl:'([\s\S]*?)'"#).unwrap();
    match re.captures(&text) {
        Some(cap) => {
            let nodes = vec![parse_url(cap.get(1).unwrap().as_str().to_string())];
            Ok(ShowType::On(Detail::new("qf".to_owned(), nodes)))
        }
        None => Ok(ShowType::Off),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_get_url() {
        println!("{}", get("520006").await.unwrap());
    }
}
