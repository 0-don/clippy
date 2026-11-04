use sea_orm_migration::{prelude::*, schema::boolean};

#[derive(DeriveIden)]
enum Settings {
    Table,
    Ocr,
}

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                Table::alter()
                    .table(Settings::Table)
                    .add_column(boolean(Settings::Ocr).default(true))
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                Table::alter()
                    .table(Settings::Table)
                    .drop_column(Settings::Ocr)
                    .to_owned(),
            )
            .await
    }
}
