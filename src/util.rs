use crate::{
    common::{CLIENT, DO_JS_URL},
    modle::ShowType,
};
use md5::{Digest, Md5};
use serde_json::json;

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

// TODO 报错信息显示
/// js在线运行时
pub async fn do_js(js: &str) -> String {
    let json = json!({ "js": js });
    CLIENT
        .post(DO_JS_URL)
        .json(&json)
        .send()
        .await
        .expect("msg1")
        .text()
        .await
        .expect("msg2")
}
