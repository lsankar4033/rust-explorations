// Polygon RPC Clients for HTTP and WebSocket

use crate::provider::{Chain, Provider};
use ethers::providers::{Http, Middleware, Provider as EthersProvider, Ws};
use eyre::Result;
use std::sync::Arc;

/// HTTP client for historical queries (eth_getLogs)
pub struct HttpClient {
    provider: Arc<EthersProvider<Http>>,
}

impl HttpClient {
    /// Create a new HTTP client for the given provider and chain
    pub async fn new(provider: Provider, chain: Chain, api_key: Option<&str>) -> Result<Self> {
        let url = provider.http_url(chain, api_key);
        let http_provider = EthersProvider::<Http>::try_from(url)?;

        Ok(Self {
            provider: Arc::new(http_provider),
        })
    }

    /// Get the current block number
    pub async fn get_block_number(&self) -> Result<u64> {
        let block_number = self.provider.get_block_number().await?;
        Ok(block_number.as_u64())
    }

    // TODO: Add method to fetch historical logs
    // pub async fn get_logs(&self, filter: Filter) -> Result<Vec<Log>>
}

/// WebSocket client for live event streaming (eth_subscribe)
pub struct WsClient {
    provider: Arc<EthersProvider<Ws>>,
}

impl WsClient {
    /// Create a new WebSocket client for the given provider and chain
    pub async fn new(provider: Provider, chain: Chain, api_key: Option<&str>) -> Result<Self> {
        let url = provider.ws_url(chain, api_key);
        let ws_provider = EthersProvider::<Ws>::connect(url).await?;

        Ok(Self {
            provider: Arc::new(ws_provider),
        })
    }

    /// Get the current block number
    pub async fn get_block_number(&self) -> Result<u64> {
        let block_number = self.provider.get_block_number().await?;
        Ok(block_number.as_u64())
    }

    // TODO: Add method to subscribe to logs
    // pub async fn subscribe_logs(&self, filter: Filter) -> Result<impl Stream<Item = Log>>
}
