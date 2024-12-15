use common::language::get_system_language;
use sea_orm_migration::{
    prelude::*,
    schema::{boolean, integer, pk_auto, string},
};

#[derive(Iden)]
enum Settings {
    Table,
    Id,
    Language,
    //
    Startup,
    Notification,
    Synchronize,
    DarkMode,
    //
    MaxFileSize,
    MaxImageSize,
    MaxTextSize,
    MaxRtfSize,
    MaxHtmlSize,
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
                    .col(
                        string(Settings::Language)
                            .string_len(2)
                            .default(get_system_language().to_string()),
                    )
                    .col(boolean(Settings::Startup).default(true))
                    .col(boolean(Settings::Notification).default(false))
                    .col(boolean(Settings::Synchronize).default(false))
                    .col(boolean(Settings::DarkMode).default(true))
                    // 10MB in bytes (10 * 1024 * 1024)
                    .col(integer(Settings::MaxFileSize).default(10_485_760))
                    .col(integer(Settings::MaxImageSize).default(10_485_760))
                    .col(integer(Settings::MaxTextSize).default(10_485_760))
                    .col(integer(Settings::MaxRtfSize).default(10_485_760))
                    .col(integer(Settings::MaxHtmlSize).default(10_485_760))
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
