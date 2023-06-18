use std::{fs::File, io::Read};

use arboard::{Clipboard, ImageData};
use entity::clipboard::{self, ActiveModel};
use image::{ImageBuffer, RgbaImage};
use sea_orm::{EntityTrait, QueryOrder, Set};
use tauri::{regex::Regex, Manager};

use crate::{connection, service::clipboard::upsert_db, utils::setup::APP};

pub async fn upsert_clipboard() {
    if check_if_last_is_same().await {
        return;
    }

    let model = upsert_db().await;

    APP.get()
        .unwrap()
        .get_window("main")
        .unwrap()
        .emit("clipboard_listener", model.unwrap())
        .unwrap();
}

pub async fn check_if_last_is_same() -> bool {
    let (text, image) = get_os_clipboard();

    if text.is_none() && image.is_none() {
        println!("No clipboard data found?");
        return false;
    }

    let db = connection::establish_connection().await.unwrap();

    let last_clipboard = clipboard::Entity::find()
        .order_by_desc(clipboard::Column::Id)
        .one(&db)
        .await
        .unwrap();

    if last_clipboard.is_none() {
        println!("Last clipboard does not exist in db");
        return false;
    }

    let last_clipboard = last_clipboard.unwrap();

    if text.is_some() // check if text is same
        && last_clipboard.content.is_some()
        && text.unwrap() == last_clipboard.content.as_deref().unwrap()
        || image.is_some() // check if image is same
            && last_clipboard.blob.is_some()
            && image.unwrap().bytes.to_vec() == last_clipboard.blob.as_deref().unwrap()
    {
        println!("Clipboard is the same as last clipboard");
        return true;
    }

    println!("clipboard and db are not the same");
    return false;
}

pub fn parse_model() -> ActiveModel {
    let (text, image) = get_os_clipboard();

    let re = Regex::new(r"^#?(?:[0-9a-f]{3}){1,2}$").unwrap();

    let r#type = if text.is_some() {
        Set("text".to_string())
    } else if text.is_some() && re.is_match(&text.as_deref().unwrap()) {
        Set("color".to_string())
    } else {
        Set("image".to_string())
    };

    let active_model = if image.is_none() {
        ActiveModel {
            ..Default::default()
        }
    } else {
        get_active_model(image)
    };

    ActiveModel {
        r#type,
        content: Set(text),
        star: Set(Some(false)),
        ..active_model
    }
}

pub fn get_active_model(image: Option<ImageData<'_>>) -> ActiveModel {
    let tmp_dir = tempfile::Builder::new()
        .prefix("clipboard-img")
        .tempdir()
        .map_err(|err| err.to_string())
        .unwrap();
    let fname = tmp_dir.path().join("clipboard-img.png");

    let image2: RgbaImage = ImageBuffer::from_raw(
        image.clone().unwrap().width.try_into().unwrap(),
        image.clone().unwrap().height.try_into().unwrap(),
        image.clone().unwrap().bytes.into_owned(),
    )
    .unwrap();

    image2
        .save(fname.clone())
        .map_err(|err| err.to_string())
        .unwrap();
    let mut file = File::open(fname.clone()).unwrap();
    let mut buffer = vec![];
    file.read_to_end(&mut buffer).unwrap();

    ActiveModel {
        blob: Set(Some(buffer)),
        height: Set(Some(image.clone().unwrap().height as i32)),
        width: Set(Some(image.clone().unwrap().width as i32)),
        size: Set(Some(
            image.clone().unwrap().bytes.to_vec().len().to_string(),
        )),
        ..Default::default()
    }
}

pub fn get_os_clipboard() -> (Option<String>, Option<ImageData<'static>>) {
    let mut clipboard = Clipboard::new().unwrap();

    let text = clipboard.get_text().ok();

    let image = clipboard.get_image().ok();

    (text, image)
}
