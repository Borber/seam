#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use common::GLOBAL_CLIENT;
use resp::Resp;
use seam_core::live::Node;
use tauri::Manager;
use window_shadows::set_shadow;

mod common;
mod resp;

#[tauri::command]
async fn url(live: String, rid: String) -> Resp<Node> {
    match GLOBAL_CLIENT.get(&live).unwrap().get(&rid, &None).await {
        Ok(node) => Resp::success(node),
        Err(e) => Resp::fail(1, &e.to_string()),
    }
}

#[tokio::main]
async fn main() {
    tauri::Builder::default()
        .setup(|app| {
            let window = app.get_window("main").unwrap();
            set_shadow(&window, true).expect("Unsupported platform!");
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![url])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
