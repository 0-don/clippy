use entity::{
    clipboard::{self},
    clipboard_file, clipboard_html, clipboard_image, clipboard_rtf, clipboard_text,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FullClipboardDto {
    pub clipboard: clipboard::Model,
    pub text: Option<clipboard_text::Model>,
    pub html: Option<clipboard_html::Model>,
    pub image: Option<clipboard_image::Model>,
    pub rtf: Option<clipboard_rtf::Model>,
    pub files: Vec<clipboard_file::Model>,
}

#[derive(Debug, Clone)]
pub struct FullClipboardDbo {
    pub clipboard_model: entity::clipboard::ActiveModel,
    pub clipboard_text_model: entity::clipboard_text::ActiveModel,
    pub clipboard_html_model: entity::clipboard_html::ActiveModel,
    pub clipboard_image_model: entity::clipboard_image::ActiveModel,
    pub clipboard_rtf_model: entity::clipboard_rtf::ActiveModel,
    pub clipboard_files_model: Vec<entity::clipboard_file::ActiveModel>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClipboardsResponse {
    pub clipboards: Vec<FullClipboardDto>,
    pub total: u64,
    pub has_more: bool,
}

