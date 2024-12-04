use entity::{
    clipboard::{self},
    clipboard_file, clipboard_html, clipboard_image, clipboard_rtf, clipboard_text,
};
use sea_orm::EntityName;
use sea_orm::{DbErr, FromQueryResult, QueryResult};
use serde::{Deserialize, Serialize};

impl FromQueryResult for ClipboardWithRelations {
    fn from_query_result(res: &QueryResult, _pre: &str) -> Result<Self, DbErr> {
        Ok(Self {
            clipboard: clipboard::Model::from_query_result(res, clipboard::Entity.table_name())?,
            text: clipboard_text::Model::from_query_result(
                res,
                clipboard_text::Entity.table_name(),
            )
            .ok(),
            html: clipboard_html::Model::from_query_result(
                res,
                clipboard_html::Entity.table_name(),
            )
            .ok(),
            image: clipboard_image::Model::from_query_result(
                res,
                clipboard_image::Entity.table_name(),
            )
            .ok(),
            rtf: clipboard_rtf::Model::from_query_result(res, clipboard_rtf::Entity.table_name())
                .ok(),
            file: clipboard_file::Model::from_query_result(
                res,
                clipboard_file::Entity.table_name(),
            )
            .ok(),
        })
    }
}

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
