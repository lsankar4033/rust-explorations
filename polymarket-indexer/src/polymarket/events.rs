// TokenRegistered Event Definition and Processing
//
// Event: TokenRegistered(uint256 indexed token0, uint256 indexed token1, bytes32 indexed conditionId)
// Emitted by: CTFExchange contract

use ethers::types::{Log, U256};
use eyre::{eyre, Result};

/// TokenRegistered event structure
///
/// Emitted when a new outcome token pair is registered for trading
#[derive(Debug, Clone)]
pub struct TokenRegistered {
    /// First outcome token ID (typically YES outcome)
    pub token0: U256,
    /// Second outcome token ID (typically NO outcome)
    pub token1: U256,
    /// Condition ID - unique identifier for the market
    pub condition_id: [u8; 32],
    /// Block number where event was emitted
    pub block_number: u64,
    /// Transaction hash
    pub tx_hash: String,
}

impl TokenRegistered {
    /// Parse a TokenRegistered event from a raw log
    ///
    /// Expected log structure:
    /// - topics[0]: Event signature (keccak256 of "TokenRegistered(uint256,uint256,bytes32)")
    /// - topics[1]: token0 (first indexed parameter)
    /// - topics[2]: token1 (second indexed parameter)
    /// - topics[3]: conditionId (third indexed parameter)
    pub fn from_log(log: &Log) -> Result<Self> {
        // Validate we have exactly 4 topics (signature + 3 indexed params)
        if log.topics.len() != 4 {
            return Err(eyre!(
                "Invalid TokenRegistered log: expected 4 topics, got {}",
                log.topics.len()
            ));
        }

        // Extract token0 from topics[1]
        let token0 = U256::from(log.topics[1].as_bytes());

        // Extract token1 from topics[2]
        let token1 = U256::from(log.topics[2].as_bytes());

        // Extract conditionId from topics[3] (convert H256 to [u8; 32])
        let condition_id: [u8; 32] = log.topics[3].0;

        // Extract block number
        let block_number = log
            .block_number
            .ok_or_else(|| eyre!("Log missing block_number"))?
            .as_u64();

        // Extract transaction hash
        let tx_hash = log
            .transaction_hash
            .ok_or_else(|| eyre!("Log missing transaction_hash"))?
            .to_string();

        Ok(TokenRegistered {
            token0,
            token1,
            condition_id,
            block_number,
            tx_hash,
        })
    }

    /// Pretty-print the event to console
    pub fn display(&self) {
        println!("=================================");
        println!("New Market Registered");
        println!("  Block: {}", self.block_number);
        println!("  TX: {}", self.tx_hash);
        println!("  Condition ID: 0x{}", hex::encode(self.condition_id));
        println!("  Token 0 (YES): {}", self.token0);
        println!("  Token 1 (NO):  {}", self.token1);
        println!("=================================");
    }

    /// Get the condition ID as a hex string (with 0x prefix)
    /// Used for querying the Gamma API
    pub fn condition_id_hex(&self) -> String {
        format!("0x{}", hex::encode(self.condition_id))
    }
}

// TODO: Add derive(EthEvent) once we implement event decoding
// TODO: Add methods to decode from Log
// TODO: Add filter creation for querying TokenRegistered events
