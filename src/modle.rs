use serde::Serialize;

#[derive(Debug)]
pub enum ShowType {
    /// 返回直播源地址
    On(Vec<Node>),
    Off,
    Error(String),
}

#[derive(Serialize, Debug)]
pub struct Node {
    /// 清晰度
    pub rate: String,
    /// 直播源地址 后续考虑使用数组来安装
    pub url: String,
}
