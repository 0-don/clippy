use crate::tauri_config::config::APP;
use crate::{connection, prelude::*};
use crate::service::global::get_app;
use common::types::enums::{ClipboardTextType, ClipboardType};
use common::types::orm_query::{ClipboardManager, ClipboardWithRelations};
use entity::clipboard::{self, Model};
use entity::{clipboard_file, clipboard_html, clipboard_image, clipboard_rtf, clipboard_text};
use sea_orm::{
    ActiveModelTrait, ColumnTrait, EntityTrait, LoaderTrait, PaginatorTrait, QueryFilter,
    QueryOrder, QuerySelect, QueryTrait,
};
use tauri::Manager;
use tauri_plugin_clipboard::Clipboard;
use tokio::try_join;

pub async fn load_clipboards_with_relations(
    clipboards: Vec<clipboard::Model>,
) -> Vec<ClipboardWithRelations> {
    let db = connection::establish_connection()
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
            file: f.into_iter().next(), // only files need into_iter().next() since it's load_many
        })
        .collect()
}

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

    Ok(clip_db)
}

pub async fn get_clipboard_db(id: i32) -> Result<ClipboardWithRelations, DbErr> {
    let db = connection::establish_connection().await?;
    let clipboard = clipboard::Entity::find_by_id(id).one(&db).await?;

    if clipboard.is_none() {
        return Err(DbErr::RecordNotFound("clipboard not found".to_string()));
    }

    Ok(
        load_clipboards_with_relations(vec![clipboard.expect("Failed to load clipboard")])
            .await
            .into_iter()
            .next()
            .expect("Failed to load clipboard with relations"),
    )
}

pub async fn get_last_clipboard_db() -> Result<ClipboardWithRelations, DbErr> {
    let db = connection::establish_connection().await?;

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
            .into_iter()
            .next()
            .expect("Failed to load clipboard with relations"),
    )
}

pub async fn get_clipboards_db(
    cursor: Option<u64>,
    search: Option<String>,
    star: Option<bool>,
    img: Option<bool>,
) -> Result<Vec<ClipboardWithRelations>, DbErr> {
    printlog!("get_clipboards");
    let db = connection::establish_connection().await?;

    // First get the clipboards with filters
    let clipboards: Vec<Model> = clipboard::Entity::find()
        .apply_if(star, |query, starred| {
            query.filter(clipboard::Column::Star.eq(starred))
        })
        .apply_if(img, |query, _img| {
            query.filter(clipboard::Column::Types.contains(ClipboardType::Image.to_string()))
        })
        .apply_if(search, |query, search| {
            let filter = match search.as_str() {
                "img" | "image" => {
                    clipboard::Column::Types.contains(ClipboardType::Image.to_string())
                }
                "txt" | "text" => clipboard_text::Column::Data
                    .contains(search)
                    .or(clipboard_text::Column::Type.eq(ClipboardTextType::Text.to_string())),
                "lnk" | "link" => clipboard_text::Column::Data
                    .contains(search)
                    .or(clipboard_text::Column::Type.eq(ClipboardTextType::Link.to_string())),

                "clr" | "color" | "colour" => clipboard_text::Column::Data
                    .contains(search)
                    .or(clipboard_text::Column::Type.eq(ClipboardTextType::Hex.to_string()))
                    .or(clipboard_text::Column::Type.eq(ClipboardTextType::Rgb.to_string())),

                "hex" => clipboard_text::Column::Data
                    .contains(search)
                    .or(clipboard_text::Column::Type.eq(ClipboardTextType::Hex.to_string())),

                "rgb" => clipboard_text::Column::Data
                    .contains(search)
                    .or(clipboard_text::Column::Type.eq(ClipboardTextType::Rgb.to_string())),

                _ => clipboard_text::Column::Data.contains(search),
            };
            query.filter(filter)
        })
        .offset(cursor)
        .limit(10)
        .order_by_desc(clipboard::Column::Id)
        .all(&db)
        .await?;

    // Zip everything together, taking first item from each Vec or None if empty
    Ok(load_clipboards_with_relations(clipboards).await)
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

    let model = model.expect("Failed to load clipboard");
    copy_clipboard_from_id(model.id, ClipboardType::Text).await?;

    Ok(Some(model))
}

pub async fn copy_clipboard_from_id(id: i32, requested_type: ClipboardType) -> Result<bool, DbErr> {
    printlog!("type {:?}", requested_type);
    let clipboard_data = get_clipboard_db(id).await?;
    let clipboard = get_app().state::<Clipboard>();

    let types = match ClipboardType::from_json_value(&clipboard_data.clipboard.types) {
        Some(types) => types,
        None => return Ok(false),
    };

    // Check if the requested type exists in the clipboard data
    if !types.contains(&requested_type) {
        return Ok(false);
    }

    // Write only the requested type
    let success = match requested_type {
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
        if let Some(window) = APP
            .get()
            .expect("APP not initialized")
            .get_webview_window("main")
        {
            window.hide().ok();
        }
    }

    Ok(success)
}
