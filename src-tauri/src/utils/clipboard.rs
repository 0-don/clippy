extern crate alloc;
use alloc::borrow::Cow;
use arboard::{Clipboard, ImageData};
use clipboard_master::{CallbackResult, ClipboardHandler};
use entity::clipboard::{ActiveModel, Model};
use futures::executor::block_on;
use sea_orm::{ActiveModelTrait, DatabaseConnection, DbErr, Set};
use std::io;
use tauri::regex::Regex;

use tokio::runtime::{Builder, Runtime};

use crate::connection;

pub struct Handler;

impl ClipboardHandler for Handler {
    fn on_clipboard_change(&mut self) -> CallbackResult {
        // let _ = tokio::runtime::Runtime::new().unwrap().spawn(async move {
        //     println!("text: {:?}", 1);
        //     let model = parse_model();
        //     println!("text: {:?}", 2);
        //     let res = insert(model).await;
        //     println!("text: {:?}", 3);
        // });

        // let model = parse_model();
        let model = parse_model();
        tauri::async_runtime::spawn(async move {

            let res = insert(model).await;
            println!("text: {:?}", res.unwrap());
        });

        // let _ = tauri::async_runtime::block_on(insert(model));
        // let runtime = Builder::new_multi_thread()
        //     .worker_threads(1)
        //     .enable_all()
        //     .build()
        //     .unwrap();

        // let thread = runtime.spawn(insert(model));

        CallbackResult::Next
    }

    fn on_clipboard_error(&mut self, error: io::Error) -> CallbackResult {
        println!("Error: {}", error);
        CallbackResult::Next
    }
}

fn parse_model() -> ActiveModel {
    let (text, image) = get_clipboard();

    let re = Regex::new(r"^#?(?:[0-9a-f]{3}){1,2}$").unwrap();

    let r#type = if text.is_empty() {
        Set("image".to_string())
    } else if re.is_match(&text) {
        Set("color".to_string())
    } else {
        Set("text".to_string())
    };

    // println!("type: {:?}", r#type);
    println!("text: {:?}", text);
    // println!("image: {:?}", image);

    let img = if image.width > 0 {
        ActiveModel {
            blob: Set(Some(image.bytes.to_vec())),
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
        content: Set(Some(text)),

        star: Set(Some(false)),
        ..img
    }
}

fn get_clipboard() -> (String, ImageData<'static>) {
    // Command::new("clear").status().unwrap();

    let mut clipboard = Clipboard::new().unwrap();

    let text = match clipboard.get_text() {
        Ok(text) => text,
        Err(_) => "".to_string(),
    };

    let image = match clipboard.get_image() {
        Ok(image) => image,
        Err(_) => ImageData {
            width: 0,
            height: 0,
            bytes: Cow::from([0].as_ref()),
        },
    };

    (text, image)
}

async fn insert(clipboard: ActiveModel) -> Result<Model, DbErr> {
    let db: DatabaseConnection = connection::establish_connection().await?;

    let clip_db: Model = clipboard.insert(&db).await?;

    Ok(clip_db)
}
