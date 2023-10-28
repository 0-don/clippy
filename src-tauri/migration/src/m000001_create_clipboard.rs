use sea_orm_migration::prelude::*;

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
    Base64,
    Star,
    CreatedDate,
}

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
                    .col(ColumnDef::new(Clipboard::Type).string().not_null())
                    .col(ColumnDef::new(Clipboard::Content).string())
                    .col(ColumnDef::new(Clipboard::Width).integer())
                    .col(ColumnDef::new(Clipboard::Height).integer())
                    .col(ColumnDef::new(Clipboard::Size).string())
                    .col(ColumnDef::new(Clipboard::Blob).blob(BlobSize::Long))
                    .col(ColumnDef::new(Clipboard::Base64).text())
                    .col(ColumnDef::new(Clipboard::Star).boolean().default(true))
                    .col(
                        ColumnDef::new(Clipboard::CreatedDate)
                            .date_time()
                            .extra("DEFAULT CURRENT_TIMESTAMP".to_owned()), // .default("CURRENT_TIMESTAMP".to_owned()),
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
