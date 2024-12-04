extern crate alloc;
extern crate image;
use crate::{
    printlog,
    service::clipboard::{
        clear_clipboards_db, copy_clipboard_from_id, delete_clipboard_db, get_clipboard_db,
        get_clipboards_db, star_clipboard_db,
    },
    types::orm_query::ClipboardWithRelations,
    utils::{hotkey_manager::unregister_hotkeys, tauri::config::APP},
};
use std::fs::File;
use tauri::Manager;

#[tauri::command]
pub async fn get_clipboards(
    cursor: Option<u64>,
    search: Option<String>,
    star: Option<bool>,
    img: Option<bool>,
) -> Result<Vec<ClipboardWithRelations>, ()> {
    printlog!("get_clipboards");
    let clipboards_result = get_clipboards_db(cursor, search, star, img).await;
    if clipboards_result.is_err() {
        printlog!("get_clipboards error {:?}", clipboards_result);
        return Err(());
    }
    let clipboard = clipboards_result.unwrap();
    printlog!("get_clipboards: {:?}", clipboard);
    Ok(clipboard)
}

#[tauri::command]
pub async fn copy_clipboard(id: i32) -> Result<(), ()> {
    let _ = copy_clipboard_from_id(id).await;
    unregister_hotkeys(false);
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
pub async fn save_clipboard_image(id: i32) -> Result<bool, ()> {
    let clipboard = get_clipboard_db(id).await.unwrap();

    let image = image::load_from_memory(&clipboard.image.unwrap().data).unwrap();

    // Create a path for the new image file on the desktop
    let image_path = APP
        .get()
        .unwrap()
        .path()
        .desktop_dir()
        .unwrap()
        .join(format!("clipboard-{}.png", id));

    // Save the image to the desktop
    let mut file = File::create(image_path).unwrap();
    image.write_to(&mut file, image::ImageFormat::Png).unwrap();

    Ok(true)
}
