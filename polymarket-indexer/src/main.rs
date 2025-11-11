// Polymarket Indexer - Phase 1a: TokenRegistered Event Listener
//
// Goal: Fetch and display TokenRegistered events from Polygon
//
// Usage:
//   cargo run -p polymarket-indexer -- historical [from_block] [to_block]
//   cargo run -p polymarket-indexer -- live

use eyre::Result;
use tracing::{info, Level};
use tracing_subscriber;

mod client;
mod events;

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize logging
    tracing_subscriber::fmt()
        .with_max_level(Level::INFO)
        .init();

    info!("Polymarket Indexer starting...");

    // Parse command line arguments
    let args: Vec<String> = std::env::args().collect();

    if args.len() < 2 {
        println!("Usage:");
        println!("  {} historical [from_block] [to_block]", args[0]);
        println!("  {} live", args[0]);
        return Ok(());
    }

    let mode = &args[1];

    match mode.as_str() {
        "historical" => {
            info!("Running in historical mode");
            // TODO: Parse from_block and to_block
            // TODO: Create client
            // TODO: Fetch historical events
            // TODO: Display events
            println!("Historical mode not yet implemented");
        }
        "live" => {
            info!("Running in live mode");
            // TODO: Create client
            // TODO: Subscribe to live events
            // TODO: Display events as they arrive
            // TODO: Handle Ctrl+C gracefully
            println!("Live mode not yet implemented");
        }
        _ => {
            println!("Unknown mode: {}", mode);
            println!("Use 'historical' or 'live'");
        }
    }

    Ok(())
}
