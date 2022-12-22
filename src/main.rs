pub mod config;
pub mod http;
use crate::http::serve;
use anyhow::Context;
use sqlx::postgres::PgPoolOptions;
use clap::Parser;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Return an error if `.env` file don't exist
    dotenv::dotenv().ok();

    // Initialize the logger
    env_logger::init();

    // Parse our configuration from the environment.
    // This will exit with a help message if something is wrong.
    let config = config::Config::parse();
    // Create a single connection pool for SQLx, shared for the entire app (instead of one per API)
    let db = PgPoolOptions::new()
        // Default connection limit == 100 (3 of them reserved for super users)
        // I'm intending 1 replica, so I'll allow 90
        .max_connections(90)
        .connect(&config.database_url)
        .await
        .context("Unable to connect to database")?;

    // Embed DB migration into the binary, ensuring migrations has run on startup
    // sqlx::migrate!().run(&db).await?;

    // Run API app, using the `core.rs` `serve` function.
    serve(config, db).await?;

    // The () type has exactly one value (), and is used when there is no other meaningful value that could be returned.
    Ok(())
}
