use super::cipher::is_encryption_key_set;
use super::decrypt::decrypt_clipboard;
use super::settings::get_global_settings;
use super::sync::{get_sync_manager, get_sync_provider};
use crate::prelude::*;
use crate::tao::connection::db;
use crate::tao::global::{get_app, get_main_window};
use crate::utils::providers::uuid_to_datetime;
use chrono::NaiveDateTime;
use common::builder::keyword::KeywordBuilder;
use common::io::clipboard::trim_clipboard_data;
use common::types::enums::{ClipboardTextType, ClipboardType, Language, ListenEvent};
use common::types::orm_query::{FullClipboardDbo, FullClipboardDto};
use common::types::types::CommandError;
use entity::clipboard::{self, Model};
use entity::{
    clipboard_file, clipboard_html, clipboard_image, clipboard_rtf, clipboard_text, settings,
};
use sea_orm::prelude::Uuid;
use sea_orm::RelationTrait;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, EntityTrait, JoinType, LoaderTrait, PaginatorTrait, QueryFilter,
    QueryOrder, QuerySelect, QueryTrait,
};
use std::collections::HashMap;
use std::sync::Mutex;
use tauri::{Emitter, Manager};
use tauri_plugin_clipboard::Clipboard;
use tokio::try_join;

pub async fn load_clipboards_with_relations(
    clipboards: Vec<clipboard::Model>,
) -> Vec<FullClipboardDto> {
    let db = db().await.expect("Failed to establish connection");

    let (texts, htmls, images, rtfs, files) = try_join!(
        clipboards.load_one(clipboard_text::Entity, &db),
        clipboards.load_one(clipboard_html::Entity, &db),
        clipboards.load_one(clipboard_image::Entity, &db),
        clipboards.load_one(clipboard_rtf::Entity, &db),
        clipboards.load_many(clipboard_file::Entity, &db),
    )
    .expect("Failed to load clipboard relations");

    // Zip everything together, taking first item from each Vec or None if empty
    clipboards
        .into_iter()
        .zip(texts)
        .zip(htmls)
        .zip(images)
        .zip(rtfs)
        .zip(files)
        .map(|(((((c, t), h), i), r), f)| FullClipboardDto {
            clipboard: c,
            text: t,
            html: h,
            image: i,
            rtf: r,
            files: f,
        })
        .collect()
}

pub async fn get_clipboard_count_db() -> Result<u64, DbErr> {
    let db = db().await?;

    let count = clipboard::Entity::find().count(&db).await?;

    Ok(count)
}

pub async fn insert_clipboard_dbo(model: FullClipboardDbo) -> Result<FullClipboardDto, DbErr> {
    let db = db().await?;
    let clipboard = model.clipboard_model.insert(&db).await?;

    // Insert text if data exists
    let text = match &model.clipboard_text_model.data {
        sea_orm::ActiveValue::Set(data) if !data.is_empty() => {
            let mut text_model = model.clipboard_text_model;
            text_model.id = Set(Uuid::now_v7());
            text_model.clipboard_id = Set(clipboard.id);
            Some(text_model.insert(&db).await?)
        }
        _ => None,
    };

    // Insert html if data exists
    let html = match &model.clipboard_html_model.data {
        sea_orm::ActiveValue::Set(data) if !data.is_empty() => {
            let mut html_model = model.clipboard_html_model;
            html_model.id = Set(Uuid::now_v7());
            html_model.clipboard_id = Set(clipboard.id);
            Some(html_model.insert(&db).await?)
        }
        _ => None,
    };

    // Insert rtf if data exists
    let rtf = match &model.clipboard_rtf_model.data {
        sea_orm::ActiveValue::Set(data) if !data.is_empty() => {
            let mut rtf_model = model.clipboard_rtf_model;
            rtf_model.id = Set(Uuid::now_v7());
            rtf_model.clipboard_id = Set(clipboard.id);
            Some(rtf_model.insert(&db).await?)
        }
        _ => None,
    };

    // Insert image if data exists
    let image = match &model.clipboard_image_model.data {
        sea_orm::ActiveValue::Set(data) if !data.is_empty() => {
            let mut image_model = model.clipboard_image_model;
            image_model.id = Set(Uuid::now_v7());
            image_model.clipboard_id = Set(clipboard.id);
            Some(image_model.insert(&db).await?)
        }
        _ => None,
    };

    // Insert files if they exist
    let files = if !model.clipboard_files_model.is_empty() {
        let mut files = Vec::new();
        for mut file_model in model.clipboard_files_model {
            file_model.id = Set(Uuid::now_v7());
            file_model.clipboard_id = Set(clipboard.id);
            files.push(file_model.insert(&db).await?);
        }
        files
    } else {
        Vec::new()
    };

    Ok(FullClipboardDto {
        clipboard,
        text,
        html,
        image,
        rtf,
        files,
    })
}

