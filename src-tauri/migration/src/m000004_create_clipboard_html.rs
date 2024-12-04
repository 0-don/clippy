use crate::m000001_create_clipboard::Clipboard;
use sea_orm_migration::{
    prelude::*,
    schema::{integer, pk_auto, text},
};

#[derive(Iden)]
pub enum ClipboardHtml {
    Table,
    Id,
    ClipboardId,
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
                    .table(ClipboardHtml::Table)
                    .if_not_exists()
                    .col(pk_auto(ClipboardHtml::Id))
                    .col(integer(ClipboardHtml::ClipboardId))
                    .col(text(ClipboardHtml::Data))
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-clipboard-html")
                            .from(ClipboardHtml::Table, ClipboardHtml::ClipboardId)
                            .to(Clipboard::Table, Clipboard::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(ClipboardHtml::Table).to_owned())
            .await
    }
}
