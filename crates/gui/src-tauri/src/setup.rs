use std::error::Error;

use tauri::App;

pub fn handler(app: &mut App) -> Result<(), Box<dyn Error>> {
    Ok(())
}
