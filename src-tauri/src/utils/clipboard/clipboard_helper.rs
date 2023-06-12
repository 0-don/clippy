use arboard::{Clipboard, ImageData};
use entity::clipboard::{self, ActiveModel, Model};
use sea_orm::{EntityTrait, QueryOrder, Set};
use tauri::regex::Regex;

use crate::connection;

pub async fn check_if_last_same() -> Option<Model> {
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
    let last_clipboard = last_clipboard.unwrap();

    let content = if text.is_some() && last_clipboard.content.is_some() {
        text.unwrap() == last_clipboard.content.as_deref().unwrap()
    } else {
        false
    };
    let blob = if image.is_some() && last_clipboard.blob.is_some() {
        image.unwrap().bytes.to_vec() == last_clipboard.blob.as_deref().unwrap()
    } else {
        false
    };

    if content && blob {
        return Some(last_clipboard);
    }
    None
}

pub fn parse_model() -> ActiveModel {
    let (text, image) = get_os_clipboard();

    let re = Regex::new(r"^#?(?:[0-9a-f]{3}){1,2}$").unwrap();

    let r#type = if text.is_some() {
        Set("text".to_string())
    } else if re.is_match(&text.as_deref().unwrap()) {
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
