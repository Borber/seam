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
    async fn get(&self, rid: &str, headers: &Option<HashMap<String, String>>) -> Result<Node> {
        let text = CLIENT
            .get(format!("{URL}{rid}"))
            .headers(hash2header(headers))
            .send()
            .await?
            .text()
            .await?;
        println!("{}", text);
        let re1 = Regex::new(r#"sn":"([\s\S]*?)""#).unwrap();
        let re2 = Regex::new(r#""replay_status":([0-9]*)"#).unwrap();
        let sn = match re1.captures(&text) {
            Some(cap) => cap.get(1).unwrap().as_str(),
            None => return Err(SeamError::None),
        };
        let pls: Vec<&str> = sn.split('_').collect();
        let pl = pls[2].to_lowercase();
        // TODO 处理 unwarp
        match re2
            .captures(&text)
            .unwrap()
            .get(1)
            .unwrap()
            .as_str()
            .parse::<i32>()?
        {
            0 => {
                let urls = vec![parse_url(format!(
                    "https://{pl}-flv.live.huajiao.com/live_huajiao_v2/{sn}.m3u8"
                ))];
                Ok(Node {
                    rid: rid.to_owned(),
                    title: "huajiao".to_owned(),
                    urls,
                })
            }
            _ => Err(SeamError::None),
        }
    }
}

#[cfg(test)]
macros::gen_test!(337633032);
