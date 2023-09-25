use crate::{database, resp::Resp, service};

#[tauri::command]
pub async fn subscribe_all() -> Resp<Vec<database::subscribe::Model>> {
    service::subscribe::all().await.into()
}

#[tauri::command]
pub async fn subscribe_add(live: String, rid: String) -> Resp<bool> {
    service::subscribe::add(live, rid).await.into()
}

#[tauri::command]
pub async fn subscribe_remove(live: String, rid: String) -> Resp<bool> {
    service::subscribe::remove(live, rid).await.into()
}
