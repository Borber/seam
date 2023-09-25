#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use command::{
    live::{play, url},
    refresh::{refresh_all, refresh_one},
    subscribe::{subscribe_add, subscribe_all, subscribe_remove},
};
use common::CONTEXT;

mod command;
mod common;
mod config;
mod database;
mod manager;
mod model;
mod resp;
mod service;
mod setup;
mod util;

#[tokio::main]
async fn main() {
    CONTEXT.get_or_init(common::load).await;

    tauri::Builder::default()
        .setup(setup::handler)
        .invoke_handler(tauri::generate_handler![
            url,
            play,
            subscribe_all,
            subscribe_add,
            subscribe_remove,
            refresh_all,
            refresh_one,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
