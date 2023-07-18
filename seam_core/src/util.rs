use reqwest::header::HeaderMap;
use reqwest::header::HeaderName;

use crate::live::Format;
use crate::live::Url;
use std::collections::HashMap;
use std::path::PathBuf;
use std::str::FromStr;

/// js运行时
pub async fn eval(js: &str) -> String {
    let plugin_path = get_plugin_path();
    // 调用命令执行并返回字符串
    let output = tokio::process::Command::new(plugin_path)
        .arg(js)
        .output()
        .await
        .unwrap();
    String::from_utf8(output.stdout).unwrap()
}

// 获取插件地址
pub fn get_plugin_path() -> PathBuf {
    let exe_path = std::env::current_exe().unwrap();
    let exe_dir = exe_path.parent().unwrap();
    // 通过系统定义插件后缀
    #[cfg(windows)]
    let plugin_file = "jin.exe";
    #[cfg(not(windows))]
    let plugin_file = "jin";
    exe_dir.join(plugin_file)
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

pub fn hash2header(map: &Option<HashMap<String, String>>) -> HeaderMap {
    if let Some(map) = map {
        let mut headers = HeaderMap::new();
        for (k, v) in map {
            if let Ok(k) = HeaderName::from_str(k) {
                headers.insert(k, v.parse().unwrap());
            }
        }
        headers
    } else {
        HeaderMap::default()
    }
}
