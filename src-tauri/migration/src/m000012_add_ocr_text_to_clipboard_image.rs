use sea_orm_migration::{prelude::*, schema::text_null};

#[derive(Iden)]
enum ClipboardImage {
    Table,
    OcrText,
}

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                Table::alter()
                    .table(ClipboardImage::Table)
                    .add_column(text_null(ClipboardImage::OcrText))
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                Table::alter()
                    .table(ClipboardImage::Table)
                    .drop_column(ClipboardImage::OcrText)
                    .to_owned(),
            )
            .await
    }
}
