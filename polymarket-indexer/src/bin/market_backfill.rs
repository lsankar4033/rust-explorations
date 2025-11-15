// Market Backfill - Index historical TokenRegistered events and enrich with metadata
//
// Usage:
//   cargo run --bin market_backfill -- --days 7
//   cargo run --bin market_backfill -- --hours 6
//   cargo run --bin market_backfill -- --minutes 30
//   cargo run --bin market_backfill -- --from-block 50000000 --to-block 50001000

use ethers::types::Filter;
use eyre::Result;
use polymarket_indexer::client::evm::HttpClient;
use polymarket_indexer::client::gamma::GammaClient;
use polymarket_indexer::client::{Chain, Provider};
use polymarket_indexer::db::{create_pool, market_tags, markets};
use polymarket_indexer::polymarket::constants::{
    ctf_exchange_address, token_registered_event_signature,
};
use polymarket_indexer::polymarket::events::TokenRegistered;
use std::collections::HashMap;
use std::env;
use tracing::{info, warn, Level};

/// Polygon block time: ~2 seconds
const POLYGON_BLOCK_TIME_SECS: i64 = 2;

#[tokio::main]
async fn main() -> Result<()> {
    dotenv::dotenv().ok();
    tracing_subscriber::fmt().with_max_level(Level::INFO).init();

    info!("Market Backfill starting...");

    // Parse CLI arguments
    let args: Vec<String> = env::args().collect();
    let (from_block, to_block) = parse_block_range(&args).await?;

    info!("Backfill range: blocks {} to {}", from_block, to_block);

    // Initialize clients
    let api_key = env::var("ALCHEMY_API_KEY").expect("ALCHEMY_API_KEY not set");
    let evm_client = HttpClient::new(Provider::Alchemy, Chain::Polygon, Some(&api_key)).await?;
    let gamma_client = GammaClient::new();
    let db_pool = create_pool().await?;

    // Fetch TokenRegistered events
    info!("Fetching TokenRegistered events...");
    let filter = Filter::new()
        .address(ctf_exchange_address())
        .topic0(token_registered_event_signature())
        .from_block(from_block)
        .to_block(to_block);

    let logs = evm_client.get_logs(&filter).await?;
    info!("Found {} TokenRegistered events", logs.len());

    // Deduplicate by condition_id (each market emits 2 events with swapped tokens)
    let mut unique_events: HashMap<String, TokenRegistered> = HashMap::new();
    for log in &logs {
        match TokenRegistered::from_log(log) {
            Ok(event) => {
                unique_events.insert(event.condition_id_hex(), event);
            }
            Err(e) => {
                warn!("Failed to parse log: {}", e);
            }
        }
    }

    info!(
        "Unique markets: {} (deduped from {} events)",
        unique_events.len(),
        logs.len()
    );

    // Process each unique market
    let mut inserted = 0;
    let mut skipped = 0;
    let mut failed = 0;
    let mut tags_inserted = 0;
    let mut tags_failed = 0;

    for (condition_id, event) in unique_events {
        // Check if already in DB
        if markets::get_market_by_condition_id(&db_pool, &condition_id)
            .await?
            .is_some()
        {
            skipped += 1;
            continue;
        }

        // Fetch metadata from Gamma API
        let metadata = match gamma_client.get_market_with_retry(&condition_id, 5).await {
            Ok(Some(m)) => Some(m),
            Ok(None) => {
                warn!("No metadata found for {}", condition_id);
                None
            }
            Err(e) => {
                warn!("Failed to fetch metadata for {}: {}", condition_id, e);
                None
            }
        };

        // Insert into database
        match markets::upsert_market(&db_pool, &event, metadata.as_ref()).await {
            Ok(_) => {
                info!("✓ Inserted market {}", condition_id);

                // Fetch and insert tags if we have a pm_market_id
                if let Some(ref meta) = metadata {
                    if let Some(ref market_id) = meta.id {
                        match gamma_client.get_market_tags(market_id).await {
                            Ok(tags) if !tags.is_empty() => {
                                match market_tags::insert_market_tags(
                                    &db_pool,
                                    &condition_id,
                                    &tags,
                                )
                                .await
                                {
                                    Ok(_) => {
                                        info!("  ✓ Inserted {} tags", tags.len());
                                        tags_inserted += tags.len();
                                    }
                                    Err(e) => {
                                        warn!("  Failed to insert tags: {}", e);
                                        tags_failed += 1;
                                    }
                                }
                            }
                            Ok(_) => {} // No tags, skip silently
                            Err(e) => {
                                warn!("  Failed to fetch tags: {}", e);
                                tags_failed += 1;
                            }
                        }
                    }
                }

                inserted += 1;
            }
            Err(e) => {
                warn!("Failed to insert market {}: {}", condition_id, e);
                failed += 1;
            }
        }
    }

    // Summary
    info!("Backfill complete!");
    info!("  Markets inserted: {}", inserted);
    info!("  Markets skipped (already in DB): {}", skipped);
    info!("  Markets failed: {}", failed);
    info!("  Tags inserted: {}", tags_inserted);
    info!("  Tags failed: {}", tags_failed);

    Ok(())
}

async fn parse_block_range(args: &[String]) -> Result<(u64, u64)> {
    // Check for time-based arguments (--days, --hours, --minutes)
    let seconds_to_go_back = if let Some(pos) = args.iter().position(|a| a == "--days") {
        let days: i64 = args
            .get(pos + 1)
            .and_then(|s| s.parse().ok())
            .expect("--days requires a number");
        Some(days * 86400)
    } else if let Some(pos) = args.iter().position(|a| a == "--hours") {
        let hours: i64 = args
            .get(pos + 1)
            .and_then(|s| s.parse().ok())
            .expect("--hours requires a number");
        Some(hours * 3600)
    } else if let Some(pos) = args.iter().position(|a| a == "--minutes") {
        let minutes: i64 = args
            .get(pos + 1)
            .and_then(|s| s.parse().ok())
            .expect("--minutes requires a number");
        Some(minutes * 60)
    } else {
        None
    };

    if let Some(seconds) = seconds_to_go_back {
        let api_key = env::var("ALCHEMY_API_KEY").expect("ALCHEMY_API_KEY not set");
        let client = HttpClient::new(Provider::Alchemy, Chain::Polygon, Some(&api_key)).await?;
        let current_block = client.get_block_number().await?;

        // Estimate blocks based on block time
        let blocks_to_go_back = (seconds / POLYGON_BLOCK_TIME_SECS) as u64;
        let from_block = current_block.saturating_sub(blocks_to_go_back);

        return Ok((from_block, current_block));
    }

    // Check for --from-block and --to-block
    let from_block = args
        .iter()
        .position(|a| a == "--from-block")
        .and_then(|pos| args.get(pos + 1))
        .and_then(|s| s.parse().ok())
        .expect("--from-block required (or use --days/--hours/--minutes)");

    let to_block = args
        .iter()
        .position(|a| a == "--to-block")
        .and_then(|pos| args.get(pos + 1))
        .and_then(|s| s.parse().ok())
        .expect("--to-block required (or use --days/--hours/--minutes)");

    Ok((from_block, to_block))
}
