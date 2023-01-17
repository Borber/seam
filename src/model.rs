use serde::{Serialize, Serializer};

#[derive(Serialize, Debug)]
pub enum ShowType {
    // 开播
    On(Vec<Node>),
    // 未开播
    Off,
    // 错误
    Error(String),
}

impl ShowType {
    pub fn to_string(&self) -> String {
        match self {
            ShowType::On(nodes) => serde_json::to_string_pretty(&nodes).unwrap(),
            ShowType::Off => "未开播".to_string(),
            ShowType::Error(msg) => msg.to_owned(),
        }
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
    FLV,
    M3U,
    Other(String),
}
/// 自定义序列化方法
impl Serialize for Format {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let str = match self {
            Format::FLV => "flv",
            Format::M3U => "m3u",
            Format::Other(s) => s.as_str(),
        };
        serializer.serialize_str(str)
    }
}
