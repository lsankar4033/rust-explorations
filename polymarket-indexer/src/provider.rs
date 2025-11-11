// RPC Provider and Chain Configuration

#[derive(Debug, Clone, Copy)]
pub enum Provider {
    Alchemy,
    Infura,
    PublicRpc,
}

#[derive(Debug, Clone, Copy)]
pub enum Chain {
    Polygon,
    PolygonAmoy, // Testnet
    Ethereum,
}

impl Provider {
    /// Build HTTP RPC URL for the given chain
    pub(crate) fn http_url(&self, chain: Chain, api_key: Option<&str>) -> String {
        match (self, chain) {
            (Provider::Alchemy, Chain::Polygon) => {
                format!(
                    "https://polygon-mainnet.g.alchemy.com/v2/{}",
                    api_key.expect("Alchemy requires API key")
                )
            }
            (Provider::Alchemy, Chain::PolygonAmoy) => {
                format!(
                    "https://polygon-amoy.g.alchemy.com/v2/{}",
                    api_key.expect("Alchemy requires API key")
                )
            }
            (Provider::Alchemy, Chain::Ethereum) => {
                format!(
                    "https://eth-mainnet.g.alchemy.com/v2/{}",
                    api_key.expect("Alchemy requires API key")
                )
            }
            (Provider::Infura, Chain::Polygon) => {
                format!(
                    "https://polygon-mainnet.infura.io/v3/{}",
                    api_key.expect("Infura requires API key")
                )
            }
            (Provider::Infura, Chain::Ethereum) => {
                format!(
                    "https://mainnet.infura.io/v3/{}",
                    api_key.expect("Infura requires API key")
                )
            }
            (Provider::PublicRpc, Chain::Polygon) => "https://polygon-rpc.com".to_string(),
            _ => unimplemented!("Unsupported provider/chain combination"),
        }
    }

    /// Build WebSocket RPC URL for the given chain
    pub(crate) fn ws_url(&self, chain: Chain, api_key: Option<&str>) -> String {
        match (self, chain) {
            (Provider::Alchemy, Chain::Polygon) => {
                format!(
                    "wss://polygon-mainnet.g.alchemy.com/v2/{}",
                    api_key.expect("Alchemy requires API key")
                )
            }
            (Provider::Alchemy, Chain::PolygonAmoy) => {
                format!(
                    "wss://polygon-amoy.g.alchemy.com/v2/{}",
                    api_key.expect("Alchemy requires API key")
                )
            }
            (Provider::Alchemy, Chain::Ethereum) => {
                format!(
                    "wss://eth-mainnet.g.alchemy.com/v2/{}",
                    api_key.expect("Alchemy requires API key")
                )
            }
            (Provider::Infura, Chain::Polygon) => {
                format!(
                    "wss://polygon-mainnet.infura.io/ws/v3/{}",
                    api_key.expect("Infura requires API key")
                )
            }
            (Provider::Infura, Chain::Ethereum) => {
                format!(
                    "wss://mainnet.infura.io/ws/v3/{}",
                    api_key.expect("Infura requires API key")
                )
            }
            _ => unimplemented!("Unsupported provider/chain combination for WebSocket"),
        }
    }
}
