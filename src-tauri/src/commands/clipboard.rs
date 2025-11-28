use crate::service::cipher::is_encryption_key_set;
use crate::service::clipboard::{filter_clipboards, get_all_clipboards_db, init_clipboards};
use crate::service::decrypt::decrypt_clipboard;
use crate::service::settings::get_global_settings;
use crate::tao::global::{get_app, get_cache};
use crate::{
    service::clipboard::{
        clear_clipboards_db, copy_clipboard_from_id, delete_clipboards_db, get_clipboard_count_db,
        get_clipboard_db, get_clipboards_db, star_clipboard_db,
    },
    utils::hotkey_manager::unregister_hotkeys,
};
use common::constants::CACHE_KEY;
use common::io::clipboard::trim_clipboard_data;
use common::types::orm_query::FullClipboardDto;
use common::{
    printlog,
    types::{enums::ClipboardType, orm_query::ClipboardsResponse, types::CommandError},
};
use sea_orm::prelude::Uuid;
use std::fs::File;
use tauri::Manager;

#[tauri::command]
pub async fn get_clipboards(
    cursor: Option<u64>,
    search: Option<String>,
    star: Option<bool>,
    img: Option<bool>,
) -> Result<ClipboardsResponse, CommandError> {
    printlog!(
        "Getting clipboards with cursor: {:?}, search: {:?}, star: {:?}, img: {:?}",
        cursor,
        search,
        star,
        img
    );

    let settings = get_global_settings();
    let is_encrypted = settings.encryption && is_encryption_key_set();
    let total = get_clipboard_count_db().await?;

    // Only use cache for encrypted clipboards WITH a search term
    let clipboards = if is_encrypted && search.is_some() && !search.as_ref().unwrap().is_empty() {
        // Get or populate the cache
        let all_decrypted = if let Some(cached) = get_cache().get(CACHE_KEY) {
            cached
        } else {
            // No cache hit, load and decrypt all clipboards
            let all_clipboards = get_all_clipboards_db().await?;

            // Exclude images from text search (when img filter is NOT explicitly enabled)
            // This drastically reduces memory usage and improves performance
            let clipboards_to_search = if img.is_none() || img == Some(false) {
                all_clipboards
                    .into_iter()
                    .filter(|cb| cb.image.is_none())
                    .collect::<Vec<_>>()
            } else {
                all_clipboards
            };

            // Decrypt all clipboards (excluding images for text search)
            let decrypted_clipboards: Vec<FullClipboardDto> = clipboards_to_search
                .into_iter()
                .map(|clipboard| {
                    if clipboard.clipboard.encrypted {
                        match decrypt_clipboard(clipboard.clone()) {
                            Ok(decrypted) => decrypted,
                            Err(e) => {
                                printlog!("Failed to decrypt clipboard: {:?}", e);
                                clipboard
                            }
                        }
                    } else {
                        clipboard
                    }
                })
                .collect();

            get_cache().insert(CACHE_KEY.to_string(), decrypted_clipboards.clone());
            decrypted_clipboards
        };

        // Apply filters in memory
        let filtered = filter_clipboards(&all_decrypted, search.as_ref(), star, img, &settings);

        // Apply pagination
        let start = cursor.unwrap_or(0) as usize;
        let end = (start + 25).min(filtered.len());

        if start < filtered.len() {
            filtered[start..end].to_vec()
        } else {
            Vec::new()
        }
    } else {
        // For regular search (non-encrypted OR encrypted without search string)
        // we use the standard database query
        let clipboards_from_db = get_clipboards_db(cursor, search, star, img).await?;

        // If encrypted, we still need to decrypt the results
        if is_encrypted {
            clipboards_from_db
                .into_iter()
                .map(|clipboard| {
                    if clipboard.clipboard.encrypted {
                        match decrypt_clipboard(clipboard.clone()) {
                            Ok(decrypted) => decrypted,
                            Err(e) => {
                                printlog!("Failed to decrypt clipboard: {:?}", e);
                                clipboard
                            }
                        }
                    } else {
                        clipboard
                    }
                })
                .collect()
        } else {
            clipboards_from_db
        }
    };

    let current_position = cursor.unwrap_or(0) + clipboards.len() as u64;
    let has_more = current_position < total;

    printlog!(
        "Total: {}, Current Position: {}, Has More: {}",
        total,
        current_position,
        has_more
    );

    // Note: The database search results are already trimmed in trim_clipboard_data
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
    delete_clipboards_db(vec![id], Some(true)).await?;
    Ok(())
}

#[tauri::command]
pub async fn clear_clipboards(r#type: Option<ClipboardType>) -> Result<(), CommandError> {
    clear_clipboards_db(r#type).await?;
    init_clipboards();
    Ok(())
}

#[tauri::command]
pub async fn save_clipboard_image(id: Uuid) -> Result<(), CommandError> {
    let clipboard = get_clipboard_db(id).await?;

    let extension = clipboard
        .image
        .as_ref()
        .map(|img| img.extension.clone())
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
    let image_path = get_app()
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
