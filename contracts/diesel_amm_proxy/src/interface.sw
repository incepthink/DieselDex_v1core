library;

use std::{
    identity::Identity,
    contract_id::ContractId,
};
use standards::src5::State;

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
    #[storage(read, write)]
    fn initialize(owner: Identity, implementation: ContractId);
    
    #[storage(read)]
    fn proxy_target() -> Option<ContractId>;
    
    #[storage(read)]
    fn proxy_owner() -> State;
    
    #[storage(read)]
    fn get_version() -> u64;
}
