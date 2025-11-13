// Gamma API Client for Polymarket Market Metadata
//
// Provides HTTP client for querying market information from:
// https://gamma-api.polymarket.com

use crate::polymarket::market::MarketMetadata;
use eyre::Result;
use reqwest::Client;
use tracing::warn;

/// Base URL for Gamma API
const GAMMA_API_BASE_URL: &str = "https://gamma-api.polymarket.com";

/// HTTP client for Gamma API
pub struct GammaClient {
    client: Client,
    base_url: String,
}

impl GammaClient {
    /// Create a new Gamma API client
    pub fn new() -> Self {
        Self {
            client: Client::new(),
            base_url: GAMMA_API_BASE_URL.to_string(),
        }
    }

    /// Get market metadata by condition ID
    ///
    /// # Arguments
    /// * `condition_id` - Hex string with 0x prefix (e.g., "0x1b2ca1f8...")
    ///
    /// # Returns
    /// * `Ok(Some(MarketMetadata))` - Market found
    /// * `Ok(None)` - Market not found in API (may be too new)
    /// * `Err(_)` - Network or parsing error
    pub async fn get_market_by_condition_id(
        &self,
        condition_id: &str,
    ) -> Result<Option<MarketMetadata>> {
        let url = format!("{}/markets", self.base_url);

        let response = self
            .client
            .get(&url)
            .query(&[("condition_ids", condition_id)])
            .send()
            .await?;

        if !response.status().is_success() {
            warn!(
                "Gamma API returned non-success status: {}",
                response.status()
            );
            return Ok(None);
        }

        // API returns a direct array, not wrapped in an object
        let markets: Vec<MarketMetadata> = response.json().await?;

        // Return the first market if any were found
        Ok(markets.into_iter().next())
    }

    /// Get market metadata with retry logic
    ///
    /// New markets may not immediately appear in the Gamma API.
    /// This method retries with exponential backoff.
    ///
    /// # Arguments
    /// * `condition_id` - Hex string with 0x prefix
    /// * `max_retries` - Maximum number of retry attempts
    ///
    /// # Returns
    /// * `Ok(Some(MarketMetadata))` - Market found
    /// * `Ok(None)` - Market not found after all retries
    /// * `Err(_)` - Persistent network or parsing error
    pub async fn get_market_with_retry(
        &self,
        condition_id: &str,
        max_retries: u32,
    ) -> Result<Option<MarketMetadata>> {
        let mut attempt = 0;

        loop {
            match self.get_market_by_condition_id(condition_id).await {
                Ok(Some(market)) => return Ok(Some(market)),
                Ok(None) if attempt < max_retries => {
                    // Market not found, retry with exponential backoff
                    let delay_ms = 100 * 2u64.pow(attempt);
                    warn!(
                        "Market not found for condition_id {}, retrying in {}ms (attempt {}/{})",
                        condition_id,
                        delay_ms,
                        attempt + 1,
                        max_retries
                    );
                    tokio::time::sleep(tokio::time::Duration::from_millis(delay_ms)).await;
                    attempt += 1;
                }
                Ok(None) => {
                    // Market not found after all retries
                    warn!(
                        "Market not found for condition_id {} after {} retries",
                        condition_id, max_retries
                    );
                    return Ok(None);
                }
                Err(e) => {
                    // Network or parsing error
                    if attempt < max_retries {
                        let delay_ms = 100 * 2u64.pow(attempt);
                        warn!(
                            "Error fetching market: {}. Retrying in {}ms (attempt {}/{})",
                            e,
                            delay_ms,
                            attempt + 1,
                            max_retries
                        );
                        tokio::time::sleep(tokio::time::Duration::from_millis(delay_ms)).await;
                        attempt += 1;
                    } else {
                        return Err(e);
                    }
                }
            }
        }
    }
}

impl Default for GammaClient {
    fn default() -> Self {
        Self::new()
    }
}
