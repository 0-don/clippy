use entity::hotkey::Model;
use crate::service::hotkey::get_all_hotkeys;

#[tauri::command]
pub async fn get_hotkeys() -> Result<Vec<Model>, String> {
    let res = get_all_hotkeys().await;

    Ok(res.unwrap())
}
