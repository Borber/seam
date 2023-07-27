use crate::{
    common::{CLIENT, USER_AGENT},
    error::Result,
    StatusTrait,
};
use async_trait::async_trait;
use regex::Regex;
use reqwest::header::HeaderMap;
use urlencoding::decode;

pub struct Status;

const URL: &str = "https://live.douyin.com/";

#[async_trait]
impl StatusTrait for Status {
    async fn status(rid: &str) -> Result<bool> {
        let mut header_map = HeaderMap::new();
        // 更新 cookie
        header_map.insert("user-agent", USER_AGENT.parse()?);
        let resp = CLIENT
            .get(format!("{URL}{rid}"))
            .headers(header_map.clone())
            .send()
            .await?;
        header_map.insert("cookie", resp.headers().get("set-cookie").unwrap().clone());
        // 通过网页内容获取直播地址
        let resp = CLIENT
            .get(format!("{URL}{rid}"))
            .headers(header_map)
            .send()
            .await?;
        let resp_text = resp.text().await?;

        let re =
            Regex::new(r#"<script id="RENDER_DATA" type="application/json">([\s\S]*?)</script>"#)?;
        let json = decode(re.captures(&resp_text).unwrap().get(1).unwrap().as_str())?;
        let json: serde_json::Value = serde_json::from_str(&json)?;
        let status = &json["app"]["initialState"]["roomStore"]["roomInfo"]["room"]["status"];
        Ok(status.as_i64().is_some())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_status() {
        match Status::status("82671945773").await {
            Ok(true) => println!("已开播"),
            _ => println!("未开播"),
        }
    }
}
