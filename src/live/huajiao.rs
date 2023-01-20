use anyhow::{Ok, Result};
use regex::Regex;

use crate::{
    common::CLIENT,
    default_danmu_client,
    model::{Detail, ShowType},
    util::parse_url,
};

const URL: &str = "https://www.huajiao.com/l/";

default_danmu_client!(Huajiao);

/// 花椒直播
///
/// https://www.huajiao.com/
pub async fn get(rid: &str) -> Result<ShowType> {
    let text = CLIENT
        .get(format!("{URL}{rid}"))
        .send()
        .await?
        .text()
        .await?;
    let re1 = Regex::new(r#"sn":"([\s\S]*?)""#).unwrap();
    let re2 = Regex::new(r#""replay_status":([0-9]*)"#).unwrap();
    let sn = match re1.captures(&text) {
        Some(cap) => cap.get(1).unwrap().as_str(),
        None => return Ok(ShowType::Error("直播间不存在".to_string())),
    };
    let pls: Vec<&str> = sn.split('_').collect();
    let pl = pls[2].to_lowercase();
    match re2
        .captures(&text)
        .unwrap()
        .get(1)
        .unwrap()
        .as_str()
        .parse::<i32>()?
    {
        0 => {
            let nodes = vec![parse_url(format!(
                "https://{pl}-flv.live.huajiao.com/live_huajiao_v2/{sn}.m3u8"
            ))];
            Ok(ShowType::On(Detail::new("huajiao".to_owned(), nodes)))
        }
        _ => Ok(ShowType::Off),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_get_url() {
        println!("{}", get("337633032").await.unwrap());
    }
}
