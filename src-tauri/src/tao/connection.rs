use super::config::{get_config, get_data_path};
use super::tao_constants::DB;
use common::{constants::DB_NAME, types::types::Config};
use migration::{DbErr, Migrator, MigratorTrait};
use sea_orm::sqlx::sqlite::{SqliteConnectOptions, SqliteJournalMode, SqlitePoolOptions};
use sea_orm::sqlx::ConnectOptions as SqlxConnectOptions;
use sea_orm::{DbConn, SqlxSqliteConnector};
use std::time::Duration;

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
    // sqlx only accepts mode/cache/immutable/vfs as URL query params; pragmas like
    // journal_mode MUST be set via SqliteConnectOptions (applied to every pooled
    // connection) or connect errors with "unknown query parameter".
    let opt = database_url
        .parse::<SqliteConnectOptions>()
        .map_err(|e| DbErr::Conn(sea_orm::RuntimeErr::Internal(e.to_string())))?
        // WAL: readers don't block writers, which kept connections pinned long enough
        // to starve the pool (ConnectionAcquire(Timeout) panics on clipboard insert).
        // busy_timeout(5s) + foreign_keys(on) are sqlx defaults, WAL is not.
        .journal_mode(SqliteJournalMode::Wal)
        .busy_timeout(Duration::from_secs(5))
        // Statement logging floods the log file with every query otherwise.
        .disable_statement_logging();

    // Small pool: SQLite is single-writer, but >1 connection stops one long-running
    // reader (sync loop, search scan) from starving clipboard inserts.
    let pool = SqlitePoolOptions::new()
        .max_connections(4)
        .acquire_timeout(Duration::from_secs(10))
        .connect_with(opt)
        .await
        .map_err(|e| DbErr::Conn(sea_orm::RuntimeErr::SqlxError(e)))?;

    let conn = SqlxSqliteConnector::from_sqlx_sqlite_pool(pool);
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

/// Only mode/cache/immutable/vfs are valid sqlx URL params. Pragmas (WAL etc.) are set
/// in connect_and_migrate via SqliteConnectOptions; putting them here makes connect fail.
const SQLITE_PARAMS: &str = "mode=rwc";

fn get_prod_database_url() -> String {
    let data_path = get_data_path();

    let json =
        std::fs::read_to_string(data_path.config_file_path).expect("Failed to read config file");

    let config: Config = serde_json::from_str(&json).expect("Failed to parse config file");

    format!("sqlite://{}?{}", config.db, SQLITE_PARAMS)
}

fn get_debug_database_url() -> String {
    let (config, data_path) = get_config();

    if config.db != data_path.db_file_path {
        format!("sqlite://{}?{}", config.db, SQLITE_PARAMS)
    } else {
        format!("sqlite://../{}?{}", DB_NAME, SQLITE_PARAMS)
    }
}