pub async fn upsert_clipboard_dto(model: FullClipboardDto) -> Result<(), DbErr> {
    let db = db().await?;

    // Delete existing clipboard and all related records through cascade
    entity::clipboard::Entity::delete_by_id(model.clipboard.id)
        .exec(&db)
        .await?;

    // Insert clipboard
    entity::clipboard::ActiveModel::from(model.clipboard)
        .insert(&db)
        .await?;

    // Insert text if data exists
    match model.text {
        Some(text) if !text.data.is_empty() => Some(
            entity::clipboard_text::ActiveModel::from(text)
                .insert(&db)
                .await?,
        ),
        _ => None,
    };

    // Insert html if data exists
    match model.html {
        Some(html) if !html.data.is_empty() => Some(
            entity::clipboard_html::ActiveModel::from(html)
                .insert(&db)
                .await?,
        ),
        _ => None,
    };

    // Insert image if data exists
    match model.image {
        Some(image) if !image.data.is_empty() => Some(
            entity::clipboard_image::ActiveModel::from(image)
                .insert(&db)
                .await?,
        ),
        _ => None,
    };

    // Insert rtf if data exists
    match model.rtf {
        Some(rtf) if !rtf.data.is_empty() => Some(
            entity::clipboard_rtf::ActiveModel::from(rtf)
                .insert(&db)
                .await?,
        ),
        _ => None,
    };

    // Insert files if they exist
    if !model.files.is_empty() {
        let mut files = Vec::new();
        for file in model.files {
            files.push(
                entity::clipboard_file::ActiveModel::from(file)
                    .insert(&db)
                    .await?,
            );
        }
    }

    Ok(())
}

pub async fn get_clipboard_db(id: Uuid) -> Result<FullClipboardDto, DbErr> {
    let db = db().await?;
    let clipboard = clipboard::Entity::find_by_id(id)
        .one(&db)
        .await?
        .ok_or(DbErr::RecordNotFound("Clipboard not found".into()))?;

    Ok(load_clipboards_with_relations(vec![clipboard])
        .await
        .remove(0))
}

pub async fn get_last_clipboard_db() -> Result<FullClipboardDto, DbErr> {
    let clipboard = clipboard::Entity::find()
        .order_by_desc(clipboard::Column::Id)
        .one(&db().await?)
        .await?
        .ok_or_else(|| DbErr::RecordNotFound("last clipboard not found".to_string()))?;

    let mut dto = load_clipboards_with_relations(vec![clipboard])
        .await
        .remove(0);

    if dto.clipboard.encrypted && is_encryption_key_set() {
        if let Ok(decrypted) = decrypt_clipboard(dto.clone()) {
            dto = decrypted;
        }
    }

    Ok(dto)
}

