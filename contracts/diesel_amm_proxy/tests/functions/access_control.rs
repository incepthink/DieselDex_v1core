use super::*;

mod success {
    use super::*;

    #[tokio::test]
    async fn owner_can_access_protected_functions() {
        let context = TestContext::new().await;
        let proxy = context.setup_initialized_proxy().await;

        let result = proxy
            .methods()
            .set_proxy_target(context.implementation_id)
            .call()
            .await;

        assert!(result.is_ok());
    }
}

mod revert {
    use super::*;

    #[tokio::test]
    #[should_panic(expected = "NotInitialized")]
    async fn cannot_access_before_initialization() {
        let context = TestContext::new().await;
        
        context.proxy_instance
            .methods()
            .set_proxy_target(context.implementation_id)
            .call()
            .await
            .unwrap();
    }
}
