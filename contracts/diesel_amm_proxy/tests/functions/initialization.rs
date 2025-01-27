use super::*;
use fuels::types::Identity;

mod success {
    use super::*;

    #[tokio::test]
    async fn initializes_proxy() {
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
    }

    #[tokio::test]
    async fn sets_correct_owner() {
        let context = TestContext::new().await;
        let proxy = context.setup_initialized_proxy().await;

        let owner = proxy
            .methods()
            .proxy_owner()
            .call()
            .await
            .unwrap()
            .value;

        assert_eq!(
            owner,
            State::Initialized(Identity::Address(context.deployer.address()))
        );
    }
}

mod revert {
    use super::*;

    #[tokio::test]
    #[should_panic(expected = "AlreadyInitialized")]
    async fn cannot_initialize_twice() {
        let context = TestContext::new().await;
        let proxy = context.setup_initialized_proxy().await;

        // Try to initialize again
        proxy
            .methods()
            .initialize(
                context.deployer.address(),
                context.implementation_id.into()
            )
            .call()
            .await
            .unwrap();
    }
}
