use std::collections::{HashMap, HashSet};

use crate::{
    common::CLIENT,
    error::{Result, SeamError},
    util::{eval, hash2header, parse_url},
};

use async_trait::async_trait;
use chrono::prelude::*;
use md5::{Digest, Md5};
use regex::Regex;
use reqwest::header::HeaderMap;
use serde_json::Value;

use super::{Live, Node};

const URL: &str = "https://www.douyu.com/";
const PLAY_URL: &str = "https://www.douyu.com/lapi/live/getH5Play/";
const BETARD_URL: &str = "https://www.douyu.com/betard/";
const DID: &str = "10000000000000000000000000001501";

/// 斗鱼直播
///
/// https://www.douyu.com/
pub struct Client;

#[async_trait]
impl Live for Client {
    /// `headers`: cookie 不必须, 登录状态下可以获取备用路线高清源
    async fn get(&self, rid: &str, headers: Option<HashMap<String, String>>) -> Result<Node> {
        let headers = hash2header(headers);
        // 构造时间戳
        let binding = Local::now().timestamp_millis().to_string();
        let dt = &binding.as_str()[0..10];

        // 获取指定直播间的首页源代码, 认证的sign和直播间是绑定的
        let text = CLIENT
            .get(format!("{URL}{rid}"))
            .headers(headers.clone())
            .send()
            .await?
            .text()
            .await?;

        // 正则匹配固定位置的js代码
        let re = Regex::new(r#"<script type="text/javascript">([\s\S]*?)</script>"#)?;
        let mut func = String::new();
        let mut v = "";
        for cap in re.captures_iter(&text) {
            let script = cap.get(1).ok_or(SeamError::NeedFix("script"))?.as_str();
            let re2 = Regex::new("\"([0-9]{12})\"")?;
            match re2.captures(script) {
                Some(t_cap) => {
                    v = t_cap
                        .get(1)
                        .ok_or(SeamError::NeedFix("script captures"))?
                        .as_str();
                    func = script.to_owned();
                }
                None => continue,
            }
        }

        // 将eval运行字符串更改为直接返回字符串
        let re3 = Regex::new(r"eval\(strc\)[\s\S]*?\)")?;
        let func = re3.replace_all(&func, "strc").to_string();
        let func = format!("{func}ub98484234(0,0,0)");

        // 获取eval实际运行的字符串
        let res = eval(&func).await;
        let res = res.trim_matches('"');

        // 构建函数, 替换数值
        let res = res.replace("(function", "let ccc = function");
        let res = res.replace(
            "rt;})",
            format!("rt;}}; ccc({rid}, \"{DID}\", {dt})").as_str(),
        );

        // 替换md5值避免js依赖
        let cb = format!("{rid}{DID}{dt}{v}");
        let rb = {
            let mut h = Md5::new();
            h.update(cb);
            hex::encode(h.finalize())
        };

        let res = res.replace(
            "CryptoJS.MD5(cb).toString();",
            format!("\"{}\";", &rb).as_str(),
        );

        // 运行js获取签名值
        let sign = eval(&res).await;
        let sign = sign.trim_matches('"');
        let sign = sign.rsplit_once('=').ok_or(SeamError::NeedFix("sign"))?.1;

        let mut params = HashMap::new();
        params.insert("v", v);
        params.insert("did", DID);
        params.insert("tt", dt);
        params.insert("sign", sign);

        let json = CLIENT
            .post(format!("{PLAY_URL}{rid}"))
            .form(&params)
            .headers(headers.clone())
            .send()
            .await?
            .json::<Value>()
            .await?;

        match json["error"]
            .as_i64()
            .ok_or(SeamError::NeedFix("error code"))?
        {
            0 => {
                let info = get_info(rid, headers.clone()).await?;

                let cdns = json["data"]["cdnsWithName"]
                    .as_array()
                    .ok_or(SeamError::NeedFix("cdns"))?;
                let cdns = cdns
                    .iter()
                    .map(|x| x["cdn"].as_str().unwrap_or("").to_owned())
                    .collect::<HashSet<_>>();
                let rtmp_cdn = json["data"]["rtmp_cdn"]
                    .as_str()
                    .ok_or(SeamError::NeedFix("rtmp_cdn"))?
                    .to_owned();

                let mut jsons = vec![json];

                if headers.get("cookie").is_some() {
                    for cdn in cdns {
                        if cdn == rtmp_cdn {
                            continue;
                        }
                        let mut tmp = params.clone();
                        let headers_tmp = headers.clone();
                        tmp.insert("cdn", &cdn);

                        let json = CLIENT
                            .post(format!("{PLAY_URL}{rid}"))
                            .form(&tmp)
                            .headers(headers_tmp)
                            .send()
                            .await?
                            .json::<Value>()
                            .await?;

                        jsons.push(json);
                    }
                }

                let nodes = jsons
                    .iter()
                    .map(|json| {
                        let key = json["data"]["rtmp_live"].as_str().unwrap_or("需要修复");
                        let url = json["data"]["rtmp_url"].as_str().unwrap_or("需要修复");
                        parse_url(format!("{url}/{key}"))
                    })
                    .collect::<Vec<_>>();

                Ok(Node {
                    rid: rid.to_owned(),
                    title: info.title,
                    cover: info.cover,
                    anchor: info.anchor,
                    head: info.head,
                    urls: nodes,
                })
            }
            _ => Err(SeamError::None),
        }
    }
}

struct DouyuInfo {
    title: String,
    cover: String,
    anchor: String,
    head: String,
}

async fn get_info(rid: &str, headers: HeaderMap) -> Result<DouyuInfo> {
    let json = CLIENT
        .get(format!("{BETARD_URL}{rid}"))
        .headers(headers)
        .send()
        .await?
        .json::<Value>()
        .await?;
    let title = json["room"]["room_name"]
        .as_str()
        .unwrap_or("获取失败")
        .to_string();
    let cover = json["room"]["room_pic"].as_str().unwrap_or("").to_string();
    let anchor = json["room"]["owner_name"]
        .as_str()
        .unwrap_or("")
        .to_string();
    let head = json["room"]["owner_avatar"]
        .as_str()
        .unwrap_or("")
        .to_string();
    Ok(DouyuInfo {
        title,
        cover,
        anchor,
        head,
    })
}

#[cfg(test)]
macros::gen_test!(100);
