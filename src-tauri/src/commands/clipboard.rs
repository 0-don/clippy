extern crate alloc;
extern crate image;
use crate::{
    printlog,
    service::clipboard::{
        clear_clipboards_db, copy_clipboard_from_id, delete_clipboard_db, get_clipboard_db,
        get_clipboards_db, star_clipboard_db,
    },
    types::{orm_query::ClipboardWithRelations, types::CommandError},
    utils::{hotkey_manager::unregister_hotkeys, tauri::config::APP},
};
use migration::ClipboardType;
use std::fs::File;
use tauri::Manager;

#[tauri::command]
pub async fn get_clipboards(
    cursor: Option<u64>,
    search: Option<String>,
    star: Option<bool>,
    img: Option<bool>,
) -> Result<Vec<ClipboardWithRelations>, CommandError> {
    printlog!("get_clipboards");
    Ok(get_clipboards_db(cursor, search, star, img).await?)
}

#[tauri::command]
pub async fn copy_clipboard(id: i32, r#type: ClipboardType) -> Result<bool, CommandError> {
    printlog!("type {:?}", r#type);
    unregister_hotkeys(false);
    Ok(copy_clipboard_from_id(id, r#type).await?)
}

#[tauri::command]
pub async fn star_clipboard(id: i32, star: bool) -> Result<bool, CommandError> {
    Ok(star_clipboard_db(id, star).await?)
}

#[tauri::command]
pub async fn delete_clipboard(id: i32) -> Result<bool, CommandError> {
    Ok(delete_clipboard_db(id).await?)
}

#[tauri::command]
pub async fn clear_clipboards() -> Result<bool, CommandError> {
    Ok(clear_clipboards_db().await?)
}

#[tauri::command]
pub async fn save_clipboard_image(id: i32) -> Result<bool, CommandError> {
    let clipboard = get_clipboard_db(id).await?;

    let image = image::load_from_memory(
        &clipboard
            .image
            .ok_or(CommandError::Option(
                "No image data found in clipboard".to_string(),
            ))?
            .data,
    )?;

    // Create a path for the new image file on the desktop
    let image_path = APP
        .get()
        .ok_or(CommandError::Option("No app handle found".to_string()))?
        .path()
        .desktop_dir()?
        .join(format!("clipboard-{}.png", id));

    // Save the image to the desktop
    let mut file = File::create(image_path)?;
    image.write_to(&mut file, image::ImageFormat::Png)?;

    Ok(true)
}
