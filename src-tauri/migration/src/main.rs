use sea_orm_migration::prelude::*;

#[async_std::main]
async fn main() {
    println!("asd");
    cli::run_cli(migration::Migrator).await;
}
