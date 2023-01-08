use anyhow::{Ok, Result};
use regex::Regex;

use crate::{
    common::CLIENT,
    modle::{Node, ShowType},
};

const URL: &str = "https://www.huya.com/";

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
    let json: serde_json::Value = serde_json::from_str(&stream).unwrap();
    let mut nodes = vec![];
    match json["data"][0]["gameStreamInfoList"].as_array().unwrap() {
        list if list.len() == 0 => {
            return Ok(ShowType::Off);
        }
        list => {
            for cdn in list {
                nodes.push(Node {
                    rate: format!("蓝光-{}-flv", cdn["sCdnType"].to_string().trim_matches('"')),
                    url: format!(
                        "{}/{}.flv?{}",
                        cdn["sFlvUrl"].to_string().trim_matches('"'),
                        cdn["sStreamName"].to_string().trim_matches('"'),
                        cdn["sFlvAntiCode"].to_string().trim_matches('"')
                    ),
                });
                nodes.push(Node {
                    rate: format!("蓝光-{}-hls", cdn["sCdnType"].to_string().trim_matches('"')),
                    url: format!(
                        "{}/{}.m3u8?{}",
                        cdn["sHlsUrl"].to_string().trim_matches('"'),
                        cdn["sStreamName"].to_string().trim_matches('"'),
                        cdn["sHlsAntiCode"].to_string().trim_matches('"')
                    ),
                });
            }
        }
    }

    Ok(ShowType::On(nodes))
}

#[cfg(test)]
mod tests {
    use crate::live::huya::get;
    use crate::util::match_show_type;

    #[tokio::test]
    async fn test_get_url() {
        match_show_type(get("28328839").await.unwrap());
    }
}
