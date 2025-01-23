use crate::m000001_create_clipboard::Clipboard;
use sea_orm_migration::{
    prelude::*,
    schema::{blob, integer_null, string_null, text, uuid},
};

#[derive(Iden)]
pub enum ClipboardImage {
    Table,
    Id,
    ClipboardId,
    Extension,
    Data,
    Width,
    Height,
    Size,
    Thumbnail,
}

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(ClipboardImage::Table)
                    .if_not_exists()
                    .col(uuid(ClipboardImage::Id).not_null().primary_key())
                    .col(uuid(ClipboardImage::ClipboardId).unique_key())
                    .col(blob(ClipboardImage::Data))
                    .col(text(ClipboardImage::Thumbnail))
                    .col(string_null(ClipboardImage::Extension))
                    .col(integer_null(ClipboardImage::Width))
                    .col(integer_null(ClipboardImage::Height))
                    .col(string_null(ClipboardImage::Size))
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-clipboard-image")
                            .from(ClipboardImage::Table, ClipboardImage::ClipboardId)
                            .to(Clipboard::Table, Clipboard::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(ClipboardImage::Table).to_owned())
            .await
    }
}
