use crate::prelude::*;
use crate::service::clipboard::{new_clipboard_event, upsert_clipboard_dto};
use crate::service::encrypt::{encrypt_clipboard, is_key_set};
use crate::service::settings::get_global_settings;
use crate::service::{
    clipboard::{get_last_clipboard_db, insert_clipboard_dbo},
    window::calculate_thumbnail_dimensions,
};
use crate::tao::global::get_app;
use base64::{engine::general_purpose::STANDARD, Engine};
use chrono::DateTime;
use common::types::enums::{ClipboardTextType, ClipboardType};
use common::types::orm_query::FullClipboardDbo;
use image::imageops;
use regex::Regex;
use sea_orm::prelude::Uuid;
use std::fs;
use std::io::Cursor;
use std::path::Path;
use tauri::Manager;
use tauri_plugin_clipboard::Clipboard;
use urlencoding::decode;

pub trait ClipboardManagerExt {
    fn new() -> FullClipboardDbo;
    fn upsert_clipboard() -> impl std::future::Future<Output = ()> + Send;
    fn check_if_last_is_same(&mut self) -> impl std::future::Future<Output = bool> + Send;
    fn parse_model(
        &mut self,
        text: Option<String>,
        html: Option<String>,
        rtf: Option<String>,
        image_data: Option<Vec<u8>>,
        files: Option<Vec<String>>,
    ) -> ();
    fn parse_image_model(&mut self, img_bytes: Vec<u8>);
    fn parse_file_models(&mut self, file_paths: Vec<String>) -> std::io::Result<()>;
}

impl ClipboardManagerExt for FullClipboardDbo {
    fn new() -> Self {
        FullClipboardDbo {
            clipboard_model: entity::clipboard::ActiveModel::default(),
            clipboard_text_model: entity::clipboard_text::ActiveModel::default(),
            clipboard_html_model: entity::clipboard_html::ActiveModel::default(),
            clipboard_image_model: entity::clipboard_image::ActiveModel::default(),
            clipboard_rtf_model: entity::clipboard_rtf::ActiveModel::default(),
            clipboard_files_model: Vec::new(),
        }
    }

