#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use common::CONTEXT;
use resp::Resp;
use seam_core::{error::SeamError, live::Node};
use tauri::{App, Manager};
use window_shadows::set_shadow;

mod common;
mod config;
mod database;
mod manager;
mod model;
mod resp;
mod service;
mod setup;
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
async fn all_subscribe() -> Resp<Vec<database::subscribe::Model>> {
    service::subscribe::all().await.into()
}

#[tauri::command]
async fn add_subscribe(live: String, rid: String) -> Resp<bool> {
    service::subscribe::add(live, rid).await.into()
}

#[tauri::command]
async fn remove_subscribe(live: String, rid: String) -> Resp<bool> {
    service::subscribe::remove(live, rid).await.into()
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
        .invoke_handler(tauri::generate_handler![
            url,
            all_subscribe,
            add_subscribe,
            remove_subscribe,
            play,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
