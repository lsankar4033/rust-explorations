// TokenRegistered Event Definition and Processing
//
// Event: TokenRegistered(uint256 indexed token0, uint256 indexed token1, bytes32 indexed conditionId)
// Emitted by: CTFExchange contract

use crate::polymarket::constants::CTF_EXCHANGE_ADDRESS;
use ethers::types::{Address, U256};
use std::str::FromStr;

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
}

/// Get the CTFExchange contract address as an Address type
pub fn ctf_exchange_address() -> Address {
    Address::from_str(CTF_EXCHANGE_ADDRESS).expect("Invalid CTF_EXCHANGE_ADDRESS constant")
}

// TODO: Add derive(EthEvent) once we implement event decoding
// TODO: Add methods to decode from Log
// TODO: Add filter creation for querying TokenRegistered events