    async fn upsert_clipboard() {
        let settings = get_global_settings();

        // If clipboards are encypted but not saved before unlocking, return
        if settings.encryption && !settings.enryption_save_before_unlock && !is_key_set() {
            return;
        }

        let clipboard = get_app().state::<Clipboard>();
        let mut manager = Self::new();

        manager.parse_model(
            clipboard.read_text().ok(),
            clipboard.read_html().ok(),
            clipboard.read_rtf().ok(),
            clipboard.read_image_binary().ok(),
            clipboard.read_files().ok(),
        );

        // Add check for empty types
        if let sea_orm::ActiveValue::Set(types_json) = &manager.clipboard_model.types {
            if let Some(types) = ClipboardType::from_json_value(types_json) {
                if types.is_empty() {
                    return;
                }
            }
        }

        if !manager.check_if_last_is_same().await && manager.clipboard_model.types.is_set() {
            // insert default not encrypted clipboard
            let clipboard = insert_clipboard_dbo(manager)
                .await
                .expect("Failed to insert");

            // If encryption is enabled and key is set, encrypt clipboard before upsert
            if settings.encryption && is_key_set() {
                upsert_clipboard_dto(encrypt_clipboard(clipboard.clone()))
                    .await
                    .expect("Failed to upsert");
            }

            new_clipboard_event(clipboard);
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
                            ClipboardType::Rtf => {
                                if let (Some(last), sea_orm::ActiveValue::Set(curr)) =
                                    (&last.rtf, &self.clipboard_rtf_model.data)
                                {
                                    if curr != &last.data {
                                        return false;
                                    }
                                }
                            }
                            ClipboardType::Image => {
                                if let (Some(last_img), sea_orm::ActiveValue::Set(curr_data)) =
                                    (&last.image, &self.clipboard_image_model.data)
                                {
                                    // Compare the data field from last_img with curr_data
                                    if curr_data != &last_img.data {
                                        return false;
                                    }
                                } else {
                                    // If either is None/NotSet, they're different
                                    return false;
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
                                    } else {
                                        // If either is None/NotSet, they're different
                                        return false;
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

    fn parse_model(
        &mut self,
        text: Option<String>,
        html: Option<String>,
        rtf: Option<String>,
        image: Option<Vec<u8>>,
        files: Option<Vec<String>>,
    ) {
        let settings = get_global_settings();
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
                .filter_map(|p| {
                    decode(&p)
                        .map(|path| path.into_owned())
                        .ok()
                        .and_then(|decoded_path| {
                            fs::metadata(&decoded_path).ok().and_then(|m| {
                                if m.len() as i32 <= settings.max_file_size {
                                    Some(decoded_path)
                                } else {
                                    None
                                }
                            })
                        })
                })
                .collect();

            if !valid_files.is_empty() {
                types.clear(); // Clear all types if image is present
                types.push(ClipboardType::File);
                let _ = self.parse_file_models(valid_files);
            }
        }

        printlog!("clipboard types: {:?}", types);

        self.clipboard_model = entity::clipboard::ActiveModel {
            id: Set(Uuid::new_v4()),
            types: Set(ClipboardType::to_json_value(&types)),
            ..Default::default()
        };
    }

    fn parse_image_model(&mut self, img_bytes: Vec<u8>) {
        if let Ok(image_buffer) = image::load_from_memory(&img_bytes) {
            let image_buffer = image_buffer.to_rgba8();
            let (width, height) = (image_buffer.width(), image_buffer.height());

            let extension = image::guess_format(&img_bytes)
                .ok()
                .map(|format| format.extensions_str()[0].to_string())
                .or_else(|| infer::get(&img_bytes).map(|kind| kind.extension().to_string()))
                .unwrap_or_else(|| "png".to_string());

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
                    size: Set(img_bytes.len() as i32),
                    data: Set(img_bytes),
                    thumbnail: Set(base64_thumbnail),
                    extension: Set(extension),
                    width: Set(width as i32),
                    height: Set(height as i32),
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

                let mime_type = tree_magic_mini::from_filepath(path)
                    .map(|ct| ct.to_string())
                    .or_else(|| {
                        infer::get_from_path(path)
                            .ok()
                            .flatten()
                            .map(|k| k.mime_type().to_string())
                            .or_else(|| {
                                Some(
                                    mime_guess::from_path(path)
                                        .first_or_octet_stream()
                                        .to_string(),
                                )
                            })
                    });

                let created = metadata
                    .created()
                    .ok()
                    .and_then(|t| {
                        DateTime::from_timestamp(
                            t.duration_since(std::time::UNIX_EPOCH)
                                .unwrap_or_default()
                                .as_secs() as i64,
                            0,
                        )
                    })
                    .map(|dt| dt.naive_utc());

                let modified = metadata
                    .modified()
                    .ok()
                    .and_then(|t| {
                        DateTime::from_timestamp(
                            t.duration_since(std::time::UNIX_EPOCH)
                                .unwrap_or_default()
                                .as_secs() as i64,
                            0,
                        )
                    })
                    .map(|dt| dt.naive_utc());

                if let Ok(file_bytes) = fs::read(path) {
                    let file_model = entity::clipboard_file::ActiveModel {
                        name: Set(file_name),
                        extension: Set(file_ext),
                        size: Set(metadata.len() as i32),
                        mime_type: Set(mime_type),
                        created_date: Set(
                            created.unwrap_or_else(|| chrono::Local::now().naive_utc())
                        ),
                        modified_date: Set(
                            modified.unwrap_or_else(|| chrono::Local::now().naive_utc())
                        ),
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
