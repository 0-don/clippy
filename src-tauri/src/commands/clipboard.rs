use entity::clipboard::Model;

use crate::service::clipboard::{delete_clipboard_db, get_clipboards_db, star_clipboard_db};

#[tauri::command]
pub async fn infinite_scroll_clipboards(
    cursor: Option<u64>,
    search: Option<String>,
    star: Option<bool>,
) -> Result<Vec<Model>, ()> {
    let clipboards = get_clipboards_db(cursor, search, star).await;

    Ok(clipboards.unwrap())
}

#[tauri::command]
pub async fn star_clipboard(id: i32, star: bool) -> Result<Option<bool>, ()> {
    let clipboards = star_clipboard_db(id, star).await;

    Ok(clipboards.unwrap())
}

#[tauri::command]
pub async fn delete_clipboard(id: i32) -> Result<Option<bool>, ()> {
    let clipboards = delete_clipboard_db(id).await;

    Ok(clipboards.unwrap())
}
