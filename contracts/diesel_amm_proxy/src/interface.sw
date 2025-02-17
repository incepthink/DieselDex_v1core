library;

use std::{
    identity::Identity,
    contract_id::ContractId,
};
use standards::src5::State;
use standards::src14::{SRC14, SRC14Extension};

// Events
pub struct ProxyUpgraded {
    pub old_implementation: ContractId,
    pub new_implementation: ContractId,
    pub upgraded_by: Identity,
}

pub struct ProxyInitialized {
    pub implementation: ContractId,
    pub owner: Identity,
}

// Errors
pub enum ProxyError {
    NotInitialized: (),
    AlreadyInitialized: (),
    InvalidOwner: (),
}

// Core proxy interface
abi DieselAMMProxy {
    /// Initializes the proxy with an owner and implementation contract.
    #[storage(read, write)]
    fn initialize(owner: Identity, implementation: ContractId);

    /// Returns the version of the proxy contract.
    #[storage(read)]
    fn get_version() -> u64;
}