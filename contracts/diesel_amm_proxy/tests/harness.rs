use fuels::{
    accounts::wallet::WalletUnlocked,
    prelude::*,
    programs::contract::LoadConfiguration,
    types::{ContractId, Identity},
};

// Generate bindings only for the proxy contract
abigen!(Contract(
    name = "DieselAMMProxy",
    abi = "out/debug/diesel_amm_proxy-abi.json"
));

// Import State directly from generated bindings after abigen
use standards::src5::State;

pub struct TestContext {
    pub deployer: WalletUnlocked,
    pub other_wallet: WalletUnlocked,
    pub proxy_instance: DieselAMMProxy<WalletUnlocked>,
    pub implementation_id: ContractId,
}

impl TestContext {
    pub async fn new() -> Self {
        // Setup deployer wallet
        let deployer = WalletUnlocked::new_random(None);
        let other_wallet = WalletUnlocked::new_random(None);
        
        // Deploy implementation first
        let implementation_id = Contract::load_from(
            "../diesel_amm_contract/out/debug/diesel_amm_contract.bin",
            LoadConfiguration::default()
        )
        .unwrap()
        .deploy(&deployer, TxPolicies::default())
        .await
        .unwrap()
        .into();

        // Deploy proxy
        let proxy_id = Contract::load_from(
            "./out/debug/diesel_amm_proxy.bin",
            LoadConfiguration::default()
        )
        .unwrap()
        .deploy(&deployer, TxPolicies::default())
        .await
        .unwrap();

        let proxy_instance = DieselAMMProxy::new(proxy_id.clone(), deployer.clone());

        Self {
            deployer,
            other_wallet,
            proxy_instance,
            implementation_id,
        }
    }

    pub async fn setup_initialized_proxy(&self) -> DieselAMMProxy<WalletUnlocked> {
        let call_params = CallParameters::default();
        
        self.proxy_instance
            .methods()
            .initialize(
                Identity::Address(self.deployer.address().into()),
                self.implementation_id,
            )
            .call_params(call_params)
            .unwrap()
            .call()
            .await
            .unwrap();

        self.proxy_instance.clone()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[tokio::test]
    async fn test_proxy_initialization() {
        let context = TestContext::new().await;
        let proxy = context.setup_initialized_proxy().await;

        let target = proxy
            .methods()
            .proxy_target()
            .call()
            .await
            .unwrap()
            .value;

        assert_eq!(target, Some(context.implementation_id));

        let version = proxy
            .methods()
            .get_version()
            .call()
            .await
            .unwrap()
            .value;

        assert_eq!(version, 1);
    }

    #[tokio::test]
    async fn test_proxy_owner() {
        let context = TestContext::new().await;
        let proxy = context.setup_initialized_proxy().await;

        let owner = proxy
            .methods()
            .proxy_owner()
            .call()
            .await
            .unwrap()
            .value;

        let expected_owner = Identity::Address(context.deployer.address().into());
        assert_eq!(owner, State::Initialized(expected_owner));
    }

    #[tokio::test]
    #[should_panic(expected = "AlreadyInitialized")]
    async fn test_cannot_initialize_twice() {
        let context = TestContext::new().await;
        let proxy = context.setup_initialized_proxy().await;

        // Try to initialize again
        proxy
            .methods()
            .initialize(
                Identity::Address(context.deployer.address().into()),
                context.implementation_id,
            )
            .call()
            .await
            .unwrap();
    }

    #[tokio::test]
    async fn test_proxy_upgrade() {
        let context = TestContext::new().await;
        let proxy = context.setup_initialized_proxy().await;

        // Deploy new implementation
        let new_implementation_id = Contract::load_from(
            "../diesel_amm_contract/out/debug/diesel_amm_contract.bin",
            LoadConfiguration::default()
        )
        .unwrap()
        .deploy(&context.deployer, TxPolicies::default())
        .await
        .unwrap()
        .into();

        // Upgrade proxy
        proxy
            .methods()
            .set_proxy_target(new_implementation_id)
            .call()
            .await
            .unwrap();

        let target = proxy
            .methods()
            .proxy_target()
            .call()
            .await
            .unwrap()
            .value;

        assert_eq!(target, Some(new_implementation_id));
    }

    #[tokio::test]
    #[should_panic(expected = "InvalidOwner")]
    async fn test_non_owner_cannot_upgrade() {
        let context = TestContext::new().await;
        let proxy = context.setup_initialized_proxy().await;

        // Create non-owner instance of proxy
        let non_owner_proxy = DieselAMMProxy::new(
            proxy.contract_id().clone(),
            context.other_wallet.clone(),
        );

        // Try to upgrade (should fail)
        non_owner_proxy
            .methods()
            .set_proxy_target(context.implementation_id)
            .call()
            .await
            .unwrap();
    }

    #[tokio::test]
    async fn test_uninitialized_state() {
        let context = TestContext::new().await;
        
        // Check target before initialization
        let target = context.proxy_instance
            .methods()
            .proxy_target()
            .call()
            .await
            .unwrap()
            .value;

        assert_eq!(target, None);

        // Check owner before initialization
        let owner = context.proxy_instance
            .methods()
            .proxy_owner()
            .call()
            .await
            .unwrap()
            .value;

        assert_eq!(owner, State::Uninitialized);
    }
}
