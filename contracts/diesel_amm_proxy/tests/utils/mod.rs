use fuels::prelude::*;

pub struct TestHelper;

impl TestHelper {
    pub async fn assert_proxy_state(
        proxy: &DieselAMMProxy<WalletUnlocked>,
        expected_target: ContractId,
        expected_owner: Identity,
    ) {
        let target = proxy
            .methods()
            .proxy_target()
            .call()
            .await
            .unwrap()
            .value;
        assert_eq!(target, Some(expected_target));

        let owner = proxy
            .methods()
            .proxy_owner()
            .call()
            .await
            .unwrap()
            .value;
        assert_eq!(owner, State::Initialized(expected_owner));
    }

    pub async fn assert_event_emission(
        response: &FuelCallResponse<()>,
        expected_event_name: &str,
    ) {
        let logs = response.logs();
        assert!(
            logs.iter().any(|log| log.event_name == expected_event_name),
            "Expected event {} was not emitted",
            expected_event_name
        );
    }

    pub fn setup_test_wallet(
        initial_balance: u64,
        asset_id: Option<AssetId>,
    ) -> WalletUnlocked {
        let mut wallet = WalletUnlocked::new_random(None);
        if let Some(asset) = asset_id {
            wallet.add_asset(asset, initial_balance);
        }
        wallet
    }
}

pub struct Constants;

impl Constants {
    pub const INITIAL_BALANCE: u64 = 1_000_000;
    pub const ZERO_AMOUNT: u64 = 0;
    pub const MIN_AMOUNT: u64 = 1_000;
}
