use sea_orm_migration::{prelude::*, schema::string_null};

#[derive(Iden)]
enum Clipboard {
    Table,
    Name,
}

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                Table::alter()
                    .table(Clipboard::Table)
                    .add_column(string_null(Clipboard::Name))
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                Table::alter()
                    .table(Clipboard::Table)
                    .drop_column(Clipboard::Name)
                    .to_owned(),
            )
            .await
    }
}
