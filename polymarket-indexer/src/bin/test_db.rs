// Test Database Connection
//
// Usage:
//   cargo run --bin test_db

use eyre::Result;
use polymarket_indexer::db::{create_pool, markets};
use tracing::{info, Level};

#[tokio::main]
async fn main() -> Result<()> {
    // Load .env file
    dotenv::dotenv().ok();

    // Initialize logging
    tracing_subscriber::fmt().with_max_level(Level::INFO).init();

    info!("Testing database connection...");

    // Create connection pool
    let pool = create_pool().await?;

    // Test a simple query
    let count = markets::count_markets(&pool).await?;

    info!("✓ Database connection successful!");
    info!("✓ Current market count: {}", count);

    // Test fetching a market if any exist
    if count > 0 {
        let markets_without_metadata = markets::get_markets_without_metadata(&pool, 1).await?;
        info!(
            "✓ Markets without metadata: {}",
            markets_without_metadata.len()
        );
    }

    info!("All database checks passed! 🎉");

    Ok(())
}
