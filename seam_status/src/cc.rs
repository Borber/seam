use crate::{common::CLIENT, error::Result, Client};
use async_trait::async_trait;
use regex::Regex;

pub struct CcStatusClient {}

const URL: &str = "https://cc.163.com/";

#[async_trait]
impl Client for CcStatusClient {
    async fn status(rid: &str) -> Result<bool> {
        let text = CLIENT
            .get(format!("{URL}{rid}"))
            .send()
            .await
            .unwrap()
            .text()
            .await
            .unwrap();
        let re = Regex::new(r#"<script id="__NEXT_DATA__" type="application/json" crossorigin="anonymous">([\s\S]*?)</script>"#).unwrap();
        match re.captures(&text) {
            Some(_) => Ok(true),
            None => Ok(false),
        }
    }
}
