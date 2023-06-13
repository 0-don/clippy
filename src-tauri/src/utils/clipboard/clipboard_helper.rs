use std::{fs::File, io::Read};

use arboard::{Clipboard, ImageData};
use entity::clipboard::{self, ActiveModel, Model};
use image::{ImageBuffer, RgbaImage};
use sea_orm::{EntityTrait, QueryOrder, Set};
use tauri::regex::Regex;

use crate::{connection, service::clipboard::upsert_db};

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
    } else if text.is_some() && re.is_match(&text.as_deref().unwrap()) {
        Set("color".to_string())
    } else {
        Set("image".to_string())
    };

    let img = parse_image(image.clone()).unwrap();

    let active_model = if img.is_some() {
        let img_data = image.unwrap();
        ActiveModel {
            blob: Set(img),
            height: Set(Some(img_data.height as i32)),
            width: Set(Some(img_data.width as i32)),
            size: Set(Some(img_data.bytes.to_vec().len().to_string())),
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

pub fn parse_image(image: Option<ImageData<'static>>) -> Result<Option<Vec<u8>>, String> {
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

pub async fn check_clipboard() -> Model {
    let model = parse_model();

    let res = upsert_db(model.to_owned()).await.unwrap().unwrap();

    res
}
