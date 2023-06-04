use dotenv::dotenv;
use migration::{DbErr, Migrator, MigratorTrait};
use sea_orm::{Database, DbConn};

pub async fn establish_connection() -> Result<DbConn, DbErr> {
    dotenv().ok();
    // let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set.");
    let database_url = String::from("sqlite://clippy.sqlite?mode=rwc");
    println!("Connecting to {}", database_url);
    let db = Database::connect(&database_url)
        .await
        .expect("Failed to setup the database");
    Migrator::up(&db, None)
        .await
        .expect("Failed to run migrations for tests");

    Ok(db)
}
