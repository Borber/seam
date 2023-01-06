use crate::modle::ShowType;
use md5::{Digest, Md5};

pub fn match_show_type(t: ShowType) {
    match t {
        ShowType::On(nodes) => {
            println!("{}", serde_json::to_string_pretty(&nodes).unwrap());
        }
        ShowType::Off => println!("未开播"),
        ShowType::Error(msg) => println!("{msg}"),
    }
}

/// 提取字符串md5值
pub fn md5(data: &[u8]) -> String {
    let mut h = Md5::new();
    h.update(data);
    hex::encode(h.finalize())
}
