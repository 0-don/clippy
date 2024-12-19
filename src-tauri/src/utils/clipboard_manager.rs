use crate::prelude::*;
use crate::service::settings::get_settings_db;
use crate::service::{
    clipboard::{get_last_clipboard_db, insert_clipboard_db},
    global::{get_app, get_app_window},
    window::calculate_thumbnail_dimensions,
};
use base64::{engine::general_purpose::STANDARD, Engine};
use chrono::DateTime;
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
        let mut manager = Self::new();

        manager
            .parse_model(
                clipboard.read_text().ok(),
                clipboard.read_html().ok(),
                clipboard.read_rtf().ok(),
                clipboard.read_image_binary().ok(),
                clipboard.read_files().ok(),
            )
            .await;

        if !manager.check_if_last_is_same().await {
            let clipboard = insert_clipboard_db(manager)
                .await
                .expect("Failed to insert");
            get_app_window(WebWindow::Main)
                .emit(ListenEvent::NewClipboard.to_string().as_str(), clipboard)
                .expect("Failed to emit");
        }
    }

    async fn check_if_last_is_same(&mut self) -> bool {
        if let Ok(last) = get_last_clipboard_db().await {
            let last_types = ClipboardType::from_json_value(&last.clipboard.types);
            let curr_types = ClipboardType::from_json_value(&self.clipboard_model.types.as_ref());

            match (last_types, curr_types) {
                (Some(lt), Some(ct)) if lt.len() == ct.len() => {
                    for t in ct {
                        match t {
                            ClipboardType::Text => {
                                if let (Some(last), sea_orm::ActiveValue::Set(curr)) =
                                    (&last.text, &self.clipboard_text_model.data)
                                {
                                    if curr != &last.data {
                                        return false;
                                    }
                                }
                            }
                            ClipboardType::Html => {
                                if let (Some(last), sea_orm::ActiveValue::Set(curr)) =
                                    (&last.html, &self.clipboard_html_model.data)
                                {
                                    if curr != &last.data {
                                        return false;
                                    }
                                }
                            }
                            ClipboardType::Image => {
                                if let (Some(last), sea_orm::ActiveValue::Set(curr)) =
                                    (&last.image, &self.clipboard_image_model.data)
                                {
                                    if curr != &last.data {
                                        return false;
                                    }
                                }
                            }
                            ClipboardType::Rtf => {
                                if let (Some(last), sea_orm::ActiveValue::Set(curr)) =
                                    (&last.rtf, &self.clipboard_rtf_model.data)
                                {
                                    if curr != &last.data {
                                        return false;
                                    }
                                }
                            }
                            ClipboardType::File => {
                                if self.clipboard_files_model.len() != last.files.len() {
                                    return false;
                                }
                                for (curr_file, last_file) in
                                    self.clipboard_files_model.iter().zip(&last.files)
                                {
                                    if let sea_orm::ActiveValue::Set(curr_data) = &curr_file.data {
                                        if curr_data != &last_file.data {
                                            return false;
                                        }
                                    }
                                }
                            }
                        }
                    }
                    true
                }
                _ => false,
            }
        } else {
            false
        }
    }

    async fn parse_model(
        &mut self,
        text: Option<String>,
        html: Option<String>,
        rtf: Option<String>,
        image: Option<Vec<u8>>,
        files: Option<Vec<String>>,
    ) {
        let settings = get_settings_db().await.expect("Settings failed");
        let mut types = vec![];

        if let Some(text) =
            text.filter(|t| !t.is_empty() && t.len() <= settings.max_text_size as usize)
        {
            types.push(ClipboardType::Text);
            self.clipboard_text_model.data = Set(text.clone());
            self.clipboard_text_model.r#type = Set(match text {
                t if Regex::new(r"^(https?|ftp):\/\/[^\s/$.?#].[^\s]*$")
                    .expect("Invalid regex")
                    .is_match(&t) =>
                {
                    ClipboardTextType::Link
                }
                t if Regex::new(r"^#?(?:[0-9a-fA-F]{3}){1,2}(?:[0-9]{2})?$")
                    .expect("Invalid regex")
                    .is_match(&t) =>
                {
                    ClipboardTextType::Hex
                }
                t if Regex::new(r"^(?:rgb|rgba|hsl|hsla|hsv|hwb)\((.*)\)")
                    .expect("Invalid regex")
                    .is_match(&t) =>
                {
                    ClipboardTextType::Rgb
                }
                _ => ClipboardTextType::Text,
            }
            .to_string());
        }

        if let Some(html) =
            html.filter(|h| !h.is_empty() && h.len() <= settings.max_html_size as usize)
        {
            types.push(ClipboardType::Html);
            self.clipboard_html_model.data = Set(html);
        }

        if let Some(rtf) =
            rtf.filter(|r| !r.is_empty() && r.len() <= settings.max_rtf_size as usize)
        {
            types.push(ClipboardType::Rtf);
            self.clipboard_rtf_model.data = Set(rtf);
        }

        if let Some(img) =
            image.filter(|i| !i.is_empty() && i.len() <= settings.max_image_size as usize)
        {
            types.clear(); // Clear all types if image is present
            types.push(ClipboardType::Image);
            self.parse_image_model(img);
        }

        if let Some(paths) = files.filter(|f| !f.is_empty()) {
            let valid_files: Vec<_> = paths
                .into_iter()
                .filter(|p| {
                    fs::metadata(p)
                        .map(|m| m.len() as i32 <= settings.max_file_size)
                        .unwrap_or(false)
                })
                .collect();

            if !valid_files.is_empty() {
                types.clear(); // Clear all types if files are present
                types.push(ClipboardType::File);
                let _ = self.parse_file_models(valid_files);
            }
        }

        printlog!("clipboard types: {:?}", types);

        self.clipboard_model = entity::clipboard::ActiveModel {
            types: Set(ClipboardType::to_json_value(&types)),
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
                    .file_stem()
                    .and_then(|n| n.to_str())
                    .unwrap_or("ERROR.UNKNOWN_FILE")
                    .to_string();

                let file_ext = path
                    .extension()
                    .and_then(|e| e.to_str())
                    .map(|s| s.to_string());

                // Get MIME type using tree_magic_mini
                let mime_type = Some(
                    if let Some(content_type) = tree_magic_mini::from_filepath(path) {
                        content_type.to_string()
                    } else {
                        // We only get here if tree_magic_mini returned None
                        infer::get_from_path(path)
                            .ok()
                            .flatten()
                            .map(|k| k.mime_type().to_string())
                            .unwrap_or_else(|| {
                                mime_guess::from_path(path)
                                    .first_or_octet_stream()
                                    .to_string()
                            })
                    },
                );
                // Use std::fs::Metadata for timestamps
                let created = metadata.created().ok().map(|t| {
                    let timestamp = t
                        .duration_since(std::time::UNIX_EPOCH)
                        .unwrap_or_default()
                        .as_secs() as i64;

                    DateTime::from_timestamp(timestamp, 0)
                        .unwrap_or_default()
                        .naive_utc()
                });

                let modified = metadata.modified().ok().map(|t| {
                    let timestamp = t
                        .duration_since(std::time::UNIX_EPOCH)
                        .unwrap_or_default()
                        .as_secs() as i64;

                    DateTime::from_timestamp(timestamp, 0)
                        .unwrap_or_default()
                        .naive_utc()
                });

                if let Ok(file_bytes) = fs::read(path) {
                    let file_model = entity::clipboard_file::ActiveModel {
                        name: Set(Some(file_name)),
                        extension: Set(file_ext),
                        size: Set(Some(metadata.len() as i32)),
                        mime_type: Set(mime_type),
                        created_date: Set(created),
                        modified_date: Set(modified),
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
