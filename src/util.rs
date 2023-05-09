use std::path::PathBuf;

use crate::model::{Format, Node};
use md5::{Digest, Md5};

/// 提取字符串md5值
pub fn md5(data: &[u8]) -> String {
    let mut h = Md5::new();
    h.update(data);
    hex::encode(h.finalize())
}

// TODO 提取js runtime
/// js在线运行时
pub async fn do_js(js: &str) -> String {
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

pub fn parse_url(url: String) -> Node {
    Node {
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
