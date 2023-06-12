extern crate alloc;
use crate::connection;
use alloc::borrow::Cow;
use arboard::{Clipboard, ImageData};
use clipboard_master::{CallbackResult, ClipboardHandler};
use entity::clipboard::{self, ActiveModel, Model};
use sea_orm::{ActiveModelTrait, DatabaseConnection, DbErr, EntityTrait, QueryOrder, Set};
use std::io;
use tauri::{regex::Regex, Manager};

use super::setup::APP;

pub struct Handler;

impl ClipboardHandler for Handler {
    fn on_clipboard_change(&mut self) -> CallbackResult {
        // let res = tokio::task::block_in_place(|| {
        //     tokio::runtime::Handle::current().block_on(async move {
        //         let model = parse_model();
        //         insert(model).await
        //     })
        // });

        let _ = tauri::async_runtime::spawn(async {
            let model = parse_model();

            let model = upsert(model).await.unwrap();
            // let main_window = APP.get_window("main").unwrap();
            let main_window = APP.get().unwrap().get_window("main").unwrap();
            main_window.emit("clipboard_listener", model).unwrap();
        });

        CallbackResult::Next
    }

    fn on_clipboard_error(&mut self, error: io::Error) -> CallbackResult {
        println!("Error: {}", error);
        CallbackResult::Next
    }
}

async fn check_if_last_same() -> Option<Model> {
    let (text, image) = get_os_clipboard();

    let str = text.unwrap();
    let img = image.unwrap();

    let db = connection::establish_connection().await.unwrap();

    let last_clipboard = clipboard::Entity::find()
        .order_by_desc(clipboard::Column::Id)
        .one(&db)
        .await
        .unwrap()
        .unwrap();

    let content = last_clipboard.clone().content.unwrap();
    let blob = last_clipboard.clone().blob.unwrap();

    if content == str && blob == img.bytes.to_vec() {
        return None;
    }

    Some(last_clipboard)
}

fn parse_model() -> ActiveModel {
    let (text, image) = get_os_clipboard();

    let str = text.unwrap();
    let img = image.unwrap();
    // println!("text: {:?}", text);

    let re = Regex::new(r"^#?(?:[0-9a-f]{3}){1,2}$").unwrap();

    let r#type = if str.is_empty() {
        Set("image".to_string())
    } else if re.is_match(&str) {
        Set("color".to_string())
    } else {
        Set("text".to_string())
    };

    // println!("type: {:?}", r#type);
    // println!("text: {:?}", text);
    // println!("image: {:?}", image);

    let img = if img.width > 0 {
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
        content: Set(Some(str)),

        star: Set(Some(false)),
        ..img
    }
}

fn get_os_clipboard() -> (Option<String>, Option<ImageData<'static>>) {
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

async fn upsert(clipboard: ActiveModel) -> Result<Option<Model>, DbErr> {
    let is_same = check_if_last_same().await;
    if is_same.is_some() {
        ()
    }
    let db: DatabaseConnection = connection::establish_connection().await?;

    let clip_db: Model = clipboard.insert(&db).await?;

    Ok(Some(clip_db))
}
