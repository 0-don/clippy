use crate::service::global::{get_app, get_main_window};
use crate::{connection, prelude::*};
use crate::{service::sync::SyncProvider, utils::providers::google_drive::GoogleDriveProvider};
use chrono::NaiveDateTime;
use common::builder::keyword::KeywordBuilder;
use common::types::enums::{ClipboardTextType, ClipboardType, Language};
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
use std::sync::Arc;
use tauri::Manager;
use tauri_plugin_clipboard::Clipboard;
use tokio::try_join;

pub async fn load_clipboards_with_relations(
    clipboards: Vec<clipboard::Model>,
) -> Vec<FullClipboardDto> {
    let db = connection::db()
        .await
        .expect("Failed to establish connection");

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
    let db = connection::db().await?;

    let count = clipboard::Entity::find().count(&db).await?;

    Ok(count)
}

pub async fn insert_clipboard_dbo(model: FullClipboardDbo) -> Result<FullClipboardDto, DbErr> {
    let db = connection::db().await?;
    let clipboard = model.clipboard_model.insert(&db).await?;

    // Insert text if data exists
    let text = match &model.clipboard_text_model.data {
        sea_orm::ActiveValue::Set(data) if !data.is_empty() => {
            let mut text_model = model.clipboard_text_model;
            text_model.id = Set(Uuid::new_v4());
            text_model.clipboard_id = Set(clipboard.id.clone());
            Some(text_model.insert(&db).await?)
        }
        _ => None,
    };

    // Insert html if data exists
    let html = match &model.clipboard_html_model.data {
        sea_orm::ActiveValue::Set(data) if !data.is_empty() => {
            let mut html_model = model.clipboard_html_model;
            html_model.id = Set(Uuid::new_v4());
            html_model.clipboard_id = Set(clipboard.id);
            Some(html_model.insert(&db).await?)
        }
        _ => None,
    };

    // Insert image if data exists
    let image = match &model.clipboard_image_model.data {
        sea_orm::ActiveValue::Set(data) if !data.is_empty() => {
            let mut image_model = model.clipboard_image_model;
            image_model.id = Set(Uuid::new_v4());
            image_model.clipboard_id = Set(clipboard.id.clone());
            Some(image_model.insert(&db).await?)
        }
        _ => None,
    };

    // Insert rtf if data exists
    let rtf = match &model.clipboard_rtf_model.data {
        sea_orm::ActiveValue::Set(data) if !data.is_empty() => {
            let mut rtf_model = model.clipboard_rtf_model;
            rtf_model.id = Set(Uuid::new_v4());
            rtf_model.clipboard_id = Set(clipboard.id.clone());
            Some(rtf_model.insert(&db).await?)
        }
        _ => None,
    };

    let files = if !model.clipboard_files_model.is_empty() {
        let mut files = Vec::new();
        for mut file_model in model.clipboard_files_model {
            file_model.id = Set(Uuid::new_v4());
            file_model.clipboard_id = Set(clipboard.id.clone());
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

pub async fn insert_clipboard_dto(model: FullClipboardDto) -> Result<FullClipboardDto, DbErr> {
    let db = connection::db().await?;

    // Upsert clipboard - if id exists it will update, otherwise insert
    let clipboard = entity::clipboard::ActiveModel::from(model.clipboard)
        .insert(&db)
        .await?;

    // Upsert text if data exists
    let text = match &model.text {
        Some(text) if !text.data.is_empty() => {
            let text_model: entity::clipboard_text::ActiveModel = text.clone().into();
            Some(text_model.insert(&db).await?)
        }
        _ => None,
    };

    // Upsert html if data exists
    let html = match &model.html {
        Some(html) if !html.data.is_empty() => {
            let html_model: entity::clipboard_html::ActiveModel = html.clone().into();
            Some(html_model.insert(&db).await?)
        }
        _ => None,
    };

    // Upsert image if data exists
    let image = match &model.image {
        Some(image) if !image.data.is_empty() => {
            let image_model: entity::clipboard_image::ActiveModel = image.clone().into();
            Some(image_model.insert(&db).await?)
        }
        _ => None,
    };

    // Upsert rtf if data exists
    let rtf = match &model.rtf {
        Some(rtf) if !rtf.data.is_empty() => {
            let rtf_model: entity::clipboard_rtf::ActiveModel = rtf.clone().into();
            Some(rtf_model.insert(&db).await?)
        }
        _ => None,
    };

    // Upsert files if they exist
    let files = if !model.files.is_empty() {
        let mut files = Vec::new();
        for file in model.files {
            let file_model: entity::clipboard_file::ActiveModel = file.clone().into();
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

pub async fn get_clipboard_db(id: Uuid) -> Result<FullClipboardDto, DbErr> {
    let db = connection::db().await?;
    let clipboard = clipboard::Entity::find_by_id(id)
        .one(&db)
        .await?
        .ok_or(DbErr::RecordNotFound("Clipboard not found".into()))?;

    Ok(load_clipboards_with_relations(vec![clipboard])
        .await
        .remove(0))
}

pub async fn get_last_clipboard_db() -> Result<FullClipboardDto, DbErr> {
    let db = connection::db().await?;

    let last_clipboard = clipboard::Entity::find()
        .order_by_desc(clipboard::Column::Id)
        .one(&db)
        .await?;

    if last_clipboard.is_none() {
        return Err(DbErr::RecordNotFound(
            "last clipboard not found".to_string(),
        ));
    }

    Ok(
        load_clipboards_with_relations(vec![last_clipboard.expect("Failed to load clipboard")])
            .await
            .remove(0),
    )
}

pub async fn get_clipboards_db(
    cursor: Option<u64>,
    search: Option<String>,
    star: Option<bool>,
    img: Option<bool>,
) -> Result<Vec<FullClipboardDto>, DbErr> {
    let db = connection::db().await?;
    let (clipboard_keywords, text_keywords) = KeywordBuilder::build_default();

    let settings = get_app().state::<settings::Model>();

    let query = clipboard::Entity::find()
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
        .order_by_desc(clipboard::Column::CreatedDate);

    let clipboards = query.all(&db).await?;
    // printlog!("clipboards: {:?}", clipboards);
    Ok(load_clipboards_with_relations(clipboards).await)
}

pub async fn get_sync_amount_cliboards_db() -> Result<Vec<FullClipboardDto>, DbErr> {
    let db = connection::db().await?;
    let settings: tauri::State<'_, settings::Model> = get_app().state::<settings::Model>();

    let sync_amount_clipboards = clipboard::Entity::find()
        .limit(settings.sync_limit as u64)
        .order_by_desc(clipboard::Column::CreatedDate)
        .all(&db)
        .await?;

    let sync_favorite_clipboards = clipboard::Entity::find()
        .filter(clipboard::Column::Star.eq(true))
        .order_by_desc(clipboard::Column::CreatedDate)
        .all(&db)
        .await?;

    let clipboards = sync_amount_clipboards
        .clone()
        .into_iter()
        .chain(sync_favorite_clipboards.clone())
        .map(|clipboard| (clipboard.id.clone(), clipboard))
        .collect::<HashMap<_, _>>() // Collects into HashMap using ID as key
        .into_values()
        .collect::<Vec<_>>();

    printlog!(
        "clipboards regular: {:?} clipboards favorite: {:?}",
        sync_amount_clipboards.len(),
        sync_favorite_clipboards.len()
    );

    Ok(load_clipboards_with_relations(clipboards).await)
}

pub async fn star_clipboard_db(id: Uuid, star: bool) -> Result<bool, CommandError> {
    let db = connection::db().await?;

    let model = clipboard::ActiveModel {
        id: Set(id),
        star: Set(star),
        ..Default::default()
    };

    clipboard::Entity::update(model).exec(&db).await?;

    let delete_id = id.clone();
    tokio::spawn(async move {
        let provider = Arc::new(
            GoogleDriveProvider::new()
                .await
                .expect("Failed to initialize sync provider"),
        );

        provider
            .delete_by_id(&delete_id)
            .await
            .map_err(|e| CommandError::Error(e.to_string()))
    });

    Ok(true)
}

pub async fn delete_clipboard_db(id: Uuid) -> Result<bool, CommandError> {
    let db = connection::db().await?;

    clipboard::Entity::delete_by_id(id).exec(&db).await?;

    let delete_id = id.clone();
    tokio::spawn(async move {
        let provider = Arc::new(
            GoogleDriveProvider::new()
                .await
                .expect("Failed to initialize sync provider"),
        );

        provider
            .delete_by_id(&delete_id)
            .await
            .map_err(|e| CommandError::Error(e.to_string()))
    });

    Ok(true)
}

pub async fn clear_clipboards_db(r#type: Option<ClipboardType>) -> Result<(), DbErr> {
    let db = connection::db().await?;

    match r#type {
        None => {
            // Keep existing behavior - delete all non-starred clipboards
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
                            .filter(clipboard_text::Column::ClipboardId.eq(clipboard.id.clone()))
                            .exec(&db)
                            .await?;
                    }
                    ClipboardType::Image => {
                        clipboard_image::Entity::delete_many()
                            .filter(clipboard_image::Column::ClipboardId.eq(clipboard.id.clone()))
                            .exec(&db)
                            .await?;
                    }
                    ClipboardType::Html => {
                        clipboard_html::Entity::delete_many()
                            .filter(clipboard_html::Column::ClipboardId.eq(clipboard.id.clone()))
                            .exec(&db)
                            .await?;
                    }
                    ClipboardType::Rtf => {
                        clipboard_rtf::Entity::delete_many()
                            .filter(clipboard_rtf::Column::ClipboardId.eq(clipboard.id.clone()))
                            .exec(&db)
                            .await?;
                    }
                    ClipboardType::File => {
                        clipboard_file::Entity::delete_many()
                            .filter(clipboard_file::Column::ClipboardId.eq(clipboard.id.clone()))
                            .exec(&db)
                            .await?;
                    }
                }

                // Parse and update the types JSON array
                if let Some(mut types) = ClipboardType::from_json_value(&clipboard.types) {
                    // Remove the specified type
                    types.retain(|t| t != &clipboard_type);

                    if types.is_empty() {
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

    Ok(())
}

pub async fn count_clipboards_db() -> Result<u64, DbErr> {
    let db = connection::db().await?;

    let count = clipboard::Entity::find().count(&db).await?;

    Ok(count)
}

pub async fn get_clipboard_uuids_db() -> Result<HashMap<Uuid, NaiveDateTime>, DbErr> {
    let db = connection::db().await?;

    let clipboards = clipboard::Entity::find()
        .select_only()
        .columns([clipboard::Column::Id, clipboard::Column::CreatedDate])
        .order_by_desc(clipboard::Column::CreatedDate)
        .into_tuple()
        .all(&db)
        .await?;

    Ok(clipboards.into_iter().collect::<HashMap<_, _>>())
}

pub async fn copy_clipboard_from_index(i: u64) -> Result<Option<Model>, DbErr> {
    let db = connection::db().await?;

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
    let clipboard_data = get_clipboard_db(id).await?;
    let clipboard = get_app().state::<Clipboard>();

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
                        f.name.as_ref().unwrap(),
                        f.extension.as_ref().unwrap()
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
