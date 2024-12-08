use crate::HotkeyEvent;
use sea_orm::Iterable;
use sea_orm_migration::{
    prelude::*,
    schema::{boolean, pk_auto, string},
};

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
                    .col(pk_auto(Hotkey::Id))
                    .col(
                        string(Hotkey::Event).check(
                            Expr::col(Hotkey::Event).is_in(
                                HotkeyEvent::iter()
                                    .map(|x| x.to_string())
                                    .collect::<Vec<String>>(),
                            ),
                        ),
                    )
                    .col(boolean(Hotkey::Ctrl))
                    .col(boolean(Hotkey::Alt))
                    .col(boolean(Hotkey::Shift))
                    .col(string(Hotkey::Key))
                    .col(boolean(Hotkey::Status))
                    .col(string(Hotkey::Name))
                    .col(string(Hotkey::Icon))
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
