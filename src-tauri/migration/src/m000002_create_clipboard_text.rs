use crate::{m000001_create_clipboard::Clipboard, ClipboardTextType};
use sea_orm::Iterable;
use sea_orm_migration::prelude::*;

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
                    .col(
                        ColumnDef::new(ClipboardText::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(ClipboardText::ClipboardId)
                            .integer()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(ClipboardText::Type)
                            .string()
                            .not_null()
                            .default(ClipboardTextType::iter().next().unwrap().to_string())
                            .check(
                                Expr::col(ClipboardText::Type).is_in(
                                    ClipboardTextType::iter()
                                        .map(|x| x.to_string())
                                        .collect::<Vec<String>>(),
                                ),
                            ),
                    )
                    .col(ColumnDef::new(ClipboardText::Data).text().not_null())
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
