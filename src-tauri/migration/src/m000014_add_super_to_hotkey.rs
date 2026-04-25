use sea_orm_migration::{prelude::*, schema::boolean};

#[derive(Iden)]
enum Hotkey {
    Table,
    SuperKey,
}

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                Table::alter()
                    .table(Hotkey::Table)
                    .add_column(boolean(Hotkey::SuperKey).default(false))
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                Table::alter()
                    .table(Hotkey::Table)
                    .drop_column(Hotkey::SuperKey)
                    .to_owned(),
            )
            .await
    }
}
