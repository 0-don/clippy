use crate::prelude::*;
use crate::{service::settings::get_data_path, types::types::Config};
use migration::{DbErr, Migrator, MigratorTrait};
use std::sync::Once;

#[allow(dead_code)]
static INIT: Once = Once::new();

pub async fn establish_connection() -> Result<DbConn, DbErr> {
    let database_url = if cfg!(debug_assertions) {
        String::from("sqlite://../clippy.sqlite?mode=rwc")
    } else {
        get_prod_database_url()
    };

    let db = Database::connect(&database_url)
        .await
        .expect("Failed to setup the database");

    INIT.call_once(|| {
        println!("Running migrations...");
        let conn_for_migration = db.clone();
        tokio::task::block_in_place(|| {
            tokio::runtime::Handle::current().block_on(async move {
                Migrator::up(&conn_for_migration, None)
                    .await
                    .expect("Failed to run migrations");
            })
        });
    });

    Ok(db)
}

fn get_prod_database_url() -> String {
    let data_path = get_data_path();

    let json =
        std::fs::read_to_string(data_path.config_file_path).expect("Failed to read config file");

    let config: Config = serde_json::from_str(&json).expect("Failed to parse config file");

    let db = format!("sqlite://{}?mode=rwc", config.db);

    db
}
