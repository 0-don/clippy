extern crate alloc;
use crate::service::clipboard::{get_os_clipboard, upsert};
use clipboard_master::{CallbackResult, ClipboardHandler};
use entity::clipboard::ActiveModel;
use sea_orm::Set;
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

fn parse_model() -> ActiveModel {
    let (text, image) = get_os_clipboard();

    let re = Regex::new(r"^#?(?:[0-9a-f]{3}){1,2}$").unwrap();
    let str = text.clone().unwrap();

    let r#type = if text.is_some() {
        Set("image".to_string())
    } else if re.is_match(&str) {
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
        content: Set(Some(str)),

        star: Set(Some(false)),
        ..img
    }
}
