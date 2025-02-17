mod functions;
mod utils;

use fuels::{
    prelude::*,
    programs::calls::CallParameters,
};

mod abi_gen;
pub use abi_gen::*;

// Re-export the types
pub use fuels::types::{Asset, AssetId, ContractId, Identity};

// Define PoolMetadata struct if it's not in the ABI
#[derive(Debug, Clone)]
pub struct PoolMetadata {
    // Add fields based on your contract's definition
    pub token_0: ContractId,
    pub token_1: ContractId,
    pub is_stable: bool,
}

// ... rest of your interface functions, but now using the properly imported types ...
