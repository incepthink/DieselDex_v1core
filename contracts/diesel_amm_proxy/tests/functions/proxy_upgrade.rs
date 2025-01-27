use super::*;

mod success {
    use super::*;

    #[tokio::test]
    async fn upgrades_implementation() {
        let context = TestContext::new().await;
        let proxy = context.setup_initialized_proxy().await;

        // Deploy new implementation
        let new_implementation = Contract::load_from(
            "../diesel_amm_implementation/out/debug/diesel_amm_implementation.bin",
            LoadConfiguration::default()
        )
        .unwrap()
        .deploy(&context.deployer, TxParameters::default())
        .await
        .unwrap();

        // Upgrade proxy
        proxy
            .methods()
            .set_proxy_target(new_implementation.clone().into())
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

        assert_eq!(target, Some(ContractId::from(new_implementation)));
    }
}

mod revert {
    use super::*;

    #[tokio::test]
    #[should_panic(expected = "InvalidOwner")]
    async fn non_owner_cannot_upgrade() {
        let context = TestContext::new().await;
        let proxy = context.setup_initialized_proxy().await;

        // Create non-owner wallet
        let non_owner = WalletUnlocked::new_random(None);
        let non_owner_proxy = proxy.with_wallet(non_owner);

        non_owner_proxy
            .methods()
            .set_proxy_target(context.implementation_id)
            .call()
            .await
            .unwrap();
    }
}
