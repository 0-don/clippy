use entity::clipboard::Model;

use crate::service::clipboard::{delete_clipboard_db, get_clipboards};

#[tauri::command]
pub async fn infinite_scroll_clipboards(
    cursor: Option<u64>,
    search: Option<String>,
    star: Option<bool>,
) -> Result<Vec<Model>, ()> {
    let clipboards = get_clipboards(cursor, search, star).await;

    Ok(clipboards.unwrap())
}

#[tauri::command]
pub async fn delete_clipboard(id: i32) -> Result<Option<bool>, ()> {
    let clipboards = delete_clipboard_db(id).await;

    Ok(clipboards.unwrap())
}
