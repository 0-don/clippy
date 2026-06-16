use sea_orm_migration::{
    prelude::*,
    schema::{boolean, string},
};

#[derive(Iden)]
enum Settings {
    Table,
    Theme,
    Glass,
}

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // SQLite supports only one column per ALTER TABLE, so add each separately.
        manager
            .alter_table(
                Table::alter()
                    .table(Settings::Table)
                    .add_column(string(Settings::Theme).default("neutral"))
                    .to_owned(),
            )
            .await?;

        manager
            .alter_table(
                Table::alter()
                    .table(Settings::Table)
                    .add_column(boolean(Settings::Glass).default(false))
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                Table::alter()
                    .table(Settings::Table)
                    .drop_column(Settings::Theme)
                    .to_owned(),
            )
            .await?;

        manager
            .alter_table(
                Table::alter()
                    .table(Settings::Table)
                    .drop_column(Settings::Glass)
                    .to_owned(),
            )
            .await
    }
}
