use sea_orm_migration::{prelude::*, schema::float};

#[derive(Iden)]
enum Settings {
    Table,
    GlassOpacity,
    GlassGrain,
}

// Defaults/bounds mirror the CSS frost engine: opacity is the surface "milk"
// density (how see-through the frost is), grain is the acrylic noise strength.
// Both are normalized 0..1 so the UI sliders map straight onto them.
const GLASS_OPACITY: f32 = 0.55;
const GLASS_GRAIN: f32 = 0.5;
const GLASS_MIN: f32 = 0.0;
const GLASS_MAX: f32 = 1.0;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // SQLite supports only one column per ALTER TABLE, so add each separately.
        manager
            .alter_table(
                Table::alter()
                    .table(Settings::Table)
                    .add_column(
                        float(Settings::GlassOpacity)
                            .default(Value::Float(Some(GLASS_OPACITY)))
                            .check(
                                Expr::col(Settings::GlassOpacity)
                                    .gte(Value::Float(Some(GLASS_MIN)))
                                    .and(
                                        Expr::col(Settings::GlassOpacity)
                                            .lte(Value::Float(Some(GLASS_MAX))),
                                    ),
                            ),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .alter_table(
                Table::alter()
                    .table(Settings::Table)
                    .add_column(
                        float(Settings::GlassGrain)
                            .default(Value::Float(Some(GLASS_GRAIN)))
                            .check(
                                Expr::col(Settings::GlassGrain)
                                    .gte(Value::Float(Some(GLASS_MIN)))
                                    .and(
                                        Expr::col(Settings::GlassGrain)
                                            .lte(Value::Float(Some(GLASS_MAX))),
                                    ),
                            ),
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
                    .drop_column(Settings::GlassOpacity)
                    .to_owned(),
            )
            .await?;

        manager
            .alter_table(
                Table::alter()
                    .table(Settings::Table)
                    .drop_column(Settings::GlassGrain)
                    .to_owned(),
            )
            .await
    }
}
