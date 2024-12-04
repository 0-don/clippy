use super::tauri::config::APP;
use crate::{
    connection,
    service::clipboard::insert_clipboard_db,
    types::orm_query::{ClipboardManager, ClipboardWithRelations},
};
use base64::{engine::general_purpose::STANDARD, Engine};
use entity::clipboard::{self};
use image::imageops;
use migration::{ClipboardTextType, ClipboardType};
use regex::Regex;
use sea_orm::RelationTrait;
use sea_orm::{DbErr, Iden};
use sea_orm::{EntityTrait, QueryOrder, Set};
use sea_orm::{JoinType, QuerySelect};
use std::io::Cursor;
use tauri::{Emitter, Manager};
use tauri_plugin_clipboard::Clipboard;

const MAX_IMAGE_SIZE: u32 = 1280;

impl ClipboardManager {
    pub fn new() -> Self {
        ClipboardManager {
            clipboard_model: entity::clipboard::ActiveModel::default(),
            clipboard_text_model: entity::clipboard_text::ActiveModel::default(),
            clipboard_html_model: entity::clipboard_html::ActiveModel::default(),
            clipboard_image_model: entity::clipboard_image::ActiveModel::default(),
            clipboard_rtf_model: entity::clipboard_rtf::ActiveModel::default(),
            clipboard_file_model: entity::clipboard_file::ActiveModel::default(),
        }
    }

    pub async fn upsert_clipboard() {
        let clipboard = APP.get().expect("APP not initialized").state::<Clipboard>();
        let mut clipboard_manager = ClipboardManager::new();

        let text = clipboard.read_text().ok();
        let html = clipboard.read_html().ok();
        let rtf = clipboard.read_rtf().ok();
        let image_data = clipboard.read_image_binary().ok();
        let files = clipboard.read_files().ok();

        clipboard_manager.parse_model(text, html, rtf, image_data, files);

        if clipboard_manager.check_if_last_is_same().await {
            return;
        }

        insert_clipboard_db(clipboard_manager).await.unwrap();

        APP.get()
            .unwrap()
            .get_webview_window("main")
            .unwrap()
            .emit("init", ())
            .unwrap();
    }

