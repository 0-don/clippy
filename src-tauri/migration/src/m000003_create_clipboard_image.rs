use crate::m000001_create_clipboard::Clipboard;
use sea_orm_migration::{
    prelude::*,
    schema::{blob, integer, integer_null, pk_auto, string_null, text_null},
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
                    .col(pk_auto(ClipboardImage::Id))
                    .col(integer(ClipboardImage::ClipboardId).unique_key())
                    .col(blob(ClipboardImage::Data))
                    .col(string_null(ClipboardImage::Extension))
                    .col(integer_null(ClipboardImage::Width))
                    .col(integer_null(ClipboardImage::Height))
                    .col(string_null(ClipboardImage::Size))
                    .col(text_null(ClipboardImage::Thumbnail))
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
