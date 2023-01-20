use crate::{
    common::CLIENT,
    default_danmu_client,
    model::{Detail, ShowType},
    util::parse_url,
};
use anyhow::{Ok, Result};
use regex::Regex;

const URL: &str = "https://cc.163.com/";

default_danmu_client!(Cc);

/// 网易CC直播
///
/// https://cc.163.com/
pub async fn get(rid: &str) -> Result<ShowType> {
    let text = CLIENT
        .get(format!("{URL}{rid}"))
        .send()
        .await?
        .text()
        .await?;
    let re = Regex::new(r#"<script id="__NEXT_DATA__" type="application/json" crossorigin="anonymous">([\s\S]*?)</script>"#).unwrap();
    let json = match re.captures(&text) {
        Some(rap) => rap.get(1).unwrap().as_str(),
        None => {
            return Ok(ShowType::Error("未找到直播间".to_string()));
        }
    };
    let json: serde_json::Value = serde_json::from_str(json)?;
    let resolution =
        match &json["props"]["pageProps"]["roomInfoInitData"]["live"]["quickplay"]["resolution"] {
            serde_json::Value::Null => return Ok(ShowType::Off),
            v => v,
        };
    let title = json["props"]["pageProps"]["roomInfoInitData"]["live"]["title"]
        .as_str()
        .unwrap();
    let mut nodes = vec![];
    for vbr in ["blueray", "ultra", "high", "standard"] {
        if resolution[vbr] != serde_json::Value::Null {
            if resolution[vbr]["cdn"]["ali"] != serde_json::Value::Null {
                nodes.push(parse_url(
                    resolution[vbr]["cdn"]["ali"].as_str().unwrap().to_string(),
                ));
            }
            if resolution[vbr]["cdn"]["ks"] != serde_json::Value::Null {
                nodes.push(parse_url(
                    resolution[vbr]["cdn"]["ks"].as_str().unwrap().to_string(),
                ));
            }
            break;
        }
    }
    Ok(ShowType::On(Detail::new(title.to_owned(), nodes)))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_get_url() {
        println!("{}", get("361433").await.unwrap());
    }
}
