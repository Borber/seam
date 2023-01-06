use crate::{
    common::CLIENT,
    modle::{Node, ShowType},
};

use anyhow::{Ok, Result};
use regex::Regex;
use urlencoding::decode;

/// 抖音直播
///
/// https://live.douyin.com/
pub async fn get(rid: &str) -> Result<ShowType> {
    let resp = CLIENT
        .get(format!("https://live.douyin.com/{}", rid))
        .header("cookie", "__ac_nonce=063b59ce0001243a9217f;")
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
    // TODO: 修改是否开播判定
    Ok(ShowType::On(vec![Node {
        rate: "flv".to_string(),
        url: json["app"]["initialState"]["roomStore"]["roomInfo"]["room"]["stream_url"]
            ["flv_pull_url"]["FULL_HD1"]
            .to_string()
            .trim_matches('"')
            .to_string(),
    },
    Node {
        rate: "hls".to_string(),
        url: json["app"]["initialState"]["roomStore"]["roomInfo"]["room"]["stream_url"]
            ["hls_pull_url"]
            .to_string()
            .trim_matches('"')
            .to_string(),
    }]))
}

#[cfg(test)]
mod tests {
    use crate::live::douyin::get;
    use crate::util::match_show_type;
    #[tokio::test]
    async fn test_get_url() {
        match_show_type(get("228619203678").await.unwrap());
    }
}
