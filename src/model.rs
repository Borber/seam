use std::collections::HashMap;
use std::fmt::{Display, Formatter, Result as FmtResult};
use std::ops::Deref;

use anyhow::{anyhow, Result};
use serde::{Serialize, Serializer};

/// 直播源
/// 1. 正确解析后的直播源
///
/// - rid: 直播间号
/// - title: 直播间标题
/// - nodes: 直播源列表
#[derive(Serialize, Debug)]
pub struct Node {
    pub rid: String,
    pub title: String,
    pub urls: Vec<Url>,
}

impl Deref for Node {
    type Target = Vec<Url>;

    fn deref(&self) -> &Self::Target {
        &self.urls
    }
}

#[derive(Serialize, Debug)]
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

#[derive(Debug)]
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_on() {
        let detail = Detail::new("title".to_string(), vec![]);
        let show_type = ShowType::On(detail);
        assert!(show_type.is_on());

        let show_type = ShowType::Off;
        assert!(!show_type.is_on());

        let show_type = ShowType::Error("error".to_string());
        assert!(!show_type.is_on());
    }

    #[test]
    fn test_is_bad_rid() {
        let detail = Detail::new("title".to_string(), vec![]);
        let show_type = ShowType::On(detail);
        assert!(!show_type.is_bad_rid());

        let show_type = ShowType::Off;
        assert!(!show_type.is_bad_rid());

        let show_type = ShowType::Error("error".to_string());
        assert!(show_type.is_bad_rid());
    }
}
