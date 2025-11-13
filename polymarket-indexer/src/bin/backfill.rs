// Backfill - Fetch historical TokenRegistered events
//
// Usage (run from polymarket-indexer directory):
//   cd polymarket-indexer
//   cargo run --bin backfill -- --from-block 50000000 --to-block 50001000
//   cargo run --bin backfill  (defaults to last 10 blocks)

use ethers::types::Filter;
use eyre::Result;
use polymarket_indexer::client::evm::HttpClient;
use polymarket_indexer::client::{Chain, Provider};
use polymarket_indexer::polymarket::constants::{
    ctf_exchange_address, token_registered_event_signature,
};
use tracing::{info, Level};

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

    let target_block = 78975130;
    info!("Fetching logs from block {}", target_block);

    let filter = Filter::new()
        .address(ctf_exchange_address())
        .topic0(token_registered_event_signature())
        .from_block(target_block)
        .to_block(target_block);

    let logs = match client.get_logs(&filter).await {
        Ok(logs) => match !logs.is_empty() {
            true => {
                info!("Found {} logs in block {}", logs.len(), target_block,);
                logs
            }
            false => vec![],
        },
        Err(e) => {
            info!("Error fetching block {}: {}", target_block, e);
            vec![]
        }
    };

    info!("Total logs found: {}", logs.len());

    // Print raw logs for inspection
    for (i, log) in logs.iter().enumerate() {
        println!("\n========== Log {} ==========", i + 1);
        println!("{:#?}", log);
    }

    Ok(())
}