pub async fn get_clipboards_db(
    cursor: Option<u64>,
    search: Option<String>,
    star: Option<bool>,
    img: Option<bool>,
) -> Result<Vec<FullClipboardDto>, DbErr> {
    let db = db().await?;
    let (clipboard_keywords, text_keywords) = KeywordBuilder::build_default();

    let settings = get_global_settings();

    let query = clipboard::Entity::find()
        .distinct()
        .join(JoinType::LeftJoin, clipboard::Relation::ClipboardText.def())
        .join(JoinType::LeftJoin, clipboard::Relation::ClipboardHtml.def())
        .join(
            JoinType::LeftJoin,
            clipboard::Relation::ClipboardImage.def(),
        )
        .join(JoinType::LeftJoin, clipboard::Relation::ClipboardRtf.def())
        .join(JoinType::LeftJoin, clipboard::Relation::ClipboardFile.def())
        .apply_if(star, |q, s| q.filter(clipboard::Column::Star.eq(s)))
        .apply_if(img, |q, _| {
            q.filter(clipboard::Column::Types.contains(ClipboardType::Image.to_string()))
        })
        .apply_if(search.as_ref().map(|s| s.to_lowercase()), |q, s| {
            if let Some(clip_type) = KeywordBuilder::find_clipboard_type(
                &s,
                &Language::from_iso_code(&settings.language),
                &clipboard_keywords,
            ) {
                match clip_type {
                    ClipboardType::Text => {
                        if let Some(text_type) = KeywordBuilder::find_text_type(
                            &s,
                            &Language::from_iso_code(&settings.language),
                            &text_keywords,
                        ) {
                            q.filter(clipboard_text::Column::Type.eq(text_type.to_string()))
                        } else {
                            q.filter(
                                clipboard_text::Column::Type
                                    .eq(ClipboardTextType::Text.to_string()),
                            )
                        }
                    }
                    _ => q.filter(clipboard::Column::Types.contains(clip_type.to_string())),
                }
            } else {
                // Fallback to full-text search
                q.filter(
                    clipboard_text::Column::Data
                        .contains(&s)
                        .or(clipboard_file::Column::Name.contains(&s))
                        .or(clipboard_file::Column::Extension.contains(&s))
                        .or(clipboard_file::Column::MimeType.contains(&s))
                        .or(clipboard_image::Column::Extension.contains(&s))
                        .or(clipboard_text::Column::Type.contains(&s))
                        .or(clipboard_html::Column::Data.contains(&s))
                        .or(clipboard_rtf::Column::Data.contains(&s)),
                )
            }
        })
        .apply_if(search.as_ref().map(|s| s.to_lowercase()), |q, s| {
            if let Some(clip_type) = KeywordBuilder::find_clipboard_type(
                &s,
                &Language::from_iso_code(&settings.language),
                &clipboard_keywords,
            ) {
                match clip_type {
                    ClipboardType::Text => {
                        if let Some(text_type) = KeywordBuilder::find_text_type(
                            &s,
                            &Language::from_iso_code(&settings.language),
                            &text_keywords,
                        ) {
                            q.filter(clipboard_text::Column::Type.eq(text_type.to_string()))
                        } else {
                            q.filter(
                                clipboard_text::Column::Type
                                    .eq(ClipboardTextType::Text.to_string()),
                            )
                        }
                    }
                    _ => q.filter(clipboard::Column::Types.contains(clip_type.to_string())),
                }
            } else {
                // Fallback to full-text search
                q.filter(
                    clipboard_text::Column::Data
                        .contains(&s)
                        .or(clipboard_file::Column::Name.contains(&s))
                        .or(clipboard_file::Column::Extension.contains(&s))
                        .or(clipboard_file::Column::MimeType.contains(&s))
                        .or(clipboard_image::Column::Extension.contains(&s))
                        .or(clipboard_text::Column::Type.contains(&s))
                        .or(clipboard_html::Column::Data.contains(&s))
                        .or(clipboard_rtf::Column::Data.contains(&s)),
                )
            }
        })
        .offset(cursor)
        .limit(25)
        .order_by_desc(clipboard::Column::Id);

    let clipboards = query.all(&db).await?;

    Ok(load_clipboards_with_relations(clipboards).await)
}

