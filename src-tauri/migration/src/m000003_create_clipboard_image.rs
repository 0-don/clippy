use crate::m000001_create_clipboard::Clipboard;
use sea_orm_migration::prelude::*;

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
                    .col(
                        ColumnDef::new(ClipboardImage::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(ClipboardImage::ClipboardId)
                            .integer()
                            .not_null(),
                    )
                    .col(ColumnDef::new(ClipboardImage::Data).blob().not_null())
                    .col(ColumnDef::new(ClipboardImage::Extension).string())
                    .col(ColumnDef::new(ClipboardImage::Width).integer())
                    .col(ColumnDef::new(ClipboardImage::Height).integer())
                    .col(ColumnDef::new(ClipboardImage::Size).string())
                    .col(ColumnDef::new(ClipboardImage::Thumbnail).text())
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
