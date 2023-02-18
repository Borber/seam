use anyhow::{Ok, Result};
use regex::Regex;
use reqwest::header::HeaderMap;

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
    let mut header_map = HeaderMap::new();
    header_map.insert("user-agent", USER_AGENT.parse()?);
    let resp = CLIENT
        .get(format!("{URL}{rid}"))
        .headers(header_map.clone())
        .send()
        .await?;
    let cookie = resp
        .headers()
        .get_all("set-cookie")
        .iter()
        .map(|x| x.to_str().unwrap().to_string())
        .collect::<Vec<String>>()
        .join("; ")
        .to_string();
    header_map.insert("cookie", cookie.parse()?);
    let text = CLIENT
        .get(format!("{URL}{rid}"))
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
