// Backfill - Fetch historical TokenRegistered events
//
// Usage:
//   cargo run --bin backfill -- --from-block 50000000 --to-block 50001000
//   cargo run --bin backfill  (defaults to last 1000 blocks)

use eyre::Result;
use polymarket_indexer::client::evm::HttpClient;
use polymarket_indexer::provider::{Chain, Provider};
use tracing::{info, Level};
use tracing_subscriber;

#[tokio::main]
async fn main() -> Result<()> {
    // Load .env file
    dotenv::dotenv().ok();

    // Initialize logging
    tracing_subscriber::fmt().with_max_level(Level::INFO).init();

    info!("Polymarket Backfill starting...");

    // Get API key from environment
    let api_key = std::env::var("ALCHEMY_API_KEY").expect("ALCHEMY_API_KEY not set in .env file");

    // Create HTTP client
    let client = HttpClient::new(Provider::Alchemy, Chain::Polygon, Some(&api_key)).await?;

    // Get current block number
    let current_block = client.get_block_number().await?;
    info!("Current block: {}", current_block);

    // TODO: Parse CLI arguments for from_block and to_block
    // TODO: Default to last 1000 blocks if not specified
    // TODO: Create Filter for TokenRegistered events
    // TODO: Call client.get_logs() with filter
    // TODO: Parse logs into TokenRegistered events
    // TODO: Display each event using event.display()

    println!("Backfill logic not yet implemented");
    println!("Will fetch TokenRegistered events from block range");

    Ok(())
}
