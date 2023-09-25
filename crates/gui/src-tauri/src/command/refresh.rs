use tauri::AppHandle;

use crate::{manager, resp::Resp};

#[tauri::command]
pub async fn refresh_all(app: AppHandle) -> Resp<()> {
    manager::refresh::all(&app).await.into()
}

#[tauri::command]
pub async fn refresh_one(app: AppHandle, live: String, rid: String) -> Resp<()> {
    manager::refresh::one(&app, live, rid).await.into()
}
