use sea_orm_migration::{
    prelude::*,
    schema::{boolean, pk_auto},
};

#[derive(Iden)]
enum Settings {
    Table,
    Id,
    Startup,
    Notification,
    Synchronize,
    DarkMode,
}

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Settings::Table)
                    .if_not_exists()
                    .col(pk_auto(Settings::Id))
                    .col(boolean(Settings::Startup))
                    .col(boolean(Settings::Notification))
                    .col(boolean(Settings::Synchronize))
                    .col(boolean(Settings::DarkMode))
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Settings::Table).to_owned())
            .await
    }
}
