use super::config::{get_config, get_data_path};
use super::tao_constants::DB;
use common::{constants::DB_NAME, types::types::Config};
use migration::{DbErr, Migrator, MigratorTrait};
use sea_orm::{ConnectOptions, Database, DbConn};

pub async fn init_db() -> Result<(), DbErr> {
    let database_url = if cfg!(debug_assertions) {
        get_debug_database_url()
    } else {
        get_prod_database_url()
    };

    // Try the existing database first. If connecting or migrating fails (e.g. a
    // schema from an incompatible version, or a corrupt file), we don't want to
    // crash on startup and lose the user's data silently. Instead, move the broken
    // file aside as a timestamped backup and start fresh so the app still launches.
    match connect_and_migrate(&database_url).await {
        Ok(conn) => set_db(conn),
        Err(err) => {
            log::error!("Database init failed: {err}. Backing up and recreating.");
            if let Some(path) = sqlite_path_from_url(&database_url) {
                backup_broken_db(&path);
            }
            // Second attempt against the now-empty location. If this also fails the
            // problem isn't the data, so propagate the error.
            let conn = connect_and_migrate(&database_url).await?;
            set_db(conn);
        }
    }
    Ok(())
}

async fn connect_and_migrate(database_url: &str) -> Result<DbConn, DbErr> {
    // Disable per-statement SQL logging: it floods the log file/console with every
    // query at Info, which tauri-plugin-log would otherwise capture.
    let mut opt = ConnectOptions::new(database_url.to_owned());
    opt.sqlx_logging(false);

    let conn = Database::connect(opt).await?;
    Migrator::up(&conn, None).await?;
    Ok(conn)
}

fn set_db(conn: DbConn) {
    DB.set(conn)
        .unwrap_or_else(|_| panic!("Database already initialized"));
}

/// Extract the filesystem path from a `sqlite://<path>?...` URL. Returns None for
/// in-memory databases or anything without a real file path.
fn sqlite_path_from_url(database_url: &str) -> Option<String> {
    let without_scheme = database_url.strip_prefix("sqlite://")?;
    let path = without_scheme.split('?').next().unwrap_or(without_scheme);
    if path.is_empty() || path == ":memory:" {
        return None;
    }
    Some(path.to_string())
}

/// Rename the broken database file (and its -wal/-shm siblings) to a backup so the
/// data is preserved while we recreate a fresh database in its place.
fn backup_broken_db(path: &str) {
    use std::path::Path;
    if !Path::new(path).exists() {
        return;
    }
    // No clock dependency: pick the first non-colliding suffix.
    let mut backup = format!("{path}.broken");
    let mut n = 1;
    while Path::new(&backup).exists() {
        backup = format!("{path}.broken.{n}");
        n += 1;
    }

    match std::fs::rename(path, &backup) {
        Ok(()) => {
            log::warn!("Moved broken database to {backup}");
            // Move the SQLite sidecar files too so the fresh DB starts clean.
            for ext in ["-wal", "-shm"] {
                let sidecar = format!("{path}{ext}");
                if Path::new(&sidecar).exists() {
                    let _ = std::fs::rename(&sidecar, format!("{backup}{ext}"));
                }
            }
        }
        Err(e) => log::error!("Failed to back up broken database {path}: {e}"),
    }
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
