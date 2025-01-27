contract;

mod interface;

use std::{
    execution::run_external,
    identity::Identity,
    auth::msg_sender,
    contract_id::ContractId,
};
use standards::src5::State;
use standards::src14::{SRC14, SRC14Extension};
use interface::*;

storage {
    /// The target contract address
    target: ContractId = ContractId::from(0x7bb458adc1d118713319a5baa00a2d049dd64d2916477d2688d76970c898cd55),
    /// The proxy owner state
    owner: State = State::Uninitialized,
    /// Initialization flag
    initialized: bool = false,
}

impl DieselAMMProxy for Contract {
    #[storage(read, write)]
    fn set_proxy_target(new_target: ContractId) {
        // Check owner
        require(
            storage.owner.read() == State::Initialized(msg_sender().unwrap()),
            ProxyError::InvalidOwner
        );

        let old_target = storage.target.read();
        storage.target.write(new_target);

        log(ProxyUpgraded {
            old_implementation: old_target,
            new_implementation: new_target,
            upgraded_by: msg_sender().unwrap(),
        });
    }

    #[storage(read)]
    fn proxy_target() -> Option<ContractId> {
        Some(storage.target.read())
    }

    #[storage(read)]
    fn proxy_owner() -> State {
        storage.owner.read()
    }

    #[storage(read, write)]
    fn initialize(owner: Identity, implementation: ContractId) {
        require(!storage.initialized.read(), ProxyError::AlreadyInitialized);
        
        storage.target.write(implementation);
        storage.owner.write(State::Initialized(owner));
        storage.initialized.write(true);

        log(ProxyInitialized {
            implementation,
            owner,
        });
    }

    #[storage(read)]
    fn get_version() -> u64 {
        1
    }
}

impl SRC14 for Contract {
    #[storage(read, write)]
    fn set_proxy_target(new_target: ContractId) {
        // Check owner
        require(
            storage.owner.read() == State::Initialized(msg_sender().unwrap()),
            ProxyError::InvalidOwner
        );

        let old_target = storage.target.read();
        storage.target.write(new_target);

        log(ProxyUpgraded {
            old_implementation: old_target,
            new_implementation: new_target,
            upgraded_by: msg_sender().unwrap(),
        });
    }

    #[storage(read)]
    fn proxy_target() -> Option<ContractId> {
        Some(storage.target.read())
    }
}

impl SRC14Extension for Contract {
    #[storage(read)]
    fn proxy_owner() -> State {
        storage.owner.read()
    }
}

#[fallback]
#[storage(read)]
fn fallback() {
    require(storage.initialized.read(), ProxyError::NotInitialized);
    run_external(storage.target.read())
}
