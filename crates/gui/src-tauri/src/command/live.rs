use seam_core::{error::SeamError, live::Node};

use crate::{clients, config::headers, resp::Resp, util};

#[tauri::command]
pub async fn url(live: String, rid: String) -> Resp<Node> {
    let cli = match clients!().get(&live) {
        Some(cli) => cli,
        None => return Resp::fail(0, "目前不支持该平台"),
    };
    match cli.get(&rid, Some(headers(&live))).await {
        Ok(node) => Resp::success(node),
        Err(e) => match e {
            SeamError::None => Resp::fail(1, "未开播"),
            SeamError::NeedFix(msg) => Resp::fail(2, msg),
            _ => Resp::fail(3, &e.to_string()),
        },
    }
}

#[tauri::command]
pub async fn play(url: String) -> Resp<bool> {
    util::play(&url).into()
}
