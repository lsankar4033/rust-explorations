pub mod evm;

// RPC Provider and Chain Configuration

#[derive(Debug, Clone, Copy)]
pub enum Provider {
    Alchemy,
}

#[derive(Debug, Clone, Copy)]
pub enum Chain {
    Polygon,
}

impl Provider {
    /// Build HTTP RPC URL for Polygon mainnet on Alchemy
    pub(crate) fn http_url(&self, chain: Chain, api_key: Option<&str>) -> String {
        match (self, chain) {
            (Provider::Alchemy, Chain::Polygon) => {
                format!(
                    "https://polygon-mainnet.g.alchemy.com/v2/{}",
                    api_key.expect("Alchemy requires API key")
                )
            }
        }
    }

    /// Build WebSocket RPC URL for Polygon mainnet on Alchemy
    pub(crate) fn ws_url(&self, chain: Chain, api_key: Option<&str>) -> String {
        match (self, chain) {
            (Provider::Alchemy, Chain::Polygon) => {
                format!(
                    "wss://polygon-mainnet.g.alchemy.com/v2/{}",
                    api_key.expect("Alchemy requires API key")
                )
            }
        }
    }
}
