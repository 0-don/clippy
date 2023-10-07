extern crate alloc;
use crate::{service::clipboard::{
    clear_clipboards_db, copy_clipboard_from_id, delete_clipboard_db, get_clipboards_db,
    star_clipboard_db,
}, utils::clipboard_manager::type_last_clipboard};
use entity::clipboard::Model;

#[tauri::command]
pub async fn get_clipboards(
    cursor: Option<u64>,
    search: Option<String>,
    star: Option<bool>,
    img: Option<bool>,
) -> Result<Vec<Model>, ()> {
    let clipboards = get_clipboards_db(cursor, search, star, img).await;
    Ok(clipboards.unwrap())
}

#[tauri::command]
pub async fn copy_clipboard(id: i32) -> Result<(), ()> {
    let _ = copy_clipboard_from_id(id).await;
    Ok(())
}

#[tauri::command]
pub async fn star_clipboard(id: i32, star: bool) -> Result<bool, ()> {
    let clipboard = star_clipboard_db(id, star).await;

    Ok(clipboard.unwrap())
}

#[tauri::command]
pub async fn delete_clipboard(id: i32) -> Result<bool, ()> {
    let clipboard = delete_clipboard_db(id).await;

    Ok(clipboard.unwrap())
}

#[tauri::command]
pub async fn clear_clipboards() -> Result<bool, ()> {
    let deleted = clear_clipboards_db().await;

    Ok(deleted.unwrap())
}

#[tauri::command]
pub async fn type_clipboard() -> Result<bool, ()> {
    type_last_clipboard().await;

    Ok(true)
}
