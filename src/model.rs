use std::fmt::{Display, Formatter, Result as FmtResult};

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

impl Display for ShowType {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match self {
            ShowType::On(nodes) => write!(f, "{}", serde_json::to_string_pretty(&nodes).unwrap()),
            ShowType::Off => write!(f, "未开播"),
            ShowType::Error(msg) => write!(f, "{}", msg),
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

#[derive(Serialize, Debug)]
pub struct Node {
    // 直播源格式
    pub format: Format,
    // 直播源地址, 默认均为最高清晰度, 故而无需额外标注清晰度
    pub url: String,
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
