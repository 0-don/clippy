use super::tauri::config::APP;
use crate::{
    connection,
    service::clipboard::{get_last_clipboard_db, insert_clipboard_db},
};
use core::time::Duration;
use enigo::{Enigo, Keyboard, Settings};
use entity::clipboard::{self, ActiveModel};
use image::imageops;
use regex::Regex;
use sea_orm::{EntityTrait, QueryOrder, Set};
use std::{io::Cursor, process::Command};
use tauri::{Emitter, Manager};
use tauri_plugin_clipboard::Clipboard;
use tauri_plugin_dialog::{DialogExt, MessageDialogButtons, MessageDialogKind};

const MAX_IMAGE_SIZE: u32 = 1280;

#[derive(Debug, Clone)]
pub struct ClipboardHelper {
    pub active_model: ActiveModel,
}

impl ClipboardHelper {
    pub fn new() -> Self {
        ClipboardHelper {
            active_model: ActiveModel::default(),
        }
    }

    pub async fn upsert_clipboard() {
        let clipboard = APP.get().expect("APP not initialized").state::<Clipboard>();
        let mut clipboard_helper = ClipboardHelper::new();

        let text = clipboard.read_text().ok();
        let image_data = clipboard.read_image_binary().ok();

        clipboard_helper.parse_model(text, image_data);

        if clipboard_helper.check_if_last_is_same().await {
            return;
        }

        let _ = insert_clipboard_db(clipboard_helper.active_model).await;

        APP.get()
            .unwrap()
            .get_webview_window("main")
            .unwrap()
            .emit("init", ())
            .unwrap();
    }

    async fn check_if_last_is_same(&mut self) -> bool {
        let text = self.active_model.content.as_ref();
        let image = self.active_model.blob.as_ref();

        if text.is_none() && image.is_none() {
            return true;
        }

        let db = connection::establish_connection().await.unwrap();

        let last_clipboard = clipboard::Entity::find()
            .order_by_desc(clipboard::Column::Id)
            .one(&db)
            .await
            .unwrap();

        if last_clipboard.is_none() {
            return false;
        }

        let last_clipboard = last_clipboard.unwrap();

        if text.is_some() // check if text is same
        && last_clipboard.content.is_some()
        && text.as_ref().unwrap() == last_clipboard.content.as_ref().unwrap()
            || image.is_some() // check if image is same
            && last_clipboard.blob.is_some()
            && image.as_ref().unwrap() == last_clipboard.blob.as_ref().unwrap()
        {
            return true;
        }

        return false;
    }

    pub fn parse_model(&mut self, text: Option<String>, image_data: Option<Vec<u8>>) {
        let is_link = Regex::new(r"^(https?|ftp):\/\/[^\s/$.?#].[^\s]*$").unwrap();
        let is_hex = Regex::new(r"^#?(?:[0-9a-fA-F]{3}){1,2}(?:[0-9]{2})?$").unwrap();
        let is_rgb = Regex::new(r"^(?:rgb|rgba|hsl|hsla|hsv|hwb)\((.*)\)").unwrap();

        let r#type = if image_data.is_some() {
            Set("image".to_string())
        } else {
            match &text {
                Some(text) => {
                    if is_link.is_match(text) {
                        Set("link".to_string())
                    } else if is_hex.is_match(text) {
                        Set("hex".to_string())
                    } else if is_rgb.is_match(text) {
                        Set("rgb".to_string())
                    } else {
                        Set("text".to_string())
                    }
                }
                None => Set("text".to_string())  // Default to text if neither image nor text present
            }
        };
        
        let active_model = if let Some(img_bytes) = image_data {
            // Process image data
            if let Ok(image_buffer) = image::load_from_memory(&img_bytes) {
                let image_buffer = image_buffer.to_rgba8();

                // Determine new dimensions
                let (new_width, new_height) = {
                    let aspect_ratio = image_buffer.width() as f64 / image_buffer.height() as f64;
                    if image_buffer.width() > MAX_IMAGE_SIZE || image_buffer.height() > MAX_IMAGE_SIZE {
                        if image_buffer.width() > image_buffer.height() {
                            (MAX_IMAGE_SIZE, (MAX_IMAGE_SIZE as f64 / aspect_ratio) as u32)
                        } else {
                            ((MAX_IMAGE_SIZE as f64 * aspect_ratio) as u32, MAX_IMAGE_SIZE)
                        }
                    } else {
                        (image_buffer.width(), image_buffer.height())
                    }
                };

                // Resize image
                let resized_image =
                    imageops::resize(&image_buffer, new_width, new_height, imageops::Nearest);

                let mut bytes: Vec<u8> = Vec::new();
                if resized_image
                    .write_to(&mut Cursor::new(&mut bytes), image::ImageFormat::Png)
                    .is_ok()
                {
                    ActiveModel {
                        size: Set(Some(bytes.len().to_string())),
                        height: Set(Some(resized_image.height() as i32)),
                        width: Set(Some(resized_image.width() as i32)),
                        blob: Set(Some(bytes)),
                        ..Default::default()
                    }
                } else {
                    ActiveModel {
                        blob: Set(None),
                        ..Default::default()
                    }
                }
            } else {
                ActiveModel {
                    blob: Set(None),
                    ..Default::default()
                }
            }
        } else {
            ActiveModel {
                blob: Set(None),
                ..Default::default()
            }
        };

        self.active_model = ActiveModel {
            r#type,
            content: Set(text),
            star: Set(Some(false)),
            ..active_model
        };
    }
}

pub async fn type_last_clipboard() {
    let clipboard = get_last_clipboard_db().await;

    if clipboard.is_ok() {
        let clipboard = clipboard.unwrap();
        let content = clipboard.clone().content.unwrap();
        let r#type = clipboard.clone().r#type;

        if r#type != "image" && content.len() < 32 {
            let mut enigo = Enigo::new(&Settings::default()).unwrap();
            let _ = enigo.text(&content);
        }
    }
}

pub async fn type_last_clipboard_linux() -> Result<(), Box<dyn std::error::Error>> {
    println!("type_last_clipboard_linux");
    // Check if xdotool is installed
    if !is_tool_installed("xdotool") {
        APP.get()
            .unwrap()
            .dialog()
            .message("xdotool is not installed. Please install it to continue.")
            .title("Missing Dependency")
            .kind(MessageDialogKind::Error)
            .buttons(MessageDialogButtons::Ok)
            .blocking_show();
        return Ok(());
    }

    let clipboard = get_last_clipboard_db().await;

    if clipboard.is_ok() {
        let clipboard = clipboard?;
        let content = clipboard.clone().content.unwrap();
        let r#type = clipboard.clone().r#type;

        if r#type != "image" && content.len() < 500 {
            std::thread::sleep(Duration::from_millis(300));
            Command::new("xdotool")
                .args(&["type", "--clearmodifiers", "--", &content])
                .output()?;
        }
    }

    return Ok(());
}

pub fn is_tool_installed(tool: &str) -> bool {
    Command::new(tool)
        .arg("--version")
        .output()
        .map(|output| output.status.success())
        .unwrap_or(false)
}
