use std::collections::HashMap;

use crate::{common::CLIENT, modle::ShowType};

use crate::util::md5;
use anyhow::{Ok, Result};
use chrono::prelude::*;
use regex::Regex;
use reqwest::header::HeaderMap;

const DID: &str = "10000000000000000000000000001501";

/// 斗鱼直播
///
/// https://www.douyu.com/
pub async fn get(rid: &str) -> Result<ShowType> {
    let dt = Local::now().timestamp_millis().to_string();
    let rid = real_rid(rid).await;
    get_pre(&rid, dt).await;
    Ok(ShowType::Off)
}

async fn real_rid(rid: &str) -> String {
    let resp = CLIENT
        .get(format!("https://m.douyu.com/{rid}"))
        .send()
        .await
        .unwrap();
    let text = resp.text().await.unwrap();
    let re = Regex::new(r#"rid":(\d{1,8}),"vipId"#).unwrap();
    let caps = re.captures(&text).unwrap();
    caps.get(1).unwrap().as_str().to_owned()
}
async fn get_pre(rid: &str, dt: String) {
    let url = format!("https://playweb.douyucdn.cn/lapi/live/hlsH5Preview/{rid}");
    let auth = md5(format!("{rid}{dt}").as_bytes());

    let mut headers = HeaderMap::new();
    headers.insert("rid", rid.parse().unwrap());
    headers.insert("time", "dt".parse().unwrap());
    headers.insert("auth", auth.parse().unwrap());

    let mut params = HashMap::new();
    params.insert("rid", rid);
    params.insert("did", DID);

    let resp: serde_json::Value = CLIENT
        .post(url)
        .headers(headers)
        .form(&params)
        .send()
        .await
        .unwrap()
        .json()
        .await
        .unwrap();
    println!("{}", resp);
}

#[cfg(test)]
mod tests {
    use crate::live::douyu::get;
    use crate::util::match_show_type;

    #[tokio::test]
    async fn test_get_url() {
        match_show_type(get("88080").await.unwrap());
    }
}
