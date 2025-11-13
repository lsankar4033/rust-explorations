// Backfill - Fetch historical TokenRegistered events
//
// Usage (run from polymarket-indexer directory):
//   cd polymarket-indexer
//   cargo run --bin backfill -- --from-block 50000000 --to-block 50001000
//   cargo run --bin backfill  (defaults to last 10 blocks)

use ethers::types::{Filter, H160, H256};
use eyre::Result;
use polymarket_indexer::client::evm::HttpClient;
use polymarket_indexer::client::{Chain, Provider};
use polymarket_indexer::polymarket::addresses::CTF_EXCHANGE_ADDRESS;
use std::str::FromStr;
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

    // TODO: Parse CLI arguments for from_block and to_block
    // For testing, use block 78975130 which has known registrations
    let target_from_block = 78975130;
    let target_to_block = 78975130;
    let batch_size = 10u64;

    info!(
        "Fetching logs from block {} to {} in batches of {}",
        target_from_block, target_to_block, batch_size
    );

    // Parse CTF Exchange address
    let contract_address =
        H160::from_str(CTF_EXCHANGE_ADDRESS).expect("Invalid CTF_EXCHANGE_ADDRESS");

    // Create event signature for TokenRegistered
    // Event signature: TokenRegistered(uint256,uint256,bytes32)
    let event_signature =
        ethers::core::utils::keccak256(b"TokenRegistered(uint256,uint256,bytes32)");
    let event_signature_h256 = H256::from(event_signature);

    let mut all_logs = Vec::new();
    let mut current_from = target_from_block;

    // Fetch in batches
    while current_from <= target_to_block {
        let current_to = (current_from + batch_size - 1).min(target_to_block);

        let filter = Filter::new()
            .address(contract_address)
            .topic0(event_signature_h256)
            .from_block(current_from)
            .to_block(current_to);

        match client.get_logs(&filter).await {
            Ok(logs) => {
                if !logs.is_empty() {
                    info!(
                        "Found {} logs in blocks {} to {}",
                        logs.len(),
                        current_from,
                        current_to
                    );
                    all_logs.extend(logs);
                }
            }
            Err(e) => {
                info!(
                    "Error fetching blocks {} to {}: {}",
                    current_from, current_to, e
                );
            }
        }

        current_from = current_to + 1;

        // Small delay to avoid rate limiting
        tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
    }

    info!("Total logs found: {}", all_logs.len());

    // Print raw logs for inspection
    for (i, log) in all_logs.iter().enumerate() {
        println!("\n========== Log {} ==========", i + 1);
        println!("{:#?}", log);
    }

    Ok(())
}
