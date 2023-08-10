use std::collections::HashMap;

use async_trait::async_trait;
use regex::Regex;

use crate::{
    common::CLIENT,
    error::{Result, SeamError},
    util::{hash2header, parse_url},
};

use super::{Live, Node};

const URL: &str = "https://www.huajiao.com/l/";

/// 花椒直播
///
/// https://www.huajiao.com/
pub struct Client;

#[async_trait]
impl Live for Client {
    async fn get(&self, rid: &str, headers: Option<HashMap<String, String>>) -> Result<Node> {
        let text = CLIENT
            .get(format!("{URL}{rid}"))
            .headers(hash2header(headers))
            .send()
            .await?
            .text()
            .await?;

        let re1 = Regex::new(r#"sn":"([\s\S]*?)""#)?;
        let re2 = Regex::new(r#""replay_status":([0-9]*)"#)?;
        let sn = match re1.captures(&text) {
            Some(cap) => cap.get(1).ok_or(SeamError::NeedFix("sn"))?.as_str(),
            None => return Err(SeamError::None),
        };

        let re_title = Regex::new(r#"content="【(.+)】"#)?;
        let title = match re_title.captures(&text) {
            Some(cap) => match cap.get(1) {
                Some(title) => title.as_str().to_owned(),
                None => "获取失败".to_owned(),
            },
            None => "获取失败".to_owned(),
        };

        let pls: Vec<&str> = sn.split('_').collect();
        let pl = pls[2].to_lowercase();

        let captures = re2.captures(&text).ok_or(SeamError::None)?;
        let code = captures.get(1).ok_or(SeamError::NeedFix("code"))?.as_str();

        if code == "0" {
            Ok(Node {
                rid: rid.to_owned(),
                title,
                urls: vec![parse_url(format!(
                    "https://{pl}-flv.live.huajiao.com/live_huajiao_v2/{sn}.m3u8"
                ))],
            })
        } else {
            Err(SeamError::None)
        }
    }
}

#[cfg(test)]
macros::gen_test!(337633032);
