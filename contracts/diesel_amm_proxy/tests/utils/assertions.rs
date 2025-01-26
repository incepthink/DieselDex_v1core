use fuels::prelude::*;

pub async fn assert_proxy_upgrade_success(
    proxy: &DieselAMMProxy<WalletUnlocked>,
    old_implementation: ContractId,
    new_implementation: ContractId,
) {
    let response = proxy
        .methods()
        .set_proxy_target(new_implementation)
        .call()
        .await
        .unwrap();

    // Check event emission
    let logs = response.logs();
    let upgrade_event = logs
        .iter()
        .find(|log| log.event_name == "ProxyUpgraded")
        .expect("ProxyUpgraded event not found");

    // Verify target was updated
    let target = proxy
        .methods()
        .proxy_target()
        .call()
        .await
        .unwrap()
        .value;
    assert_eq!(target, Some(new_implementation));
}

pub async fn assert_initialization_success(
    proxy: &DieselAMMProxy<WalletUnlocked>,
    owner: Identity,
    implementation: ContractId,
) {
    let response = proxy
        .methods()
        .initialize(owner, implementation)
        .call()
        .await
        .unwrap();

    // Check event emission
    let logs = response.logs();
    assert!(
        logs.iter().any(|log| log.event_name == "ProxyInitialized"),
        "ProxyInitialized event not found"
    );

    // Verify state
    let actual_target = proxy
        .methods()
        .proxy_target()
        .call()
        .await
        .unwrap()
        .value;
    assert_eq!(actual_target, Some(implementation));

    let actual_owner = proxy
        .methods()
        .proxy_owner()
        .call()
        .await
        .unwrap()
        .value;
    assert_eq!(actual_owner, State::Initialized(owner));
}
