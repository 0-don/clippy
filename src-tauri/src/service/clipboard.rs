use crate::service::global::{get_app, get_main_window};
use crate::{connection, prelude::*};
use common::types::enums::{ClipboardTextType, ClipboardType};
use common::types::orm_query::{ClipboardManager, ClipboardWithRelations};
use entity::clipboard::{self, Model};
use entity::{clipboard_file, clipboard_html, clipboard_image, clipboard_rtf, clipboard_text};
use sea_orm::RelationTrait;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, EntityTrait, JoinType, LoaderTrait, PaginatorTrait, QueryFilter,
    QueryOrder, QuerySelect, QueryTrait,
};
use tauri::Manager;
use tauri_plugin_clipboard::Clipboard;
use tokio::try_join;

pub async fn load_clipboards_with_relations(
    clipboards: Vec<clipboard::Model>,
) -> Vec<ClipboardWithRelations> {
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
        .map(|(((((c, t), h), i), r), f)| ClipboardWithRelations {
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

pub async fn insert_clipboard_db(model: ClipboardManager) -> Result<ClipboardWithRelations, DbErr> {
    let db = connection::db().await?;
    let clipboard = model.clipboard_model.insert(&db).await?;

    // Insert text if data exists
    let text = match &model.clipboard_text_model.data {
        sea_orm::ActiveValue::Set(data) if !data.is_empty() => {
            let mut text_model = model.clipboard_text_model;
            text_model.clipboard_id = Set(clipboard.id);
            Some(text_model.insert(&db).await?)
        }
        _ => None,
    };

    // Insert html if data exists
    let html = match &model.clipboard_html_model.data {
        sea_orm::ActiveValue::Set(data) if !data.is_empty() => {
            let mut html_model = model.clipboard_html_model;
            html_model.clipboard_id = Set(clipboard.id);
            Some(html_model.insert(&db).await?)
        }
        _ => None,
    };

    // Insert image if data exists
    let image = match &model.clipboard_image_model.data {
        sea_orm::ActiveValue::Set(data) if !data.is_empty() => {
            let mut image_model = model.clipboard_image_model;
            image_model.clipboard_id = Set(clipboard.id);
            Some(image_model.insert(&db).await?)
        }
        _ => None,
    };

    // Insert rtf if data exists
    let rtf = match &model.clipboard_rtf_model.data {
        sea_orm::ActiveValue::Set(data) if !data.is_empty() => {
            let mut rtf_model = model.clipboard_rtf_model;
            rtf_model.clipboard_id = Set(clipboard.id);
            Some(rtf_model.insert(&db).await?)
        }
        _ => None,
    };

    let files = if !model.clipboard_files_model.is_empty() {
        let mut files = Vec::new();
        for mut file_model in model.clipboard_files_model {
            file_model.clipboard_id = Set(clipboard.id);
            files.push(file_model.insert(&db).await?);
        }
        files
    } else {
        Vec::new()
    };

    Ok(ClipboardWithRelations {
        clipboard,
        text,
        html,
        image,
        rtf,
        files,
    })
}

pub async fn get_clipboard_db(id: i32) -> Result<ClipboardWithRelations, DbErr> {
    let db = connection::db().await?;
    let clipboard = clipboard::Entity::find_by_id(id)
        .one(&db)
        .await?
        .ok_or(DbErr::RecordNotFound("Clipboard not found".into()))?;

    Ok(load_clipboards_with_relations(vec![clipboard])
        .await
        .remove(0))
}

pub async fn get_last_clipboard_db() -> Result<ClipboardWithRelations, DbErr> {
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
) -> Result<Vec<ClipboardWithRelations>, DbErr> {
    let db = connection::db().await?;

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
        .apply_if(search, |q, s| {
            let f = match s.as_str() {
                "img" | "image" => {
                    clipboard::Column::Types.contains(ClipboardType::Image.to_string())
                }
                "file" | "files" => {
                    clipboard::Column::Types.contains(ClipboardType::File.to_string())
                }
                "txt" | "text" => {
                    clipboard_text::Column::Type.eq(ClipboardTextType::Text.to_string())
                }
                "lnk" | "link" => {
                    clipboard_text::Column::Type.eq(ClipboardTextType::Link.to_string())
                }
                "clr" | "color" | "colour" => clipboard_text::Column::Type
                    .eq(ClipboardTextType::Hex.to_string())
                    .or(clipboard_text::Column::Type.eq(ClipboardTextType::Rgb.to_string())),
                t @ ("hex" | "rgb") => clipboard_text::Column::Type.eq(t.to_string()),
                _ => clipboard_text::Column::Data
                    .contains(&s)
                    .or(clipboard_file::Column::Name.contains(&s))
                    .or(clipboard_file::Column::Extension.contains(&s))
                    .or(clipboard_file::Column::MimeType.contains(&s))
                    .or(clipboard_file::Column::Name.contains(&s))
                    .or(clipboard_file::Column::Extension.contains(&s))
                    .or(clipboard_image::Column::Extension.contains(&s))
                    .or(clipboard_text::Column::Type.contains(&s))
                    .or(clipboard_html::Column::Data.contains(&s))
                    .or(clipboard_rtf::Column::Data.contains(&s)),
            };
            q.filter(f)
        })
        .offset(cursor)
        .limit(25)
        .order_by_desc(clipboard::Column::Id);

    let clipboards = query.all(&db).await?;
    // printlog!("clipboards: {:?}", clipboards);
    Ok(load_clipboards_with_relations(clipboards).await)
}

pub async fn star_clipboard_db(id: i32, star: bool) -> Result<bool, DbErr> {
    let db = connection::db().await?;

    let model = clipboard::ActiveModel {
        id: Set(id),
        star: Set(star),
        ..Default::default()
    };

    let _clipboard = clipboard::Entity::update(model).exec(&db).await?;

    Ok(true)
}

pub async fn delete_clipboard_db(id: i32) -> Result<bool, DbErr> {
    let db = connection::db().await?;

    clipboard::Entity::delete_by_id(id).exec(&db).await?;

    Ok(true)
}

pub async fn clear_clipboards_db() -> Result<(), DbErr> {
    let db = connection::db().await?;

    clipboard::Entity::delete_many()
        .filter(clipboard::Column::Star.eq(false))
        .exec(&db)
        .await?;

    Ok(())
}

pub async fn count_clipboards_db() -> Result<u64, DbErr> {
    let db = connection::db().await?;

    let count = clipboard::Entity::find().count(&db).await?;

    Ok(count)
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

pub async fn copy_clipboard_from_id(id: i32, requested_type: ClipboardType) -> Result<bool, DbErr> {
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
