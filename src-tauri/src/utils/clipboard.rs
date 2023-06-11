extern crate alloc;
use alloc::borrow::Cow;
use arboard::{Clipboard, ImageData};
use async_trait::async_trait;
use clipboard_master::{CallbackResult, ClipboardHandler};
use entity::clipboard::{ActiveModel, Model};
use sea_orm::{ActiveModelTrait, DatabaseConnection, DbErr, Set};
use std::{io, process::Command};
use tauri::regex::Regex;

use crate::connection;

pub struct Handler;

impl ClipboardHandler for Handler {
    fn on_clipboard_change(&mut self) -> CallbackResult {
        let (text, image) = get_clipboard();

        let re = Regex::new(r"^#?(?:[0-9a-f]{3}){1,2}$").unwrap();

        let r#type = if text.is_empty() {
            Set("image".to_string())
        } else if re.is_match(&text) {
            Set("color".to_string())
        } else {
            Set("text".to_string())
        };

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

        let model = ActiveModel {
            r#type,
            content: Set(Some(text)),

            star: Set(Some(false)),
            ..img
        };

        CallbackResult::Next
    }

    fn on_clipboard_error(&mut self, error: io::Error) -> CallbackResult {
        eprintln!("Error: {}", error);
        CallbackResult::Next
    }
}

fn get_clipboard() -> (String, ImageData<'static>) {
    Command::new("clear").status().unwrap();

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
