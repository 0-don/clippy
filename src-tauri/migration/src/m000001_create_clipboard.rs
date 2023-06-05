use sea_orm::Schema;
use sea_orm_migration::prelude::*;

pub mod clipboard {
    use sea_orm::entity::prelude::*;
    use sea_orm_migration::sea_orm;

    #[derive(Debug, Clone, PartialEq, Eq, EnumIter, DeriveActiveEnum)]
    #[sea_orm(rs_type = "String", db_type = "String(Some(16))")]
    pub enum ClipboardType {
        #[sea_orm(string_value = "text")]
        Text,
        #[sea_orm(string_value = "image")]
        Image,
        #[sea_orm(string_value = "color")]
        Color,
    }

    #[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
    #[sea_orm(table_name = "clipboard")]
    pub struct Model {
        #[sea_orm(primary_key, auto_increment = true)]
        pub id: i32,
        #[sea_orm(default_value = "text")]
        pub r#type: ClipboardType,
        pub content: Option<String>,
        pub width: Option<i32>,
        pub height: Option<i32>,
        pub size: Option<String>,
        pub blob: Option<Vec<u8>>,
        pub star: bool,
        pub created_date: DateTimeUtc,
    }

    #[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
    pub enum Relation {}

    impl ActiveModelBehavior for ActiveModel {}
}

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let builder = manager.get_database_backend();
        let schema = Schema::new(builder);
        manager
            .create_table(schema.create_table_from_entity(clipboard::Entity))
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts

        manager
            .drop_table(Table::drop().table(clipboard::Entity).to_owned())
            .await
    }
}

// #[derive(Iden)]
// enum Settings {
//     Table,
//     Id,
//     Startup,
//     Notification,
//     Synchronize,
//     SynctTime,
//     DarkMode,
// }

// #[derive(Iden)]
// enum Hotkey {
//     Table,
//     Id,
//     Event,
//     Ctrl,
//     Alt,
//     Shift,
//     Key,
//     Status,
//     Name,
//     Icon,
// }
