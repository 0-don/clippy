use sea_orm_migration::{
    prelude::*,
    sea_orm::{EnumIter, Iterable},
};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts

        manager
            .create_table(
                Table::create()
                    .table(Clipboard::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Clipboard::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(Clipboard::Type)
                            .enumeration(Type::Table, [Type::Image, Type::Text, Type::Color])
                            .enumeration(Type::Table, Type::iter().skip(1))
                            .not_null(),
                    )
                    .col(ColumnDef::new(Clipboard::Content).string())
                    .col(ColumnDef::new(Clipboard::Width).integer())
                    .col(ColumnDef::new(Clipboard::Height).integer())
                    .col(ColumnDef::new(Clipboard::Size).string())
                    .col(ColumnDef::new(Clipboard::Blob).blob(BlobSize::Long))
                    .col(ColumnDef::new(Clipboard::Star).boolean().default(false))
                    .col(
                        ColumnDef::new(Clipboard::CreatedDate)
                            .date_time()
                            .default("test"),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts

        manager
            .drop_table(Table::drop().table(Clipboard::Table).to_owned())
            .await
    }
}

/// Learn more at https://docs.rs/sea-query#iden
#[derive(Iden)]
enum Clipboard {
    Table,
    Id,
    Type,
    Content,
    Width,
    Height,
    Size,
    Blob,
    Star,
    CreatedDate,
}

#[derive(Iden, EnumIter)]
pub enum Type {
    Table,
    #[iden = "Text"]
    Text,
    #[iden = "Image"]
    Image,
    #[iden = "Color"]
    Color,
}

#[derive(Iden)]
enum Settings {
    Table,
    Id,
    Startup,
    Notification,
    Synchronize,
    SynctTime,
    DarkMode,
}

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
