use std::error::Error;

use tauri::{App, Manager};
use window_shadows::set_shadow;

/// Tauri 启动
pub fn handler(app: &mut App) -> Result<(), Box<dyn Error>> {
    if cfg!(any(target_os = "macos", target_os = "windows")) {
        let window = app.get_window("main").unwrap();
        set_shadow(&window, true).expect("Unknow error in the macos or windows platform");
    }
    Ok(())
}
