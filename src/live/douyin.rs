use crate::modle::ShowType;

use anyhow::{Ok, Result};
use regex::Regex;
use reqwest::{header::HeaderMap, Client};
use urlencoding::decode;

/// 抖音直播
///
/// https://live.douyin.com/
pub async fn get(rid: &str, client: &Client) -> Result<ShowType> {
    let mut headers = HeaderMap::new();
    headers.insert(
        "cookie",
        "__ac_nonce=063b59ce0001243a9217f;".parse().unwrap(),
    );
    let resp = client
        .get(format!("https://live.douyin.com/{}", rid))
        .headers(headers)
        .send()
        .await?;
    let resp_text = resp.text().await?;
    let re = Regex::new(r#"<script id="RENDER_DATA" \b[^>]*>[\s\S]*?</script>"#).unwrap();
    let script_id = re.captures(&resp_text).unwrap().get(0).unwrap().as_str();
    let json_encode = script_id
        .replace(r#"<script id="RENDER_DATA" type="application/json">"#, "")
        .replace("</script>", "");
    let json = decode(&json_encode)?;
    let json: serde_json::Value = serde_json::from_str(&json)?;
    Ok(ShowType::On(vec![json["app"]["initialState"]["roomStore"]
        ["roomInfo"]["room"]["stream_url"]["flv_pull_url"]
        ["FULL_HD1"]
        .to_string()]))
}

#[cfg(test)]
mod tests {
    use reqwest::Client;

    use crate::live::douyin::get;
    #[tokio::test]
    async fn test_get_url() {
        let client = Client::new();
        println!("{:#?}", get("228619203678", &client).await.unwrap());
    }
}
