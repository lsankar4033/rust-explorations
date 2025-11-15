// Database module - PostgreSQL connection and operations

pub mod market_tags;
pub mod markets;
pub mod models;

use eyre::Result;
use sqlx::postgres::{PgPool, PgPoolOptions};
use std::time::Duration;
use tracing::info;

/// Create a PostgreSQL connection pool
///
/// Reads DATABASE_URL from environment variable
pub async fn create_pool() -> Result<PgPool> {
    let database_url =
        std::env::var("DATABASE_URL").expect("DATABASE_URL must be set in .env file");

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .acquire_timeout(Duration::from_secs(3))
        .connect(&database_url)
        .await?;

    info!("✓ Connected to PostgreSQL database");

    Ok(pool)
}
