extern crate alloc;
use super::global::get_main_window;
use crate::{connection, utils::tauri::config::APP};
use entity::clipboard::{self, ActiveModel, Model};
use sea_orm::Iden;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, DbErr, EntityTrait, PaginatorTrait, QueryFilter, QueryOrder,
    QuerySelect, QueryTrait, Set,
};
use tauri::Manager;
use tauri_plugin_clipboard::Clipboard;

pub async fn insert_clipboard_db(clipboard: ActiveModel) -> Result<Model, DbErr> {
    let db = connection::establish_connection().await?;

    let mut clip_db: Model = clipboard.insert(&db).await?;

    if clip_db.content.is_some() && clip_db.content.as_ref().unwrap().len() > 100 {
        clip_db.content = clip_db.content.unwrap()[..100].to_string().into();
    }

    Ok(clip_db)
}

pub async fn get_clipboard_db(id: i32) -> Result<Model, DbErr> {
    let db = connection::establish_connection().await?;

    let model = clipboard::Entity::find_by_id(id).one(&db).await?;

    Ok(model.unwrap())
}

pub async fn get_last_clipboard_db() -> Result<Model, DbErr> {
    let db = connection::establish_connection().await?;

    let model = clipboard::Entity::find()
        .order_by_desc(clipboard::Column::Id)
        .one(&db)
        .await?;

    Ok(model.unwrap())
}

pub async fn get_clipboards_db(
    cursor: Option<u64>,
    search: Option<String>,
    star: Option<bool>,
    img: Option<bool>,
) -> Result<Vec<Model>, DbErr> {
    let db = connection::establish_connection().await?;

    let model = clipboard::Entity::find()
        .apply_if(star, |query, starred| {
            query.filter(clipboard::Column::Star.eq(starred))
        })
        .apply_if(search, |query, search| {
            let filter = match search.as_str() {
                "txt" | "text" => clipboard::Column::Content
                    .contains(search)
                    .or(clipboard::Column::Type.eq(migration::ClipboardType::Text.to_string())),
                "img" | "image" => clipboard::Column::Content
                    .contains(search)
                    .or(clipboard::Column::Type.eq(migration::ClipboardType::Image.to_string())),

                "lnk" | "link" => clipboard::Column::Content
                    .contains(search)
                    .or(clipboard::Column::Type.eq(migration::ClipboardType::Link.to_string())),

                "clr" | "color" | "colour" => clipboard::Column::Content
                    .contains(search)
                    .or(clipboard::Column::Type.eq(migration::ClipboardType::Hex.to_string()))
                    .or(clipboard::Column::Type.eq(migration::ClipboardType::Rgb.to_string())),

                "hex" => clipboard::Column::Content
                    .contains(search)
                    .or(clipboard::Column::Type.eq(migration::ClipboardType::Hex.to_string())),

                "rgb" => clipboard::Column::Content
                    .contains(search)
                    .or(clipboard::Column::Type.eq(migration::ClipboardType::Rgb.to_string())),

                _ => clipboard::Column::Content.contains(search),
            };
            query.filter(filter)
        })
        .apply_if(img, |query, _img| {
            query.filter(clipboard::Column::Type.eq(migration::ClipboardType::Image.to_string()))
        })
        .offset(cursor)
        .limit(10)
        .order_by_desc(clipboard::Column::Id)
        .all(&db)
        .await?;

    let parsed_model: Vec<Model> = model
        .into_iter()
        .map(|mut m| {
            m.image = None;

            // Safely truncate content if it's longer than 100 characters
            if let Some(content) = &m.content {
                if content.chars().count() > 100 {
                    // Take the first 100 characters, and collect them back into a String
                    let truncated = content.chars().take(100).collect::<String>();
                    m.content = Some(truncated);
                }
            }
            m
        })
        .collect();

    Ok(parsed_model)
}

pub async fn star_clipboard_db(id: i32, star: bool) -> Result<bool, DbErr> {
    let db = connection::establish_connection().await?;

    let model = clipboard::ActiveModel {
        id: Set(id),
        star: Set(Some(star)),
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

    let result = match clipboard_data.r#type.as_str() {
        r#type if r#type == migration::ClipboardType::Image.to_string() => {
            match clipboard_data.image {
                Some(blob) => clipboard.write_image_binary(blob).is_ok(),
                None => false,
            }
        }
        _ => match clipboard_data.content {
            Some(content) => clipboard.write_text(content).is_ok(),
            None => false,
        },
    };

    if result {
        get_main_window().hide().ok();
    }

    Ok(result)
}
