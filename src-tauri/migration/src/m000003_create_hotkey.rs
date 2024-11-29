use sea_orm_migration::prelude::*;

#[derive(Iden)]
enum Hotkey {
    Table,
    Id,
    Event,
    Ctrl,
    Alt,
    Shift,
    Key,
    Status,
    Name,
    Icon,
}

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Hotkey::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Hotkey::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Hotkey::Event).string().not_null())
                    .col(ColumnDef::new(Hotkey::Ctrl).boolean().not_null())
                    .col(ColumnDef::new(Hotkey::Alt).boolean().not_null())
                    .col(ColumnDef::new(Hotkey::Shift).boolean().not_null())
                    .col(ColumnDef::new(Hotkey::Key).string().not_null())
                    .col(ColumnDef::new(Hotkey::Status).boolean().not_null())
                    .col(ColumnDef::new(Hotkey::Name).string().not_null())
                    .col(ColumnDef::new(Hotkey::Icon).string().not_null())
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Hotkey::Table).to_owned())
            .await
    }
}
