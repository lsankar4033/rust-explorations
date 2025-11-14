// Database model structs

use chrono::{DateTime, Utc};
use serde_json::Value as JsonValue;
use sqlx::FromRow;

/// Market database row
///
/// Combines on-chain event data with enriched metadata from Gamma API
#[derive(Debug, Clone, FromRow)]
pub struct Market {
    /// Unique condition ID (hex string with 0x prefix)
    pub condition_id: String,

    /// Token IDs from TokenRegistered event
    pub token0: String,
    pub token1: String,

    /// Block where market was registered
    pub block_number: i64,

    /// Transaction hash of registration
    pub tx_hash: String,

    /// Market question (from Gamma API)
    pub question: Option<String>,

    /// URL-friendly slug (from Gamma API)
    pub slug: Option<String>,

    /// Market category (from Gamma API)
    pub category: Option<String>,

    /// Outcome labels as JSON array (from Gamma API)
    pub outcomes: Option<JsonValue>,

    /// Market start date ISO 8601 (from Gamma API)
    pub start_date: Option<String>,

    /// Market end date ISO 8601 (from Gamma API)
    pub end_date: Option<String>,

    /// When this record was created
    pub created_at: DateTime<Utc>,

    /// When this record was last updated
    pub updated_at: DateTime<Utc>,

    /// When metadata was fetched from Gamma API (null if not fetched)
    pub metadata_fetched_at: Option<DateTime<Utc>>,
}
