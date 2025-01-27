use fuels::prelude::*;
use crate::harness::TestContext;

pub async fn setup_proxy_with_implementation(
    deployer: &WalletUnlocked,
) -> (DieselAMMProxy<WalletUnlocked>, ContractId) {
    // Deploy implementation
    let implementation_id = Contract::load_from(
        "../diesel_amm_implementation/out/debug/diesel_amm_implementation.bin",
        LoadConfiguration::default()
    )
    .unwrap()
    .deploy(deployer, TxParameters::default())
    .await
    .unwrap()
    .into();

    // Deploy proxy
    let proxy_id = Contract::load_from(
        "./out/debug/diesel_amm_proxy.bin",
        LoadConfiguration::default()
    )
    .unwrap()
    .deploy(deployer, TxParameters::default())
    .await
    .unwrap();

    let proxy_instance = DieselAMMProxy::new(proxy_id, deployer.clone());

    (proxy_instance, implementation_id)
}

pub async fn setup_initialized_context() -> TestContext {
    let context = TestContext::new().await;
    context.setup_initialized_proxy().await;
    context
}
