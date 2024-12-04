use crate::m000001_create_clipboard::Clipboard;
use sea_orm_migration::prelude::*;

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
                    .col(
                        ColumnDef::new(ClipboardFile::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(ClipboardFile::ClipboardId)
                            .integer()
                            .not_null(),
                    )
                    .col(ColumnDef::new(ClipboardFile::Data).blob().not_null())
                    .col(ColumnDef::new(ClipboardFile::Name).string())
                    .col(ColumnDef::new(ClipboardFile::Extension).string())
                    .col(ColumnDef::new(ClipboardFile::Size).integer())
                    .col(ColumnDef::new(ClipboardFile::CreatedDate).date_time())
                    .col(ColumnDef::new(ClipboardFile::UpdatedDate).date_time())
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
