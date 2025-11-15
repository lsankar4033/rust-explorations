// Market Metadata from Polymarket Gamma API
//
// Structures for deserializing market information returned from
// https://gamma-api.polymarket.com/markets

use serde::{de, Deserialize, Deserializer};

/// Market metadata from Gamma API
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MarketMetadata {
    /// Polymarket's internal market ID
    #[serde(default)]
    pub id: Option<String>,

    /// The market question text
    pub question: String,

    /// URL-friendly slug
    pub slug: String,

    /// Condition ID (hex string with 0x prefix)
    pub condition_id: String,

    /// Outcome labels (e.g., ["Yes", "No"] or ["Up", "Down"])
    /// API returns this as a JSON-encoded string, so we need custom deserialization
    #[serde(deserialize_with = "deserialize_json_string_array")]
    pub outcomes: Vec<String>,

    /// Market start date (ISO 8601 string)
    #[serde(default)]
    pub start_date: Option<String>,

    /// Market end date (ISO 8601 string)
    #[serde(default)]
    pub end_date: Option<String>,
}

/// Custom deserializer for JSON-encoded string arrays
/// API returns arrays as strings like "[\"Up\", \"Down\"]"
fn deserialize_json_string_array<'de, D>(deserializer: D) -> Result<Vec<String>, D::Error>
where
    D: Deserializer<'de>,
{
    let s: String = Deserialize::deserialize(deserializer)?;
    serde_json::from_str(&s).map_err(de::Error::custom)
}

/// Tag associated with a market
#[derive(Debug, Clone, Deserialize)]
pub struct Tag {
    /// Polymarket's internal tag ID
    pub id: String,

    /// Human-readable tag label
    pub label: Option<String>,

    /// URL-friendly tag slug
    pub slug: Option<String>,
}

impl MarketMetadata {
    /// Pretty-print market metadata
    pub fn display(&self) {
        println!("=================================");
        println!("Market Metadata");
        if let Some(id) = &self.id {
            println!("  PM Market ID: {}", id);
        }
        println!("  Question: {}", self.question);
        println!("  Slug: {}", self.slug);
        println!("  Outcomes: {:?}", self.outcomes);
        if let Some(start) = &self.start_date {
            println!("  Start Date: {}", start);
        }
        if let Some(end) = &self.end_date {
            println!("  End Date: {}", end);
        }
        println!("=================================");
    }
}
