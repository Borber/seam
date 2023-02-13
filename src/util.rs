use boa_engine::Context;
use md5::{Digest, Md5};

use crate::model::{Format, Url};

/// 提取字符串md5值
#[allow(dead_code)]
pub fn md5(data: &[u8]) -> String {
    let mut h = Md5::new();
    h.update(data);
    hex::encode(h.finalize())
}

/// 运行js代码
#[allow(dead_code)]
pub async fn eval(js: &str) -> String {
    let mut context = Context::default();
    context.eval(js).unwrap().as_string().unwrap().to_string()
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

#[cfg(windows)]
const SEPARATOR: &str = "\\";

#[cfg(not(windows))]
const SEPARATOR: &str = "/";

pub fn bin_dir() -> String {
    let p = std::env::current_exe()
        .unwrap()
        .parent()
        .unwrap()
        .to_str()
        .unwrap()
        .to_owned();
    format!("{p}{SEPARATOR}")
}

/// 获取当前时间
/// 格式：20230121-000000-000 (年月日-时分秒-毫秒)
#[inline]
pub fn get_datetime() -> String {
    chrono::Local::now().format("%Y%m%d-%H%M%S-%3f").to_string()
}
