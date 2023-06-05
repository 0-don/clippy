use sea_orm::Schema;
use sea_orm_migration::prelude::*;

pub mod hotkey {
    use sea_orm::entity::prelude::*;
    use sea_orm_migration::sea_orm;

    #[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
    #[sea_orm(table_name = "hotkey")]
    pub struct Model {
        #[sea_orm(primary_key, auto_increment = true)]
        pub id: i32,
        pub event: String,
        pub ctrl: bool,
        pub alt: bool,
        pub shift: bool,
        pub key: String,
        pub status: bool,
        pub name: String,
        pub icon: String,
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
            .create_table(schema.create_table_from_entity(hotkey::Entity))
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts

        manager
            .drop_table(Table::drop().table(hotkey::Entity).to_owned())
            .await
    }
}
