use serde::Serialize;

#[derive(Debug)]
pub enum ShowType {
    /// 返回直播源地址
    On(Vec<String>),
    Off,
    Error(String),
}

#[derive(Serialize, Debug)]
pub struct On {
    /// 房间id
    rid: String,
    /// 直播源地址
    url: String,
}

/// 返回数据
#[derive(Serialize, Debug)]
pub struct Data {
    on: Vec<On>,
}