use crate::m000001_create_clipboard::Clipboard;
use sea_orm_migration::{
    prelude::*,
    schema::{integer, pk_auto, text},
};

#[derive(Iden)]
pub enum ClipboardRtf {
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
                    .table(ClipboardRtf::Table)
                    .if_not_exists()
                    .col(pk_auto(ClipboardRtf::Id))
                    .col(integer(ClipboardRtf::ClipboardId).unique_key())
                    .col(text(ClipboardRtf::Data))
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-clipboard-rtf")
                            .from(ClipboardRtf::Table, ClipboardRtf::ClipboardId)
                            .to(Clipboard::Table, Clipboard::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(ClipboardRtf::Table).to_owned())
            .await
    }
}
