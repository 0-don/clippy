use crate::service::cipher::is_encryption_key_set;
use crate::service::clipboard::{filter_clipboards, get_all_clipboards_db, init_clipboards};
use crate::service::decrypt::decrypt_clipboard;
use crate::service::settings::get_global_settings;
use crate::tao::global::{get_app, get_cache};
use crate::{
    service::clipboard::{
        clear_clipboards_db, copy_clipboard_from_id, delete_clipboards_db, get_clipboard_count_db,
        get_clipboard_db, get_clipboards_db, rename_clipboard_db, star_clipboard_db,
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
use serde::Serialize;
use std::fs::File;
use tauri::ipc::Channel;
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
        // If cache is ready, use it for fast in-memory search
        if let Some(cached) = get_cache().get(CACHE_KEY) {
            let filtered =
                filter_clipboards(&cached, search.as_ref(), star, img, &settings);
            let start = cursor.unwrap_or(0) as usize;
            let end = (start + 25).min(filtered.len());
            if start < filtered.len() {
                filtered[start..end].to_vec()
            } else {
                Vec::new()
            }
        } else {
            // Cache not ready: decrypt only enough to fill the first page,
            // then populate the full cache in background for subsequent searches
            let all_clipboards = get_all_clipboards_db().await?;
            let page_size = 25usize;
            let start = cursor.unwrap_or(0) as usize;
            let needed = start + page_size;

            let mut matched: Vec<FullClipboardDto> = Vec::new();
            let mut decrypted_so_far = 0usize;

            // Decrypt only until we have enough matches for the requested page
            for clipboard in &all_clipboards {
                let mut cb = if clipboard.clipboard.encrypted {
                    match decrypt_clipboard(clipboard.clone()) {
                        Ok(decrypted) => decrypted,
                        Err(e) => {
                            printlog!("Failed to decrypt clipboard: {:?}", e);
                            clipboard.clone()
                        }
                    }
                } else {
                    clipboard.clone()
                };

                if let Some(ref mut image) = cb.image {
                    image.data = Vec::new();
                }

                decrypted_so_far += 1;

                let single = vec![cb];
                let matches =
                    filter_clipboards(&single, search.as_ref(), star, img, &settings);
                matched.extend(matches);

                if matched.len() >= needed {
                    break;
                }
            }

            // Populate full cache in background for instant subsequent searches
            let remaining = all_clipboards.len().saturating_sub(decrypted_so_far);
            if remaining > 0 {
                tokio::spawn(async move {
                    let mut all_decrypted: Vec<FullClipboardDto> =
                        Vec::with_capacity(all_clipboards.len());

                    for (i, clipboard) in all_clipboards.into_iter().enumerate() {
                        let mut cb = if clipboard.clipboard.encrypted {
                            decrypt_clipboard(clipboard.clone()).unwrap_or(clipboard)
                        } else {
                            clipboard
                        };

                        if let Some(ref mut image) = cb.image {
                            image.data = Vec::new();
                        }

                        all_decrypted.push(cb);

                        // Yield every 50 records to keep the runtime responsive
                        if (i + 1) % 50 == 0 {
                            tokio::task::yield_now().await;
                        }
                    }

                    get_cache().insert(CACHE_KEY.to_string(), all_decrypted);
                });
            }

            let end = needed.min(matched.len());
            if start < matched.len() {
                matched[start..end].to_vec()
            } else {
                Vec::new()
            }
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

#[derive(Clone, Serialize)]
#[serde(rename_all = "camelCase", tag = "event", content = "data")]
pub enum SearchEvent {
    Batch {
        clipboards: Vec<FullClipboardDto>,
    },
    Done {
        total: u64,
    },
}

#[tauri::command]
pub async fn search_clipboards(
    search: Option<String>,
    star: Option<bool>,
    img: Option<bool>,
    on_chunk: Channel<SearchEvent>,
) -> Result<(), CommandError> {
    let settings = get_global_settings();
    let is_encrypted = settings.encryption && is_encryption_key_set();
    let total = get_clipboard_count_db().await?;

    if !is_encrypted {
        let clipboards = get_clipboards_db(None, search, star, img).await?;
        on_chunk.send(SearchEvent::Batch {
            clipboards: trim_clipboard_data(clipboards),
        })?;
        on_chunk.send(SearchEvent::Done { total })?;
        return Ok(());
    }

    // Encrypted: check cache first
    if let Some(cached) = get_cache().get(CACHE_KEY) {
        let filtered = filter_clipboards(&cached, search.as_ref(), star, img, &settings);
        for chunk in filtered.chunks(100) {
            on_chunk.send(SearchEvent::Batch {
                clipboards: trim_clipboard_data(chunk.to_vec()),
            })?;
        }
        on_chunk.send(SearchEvent::Done { total })?;
        return Ok(());
    }

    // Encrypted, no cache: decrypt all, stream matches, build cache
    let all_clipboards = get_all_clipboards_db().await?;
    let mut all_decrypted: Vec<FullClipboardDto> = Vec::with_capacity(all_clipboards.len());
    let mut batch: Vec<FullClipboardDto> = Vec::new();

    for clipboard in all_clipboards {
        let mut cb = if clipboard.clipboard.encrypted {
            decrypt_clipboard(clipboard.clone()).unwrap_or(clipboard)
        } else {
            clipboard
        };

        if let Some(ref mut image) = cb.image {
            image.data = Vec::new();
        }

        let single = vec![cb.clone()];
        let matches = filter_clipboards(&single, search.as_ref(), star, img, &settings);
        if !matches.is_empty() {
            batch.extend(matches);
            if batch.len() >= 100 {
                on_chunk.send(SearchEvent::Batch {
                    clipboards: trim_clipboard_data(batch.clone()),
                })?;
                batch.clear();
            }
        }

        all_decrypted.push(cb);
    }

    if !batch.is_empty() {
        on_chunk.send(SearchEvent::Batch {
            clipboards: trim_clipboard_data(batch),
        })?;
    }

    get_cache().insert(CACHE_KEY.to_string(), all_decrypted);
    on_chunk.send(SearchEvent::Done { total })?;
    Ok(())
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
pub async fn rename_clipboard(id: Uuid, name: Option<String>) -> Result<bool, CommandError> {
    Ok(rename_clipboard_db(id, name).await?)
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
