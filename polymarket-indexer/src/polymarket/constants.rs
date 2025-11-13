// Polymarket Contract Addresses and Constants on Polygon

use ethers::types::{H160, H256};
use std::str::FromStr;

/// CTFExchange contract address (Polygon mainnet) - raw string
///
/// This is the main exchange contract that emits TokenRegistered events
pub const CTF_EXCHANGE_ADDRESS: &str = "0x4bFb41d5B3570DeFd03C39a9A4D8dE6Bd8B8982E";

/// Conditional Tokens Framework contract address (Polygon mainnet) - raw string
pub const CTF_CONTRACT_ADDRESS: &str = "0x4D97DCd97eC945f40cF65F87097ACe5EA0476045";

/// Get CTFExchange address as H160 (parsed)
pub fn ctf_exchange_address() -> H160 {
    H160::from_str(CTF_EXCHANGE_ADDRESS).expect("Invalid CTF_EXCHANGE_ADDRESS constant")
}

/// Get CTF contract address as H160 (parsed)
pub fn ctf_contract_address() -> H160 {
    H160::from_str(CTF_CONTRACT_ADDRESS).expect("Invalid CTF_CONTRACT_ADDRESS constant")
}

/// TokenRegistered event signature: TokenRegistered(uint256,uint256,bytes32)
pub fn token_registered_event_signature() -> H256 {
    let signature = ethers::core::utils::keccak256(b"TokenRegistered(uint256,uint256,bytes32)");
    H256::from(signature)
}
