use sea_orm_migration::{prelude::*, schema::json};

#[derive(DeriveIden)]
enum Settings {
    Table,
    Commands,
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
                    .add_column(json(Settings::Commands).default(Expr::value("[]")))
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                Table::alter()
                    .table(Settings::Table)
                    .drop_column(Settings::Commands)
                    .to_owned(),
            )
            .await
    }
}
