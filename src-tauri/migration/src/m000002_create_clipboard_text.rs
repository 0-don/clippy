use crate::m000001_create_clipboard::Clipboard;
use common::types::enums::ClipboardTextType;
use sea_orm::Iterable;
use sea_orm_migration::{
    prelude::*,
    schema::{string, text, uuid},
};

#[derive(Iden)]
pub enum ClipboardText {
    Table,
    Id,
    ClipboardId,
    Type,
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
                    .table(ClipboardText::Table)
                    .if_not_exists()
                    .col(
                        uuid(ClipboardText::Id)
                            .not_null()
                            .primary_key()
                    )
                    .col(uuid(ClipboardText::ClipboardId).unique_key())
                    .col(
                        string(ClipboardText::Type)
                            .default(
                                ClipboardTextType::iter()
                                    .next()
                                    .expect("no default value")
                                    .to_string(),
                            )
                            .check(
                                Expr::col(ClipboardText::Type).is_in(
                                    ClipboardTextType::iter()
                                        .map(|x| x.to_string())
                                        .collect::<Vec<String>>(),
                                ),
                            ),
                    )
                    .col(text(ClipboardText::Data))
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-clipboard-text")
                            .from(ClipboardText::Table, ClipboardText::ClipboardId)
                            .to(Clipboard::Table, Clipboard::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(ClipboardText::Table).to_owned())
            .await
    }
}
