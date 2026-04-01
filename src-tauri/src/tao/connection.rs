use super::config::{get_config, get_data_path};
use super::tao_constants::DB;
use common::{constants::DB_NAME, types::types::Config};
use migration::{DbErr, Migrator, MigratorTrait};
use sea_orm::{Database, DbConn};

pub async fn init_db() -> Result<(), DbErr> {
    let database_url = if cfg!(debug_assertions) {
        get_debug_database_url()
    } else {
        get_prod_database_url()
    };

    let conn = Database::connect(&database_url).await?;
    Migrator::up(&conn, None).await?;
    DB.set(conn)
        .unwrap_or_else(|_| panic!("Database already initialized"));
    Ok(())
}

pub fn db() -> &'static DbConn {
    DB.get().expect("Database not initialized")
}

fn get_prod_database_url() -> String {
    let data_path = get_data_path();

    let json =
        std::fs::read_to_string(data_path.config_file_path).expect("Failed to read config file");

    let config: Config = serde_json::from_str(&json).expect("Failed to parse config file");

    format!("sqlite://{}?mode=rwc", config.db)
}

fn get_debug_database_url() -> String {
    let (config, data_path) = get_config();

    if config.db != data_path.db_file_path {
        format!("sqlite://{}?mode=rwc", config.db)
    } else {
        format!("sqlite://../{}?mode=rwc", DB_NAME)
    }
}
