extern crate alloc;

use arboard::{Clipboard, ImageData};
use entity::clipboard::{self, ActiveModel, Model};
use sea_orm::{
    ActiveModelTrait, ColumnTrait, DatabaseConnection, DbErr, EntityTrait, QueryFilter, QueryOrder,
    QuerySelect, QueryTrait, Set,
};
use tauri::regex::Regex;

use crate::connection;

pub async fn upsert(clipboard: ActiveModel) -> Result<Option<Model>, DbErr> {
    let is_same = check_if_last_same().await;
    if is_same.is_none() {
        ()
    }
    let db: DatabaseConnection = connection::establish_connection().await?;

    let clip_db: Model = clipboard.insert(&db).await?;

    Ok(Some(clip_db))
}

pub async fn get_clipboards(
    cursor: Option<u64>,
    search: Option<String>,
    star: Option<bool>,
) -> Result<Vec<Model>, DbErr> {
    let db = connection::establish_connection().await?;

    let model = clipboard::Entity::find()
        .wapply_if(star, |query, starred| {
            query.filter(clipboard::Column::Star.eq(starred))
        })
        .wapply_if(search, |query, content| {
            query.filter(clipboard::Column::Content.contains(&content))
        })
        .offset(cursor)
        .limit(11)
        .order_by_desc(clipboard::Column::Id)
        .all(&db)
        .await?;

    Ok(model)
}

pub async fn delete_clipboard_db(id: i32) -> Result<Option<bool>, DbErr> {
    let db = connection::establish_connection().await?;

    clipboard::Entity::delete_by_id(id).exec(&db).await?;

    Ok(Some(true))
}

async fn check_if_last_same() -> Option<Model> {
    let (text, image) = get_os_clipboard();

    if text.is_none() && image.is_none() {
        return None;
    }

    let db = connection::establish_connection().await.unwrap();

    let last_clipboard = clipboard::Entity::find()
        .order_by_desc(clipboard::Column::Id)
        .one(&db)
        .await
        .unwrap();

    if last_clipboard.is_none() {
        return None;
    }

    let last = last_clipboard.as_ref().unwrap();

    let content = if text.is_some() && last.content.is_some() {
        text.unwrap() == last.content.unwrap()
    } else {
        false
    };
    let blob = if image.is_some() && last.blob.is_some() {
        image.unwrap().bytes.to_vec() == last.blob.unwrap()
    } else {
        false
    };

    if content && blob {
        return Some(last);
    }

    None
}

pub fn parse_model() -> ActiveModel {
    let (text, image) = get_os_clipboard();

    let re = Regex::new(r"^#?(?:[0-9a-f]{3}){1,2}$").unwrap();

    let r#type = if text.is_some() {
        Set("image".to_string())
    } else if re.is_match(&text.clone().unwrap()) {
        Set("color".to_string())
    } else {
        Set("text".to_string())
    };

    let img = if image.is_some() {
        let img = image.unwrap();
        ActiveModel {
            blob: Set(Some(img.bytes.to_vec())),
            height: Set(Some(img.height as i32)),
            width: Set(Some(img.width as i32)),
            size: Set(Some(img.bytes.to_vec().len().to_string())),
            ..Default::default()
        }
    } else {
        ActiveModel {
            ..Default::default()
        }
    };

    ActiveModel {
        r#type,
        content: Set(text),

        star: Set(Some(false)),
        ..img
    }
}

pub fn get_os_clipboard() -> (Option<String>, Option<ImageData<'static>>) {
    // Command::new("clear").status().unwrap();

    let mut clipboard = Clipboard::new().unwrap();

    let text: Option<String> = match clipboard.get_text() {
        Ok(text) => Some(text),
        Err(_) => None,
    };

    let image: Option<ImageData<'_>> = match clipboard.get_image() {
        Ok(image) => Some(image),
        Err(_) => None,
    };

    (text, image)
}
