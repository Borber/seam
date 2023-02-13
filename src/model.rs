use std::collections::HashMap;

use anyhow::{anyhow, Result};
use serde::{Serialize, Serializer};

/// 直播源
/// 1. 正确解析后的直播源
///
/// - rid: 直播间号
/// - title: 直播间标题
/// - nodes: 直播源列表
#[derive(Serialize, Debug, Clone, PartialEq)]
pub struct Node {
    pub rid: String,
    pub title: String,
    pub urls: Vec<Url>,
}

/// 开播状态
/// 1. 用于检测直播间是否开播
/// 2. 用于直播间初始化, 可能返回附加信息, 但此字段未固定可能被删除
///
/// - 开播状态 bool
/// - 附加信息 Option<HashMap<String, String>>
pub enum Status {
    // 开播
    On(Option<HashMap<String, String>>),
    // 附加信息
    #[allow(dead_code)]
    Not,
}

#[derive(Serialize, Debug, Clone, PartialEq)]
pub struct Url {
    // 直播源格式
    pub format: Format,
    // 直播源地址, 默认均为最高清晰度, 故而无需额外标注清晰度
    pub url: String,
}

impl Url {
    pub fn is_m3u8(&self) -> Result<String> {
        match self.format {
            Format::M3U => Ok(self.url.clone()),
            _ => Err(anyhow!("不是m3u8格式")),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Format {
    Flv,
    M3U,
    Rtmp,
    Other(String),
}
/// 自定义序列化方法
impl Serialize for Format {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let str = match self {
            Format::Flv => "flv",
            Format::M3U => "m3u",
            Format::Rtmp => "rtmp",
            Format::Other(s) => s.as_str(),
        };
        serializer.serialize_str(str)
    }
}
