use crate::m000001_create_clipboard::Clipboard;
use sea_orm_migration::{
    prelude::*,
    schema::{blob, date_time, integer, string, string_null, uuid},
};

#[derive(Iden)]
pub enum ClipboardFile {
    Table,
    Id,
    ClipboardId,
    Name,
    Extension,
    Size,
    MimeType,
    CreatedDate,
    ModifiedDate,
    Data,
}

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(ClipboardFile::Table)
                    .if_not_exists()
                    .col(uuid(ClipboardFile::Id).not_null().primary_key())
                    .col(uuid(ClipboardFile::ClipboardId))
                    .col(string(ClipboardFile::Name))
                    .col(integer(ClipboardFile::Size).default(0))
                    .col(string_null(ClipboardFile::Extension))
                    .col(string_null(ClipboardFile::MimeType))
                    .col(date_time(ClipboardFile::CreatedDate))
                    .col(date_time(ClipboardFile::ModifiedDate))
                    .col(blob(ClipboardFile::Data))
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-clipboard-file")
                            .from(ClipboardFile::Table, ClipboardFile::ClipboardId)
                            .to(Clipboard::Table, Clipboard::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(ClipboardFile::Table).to_owned())
            .await
    }
}
