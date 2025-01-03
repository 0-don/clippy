extern crate alloc;
extern crate image;
use crate::prelude::*;
use crate::{
    service::{
        clipboard::{
            clear_clipboards_db, copy_clipboard_from_id, delete_clipboard_db,
            get_clipboard_count_db, get_clipboard_db, get_clipboards_db, star_clipboard_db,
        },
        global::get_main_window,
    },
    tauri_config::config::APP,
    utils::hotkey_manager::unregister_hotkeys,
};
use common::io::clipboard::trim_clipboard_data;
use common::{
    printlog,
    types::{
        enums::{ClipboardType, ListenEvent},
        orm_query::ClipboardsResponse,
        types::CommandError,
    },
};
use sea_orm::prelude::Uuid;
use std::fs::File;
use tauri::{Emitter, Manager};

#[tauri::command]
pub async fn get_clipboards(
    cursor: Option<u64>,
    search: Option<String>,
    star: Option<bool>,
    img: Option<bool>,
) -> Result<ClipboardsResponse, CommandError> {
    let clipboards = get_clipboards_db(cursor, search, star, img)
        .await
        .expect("Error getting clipboards");

    let total = get_clipboard_count_db().await?;

    // Calculate if there are more items
    let current_position = cursor.unwrap_or(0) + clipboards.len() as u64;
    let has_more = current_position < total;

    printlog!(
        "Total: {}, Current Position: {}, Has More: {}",
        total,
        current_position,
        has_more
    );

    Ok(ClipboardsResponse {
        clipboards: trim_clipboard_data(clipboards),
        total,
        has_more,
    })
}

#[tauri::command]
pub async fn copy_clipboard(id: Uuid, r#type: ClipboardType) -> Result<bool, CommandError> {
    unregister_hotkeys(false);
    Ok(copy_clipboard_from_id(id, r#type).await?)
}

#[tauri::command]
pub async fn star_clipboard(id: Uuid, star: bool) -> Result<bool, CommandError> {
    Ok(star_clipboard_db(id, star).await?)
}

#[tauri::command]
pub async fn delete_clipboard(id: Uuid) -> Result<(), CommandError> {
    delete_clipboard_db(id).await?;
    Ok(())
}

#[tauri::command]
pub async fn clear_clipboards(r#type: Option<ClipboardType>) -> Result<(), CommandError> {
    clear_clipboards_db(r#type).await?;
    get_main_window()
        .emit(ListenEvent::Init.to_string().as_str(), ())
        .expect("Failed to emit");
    Ok(())
}

#[tauri::command]
pub async fn save_clipboard_image(id: Uuid) -> Result<(), CommandError> {
    let clipboard = get_clipboard_db(id).await?;

    let extension = clipboard
        .image
        .as_ref()
        .and_then(|img| img.extension.clone())
        .unwrap_or_else(|| "png".to_string());

    let image = image::load_from_memory(
        &clipboard
            .image
            .ok_or(CommandError::Error(
                "No image data found in clipboard".to_string(),
            ))?
            .data,
    )?;

    // Create a path for the new image file on the desktop
    let image_path = APP
        .get()
        .ok_or(CommandError::Error("No app handle found".to_string()))?
        .path()
        .desktop_dir()?
        .join(format!("clipboard-{}.{}", id, extension));

    // Convert extension to ImageFormat
    let format = match extension.to_lowercase().as_str() {
        "jpg" | "jpeg" => image::ImageFormat::Jpeg,
        "png" => image::ImageFormat::Png,
        "gif" => image::ImageFormat::Gif,
        "bmp" => image::ImageFormat::Bmp,
        "webp" => image::ImageFormat::WebP,
        _ => image::ImageFormat::Png, // Default to PNG if unknown format
    };

    // Save the image to the desktop with the correct format
    let mut file = File::create(image_path)?;
    image.write_to(&mut file, format)?;

    Ok(())
}