pub async fn get_latest_syncable_cliboards_db() -> Result<Vec<FullClipboardDto>, DbErr> {
    let db = db().await?;
    let settings = get_global_settings();

    let latest_syncable_clipboards = clipboard::Entity::find()
        .limit(settings.sync_limit as u64)
        .order_by_desc(clipboard::Column::Id)
        .all(&db)
        .await?;

    let sync_favorite_clipboards = clipboard::Entity::find()
        .filter(clipboard::Column::Star.eq(true))
        .order_by_desc(clipboard::Column::Id)
        .all(&db)
        .await?;

    let latest_syncable_clipboards_len = latest_syncable_clipboards.len();
    let sync_favorite_clipboards_len = sync_favorite_clipboards.len();

    let mut clipboards = latest_syncable_clipboards
        .into_iter()
        .chain(sync_favorite_clipboards)
        .map(|clipboard| (clipboard.id, clipboard))
        .collect::<HashMap<_, _>>() // Collects into HashMap using ID as key
        .into_values()
        .collect::<Vec<_>>();

    clipboards.sort_by(|a, b| b.id.cmp(&a.id));

    if !clipboards.is_empty() {
        let newest = &clipboards[0];
        let oldest = &clipboards[clipboards.len() - 1];
        printlog!(
            "(local) clipboards: {} favorite: {} from {} to {}",
            latest_syncable_clipboards_len,
            sync_favorite_clipboards_len,
            uuid_to_datetime(&oldest.id),
            uuid_to_datetime(&newest.id)
        );
    } else {
        printlog!(
            "(local) clipboards: {} favorite: {} (empty)",
            latest_syncable_clipboards_len,
            sync_favorite_clipboards_len
        );
    }

    Ok(load_clipboards_with_relations(clipboards).await)
}

pub async fn star_clipboard_db(id: Uuid, star: bool) -> Result<bool, CommandError> {
    let db = db().await?;

    let model = clipboard::ActiveModel {
        id: Set(id),
        star: Set(star),
        ..Default::default()
    };

    let clipboard = clipboard::Entity::update(model).exec(&db).await?;

    let settings = get_app().state::<Mutex<settings::Model>>();
    if settings.lock().expect("Failed to lock settings").sync {
        let clipboard = load_clipboards_with_relations(vec![clipboard])
            .await
            .remove(0);

        tauri::async_runtime::spawn(async move {
            let provider = get_sync_provider().await;
            provider.star_clipboard(&clipboard).await
        });
    }

    Ok(true)
}

pub async fn delete_clipboards_db(
    ids: Vec<Uuid>,
    command: Option<bool>,
) -> Result<(), CommandError> {
    let settings = get_global_settings();
    let db = db().await?;

    let result = clipboard::Entity::delete_many()
        .filter(clipboard::Column::Id.is_in(ids.clone()))
        .exec(&db)
        .await?;

    // Only spawn deletion task if records were actually deleted
    if result.rows_affected > 0 && settings.sync {
        // Get the actually deleted IDs by querying what remains
        let remaining_ids = clipboard::Entity::find()
            .filter(clipboard::Column::Id.is_in(ids.clone()))
            .select_only()
            .column(clipboard::Column::Id)
            .into_tuple()
            .all(&db)
            .await?;

        let deleted_ids: Vec<Uuid> = ids
            .into_iter()
            .filter(|id| !remaining_ids.contains(id))
            .collect();

        tauri::async_runtime::spawn(async move {
            let provider = get_sync_provider().await;

            let clipboards = provider
                .fetch_all_clipboards()
                .await
                .expect("Failed to fetch clipboards")
                .into_iter()
                .filter(|c| deleted_ids.contains(&c.id))
                .collect::<Vec<_>>();

            if command.is_none() && !clipboards.is_empty() {
                init_clipboards();
            }

            for clippy in clipboards {
                printlog!(
                    "deleting remote clipboard: {:?} command: {:?}",
                    clippy.id,
                    command
                );
                provider.mark_for_deletion(&clippy).await;
                break;
            }
        });
    }

    Ok(())
}