    async fn check_if_last_is_same(&mut self) -> bool {
        let db = connection::establish_connection().await.unwrap();

        let last_result: Result<Option<ClipboardWithRelations>, DbErr> = clipboard::Entity::find()
            .select_only()
            .column_as(clipboard::Column::Id, "clipboard_id")
            .columns([
                clipboard::Column::Type,
                clipboard::Column::Star,
                clipboard::Column::CreatedDate,
            ])
            .join(JoinType::LeftJoin, clipboard::Relation::ClipboardText.def())
            .join(JoinType::LeftJoin, clipboard::Relation::ClipboardHtml.def())
            .join(
                JoinType::LeftJoin,
                clipboard::Relation::ClipboardImage.def(),
            )
            .join(JoinType::LeftJoin, clipboard::Relation::ClipboardRtf.def())
            .join(JoinType::LeftJoin, clipboard::Relation::ClipboardFile.def())
            .order_by_desc(clipboard::Column::Id)
            .into_model::<ClipboardWithRelations>()
            .one(&db)
            .await;

        if let Ok(Some(last_clipboard)) = last_result {
            // Compare based on type
            let current_type = match &self.clipboard_model.r#type {
                sea_orm::ActiveValue::Set(val) => Some(val),
                _ => None,
            };

            if let Some(current_type) = current_type {
                // If types don't match, it's definitely different
                if current_type != &last_clipboard.clipboard.r#type {
                    return false;
                }

                // Compare content based on type
                match current_type.as_str() {
                    type_str if type_str == ClipboardType::Text.to_string() => {
                        if let Some(text_model) = last_clipboard.text {
                            if let sea_orm::ActiveValue::Set(current_text) =
                                &self.clipboard_text_model.data
                            {
                                return current_text == &text_model.data;
                            }
                        }
                    }
                    type_str if type_str == ClipboardType::Html.to_string() => {
                        if let Some(html_model) = last_clipboard.html {
                            if let sea_orm::ActiveValue::Set(current_html) =
                                &self.clipboard_html_model.data
                            {
                                return current_html == &html_model.data;
                            }
                        }
                    }
                    type_str if type_str == ClipboardType::Rtf.to_string() => {
                        if let Some(rtf_model) = last_clipboard.rtf {
                            if let sea_orm::ActiveValue::Set(current_rtf) =
                                &self.clipboard_rtf_model.data
                            {
                                return current_rtf == &rtf_model.data;
                            }
                        }
                    }
                    type_str if type_str == ClipboardType::Image.to_string() => {
                        if let Some(image_model) = last_clipboard.image {
                            if let sea_orm::ActiveValue::Set(current_image) =
                                &self.clipboard_image_model.data
                            {
                                return current_image == &image_model.data;
                            }
                        }
                    }
                    type_str if type_str == ClipboardType::File.to_string() => {
                        if let Some(file_model) = last_clipboard.file {
                            if let sea_orm::ActiveValue::Set(current_file) =
                                &self.clipboard_file_model.data
                            {
                                return current_file == &file_model.data;
                            }
                        }
                    }
                    _ => return false,
                }
            }
        }

        false
    }

    pub fn parse_model(
        &mut self,
        text: Option<String>,
        html: Option<String>,
        rtf: Option<String>,
        image_data: Option<Vec<u8>>,
        files: Option<Vec<String>>,
    ) {
        let mut r#type = String::new();

        if text.is_some() {
            r#type = ClipboardType::Text.to_string();

            let is_link = Regex::new(r"^(https?|ftp):\/\/[^\s/$.?#].[^\s]*$").unwrap();
            let is_hex = Regex::new(r"^#?(?:[0-9a-fA-F]{3}){1,2}(?:[0-9]{2})?$").unwrap();
            let is_rgb = Regex::new(r"^(?:rgb|rgba|hsl|hsla|hsv|hwb)\((.*)\)").unwrap();

            self.clipboard_text_model.r#type = Set(ClipboardTextType::Text.to_string());

            if is_link.is_match(text.as_ref().unwrap()) {
                self.clipboard_text_model.r#type = Set(ClipboardTextType::Link.to_string());
            } else if is_hex.is_match(text.as_ref().unwrap()) {
                self.clipboard_text_model.r#type = Set(ClipboardTextType::Hex.to_string());
            } else if is_rgb.is_match(text.as_ref().unwrap()) {
                self.clipboard_text_model.r#type = Set(ClipboardTextType::Rgb.to_string());
            }

            self.clipboard_text_model.data = Set(text.unwrap());
        }

        if html.is_some() {
            r#type = ClipboardType::Html.to_string();
            self.clipboard_html_model.data = Set(html.unwrap());
        }

        if rtf.is_some() {
            r#type = ClipboardType::Rtf.to_string();
            self.clipboard_rtf_model.data = Set(rtf.unwrap());
        }

        if image_data.is_some() {
            r#type = ClipboardType::Image.to_string();
            self.parse_image_model(image_data.unwrap());
        }

        if files.is_some() {
            r#type = ClipboardType::File.to_string();
            println!("{:?}", files);
            // self.clipboard_file_model.data = Set(files.unwrap());
        }

        self.clipboard_model = entity::clipboard::ActiveModel {
            r#type: Set(r#type),
            star: Set(Some(false)),
            ..Default::default()
        };
    }

    fn parse_image_model(&mut self, img_bytes: Vec<u8>) {
        if let Ok(image_buffer) = image::load_from_memory(&img_bytes) {
            let image_buffer = image_buffer.to_rgba8();
            let (width, height) = (image_buffer.width(), image_buffer.height());

            let (new_width, new_height) = calculate_thumbnail_dimensions(width, height);

            let thumbnail =
                imageops::resize(&image_buffer, new_width, new_height, imageops::Nearest);
            let mut thumbnail_bytes = Vec::new();

            if thumbnail
                .write_to(
                    &mut Cursor::new(&mut thumbnail_bytes),
                    image::ImageFormat::Png,
                )
                .is_ok()
            {
                let base64_thumbnail = STANDARD.encode(&thumbnail_bytes);

                self.clipboard_image_model = entity::clipboard_image::ActiveModel {
                    size: Set(Some(img_bytes.len().to_string())),
                    data: Set(img_bytes),
                    width: Set(Some(width as i32)),
                    height: Set(Some(height as i32)),
                    thumbnail: Set(Some(base64_thumbnail)),
                    ..Default::default()
                };
            }
        }
    }
}

fn calculate_thumbnail_dimensions(width: u32, height: u32) -> (u32, u32) {
    let aspect_ratio = width as f64 / height as f64;
    if width > MAX_IMAGE_SIZE || height > MAX_IMAGE_SIZE {
        if width > height {
            (
                MAX_IMAGE_SIZE,
                (MAX_IMAGE_SIZE as f64 / aspect_ratio) as u32,
            )
        } else {
            (
                (MAX_IMAGE_SIZE as f64 * aspect_ratio) as u32,
                MAX_IMAGE_SIZE,
            )
        }
    } else {
        (width, height)
    }
}
