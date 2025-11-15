// Market database operations

use crate::db::models::Market;
use crate::polymarket::events::TokenRegistered;
use crate::polymarket::market::MarketMetadata;
use eyre::Result;
use sqlx::PgPool;

/// Insert or update a market with on-chain data and optional metadata
///
/// This is idempotent - safe to call multiple times with the same condition_id.
/// If metadata is provided, it will update the existing record.
pub async fn upsert_market(
    pool: &PgPool,
    event: &TokenRegistered,
    metadata: Option<&MarketMetadata>,
) -> Result<()> {
    let outcomes_json = metadata.and_then(|m| serde_json::to_value(&m.outcomes).ok());

    sqlx::query!(
        r#"
        INSERT INTO markets (
            condition_id, token0, token1, block_number, tx_hash,
            question, slug, outcomes, start_date, end_date,
            metadata_fetched_at
        ) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11)
        ON CONFLICT (condition_id) DO UPDATE SET
            question = COALESCE(EXCLUDED.question, markets.question),
            slug = COALESCE(EXCLUDED.slug, markets.slug),
            outcomes = COALESCE(EXCLUDED.outcomes, markets.outcomes),
            start_date = COALESCE(EXCLUDED.start_date, markets.start_date),
            end_date = COALESCE(EXCLUDED.end_date, markets.end_date),
            metadata_fetched_at = COALESCE(EXCLUDED.metadata_fetched_at, markets.metadata_fetched_at),
            updated_at = NOW()
        "#,
        event.condition_id_hex(),
        event.token0.to_string(),
        event.token1.to_string(),
        event.block_number as i64,
        event.tx_hash,
        metadata.map(|m| m.question.as_str()),
        metadata.map(|m| m.slug.as_str()),
        outcomes_json,
        metadata.and_then(|m| m.start_date.as_ref().map(|s| s.as_str())),
        metadata.and_then(|m| m.end_date.as_ref().map(|s| s.as_str())),
        if metadata.is_some() {
            Some(chrono::Utc::now())
        } else {
            None
        }
    )
    .execute(pool)
    .await?;

    Ok(())
}

/// Get a market by condition ID
pub async fn get_market_by_condition_id(
    pool: &PgPool,
    condition_id: &str,
) -> Result<Option<Market>> {
    let market = sqlx::query_as!(
        Market,
        r#"
        SELECT * FROM markets
        WHERE condition_id = $1
        "#,
        condition_id
    )
    .fetch_optional(pool)
    .await?;

    Ok(market)
}

/// Get markets that don't have metadata yet (for retry logic)
pub async fn get_markets_without_metadata(pool: &PgPool, limit: i64) -> Result<Vec<Market>> {
    let markets = sqlx::query_as!(
        Market,
        r#"
        SELECT * FROM markets
        WHERE metadata_fetched_at IS NULL
        ORDER BY created_at ASC
        LIMIT $1
        "#,
        limit
    )
    .fetch_all(pool)
    .await?;

    Ok(markets)
}

/// Count total markets in database
pub async fn count_markets(pool: &PgPool) -> Result<i64> {
    let result = sqlx::query!(
        r#"
        SELECT COUNT(*) as count FROM markets
        "#
    )
    .fetch_one(pool)
    .await?;

    Ok(result.count.unwrap_or(0))
}