pub async fn clear_clipboards_db(r#type: Option<ClipboardType>) -> Result<(), DbErr> {
    let db = db().await?;
    let settings = get_global_settings();
    let mut remote_clipboards_to_delete = Vec::new();

    match r#type {
        None => {
            let clipboards_to_delete = clipboard::Entity::find()
                .filter(clipboard::Column::Star.eq(false))
                .all(&db)
                .await?;
            remote_clipboards_to_delete.extend(clipboards_to_delete);

            // Delete all non-starred clipboards
            clipboard::Entity::delete_many()
                .filter(clipboard::Column::Star.eq(false))
                .exec(&db)
                .await?;
        }
        Some(clipboard_type) => {
            // Find all non-starred clipboards that contain the specified type
            let clipboards = clipboard::Entity::find()
                .filter(
                    clipboard::Column::Star
                        .eq(false)
                        .and(clipboard::Column::Types.contains(clipboard_type.to_string())),
                )
                .all(&db)
                .await?;

            for clipboard in clipboards {
                // Delete the type-specific data first
                match clipboard_type {
                    ClipboardType::Text => {
                        clipboard_text::Entity::delete_many()
                            .filter(clipboard_text::Column::ClipboardId.eq(clipboard.id))
                            .exec(&db)
                            .await?;
                    }
                    ClipboardType::Image => {
                        clipboard_image::Entity::delete_many()
                            .filter(clipboard_image::Column::ClipboardId.eq(clipboard.id))
                            .exec(&db)
                            .await?;
                    }
                    ClipboardType::Html => {
                        clipboard_html::Entity::delete_many()
                            .filter(clipboard_html::Column::ClipboardId.eq(clipboard.id))
                            .exec(&db)
                            .await?;
                    }
                    ClipboardType::Rtf => {
                        clipboard_rtf::Entity::delete_many()
                            .filter(clipboard_rtf::Column::ClipboardId.eq(clipboard.id))
                            .exec(&db)
                            .await?;
                    }
                    ClipboardType::File => {
                        clipboard_file::Entity::delete_many()
                            .filter(clipboard_file::Column::ClipboardId.eq(clipboard.id))
                            .exec(&db)
                            .await?;
                    }
                }

                // Parse and update the types JSON array
                if let Some(mut types) = ClipboardType::from_json_value(&clipboard.types) {
                    // Remove the specified type
                    types.retain(|t| t != &clipboard_type);

                    if types.is_empty() {
                        remote_clipboards_to_delete.push(clipboard.clone());

                        // If no types remain, delete the clipboard
                        clipboard::Entity::delete_by_id(clipboard.id.clone())
                            .exec(&db)
                            .await?;
                    } else {
                        // Update the clipboard with the remaining types
                        let model = clipboard::ActiveModel {
                            id: Set(clipboard.id),
                            types: Set(ClipboardType::to_json_value(&types)),
                            ..Default::default()
                        };
                        clipboard::Entity::update(model).exec(&db).await?;
                    }
                }
            }
        }
    }

    // Handle remote deletion if sync is enabled
    if settings.sync && !remote_clipboards_to_delete.is_empty() {
        tauri::async_runtime::spawn(async move {
            let provider = get_sync_provider().await;
            let manager = get_sync_manager();

            // Fetch all remote clipboards
            let remote_clipboards = provider.fetch_all_clipboards().await.unwrap_or_default();

            // Stop the sync manager before making changes
            manager.lock().await.stop().await;

            // Filter remote clipboards that match our local ones to delete
            let remote_to_delete: Vec<_> = remote_clipboards
                .iter()
                .filter(|remote| {
                    remote_clipboards_to_delete
                        .iter()
                        .any(|local| local.id == remote.id)
                })
                .collect();

            // Delete filtered clipboards using their provider IDs
            for clippy in remote_to_delete {
                provider.mark_for_deletion(clippy).await;
            }

            // Restart the sync manager
            manager.lock().await.start().await;
        });
    }

    Ok(())
}

