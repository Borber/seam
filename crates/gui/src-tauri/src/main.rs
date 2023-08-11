#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use common::GLOBAL_CLIENT;
use resp::Resp;
use seam_core::{error::SeamError, live::Node};
use tauri::Manager;
use window_shadows::set_shadow;

mod common;
mod config;
mod resp;
mod util;

#[tauri::command]
async fn url(live: String, rid: String) -> Resp<Node> {
    let cli = match GLOBAL_CLIENT.get(&live) {
        Some(cli) => cli,
        None => return Resp::fail(0, "No such live"),
    };
    match cli.get(&rid, Some(config::headers(&live))).await {
        Ok(node) => Resp::success(node),
        Err(e) => match e {
            SeamError::None => Resp::fail(1, "Not on"),
            SeamError::NeedFix(msg) => Resp::fail(2, msg),
            _ => Resp::fail(3, &e.to_string()),
        },
    }
}

#[tauri::command]
async fn play(url: String) -> Resp<bool> {
    util::play(&url).into()
}

#[tokio::main]
async fn main() {
    tauri::Builder::default()
        .setup(|app| {
            if cfg!(any(target_os = "macos", target_os = "windows")) {
                let window = app.get_window("main").unwrap();
                set_shadow(&window, true).expect("Unknow error in the macos or windows platform");
            }
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![url, play])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
