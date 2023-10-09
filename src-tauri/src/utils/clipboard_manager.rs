use super::tauri::config::{APP, CLIPBOARD};
use crate::{
    connection,
    service::clipboard::{get_last_clipboard_db, insert_clipboard_db},
};
use enigo::{Enigo, KeyboardControllable};
use entity::clipboard::{self, ActiveModel};
use image::{ImageBuffer, RgbaImage};
use sea_orm::{EntityTrait, QueryOrder, Set};
use std::io::Cursor;
use tauri::{regex::Regex, Manager};

pub fn get_os_clipboard() -> (Option<String>, Option<RgbaImage>) {
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

    let image = CLIPBOARD.get().unwrap().lock().unwrap().get_image().ok();

    let image: Option<RgbaImage> = if image.is_some() {
        ImageBuffer::from_raw(
            image.as_ref().unwrap().width.try_into().unwrap(),
            image.as_ref().unwrap().height.try_into().unwrap(),
            image.unwrap().bytes.into_owned(),
        )
    } else {
        None
    };

    return (text, image);
}

#[derive(Debug, Clone)]
pub struct ClipboardHelper {
    pub clipboard: (Option<String>, Option<RgbaImage>),
    pub active_model: ActiveModel,
}
impl ClipboardHelper {
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
            .emit("init", "")
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
            let mut bytes: Vec<u8> = Vec::new();
            image
                .as_ref()
                .unwrap()
                .write_to(&mut Cursor::new(&mut bytes), image::ImageOutputFormat::Png)
                .unwrap();

            ActiveModel {
                size: Set(Some(bytes.len().to_string())),
                height: Set(Some(image.as_ref().unwrap().height() as i32)),
                width: Set(Some(image.as_ref().unwrap().width() as i32)),
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
        let content = clipboard.content.unwrap();
        let r#type = clipboard.r#type;

        if r#type != "image" && content.len() < 32 {
            let mut enigo = Enigo::new();
            enigo.key_sequence(content.as_str());
        }
    }
}
