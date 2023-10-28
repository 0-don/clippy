use super::tauri::config::{APP, CLIPBOARD};
use crate::{
    connection, printlog,
    service::clipboard::{get_last_clipboard_db, insert_clipboard_db},
};
use core::time::Duration;
use enigo::{Enigo, KeyboardControllable};
use entity::clipboard::{self, ActiveModel};
use fast_image_resize as fr;
use image::{ImageBuffer, Rgba};
use sea_orm::{EntityTrait, QueryOrder, Set};
use std::{io::Cursor, num::NonZeroU32, process::Command};
use tauri::{
    api::dialog::{MessageDialogBuilder, MessageDialogButtons, MessageDialogKind},
    regex::Regex,
    Manager,
};

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

        let re = Regex::new(r"^#?(?:[0-9a-f]{3}){1,2}$").unwrap();

        let r#type = if text.is_some() && re.is_match(text.as_ref().unwrap()) {
            Set("color".to_string())
        } else if text.is_some() {
            Set("text".to_string())
        } else {
            Set("image".to_string())
        };

        let active_model = if image.is_none() {
            ActiveModel {
                blob: Set(None),
                ..Default::default()
            }
        } else {
            printlog!("image is start");
            let image_buffer: Option<ImageBuffer<Rgba<u8>, Vec<u8>>> = if image.is_some() {
                ImageBuffer::from_raw(
                    image.as_ref().unwrap().width.try_into().unwrap(),
                    image.as_ref().unwrap().height.try_into().unwrap(),
                    image.as_ref().unwrap().bytes.clone().into(),
                )
            } else {
                None
            };

            // Convert to fast_image_resize::Image
            let width = NonZeroU32::new(image_buffer.as_ref().unwrap().width()).unwrap();
            let height = NonZeroU32::new(image_buffer.as_ref().unwrap().height()).unwrap();
            let src_image = fr::Image::from_vec_u8(
                width,
                height,
                image_buffer.unwrap().into_raw(),
                fr::PixelType::U8x4,
            )
            .unwrap();

            // Determine new dimensions
            let (new_width, new_height) = if width.get() > 1200 || height.get() > 1200 {
                let aspect_ratio = width.get() as f64 / height.get() as f64;
                if width.get() > height.get() {
                    (
                        NonZeroU32::new(1200).unwrap(),
                        NonZeroU32::new((1200.0 / aspect_ratio) as u32).unwrap(),
                    )
                } else {
                    (
                        NonZeroU32::new((1200.0 * aspect_ratio) as u32).unwrap(),
                        NonZeroU32::new(1200).unwrap(),
                    )
                }
            } else {
                (width, height)
            };

            // Create destination image and resizer
            let mut dst_image = fr::Image::new(new_width, new_height, src_image.pixel_type());
            let mut resizer = fr::Resizer::new(fr::ResizeAlg::Nearest);
            unsafe {
                resizer.set_cpu_extensions(fr::CpuExtensions::Sse4_1);
            }
            resizer
                .resize(&src_image.view(), &mut dst_image.view_mut())
                .unwrap();

            // Convert back to ImageBuffer for further processing
            let resized_image: ImageBuffer<Rgba<u8>, Vec<u8>> =
                ImageBuffer::from_raw(new_width.get(), new_height.get(), dst_image.into_vec())
                    .unwrap();

            let mut bytes: Vec<u8> = Vec::new();
            resized_image
                .write_to(&mut Cursor::new(&mut bytes), image::ImageOutputFormat::Png)
                .unwrap();

            printlog!("image is end");
            ActiveModel {
                size: Set(Some(bytes.len().to_string())),
                height: Set(Some(resized_image.height() as i32)),
                width: Set(Some(resized_image.width() as i32)),
                blob: Set(Some(bytes)),
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
        .kind(MessageDialogKind::Error) // this will indicate that the message is an error
        .buttons(MessageDialogButtons::Ok) // this will add an "Ok" button to the dialog
        .show(|pressed_ok| {
            if pressed_ok {
                // Handle the case when the user presses the "Ok" button
            }
        });
        return Ok(());
    }

    let clipboard = get_last_clipboard_db().await;

    if clipboard.is_ok() {
        let clipboard = clipboard?;
        let content = clipboard.clone().content.unwrap();
        let r#type = clipboard.clone().r#type;

        if r#type != "image" && content.len() < 32 {
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
