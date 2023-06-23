use std::path::PathBuf;

use migration::{DbErr, Migrator, MigratorTrait};
use sea_orm::{Database, DbConn};

use crate::{service::window::get_config_path, types::types::Config};

pub async fn establish_connection() -> Result<DbConn, DbErr> {
    // let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set.");
    let database_url = if cfg!(debug_assertions) {
        String::from("sqlite://../clippy.sqlite?mode=rwc")
    } else {
        get_prod_database_url()
    };

    let db = Database::connect(&database_url)
        .await
        .expect("Failed to setup the database");
    Migrator::up(&db, None)
        .await
        .expect("Failed to run migrations for tests");

    Ok(db)
}

fn get_prod_database_url() -> String {
    let config_dir = get_config_path();

    let config_file: PathBuf = [&config_dir, "config.json"].iter().collect();

    let config = std::fs::read_to_string(config_file).unwrap();

    let config: Config = serde_json::from_str(&config).unwrap();

    config.db
}
