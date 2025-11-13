// Backfill - Fetch historical TokenRegistered events
//
// Usage (run from polymarket-indexer directory):
//   cd polymarket-indexer
//   cargo run --bin backfill -- --from-block 50000000 --to-block 50001000
//   cargo run --bin backfill  (defaults to last 10 blocks)

use ethers::types::Filter;
use eyre::Result;
use polymarket_indexer::client::evm::HttpClient;
use polymarket_indexer::client::gamma::GammaClient;
use polymarket_indexer::client::{Chain, Provider};
use polymarket_indexer::polymarket::constants::{
    ctf_exchange_address, token_registered_event_signature,
};
use polymarket_indexer::polymarket::events::TokenRegistered;
use tracing::{info, warn, Level};

#[tokio::main]
async fn main() -> Result<()> {
    // Load .env file
    dotenv::dotenv().ok();

    // Initialize logging
    tracing_subscriber::fmt().with_max_level(Level::INFO).init();

    info!("Polymarket Backfill starting...");

    // Get API key from environment
    let api_key = std::env::var("ALCHEMY_API_KEY").expect("ALCHEMY_API_KEY not set in .env file");

    // Create HTTP client for blockchain
    let evm_client = HttpClient::new(Provider::Alchemy, Chain::Polygon, Some(&api_key)).await?;

    // Create Gamma API client for market metadata
    let gamma_client = GammaClient::new();

    // Get current block number
    let current_block = evm_client.get_block_number().await?;
    info!("Current block: {}", current_block);

    let target_block = 78975130;
    info!("Fetching logs from block {}", target_block);

    let filter = Filter::new()
        .address(ctf_exchange_address())
        .topic0(token_registered_event_signature())
        .from_block(target_block)
        .to_block(target_block);

    let logs = match evm_client.get_logs(&filter).await {
        Ok(logs) => {
            info!("Found {} logs in block {}", logs.len(), target_block);
            logs
        }
        Err(e) => {
            warn!("Error fetching block {}: {}", target_block, e);
            vec![]
        }
    };

    info!("Total logs found: {}", logs.len());

    // Parse and enrich each log
    for (i, log) in logs.iter().enumerate() {
        println!("\n========== Market {} ==========", i + 1);

        // Parse the TokenRegistered event
        let event = match TokenRegistered::from_log(log) {
            Ok(event) => event,
            Err(e) => {
                warn!("Failed to parse log: {}", e);
                continue;
            }
        };

        // Display on-chain event data
        event.display();

        // Fetch market metadata from Gamma API
        info!(
            "Fetching market metadata for condition_id: {}",
            event.condition_id_hex()
        );

        match gamma_client
            .get_market_with_retry(&event.condition_id_hex(), 3)
            .await
        {
            Ok(Some(market)) => {
                println!();
                market.display();
            }
            Ok(None) => {
                warn!("Market metadata not found in Gamma API");
            }
            Err(e) => {
                warn!("Failed to fetch market metadata: {}", e);
            }
        }
    }

    Ok(())
}
