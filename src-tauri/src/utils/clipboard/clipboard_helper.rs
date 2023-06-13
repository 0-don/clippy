use std::{fs::File, io::Read};

use arboard::{Clipboard, ImageData};
use entity::clipboard::{self, ActiveModel, Model};
use image::{ImageBuffer, RgbaImage};
use sea_orm::{EntityTrait, QueryOrder, Set};
use tauri::{regex::Regex, Manager};

use crate::{connection, service::clipboard::upsert_db, utils::setup::APP};

pub async fn check_if_last_is_same() -> Option<Model> {
    let (text, image) = get_os_clipboard();

    if text.is_none() && image.is_none() {
        println!("No clipboard data found?");
        return None;
    }

    let db = connection::establish_connection().await.unwrap();

    let last_clipboard = clipboard::Entity::find()
        .order_by_desc(clipboard::Column::Id)
        .one(&db)
        .await
        .unwrap();

    if last_clipboard.is_none() {
        println!("Last clipboard does not exist in db");
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
        println!("content: {}, blob: {}", content, blob);
        return Some(last_clipboard);
    }

    println!("not the same");
    // clipboard and db are not the same
    None
}

pub fn parse_model() -> ActiveModel {
    let (text, clipboard_image) = get_os_clipboard();

    let re = Regex::new(r"^#?(?:[0-9a-f]{3}){1,2}$").unwrap();

    let r#type = if text.is_some() {
        Set("text".to_string())
    } else if text.is_some() && re.is_match(&text.as_deref().unwrap()) {
        Set("color".to_string())
    } else {
        Set("image".to_string())
    };

    let formatted_img: Option<Vec<u8>> = parse_image().unwrap();

    let active_model = if formatted_img.is_some() {
        let image = clipboard_image.unwrap();
        ActiveModel {
            blob: Set(formatted_img),
            height: Set(Some(image.height as i32)),
            width: Set(Some(image.width as i32)),
            size: Set(Some(image.bytes.to_vec().len().to_string())),
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
        ..active_model
    }
}

pub fn parse_image() -> Result<Option<Vec<u8>>, String> {
    let (_text, image) = get_os_clipboard();

    if image.is_none() {
        return Ok(None);
    }

    let tmp_dir = tempfile::Builder::new()
        .prefix("clipboard-img")
        .tempdir()
        .map_err(|err| err.to_string())?;
    let fname = tmp_dir.path().join("clipboard-img.png");

    let image2: RgbaImage = ImageBuffer::from_raw(
        image.clone().unwrap().width.try_into().unwrap(),
        image.clone().unwrap().height.try_into().unwrap(),
        image.clone().unwrap().bytes.into_owned(),
    )
    .unwrap();

    image2.save(fname.clone()).map_err(|err| err.to_string())?;
    let mut file = File::open(fname.clone()).unwrap();
    let mut buffer = vec![];
    file.read_to_end(&mut buffer).unwrap();

    Ok(Some(buffer))
}

pub fn get_os_clipboard() -> (Option<String>, Option<ImageData<'static>>) {
    let mut clipboard = Clipboard::new().unwrap();

    let text: Option<String> = clipboard.get_text().ok();

    let image: Option<ImageData<'_>> = clipboard.get_image().ok();

    (text, image)
}

pub async fn upsert_clipboard() {
    let is_same = check_if_last_is_same().await;
    if is_same.is_some() {
        println!("Clipboard is the same as last clipboard");
        ()
    }

    let model = parse_model();

    let res = upsert_db(model.to_owned()).await.unwrap().unwrap();

    APP.get()
        .unwrap()
        .get_window("main")
        .unwrap()
        .emit("clipboard_listener", res)
        .unwrap();
}
