extern crate alloc;
use crate::types::orm_query::{ClipboardManager, ClipboardWithRelations};
use crate::{connection, utils::tauri::config::APP};
use entity::clipboard::{self, Model};
use entity::{clipboard_file, clipboard_html, clipboard_image, clipboard_rtf, clipboard_text};
use migration::ClipboardType;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, DbErr, EntityTrait, LoaderTrait, PaginatorTrait, QueryFilter,
    QueryOrder, QuerySelect, QueryTrait, Set,
};
use sea_orm::{Condition, RelationTrait};
use sea_orm::{Iden, JoinType};
use tauri::Manager;
use tauri_plugin_clipboard::Clipboard;

pub async fn insert_clipboard_db(
    active_model: ClipboardManager,
) -> Result<ClipboardWithRelations, DbErr> {
    let db = connection::establish_connection().await?;

    // First insert the clipboard model to get its ID
    let clipboard_model = active_model.clipboard_model.insert(&db).await?;
    let clipboard_id = clipboard_model.id;

    // Only insert text if data is set
    let clipboard_text_model =
        if let sea_orm::ActiveValue::Set(data) = &active_model.clipboard_text_model.data {
            if !data.is_empty() {
                let mut text_model = active_model.clipboard_text_model;
                text_model.clipboard_id = Set(clipboard_id);
                Some(text_model.insert(&db).await?)
            } else {
                None
            }
        } else {
            None
        };

    // Only insert HTML if data is set
    let clipboard_html_model =
        if let sea_orm::ActiveValue::Set(data) = &active_model.clipboard_html_model.data {
            if !data.is_empty() {
                let mut html_model = active_model.clipboard_html_model;
                html_model.clipboard_id = Set(clipboard_id);
                Some(html_model.insert(&db).await?)
            } else {
                None
            }
        } else {
            None
        };

    // Only insert image if data is set
    let clipboard_image_model =
        if let sea_orm::ActiveValue::Set(data) = &active_model.clipboard_image_model.data {
            if !data.is_empty() {
                let mut image_model = active_model.clipboard_image_model;
                image_model.clipboard_id = Set(clipboard_id);
                Some(image_model.insert(&db).await?)
            } else {
                None
            }
        } else {
            None
        };

    // Only insert RTF if data is set
    let clipboard_rtf_model =
        if let sea_orm::ActiveValue::Set(data) = &active_model.clipboard_rtf_model.data {
            if !data.is_empty() {
                let mut rtf_model = active_model.clipboard_rtf_model;
                rtf_model.clipboard_id = Set(clipboard_id);
                Some(rtf_model.insert(&db).await?)
            } else {
                None
            }
        } else {
            None
        };

    // Only insert file if data is set
    let clipboard_file_model =
        if let sea_orm::ActiveValue::Set(data) = &active_model.clipboard_file_model.data {
            if !data.is_empty() {
                let mut file_model = active_model.clipboard_file_model;
                file_model.clipboard_id = Set(clipboard_id);
                Some(file_model.insert(&db).await?)
            } else {
                None
            }
        } else {
            None
        };

    let clip_db = ClipboardWithRelations {
        clipboard: clipboard_model,
        text: clipboard_text_model,
        html: clipboard_html_model,
        image: clipboard_image_model,
        rtf: clipboard_rtf_model,
        file: clipboard_file_model,
    };

    // printlog!("Clipboard inserted: {:?}", clip_db);

    Ok(clip_db)
}

pub async fn get_clipboard_db(id: i32) -> Result<ClipboardWithRelations, DbErr> {
    let _db = connection::establish_connection().await?;

    let clipboard = clipboard::Entity::find()
        .select_only()
        .column_as(clipboard::Column::Id, "clipboard_id")
        .columns([
            clipboard::Column::Types,
            clipboard::Column::Star,
            clipboard::Column::CreatedDate,
        ])
        .join(JoinType::LeftJoin, clipboard::Relation::ClipboardText.def())
        .join(JoinType::LeftJoin, clipboard::Relation::ClipboardHtml.def())
        .join(
            JoinType::LeftJoin,
            clipboard::Relation::ClipboardImage.def(),
        )
        .join(JoinType::LeftJoin, clipboard::Relation::ClipboardRtf.def())
        .join(JoinType::LeftJoin, clipboard::Relation::ClipboardFile.def())
        .filter(clipboard::Column::Id.eq(id))
        .into_model::<ClipboardWithRelations>()
        .one(&_db)
        .await?;

    Ok(clipboard.unwrap())
}

pub async fn get_last_clipboard_db() -> Result<ClipboardWithRelations, DbErr> {
    let db = connection::establish_connection().await?;

    let last_clipboard = clipboard::Entity::find()
        .select_only()
        .column_as(clipboard::Column::Id, "clipboard_id")
        .columns([
            clipboard::Column::Types,
            clipboard::Column::Star,
            clipboard::Column::CreatedDate,
        ])
        .join(JoinType::LeftJoin, clipboard::Relation::ClipboardText.def())
        .join(JoinType::LeftJoin, clipboard::Relation::ClipboardHtml.def())
        .join(
            JoinType::LeftJoin,
            clipboard::Relation::ClipboardImage.def(),
        )
        .join(JoinType::LeftJoin, clipboard::Relation::ClipboardRtf.def())
        .join(JoinType::LeftJoin, clipboard::Relation::ClipboardFile.def())
        .order_by_desc(clipboard::Column::Id)
        .into_model::<ClipboardWithRelations>()
        .one(&db)
        .await?;

    Ok(last_clipboard.unwrap())
}

