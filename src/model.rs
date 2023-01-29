use std::fmt::{Display, Formatter, Result as FmtResult};
use std::ops::Deref;

use anyhow::{anyhow, Result};
use serde::{Serialize, Serializer};

#[derive(Serialize, Debug)]
pub enum ShowType {
    // 开播
    On(Detail),
    // 未开播
    Off,
    // 错误
    Error(String),
}

impl ShowType {
    pub fn get_room_title(&self) -> Option<&str> {
        match self {
            ShowType::On(detail) => Some(detail.title.as_str()),
            _ => None,
        }
    }

    pub fn is_on(&self) -> bool {
        matches!(self, ShowType::On(_))
    }

    pub fn is_bad_rid(&self) -> bool {
        matches!(self, ShowType::Error(_))
    }
}

impl Display for ShowType {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match self {
            ShowType::On(nodes) => write!(f, "{}", serde_json::to_string_pretty(&nodes).unwrap()),
            ShowType::Off => write!(f, "未开播"),
            ShowType::Error(msg) => write!(f, "{msg}"),
        }
    }
}

#[derive(Serialize, Debug)]
pub struct Detail {
    pub title: String,
    pub nodes: Vec<Node>,
}

impl Detail {
    pub fn new(title: String, nodes: Vec<Node>) -> Self {
        Self { title, nodes }
    }
}

impl Deref for Detail {
    type Target = Vec<Node>;

    fn deref(&self) -> &Self::Target {
        &self.nodes
    }
}

#[derive(Serialize, Debug)]
pub struct Node {
    // 直播源格式
    pub format: Format,
    // 直播源地址, 默认均为最高清晰度, 故而无需额外标注清晰度
    pub url: String,
}

impl Node {
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
