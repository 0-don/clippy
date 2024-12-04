use sea_orm_migration::prelude::*;

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
                    .col(
                        ColumnDef::new(Settings::Id)
                            .big_integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Settings::Startup).boolean().not_null())
                    .col(ColumnDef::new(Settings::Notification).boolean().not_null())
                    .col(ColumnDef::new(Settings::Synchronize).boolean().not_null())
                    .col(ColumnDef::new(Settings::DarkMode).boolean().not_null())
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
