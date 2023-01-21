use anyhow::{Ok, Result};
use regex::Regex;

use crate::{
    common::CLIENT,
    default_danmu_client,
    model::{Detail, ShowType},
    util::parse_url,
};

const URL: &str = "https://www.huya.com/";

default_danmu_client!(Huya);

/// 虎牙直播
///
/// https://huya.com/
pub async fn get(rid: &str) -> Result<ShowType> {
    let text = CLIENT
        .get(format!("{URL}{rid}"))
        .send()
        .await?
        .text()
        .await?;
    let re = Regex::new(r#"stream:([\s\S]*)window.TT_LIVE_TIMING"#).unwrap();
    let stream = match re.captures(&text) {
        Some(caps) => caps.get(1).unwrap().as_str().rsplit_once('}').unwrap().0,
        None => {
            return Ok(ShowType::Error("直播间不存在".to_string()));
        }
    };
    let re = Regex::new(r#"class="host-title" title="[\s\S]*?>([\s\S]*?)</h1>"#).unwrap();
    let title = match re.captures(&text) {
        Some(caps) => caps.get(1).unwrap().as_str(),
        None => "huya",
    };
    let json: serde_json::Value = serde_json::from_str(stream).unwrap();
    let mut nodes = vec![];
    match json["data"][0]["gameStreamInfoList"].as_array().unwrap() {
        list if list.is_empty() => {
            return Ok(ShowType::Off);
        }
        list => {
            for cdn in list {
                nodes.push(parse_url(format!(
                    "{}/{}.flv?{}",
                    cdn["sFlvUrl"].as_str().unwrap(),
                    cdn["sStreamName"].as_str().unwrap(),
                    cdn["sFlvAntiCode"].as_str().unwrap()
                )));
                nodes.push(parse_url(format!(
                    "{}/{}.m3u8?{}",
                    cdn["sHlsUrl"].as_str().unwrap(),
                    cdn["sStreamName"].as_str().unwrap(),
                    cdn["sHlsAntiCode"].as_str().unwrap()
                )));
            }
        }
    }
    Ok(ShowType::On(Detail::new(title.to_owned(), nodes)))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_get_url() {
        println!("{}", get("880236").await.unwrap());
    }
}
