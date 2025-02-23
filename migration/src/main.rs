use std::env;

use anyhow::Context as _;
use app_config::AppConfig;
use sea_orm_migration::prelude::*;

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    // CUSTOM: Load our own config.
    let conf = AppConfig::from_file()?;

    let db_url = conf.database_url();
    env::set_var("DATABASE_URL", db_url);

    cli::run_cli(migration::Migrator).await;
    Ok(())
}
