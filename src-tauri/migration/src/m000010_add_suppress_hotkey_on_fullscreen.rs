use sea_orm_migration::{prelude::*, schema::boolean};

#[derive(Iden)]
enum Settings {
    Table,
    SuppressHotkeyOnFullscreen,
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
                    .add_column(boolean(Settings::SuppressHotkeyOnFullscreen).default(false))
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                Table::alter()
                    .table(Settings::Table)
                    .drop_column(Settings::SuppressHotkeyOnFullscreen)
                    .to_owned(),
            )
            .await
    }
}
