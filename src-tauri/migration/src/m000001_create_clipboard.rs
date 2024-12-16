use sea_orm_migration::{
    prelude::*,
    schema::{boolean, date_time, json, pk_auto},
};

#[derive(Iden)]
pub enum Clipboard {
    Table,
    Id,
    Types,
    Star,
    CreatedDate,
}

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Clipboard::Table)
                    .if_not_exists()
                    .col(pk_auto(Clipboard::Id))
                    .col(json(Clipboard::Types).default(Expr::value("[]")))
                    .col(boolean(Clipboard::Star).default(false))
                    .col(date_time(Clipboard::CreatedDate).default(Expr::current_timestamp()))
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Clipboard::Table).to_owned())
            .await
    }
}
