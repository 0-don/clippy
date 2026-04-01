use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[derive(Iden)]
enum Settings {
    Table,
    #[iden = "enryption_save_before_unlock"]
    EnryptionSaveBeforeUnlock,
    #[iden = "encryption_save_before_unlock"]
    EncryptionSaveBeforeUnlock,
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                Table::alter()
                    .table(Settings::Table)
                    .rename_column(
                        Settings::EnryptionSaveBeforeUnlock,
                        Settings::EncryptionSaveBeforeUnlock,
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                Table::alter()
                    .table(Settings::Table)
                    .rename_column(
                        Settings::EncryptionSaveBeforeUnlock,
                        Settings::EnryptionSaveBeforeUnlock,
                    )
                    .to_owned(),
            )
            .await
    }
}
