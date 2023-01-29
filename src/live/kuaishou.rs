use anyhow::{Ok, Result};
use regex::Regex;

use crate::{
    common::{CLIENT, USER_AGENT},
    default_danmu_client,
    model::{Detail, ShowType},
    util::parse_url,
};

const URL: &str = "https://live.kuaishou.com/u/";

default_danmu_client!(Kuaishou);

/// 快手直播
///
/// https://live.kuaishou.com/
pub async fn get(rid: &str) -> Result<ShowType> {
    let text = CLIENT
        .get(format!("{URL}{rid}"))
        .header("User-Agent", USER_AGENT)
        .header("Cookie", "did=web_d563dca728d28b00336877723e0359ed;")
        .send()
        .await?
        .text()
        .await?;
    let re = Regex::new(r#"<script>window.__INITIAL_STATE__=([\s\S]*?);\(function"#).unwrap();
    let stream = match re.captures(&text) {
        Some(caps) => caps.get(1).unwrap().as_str(),
        None => {
            return Ok(ShowType::Error("直播间不存在".to_string()));
        }
    };
    let json: serde_json::Value = serde_json::from_str(stream).unwrap();
    // TODO 更改其他逻辑 多用Null
    match &json["liveroom"]["liveStream"]["playUrls"][0]["adaptationSet"]["representation"] {
        serde_json::Value::Null => Ok(ShowType::Off),
        reps => {
            let list = reps.as_array().unwrap();
            let url = list[list.len() - 1]["url"].as_str().unwrap();
            let nodes = vec![parse_url(url.to_string())];
            Ok(ShowType::On(Detail::new("kuaishou".to_owned(), nodes)))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_kuaishou() {
        println!("{}", get("3xgexgpig9gwwi2").await.unwrap());
    }
}
