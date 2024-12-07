use entity::{
    clipboard::{self},
    clipboard_file, clipboard_html, clipboard_image, clipboard_rtf, clipboard_text,
};
use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClipboardWithRelations {
    pub clipboard: clipboard::Model,
    pub text: Option<clipboard_text::Model>,
    pub html: Option<clipboard_html::Model>,
    pub image: Option<clipboard_image::Model>,
    pub rtf: Option<clipboard_rtf::Model>,
    pub file: Option<clipboard_file::Model>,
}

#[derive(Debug, Clone)]
pub struct ClipboardManager {
    pub clipboard_model: entity::clipboard::ActiveModel,
    pub clipboard_text_model: entity::clipboard_text::ActiveModel,
    pub clipboard_html_model: entity::clipboard_html::ActiveModel,
    pub clipboard_image_model: entity::clipboard_image::ActiveModel,
    pub clipboard_rtf_model: entity::clipboard_rtf::ActiveModel,
    pub clipboard_file_model: entity::clipboard_file::ActiveModel,
}
