use crate::m000001_create_clipboard::Clipboard;
use sea_orm_migration::{
    prelude::*,
    schema::{blob, date_time_null, integer, integer_null, pk_auto, string_null},
};

#[derive(Iden)]
pub enum ClipboardFile {
    Table,
    Id,
    ClipboardId,
    Extension,
    Name,
    Data,
    Size,
    CreatedDate,
    UpdatedDate,
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
                    .col(pk_auto(ClipboardFile::Id))
                    .col(integer(ClipboardFile::ClipboardId))
                    .col(blob(ClipboardFile::Data))
                    .col(string_null(ClipboardFile::Name))
                    .col(string_null(ClipboardFile::Extension))
                    .col(integer_null(ClipboardFile::Size))
                    .col(date_time_null(ClipboardFile::CreatedDate))
                    .col(date_time_null(ClipboardFile::UpdatedDate))
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
