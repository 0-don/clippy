use crate::prelude::*;
use crate::service::settings::get_settings_db;
use crate::service::{
    clipboard::{get_last_clipboard_db, insert_clipboard_db},
    global::{get_app, get_app_window},
    window::calculate_thumbnail_dimensions,
};
use base64::{engine::general_purpose::STANDARD, Engine};
use common::types::enums::{ClipboardTextType, ClipboardType, ListenEvent, WebWindow};
use common::types::orm_query::ClipboardManager;
use image::imageops;
use regex::Regex;
use std::fs;
use std::io::Cursor;
use std::path::Path;
use tauri::{Emitter, Manager};
use tauri_plugin_clipboard::Clipboard;

pub trait ClipboardManagerExt {
    fn new() -> ClipboardManager;
    fn upsert_clipboard() -> impl std::future::Future<Output = ()> + Send;
    fn check_if_last_is_same(&mut self) -> impl std::future::Future<Output = bool> + Send;
    fn parse_model(
        &mut self,
        text: Option<String>,
        html: Option<String>,
        rtf: Option<String>,
        image_data: Option<Vec<u8>>,
        files: Option<Vec<String>>,
    ) -> impl std::future::Future<Output = ()> + Send;
    fn parse_image_model(&mut self, img_bytes: Vec<u8>);
    fn parse_file_models(&mut self, file_paths: Vec<String>) -> std::io::Result<()>;
}

impl ClipboardManagerExt for ClipboardManager {
    fn new() -> Self {
        ClipboardManager {
            clipboard_model: entity::clipboard::ActiveModel::default(),
            clipboard_text_model: entity::clipboard_text::ActiveModel::default(),
            clipboard_html_model: entity::clipboard_html::ActiveModel::default(),
            clipboard_image_model: entity::clipboard_image::ActiveModel::default(),
            clipboard_rtf_model: entity::clipboard_rtf::ActiveModel::default(),
            clipboard_files_model: Vec::new(),
        }
    }

    async fn upsert_clipboard() {
        let clipboard = get_app().state::<Clipboard>();
        let mut clipboard_manager = ClipboardManager::new();

        let text = clipboard.read_text().ok();
        let html = clipboard.read_html().ok();
        let rtf = clipboard.read_rtf().ok();
        let image_data = clipboard.read_image_binary().ok();
        let files: Option<Vec<String>> = clipboard.read_files().ok();

        clipboard_manager
            .parse_model(text, html, rtf, image_data, files)
            .await;

        if clipboard_manager.check_if_last_is_same().await {
            return;
        }

        insert_clipboard_db(clipboard_manager)
            .await
            .expect("Failed to insert clipboard");

        get_app_window(WebWindow::Main)
            .emit(ListenEvent::Init.to_string().as_str(), ())
            .expect("Failed to emit event");
    }

    async fn check_if_last_is_same(&mut self) -> bool {
        let last_result = get_last_clipboard_db().await;

        if let Ok(last_clipboard) = last_result {
            // Get types from last clipboard
            let last_types = ClipboardType::from_json_value(&last_clipboard.clipboard.types);

            // Get types from current clipboard model
            let current_types =
                ClipboardType::from_json_value(&self.clipboard_model.types.as_ref());

            if let (Some(last_types), Some(current_types)) = (last_types, current_types) {
                // Compare if both arrays have same length
                if last_types.len() != current_types.len() {
                    return false;
                }

                // Check if all types match between arrays
                let types_match = last_types.iter().all(|t| current_types.contains(t))
                    && current_types.iter().all(|t| last_types.contains(t));

                if !types_match {
                    return false;
                }

                // Compare content for each type
                for clipboard_type in current_types {
                    match clipboard_type {
                        ClipboardType::Text => {
                            if let Some(text_model) = &last_clipboard.text {
                                if let sea_orm::ActiveValue::Set(current_text) =
                                    &self.clipboard_text_model.data
                                {
                                    if current_text != &text_model.data {
                                        return false;
                                    }
                                }
                            }
                        }
                        ClipboardType::Html => {
                            if let Some(html_model) = &last_clipboard.html {
                                if let sea_orm::ActiveValue::Set(current_html) =
                                    &self.clipboard_html_model.data
                                {
                                    if current_html != &html_model.data {
                                        return false;
                                    }
                                }
                            }
                        }
                        ClipboardType::Rtf => {
                            if let Some(rtf_model) = &last_clipboard.rtf {
                                if let sea_orm::ActiveValue::Set(current_rtf) =
                                    &self.clipboard_rtf_model.data
                                {
                                    if current_rtf != &rtf_model.data {
                                        return false;
                                    }
                                }
                            }
                        }
                        ClipboardType::Image => {
                            if let Some(image_model) = &last_clipboard.image {
                                if let sea_orm::ActiveValue::Set(current_image) =
                                    &self.clipboard_image_model.data
                                {
                                    if current_image != &image_model.data {
                                        return false;
                                    }
                                }
                            }
                        }
                        ClipboardType::File => {
                            if self.clipboard_files_model.len() != last_clipboard.files.len() {
                                return false;
                            }

                            // Compare each file
                            for (i, last_file) in last_clipboard.files.iter().enumerate() {
                                if let Some(current_file) = self.clipboard_files_model.get(i) {
                                    if let sea_orm::ActiveValue::Set(ref current_data) =
                                        current_file.data
                                    {
                                        if current_data != &last_file.data {
                                            return false;
                                        }
                                    }
                                } else {
                                    return false;
                                }
                            }
                        }
                    }
                }

                // If we get here, all types and their contents match
                return true;
            }
        }

        false
    }

