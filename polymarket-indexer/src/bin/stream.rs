// Stream - Subscribe to live TokenRegistered events
//
// Usage:
//   cargo run --bin stream

use eyre::Result;
use polymarket_indexer::client::WsClient;
use polymarket_indexer::provider::{Chain, Provider};
use tracing::{info, Level};
use tracing_subscriber;

#[tokio::main]
async fn main() -> Result<()> {
    // Load .env file
    dotenv::dotenv().ok();

    // Initialize logging
    tracing_subscriber::fmt().with_max_level(Level::INFO).init();

    info!("Polymarket Live Stream starting...");

    // Get API key from environment
    let api_key = std::env::var("ALCHEMY_API_KEY").expect("ALCHEMY_API_KEY not set in .env file");

    // Create WebSocket client
    let client = WsClient::new(Provider::Alchemy, Chain::Polygon, Some(&api_key)).await?;

    // Get current block number to verify connection
    let current_block = client.get_block_number().await?;
    info!("Connected! Current block: {}", current_block);

    // TODO: Create Filter for TokenRegistered events
    // TODO: Subscribe to logs using client.subscribe_logs()
    // TODO: Loop over stream and parse each log
    // TODO: Display each event using event.display()
    // TODO: Handle Ctrl+C gracefully (tokio::select! with shutdown signal)

    println!("Live stream logic not yet implemented");
    println!("Will subscribe to new TokenRegistered events");

    // Keep running until Ctrl+C
    tokio::signal::ctrl_c().await?;
    info!("Shutting down...");

    Ok(())
}
