#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use common::CONTEXT;
use resp::Resp;
use seam_core::{error::SeamError, live::Node};
use tauri::Manager;
use window_shadows::set_shadow;

mod common;
mod config;
mod database;
mod model;
mod resp;
mod service;
mod util;

#[tauri::command]
async fn url(live: String, rid: String) -> Resp<Node> {
    let cli = match clients!().get(&live) {
        Some(cli) => cli,
        None => return Resp::fail(0, "目前不支持该平台"),
    };
    match cli.get(&rid, Some(config::headers(&live))).await {
        Ok(node) => Resp::success(node),
        Err(e) => match e {
            SeamError::None => Resp::fail(1, "未开播"),
            SeamError::NeedFix(msg) => Resp::fail(2, msg),
            _ => Resp::fail(3, &e.to_string()),
        },
    }
}

#[tauri::command]
async fn add(live: String, rid: String) -> Resp<bool> {
    println!("live: {}, rid: {}", live, rid);
    Resp::success(false)
}

#[tauri::command]
async fn play(url: String) -> Resp<bool> {
    util::play(&url).into()
}

#[tokio::main]
async fn main() {
    CONTEXT.get_or_init(common::load).await;

    tauri::Builder::default()
        .setup(|app| {
            if cfg!(any(target_os = "macos", target_os = "windows")) {
                let window = app.get_window("main").unwrap();
                set_shadow(&window, true).expect("Unknow error in the macos or windows platform");
            }
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![url, add, play,])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