    async fn parse_model(
        &mut self,
        text: Option<String>,
        html: Option<String>,
        rtf: Option<String>,
        image_data: Option<Vec<u8>>,
        files: Option<Vec<String>>,
    ) {
        let settings = get_settings_db().await.expect("Failed to get settings");
        let mut types: Vec<ClipboardType> = vec![];

        if let Some(text_content) = text {
            if !text_content.is_empty() && text_content.len() <= settings.max_text_size as usize {
                types.push(ClipboardType::Text);

                let is_link = Regex::new(r"^(https?|ftp):\/\/[^\s/$.?#].[^\s]*$")
                    .expect("Failed to compile link regex");
                let is_hex = Regex::new(r"^#?(?:[0-9a-fA-F]{3}){1,2}(?:[0-9]{2})?$")
                    .expect("Failed to compile hex regex");
                let is_rgb = Regex::new(r"^(?:rgb|rgba|hsl|hsla|hsv|hwb)\((.*)\)")
                    .expect("Failed to compile rgb regex");

                self.clipboard_text_model.r#type = Set(ClipboardTextType::Text.to_string());

                if is_link.is_match(&text_content) {
                    self.clipboard_text_model.r#type = Set(ClipboardTextType::Link.to_string());
                } else if is_hex.is_match(&text_content) {
                    self.clipboard_text_model.r#type = Set(ClipboardTextType::Hex.to_string());
                } else if is_rgb.is_match(&text_content) {
                    self.clipboard_text_model.r#type = Set(ClipboardTextType::Rgb.to_string());
                }

                self.clipboard_text_model.data = Set(text_content);
            }
        }

        if let Some(html_content) = html {
            if !html_content.is_empty() && html_content.len() <= settings.max_html_size as usize {
                types.push(ClipboardType::Html);
                self.clipboard_html_model.data = Set(html_content);
            }
        }

        if let Some(rtf_content) = rtf {
            if !rtf_content.is_empty() && rtf_content.len() <= settings.max_rtf_size as usize {
                types.push(ClipboardType::Rtf);
                self.clipboard_rtf_model.data = Set(rtf_content);
            }
        }

        if let Some(image_content) = image_data {
            if !image_content.is_empty() && image_content.len() <= settings.max_image_size as usize
            {
                types.push(ClipboardType::Image);
                self.parse_image_model(image_content);
            }
        }

        if let Some(file_paths) = files {
            if !file_paths.is_empty() {
                let mut valid_files = Vec::new();

                for path in &file_paths {
                    if let Ok(metadata) = fs::metadata(path) {
                        let file_size = metadata.len() as i32;

                        if file_size <= settings.max_file_size {
                            valid_files.push(path.clone());
                        } else {
                            println!("Skipping file {}: exceeds size limit", path);
                        }
                    }
                }

                if !valid_files.is_empty() {
                    types.push(ClipboardType::File);
                    if let Err(e) = self.parse_file_models(valid_files) {
                        println!("Error processing files: {}", e);
                    }
                }
            }
        }

        println!("{:?}", ClipboardType::to_json_value(&types));

        self.clipboard_model = entity::clipboard::ActiveModel {
            types: Set(ClipboardType::to_json_value(&types)),
            star: Set(false),
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

    fn parse_file_models(&mut self, file_paths: Vec<String>) -> std::io::Result<()> {
        for path in file_paths {
            let path = Path::new(&path);

            if let Ok(metadata) = fs::metadata(path) {
                let file_name = path
                    .file_name()
                    .and_then(|n| n.to_str())
                    .unwrap_or("unknown")
                    .to_string();

                let file_ext = path
                    .extension()
                    .and_then(|e| e.to_str())
                    .map(|s| s.to_string());

                let file_size = metadata.len() as i32;

                // Read file bytes
                if let Ok(file_bytes) = fs::read(path) {
                    let file_model: entity::clipboard_file::ActiveModel =
                        entity::clipboard_file::ActiveModel {
                            name: Set(Some(file_name)),
                            extension: Set(file_ext),
                            size: Set(Some(file_size)),
                            data: Set(file_bytes),
                            ..Default::default()
                        };

                    self.clipboard_files_model.push(file_model);
                }
            }
        }
        Ok(())
    }
}
