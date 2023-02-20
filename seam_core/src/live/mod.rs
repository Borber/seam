//! 直播相关模块。
//!
//! 本模块提供了标准化的直播获取方式和直播状态检测的async trait 以及
//! 标准化的直播源信息和直播状态enum

use async_trait::async_trait;
use serde::{Serialize, Serializer};

use crate::error::{Result, SeamError};

pub mod bili;
pub mod cc;

/// 直播信息模块
#[async_trait]
pub trait Live {
    // 获取直播源
    // rid: 直播间号
    async fn get(rid: &str) -> Result<Node>;
}

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

impl Node {
    pub fn json(&self) -> String {
        serde_json::to_string_pretty(&self).unwrap_or("序列化失败".to_owned())
    }
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
            _ => Err(SeamError::Type("not m3u8".to_string())),
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
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
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
