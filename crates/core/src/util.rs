use boa_engine::Context;
use reqwest::header::HeaderMap;
use reqwest::header::HeaderName;
use reqwest::header::HeaderValue;

use crate::live::Format;
use crate::live::Url;
use std::collections::HashMap;
use std::str::FromStr;

/// js运行时
pub async fn eval(js: &str) -> String {
    let mut context = Context::default();
    match context.eval(js) {
        Ok(result) => result.display().to_string(),
        Err(e) => e.display().to_string(),
    }
}

pub fn match_format(url: &str) -> Format {
    if url.contains(".m3u8") {
        Format::M3U
    } else if url.contains(".flv") {
        Format::Flv
    } else if url.contains("rtmp:") {
        Format::Rtmp
    } else {
        Format::Other("unknown".to_owned())
    }
}

pub fn parse_url(url: String) -> Url {
    Url {
        format: match_format(&url),
        url: url.to_owned(),
    }
}

/// 获取当前时间
/// 格式：20230121-000000-000 (年月日-时分秒-毫秒)
#[inline]
pub fn get_datetime() -> String {
    chrono::Local::now().format("%Y%m%d-%H%M%S-%3f").to_string()
}

pub fn hash2header(map: Option<HashMap<String, String>>) -> HeaderMap {
    if let Some(map) = map {
        let mut headers = HeaderMap::new();
        for (k, v) in map.iter() {
            if let Ok(k) = HeaderName::from_str(k) {
                match v.parse() {
                    Ok(v) => {
                        headers.insert(k, v);
                    }
                    Err(_) => {
                        headers.insert(k, HeaderValue::from_static(""));
                    }
                }
            }
        }
        headers
    } else {
        HeaderMap::default()
    }
}
