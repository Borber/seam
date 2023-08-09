use std::collections::HashMap;
use std::time::{SystemTime, UNIX_EPOCH};

use async_trait::async_trait;
use base64::{engine::general_purpose, Engine as _};
use md5::{Digest, Md5};
use rand::Rng;
use regex::Regex;
use serde_json::json;

use crate::{
    common::CLIENT,
    error::{Result, SeamError},
    util::{hash2header, parse_url},
};

use super::{Live, Node};

const URL: &str = "https://m.huya.com/";

/// 虎牙直播
///
/// https://huya.com/
pub struct Client;

#[async_trait]
impl Live for Client {
    async fn get(&self, rid: &str, headers: &Option<&HashMap<String, String>>) -> Result<Node> {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_millis();
        let rand = rand::thread_rng().gen_range(0..1000);

        let uid = get_anonymous_uid().await;

        let text = CLIENT
            .get(format!("{URL}{rid}"))
            .header("Content-Type", "application/x-www-form-urlencoded")
            .header("User-Agent", "Mozilla/5.0 (Linux; Android 5.0; SM-G900P Build/LRX21T) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/75.0.3770.100 Mobile Safari/537.36")
            .headers(hash2header(headers))
            .send()
            .await?
            .text()
            .await?;

        let re = Regex::new(r"<script> window.HNF_GLOBAL_INIT = ([\s\S]*) </script>")?;

        let stream = match re.captures(&text) {
            Some(caps) => caps.get(1).ok_or(SeamError::NeedFix("stream"))?.as_str(),
            None => return Err(SeamError::NeedFix("stream none")),
        };

        let json: serde_json::Value = serde_json::from_str(stream)?;

        let status = json["roomInfo"]["eLiveStatus"]
            .as_i64()
            .ok_or(SeamError::NeedFix("eLiveStatus"))?;

        if status != 2 {
            return Err(SeamError::None);
        }

        let title = match json["roomInfo"]["tLiveInfo"]["sIntroduction"] {
            serde_json::Value::String(ref title) => title.as_str(),
            _ => "获取失败",
        };

        let mut urls = vec![];

        let streams = json["roomInfo"]["tLiveInfo"]["tLiveStreamInfo"]["vStreamInfo"]["value"]
            .as_array()
            .ok_or(SeamError::NeedFix("vStreamInfo"))?;

        for stream in streams {
            let flv = stream["sFlvUrl"]
                .as_str()
                .ok_or(SeamError::NeedFix("sFlvUrl"))?;
            let hls = stream["sHlsUrl"]
                .as_str()
                .ok_or(SeamError::NeedFix("sHlsUrl"))?;

            let anti_code = process_anticode(
                stream["sFlvAntiCode"].as_str().unwrap(),
                stream["sStreamName"].as_str().unwrap(),
                uid,
                now,
                rand,
            );
            urls.push(parse_url(format!(
                "{}/{}.{}?{}",
                flv,
                stream["sStreamName"].as_str().unwrap(),
                stream["sFlvUrlSuffix"].as_str().unwrap(),
                anti_code
            )));

            let anti_code = process_anticode(
                stream["sHlsAntiCode"].as_str().unwrap(),
                stream["sStreamName"].as_str().unwrap(),
                uid,
                now,
                rand,
            );
            urls.push(parse_url(format!(
                "{}/{}.{}?{}",
                hls,
                stream["sStreamName"].as_str().unwrap(),
                stream["sHlsUrlSuffix"].as_str().unwrap(),
                anti_code
            )));
        }

        Ok(Node {
            rid: rid.to_owned(),
            title: title.to_owned(),
            urls,
        })
    }
}

fn get_uuid(now: u128, rand: u32) -> u128 {
    (now % 10000000000 * 1000 + rand as u128) % 4294967295
}

async fn get_anonymous_uid() -> u128 {
    let resp: HashMap<String, serde_json::Value> = CLIENT
        .post("https://udblgn.huya.com/web/anonymousLogin")
        .json(&json!({
            "appId": 5002,
            "byPass": 3,
            "context": "",
            "version": "2.4",
            "data": {}
        }))
        .send()
        .await
        .unwrap()
        .json()
        .await
        .unwrap();

    resp["data"]["uid"]
        .as_str()
        .unwrap()
        .to_string()
        .parse()
        .unwrap()
}

fn process_anticode(anticode: &str, stream_name: &str, uid: u128, now: u128, rand: u32) -> String {
    let anticode = urlencoding::decode(anticode).unwrap().to_string();
    let mut anti_map = anticode.split('&').fold(HashMap::new(), |mut map, s| {
        let (k, v) = s.split_once('=').unwrap();
        map.insert(k.to_owned(), v.to_owned());
        map
    });

    anti_map.insert("ver".to_string(), "1".to_string());
    anti_map.insert("sv".to_string(), "2110211124".to_string());
    anti_map.insert("seqid".to_string(), format!("{}", uid + now * 1_000));
    anti_map.insert("uid".to_string(), uid.to_string());
    anti_map.insert("uuid".to_string(), get_uuid(now, rand).to_string());

    let seqid = anti_map["seqid"].as_str();
    let ctype = anti_map["ctype"].as_str();
    let t = anti_map["t"].as_str();

    let result = {
        let mut h = Md5::new();
        h.update(format!("{}|{}|{}", seqid, ctype, t));
        hex::encode(h.finalize())
    };

    let fm = anti_map["fm"].as_str();

    let fm = general_purpose::STANDARD.decode(fm).unwrap();
    let fm = String::from_utf8(fm).unwrap();

    let fm = fm.replace("$0", &anti_map["uid"]);
    let fm = fm.replace("$1", stream_name);
    let fm = fm.replace("$2", &result);
    let fm = fm.replace("$3", &anti_map["wsTime"]);

    let secret = {
        let mut h = Md5::new();
        h.update(fm);
        hex::encode(h.finalize())
    };

    anti_map.insert("wsSecret".to_string(), secret).unwrap();

    anti_map.remove("fm");

    let mut s = String::new();
    for (k, v) in anti_map {
        s = format!("{}&{}={}", s, k, v);
    }
    s
}

#[cfg(test)]
macros::gen_test!(880201);