pub async fn count_clipboards_db() -> Result<u64, DbErr> {
    let db = db().await?;

    let count = clipboard::Entity::find().count(&db).await?;

    Ok(count)
}

pub async fn get_clipboard_uuids_db() -> Result<HashMap<Uuid, (bool, NaiveDateTime)>, DbErr> {
    let db = db().await?;

    let clipboards = clipboard::Entity::find()
        .select_only()
        .columns([
            clipboard::Column::Id,
            clipboard::Column::Star,
            clipboard::Column::CreatedAt,
        ])
        .order_by_desc(clipboard::Column::Id)
        .into_tuple()
        .all(&db)
        .await?;

    Ok(clipboards
        .into_iter()
        .map(|(id, star, created_at)| (id, (star, created_at)))
        .collect())
}

pub async fn copy_clipboard_from_index(i: u64) -> Result<Option<Model>, DbErr> {
    let db = db().await?;

    let model = clipboard::Entity::find()
        .order_by_desc(clipboard::Column::Id)
        .offset(Some(i))
        .limit(1)
        .one(&db)
        .await?;

    if model.is_none() {
        return Ok(None);
    }

    let model = model.expect("Failed to load clipboard");
    copy_clipboard_from_id(model.id, ClipboardType::Text).await?;

    Ok(Some(model))
}

pub async fn copy_clipboard_from_id(
    id: Uuid,
    requested_type: ClipboardType,
) -> Result<bool, DbErr> {
    printlog!("copy clipboard type: {:?} id:{:?}", requested_type, id);
    let mut clipboard_data = get_clipboard_db(id).await?;
    let clipboard = get_app().state::<Clipboard>();

    // Decrypt the clipboard data if it's encrypted
    if clipboard_data.clipboard.encrypted && is_encryption_key_set() {
        clipboard_data = decrypt_clipboard(clipboard_data)
            .map_err(|e| DbErr::Custom(format!("Failed to decrypt clipboard: {}", e)))?;
    }

    let success = match requested_type {
        ClipboardType::Image => clipboard_data
            .image
            .and_then(|m| clipboard.write_image_binary(m.data).ok()),
        ClipboardType::Text => clipboard_data
            .text
            .and_then(|m| clipboard.write_text(m.data).ok()),
        ClipboardType::Html => clipboard_data
            .html
            .and_then(|m| clipboard.write_html(m.data).ok()),
        ClipboardType::Rtf => clipboard_data
            .rtf
            .and_then(|m| clipboard.write_rtf(m.data).ok()),
        ClipboardType::File => Some(
            clipboard_data
                .files
                .iter()
                .filter_map(|f| {
                    let path = std::env::temp_dir().join(format!(
                        "{}.{}",
                        &f.name,
                        f.extension.as_ref().expect("Failed to get file extension")
                    ));
                    std::fs::write(&path, &f.data).ok()?;
                    Some(if cfg!(windows) {
                        path.to_string_lossy().replace('/', "\\")
                    } else {
                        format!("file://{}", path.to_string_lossy())
                    })
                })
                .collect::<Vec<_>>(),
        )
        .filter(|f| !f.is_empty())
        .and_then(|f| clipboard.write_files_uris(f).ok()),
    }
    .is_some();

    if success && !cfg!(debug_assertions) {
        get_main_window().hide().ok();
    }

    Ok(success)
}

pub fn init_clipboards() {
    get_main_window()
        .emit(ListenEvent::InitClipboards.to_string().as_str(), ())
        .expect("Failed to emit event");
}

pub fn new_clipboard_event(mut clipboard: FullClipboardDto) {
    if clipboard.clipboard.encrypted && is_encryption_key_set() {
        clipboard = decrypt_clipboard(clipboard).expect("Failed to decrypt clipboard");
    }

    get_main_window()
        .emit(
            ListenEvent::NewClipboard.to_string().as_str(),
            trim_clipboard_data(vec![clipboard]).remove(0),
        )
        .expect("Failed to emit event");
}