pub async fn get_clipboards_db(
    cursor: Option<u64>,
    search: Option<String>,
    star: Option<bool>,
    img: Option<bool>,
) -> Result<Vec<ClipboardWithRelations>, DbErr> {
    let db = connection::establish_connection().await?;

    // First get the clipboards with filters
    let clipboards = clipboard::Entity::find()
        .apply_if(star, |query, starred| {
            query.filter(clipboard::Column::Star.eq(starred))
        })
        .apply_if(img, |query, _img| {
            query.filter(
                clipboard::Column::Types.contains(migration::ClipboardType::Image.to_string()),
            )
        })
        .apply_if(
            search,
            |query: sea_orm::Select<entity::prelude::Clipboard>, search_term| {
                query.filter(
                    Condition::any()
                        .add(clipboard_text::Column::Data.contains(&search_term))
                        .add(clipboard_html::Column::Data.contains(&search_term))
                        .add(clipboard_rtf::Column::Data.contains(&search_term))
                        .add(clipboard_image::Column::Extension.contains(&search_term)),
                )
            },
        )
        .offset(cursor)
        .limit(10)
        .order_by_desc(clipboard::Column::Id)
        .all(&db)
        .await?;

    // Then load all related data using load_many
    let texts = clipboards.load_many(clipboard_text::Entity, &db).await?;
    let htmls = clipboards.load_many(clipboard_html::Entity, &db).await?;
    let images = clipboards.load_many(clipboard_image::Entity, &db).await?;
    let rtfs = clipboards.load_many(clipboard_rtf::Entity, &db).await?;
    let files = clipboards.load_many(clipboard_file::Entity, &db).await?;

    // Zip everything together, taking first item from each Vec or None if empty
    Ok(clipboards
        .into_iter()
        .zip(texts)
        .zip(htmls)
        .zip(images)
        .zip(rtfs)
        .zip(files)
        .map(|(((((c, t), h), i), r), f)| ClipboardWithRelations {
            clipboard: c,
            text: t.into_iter().next(),
            html: h.into_iter().next(),
            image: i.into_iter().next(),
            rtf: r.into_iter().next(),
            file: f.into_iter().next(),
        })
        .collect())
}

pub async fn star_clipboard_db(id: i32, star: bool) -> Result<bool, DbErr> {
    let db = connection::establish_connection().await?;

    let model = clipboard::ActiveModel {
        id: Set(id),
        star: Set(star),
        ..Default::default()
    };

    let _clipboard = clipboard::Entity::update(model).exec(&db).await?;

    Ok(true)
}

pub async fn delete_clipboard_db(id: i32) -> Result<bool, DbErr> {
    let db = connection::establish_connection().await?;

    clipboard::Entity::delete_by_id(id).exec(&db).await?;

    Ok(true)
}

pub async fn clear_clipboards_db() -> Result<bool, DbErr> {
    let db = connection::establish_connection().await?;

    clipboard::Entity::delete_many()
        .filter(clipboard::Column::Star.eq(false))
        .exec(&db)
        .await?;

    Ok(true)
}

pub async fn count_clipboards_db() -> Result<u64, DbErr> {
    let db = connection::establish_connection().await?;

    let count = clipboard::Entity::find().count(&db).await?;

    Ok(count)
}

pub async fn copy_clipboard_from_index(i: u64) -> Result<Option<Model>, DbErr> {
    let db = connection::establish_connection().await?;

    let model = clipboard::Entity::find()
        .order_by_desc(clipboard::Column::Id)
        .offset(Some(i))
        .limit(1)
        .one(&db)
        .await?;

    if model.is_none() {
        return Ok(None);
    }

    let model = model.unwrap();
    let _ = copy_clipboard_from_id(model.id).await;

    Ok(Some(model))
}

pub async fn copy_clipboard_from_id(id: i32) -> Result<bool, DbErr> {
    let clipboard_data = get_clipboard_db(id).await?;
    let clipboard = APP.get().expect("APP not initialized").state::<Clipboard>();

    let types = match ClipboardType::from_json_value(&clipboard_data.clipboard.types) {
        Some(types) => types,
        None => return Ok(false),
    };

    // Track if any write operation succeeded
    let mut any_success = false;

    // Try to write all clipboard types
    for clipboard_type in types {
        let success = match clipboard_type {
            ClipboardType::Image => clipboard_data
                .image
                .as_ref()
                .and_then(|model| clipboard.write_image_binary(model.data.clone()).ok())
                .is_some(),

            ClipboardType::Text => clipboard_data
                .text
                .as_ref()
                .and_then(|model| clipboard.write_text(model.data.clone()).ok())
                .is_some(),

            ClipboardType::Html => clipboard_data
                .html
                .as_ref()
                .and_then(|model| clipboard.write_html(model.data.clone()).ok())
                .is_some(),

            ClipboardType::Rtf => clipboard_data
                .rtf
                .as_ref()
                .and_then(|model| clipboard.write_rtf(model.data.clone()).ok())
                .is_some(),

            ClipboardType::File => clipboard_data
                .file
                .as_ref()
                .and_then::<bool, _>(|_model| None)
                .is_some(),
        };

        if success {
            any_success = true;
        }
    }

    if any_success {
        if let Some(window) = APP
            .get()
            .expect("APP not initialized")
            .get_webview_window("main")
        {
            window.hide().ok();
        }
    }

    Ok(any_success)
}
