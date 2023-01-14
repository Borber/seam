use anyhow::{Ok, Result};
use regex::Regex;
use crate::{model::{Node, ShowType}, common::CLIENT};

const URL: &str = "https://cc.163.com/";

/// 网易CC直播
///
/// https://cc.163.com/
pub async fn get(rid: &str) -> Result<ShowType> {
    let text = CLIENT.get(format!("{URL}{rid}")).send().await?.text().await?;
    let re = Regex::new(r#"<script id="__NEXT_DATA__" type="application/json" crossorigin="anonymous">([\s\S]*?)</script>"#).unwrap();
    let json = match re.captures(&text) {
        Some(rap) => {
            rap.get(1).unwrap().as_str()
        },
        None => {
            return Ok(ShowType::Error("未找到直播间".to_string()));
        },
    };
    let json: serde_json::Value = serde_json::from_str(json)?;
    let resolution = match &json["props"]["pageProps"]["roomInfoInitData"]["live"]["quickplay"]["resolution"] {
        serde_json::Value::Null => return Ok(ShowType::Off),
        v => v,
    };
    let mut nodes = vec![];
    for vbr in ["blueray", "ultra", "high", "standard"] {
        let cdn = change_str(vbr);
        if resolution[vbr] != serde_json::Value::Null {
            nodes.push(Node{
                rate: format!("{cdn}-ali"),
                url: resolution[vbr]["cdn"]["ali"].as_str().unwrap().to_string(),
            });
            nodes.push(Node{
                rate: format!("{cdn}-ks"),
                url: resolution[vbr]["cdn"]["ks"].as_str().unwrap().to_string(),
            });
            break;
        }
    }
    Ok(ShowType::On(nodes))
}

fn change_str(s: &str) -> &str {
    match s {
        "blueray" => "蓝光",
        "ultra" => "超清",
        "high" => "高清",
        "standard" => "标清",
        _ => "未知",
    }
}


#[cfg(test)]
mod tests{
    use super::*;
    use crate::util::match_show_type;

    #[tokio::test]
    async fn test_get_url() {
        match_show_type(get("361433").await.unwrap());
    }
}