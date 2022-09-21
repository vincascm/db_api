use std::env::args;

use anyhow::{Context, Result};
use async_std::task::block_on;
use sqlx::mysql::MySqlPoolOptions;

mod config;
mod mysql;

use config::Config;
use mysql::Database;

fn main() -> Result<()> {
    let config_file = args()
        .nth(1)
        .context("missing config file, usage:\n db_api <config_file>")?;
    let config = Config::from_file(&config_file)?;
    block_on(async {
        let db = MySqlPoolOptions::new().connect(&config.db_url).await?;
        let rust_code = Database::generate_rust_code(&config, &db).await?;
        println!("{}", rust_code);
        Ok(())
    })
}
