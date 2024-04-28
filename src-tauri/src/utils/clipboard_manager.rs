use super::tauri::config::{APP, CLIPBOARD};
use crate::{
    connection, printlog,
    service::clipboard::{get_last_clipboard_db, insert_clipboard_db},
};
use core::time::Duration;
use enigo::{Enigo, KeyboardControllable};
use entity::clipboard::{self, ActiveModel};
use image::{imageops, ImageBuffer, Rgba};
use regex::Regex;
use sea_orm::{EntityTrait, QueryOrder, Set};
use std::{io::Cursor, process::Command};
use tauri::{
    api::dialog::{MessageDialogBuilder, MessageDialogButtons, MessageDialogKind},
    Manager,
};

const SIZE: u32 = 1280;

pub fn get_os_clipboard() -> (Option<String>, Option<arboard::ImageData<'static>>) {
    printlog!("get_os_clipboard");
    let mut text: Option<String> = CLIPBOARD
        .get()
        .unwrap()
        .lock()
        .unwrap()
        .get_text()
        .ok()
        .unwrap_or("".into())
        .trim()
        .to_string()
        .into();
    if text.is_some() && text.as_ref().unwrap().len() == 0 {
        text = None;
    }

    let image: Option<arboard::ImageData<'_>> =
        CLIPBOARD.get().unwrap().lock().unwrap().get_image().ok();

    printlog!("get_os_clipboard end");

    return (text, image);
}

#[derive(Debug, Clone)]
pub struct ClipboardHelper<'a> {
    pub clipboard: (Option<String>, Option<arboard::ImageData<'a>>),
    pub active_model: ActiveModel,
}
impl ClipboardHelper<'_> {
    pub fn new() -> Self {
        ClipboardHelper {
            clipboard: (None, None),
            active_model: ActiveModel::default(),
        }
    }

    pub async fn upsert_clipboard() {
        let mut clipboard_helper = ClipboardHelper::new();
        clipboard_helper.refresh_clipboard();

        if clipboard_helper.check_if_last_is_same().await {
            return;
        }

        let _ = insert_clipboard_db(clipboard_helper.active_model).await;

        APP.get()
            .unwrap()
            .get_window("main")
            .unwrap()
            .emit("init", ())
            .unwrap();
    }

    pub fn refresh_clipboard(&mut self) {
        self.clipboard = get_os_clipboard();
        self.active_model = self.parse_model();
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

    pub fn parse_model(&mut self) -> ActiveModel {
        let (text, image) = &self.clipboard;

        let is_link = Regex::new(r"^(https?|ftp):\/\/[^\s/$.?#].[^\s]*$").unwrap();
        let is_hex = Regex::new(r"^#?(?:[0-9a-fA-F]{3}){1,2}(?:[0-9]{2})?$").unwrap();
        let is_rgb = Regex::new(r"^(?:rgb|rgba|hsl|hsla|hsv|hwb)\((.*)\)").unwrap();

        let r#type = match text {
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
            None => {
                Set("image".to_string())
            }
        };

        let active_model = if let Some(img) = image {
            printlog!("image is start");

            let image_buffer: ImageBuffer<Rgba<u8>, Vec<u8>> = ImageBuffer::from_raw(
                img.width.try_into().unwrap(),
                img.height.try_into().unwrap(),
                img.bytes.clone().into(),
            )
            .expect("Failed to create image buffer from raw data");

            // Determine new dimensions
            let (new_width, new_height) = {
                let aspect_ratio = image_buffer.width() as f64 / image_buffer.height() as f64;
                if image_buffer.width() > SIZE || image_buffer.height() > SIZE {
                    if image_buffer.width() > image_buffer.height() {
                        (SIZE, (SIZE as f64 / aspect_ratio) as u32)
                    } else {
                        ((SIZE as f64 * aspect_ratio) as u32, SIZE)
                    }
                } else {
                    (image_buffer.width(), image_buffer.height())
                }
            };

            // Resize the image using the `image` library
            let resized_image =
                imageops::resize(&image_buffer, new_width, new_height, imageops::Nearest);

            let mut bytes: Vec<u8> = Vec::new();
            resized_image
                .write_to(&mut Cursor::new(&mut bytes), image::ImageFormat::Png)
                .expect("Failed to write resized image to buffer");

            printlog!("image is end");
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
        };

        ActiveModel {
            r#type,
            content: Set(text.to_owned()),
            star: Set(Some(false)),
            ..active_model
        }
    }
}

pub async fn type_last_clipboard() {
    let clipboard = get_last_clipboard_db().await;

    if clipboard.is_ok() {
        let clipboard = clipboard.unwrap();
        let content = clipboard.clone().content.unwrap();
        let r#type = clipboard.clone().r#type;

        if r#type != "image" && content.len() < 32 {
            let mut enigo = Enigo::new();
            enigo.key_sequence(&content);
        }
    }
}

pub async fn type_last_clipboard_linux() -> Result<(), Box<dyn std::error::Error>> {
    println!("type_last_clipboard_linux");
    // Check if xdotool is installed
    if !is_tool_installed("xdotool") {
        MessageDialogBuilder::new(
            "Missing Dependency",
            "xdotool is not installed. Please install it to continue.",
        )
        .kind(MessageDialogKind::Error)
        .buttons(MessageDialogButtons::Ok)
        .show(|_| {});
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
