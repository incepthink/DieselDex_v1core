use fuels::{
    prelude::*,
    tx::ContractId,
    types::Identity,
};

// Load the contract ABI
abigen!(Contract(
    name = "LPRewardsHook",
    abi = "contracts/lp_rewards_hook/out/debug/lp_rewards_hook-abi.json"
));

// Test setup helper
async fn setup_test() -> (LPRewardsHook<WalletUnlocked>, ContractId, WalletUnlocked) {
    // Launch a local network and deploy the contract
    let wallet = launch_provider_and_get_wallet().await;

    let id = Contract::deploy(
        "./out/debug/lp_rewards_hook.bin",
        &wallet,
        TxParameters::default(),
        StorageConfiguration::default(),
    )
    .await
    .unwrap();

    let instance = LPRewardsHook::new(id.clone(), wallet.clone());

    (instance, id.into(), wallet)
}

async fn launch_provider_and_get_wallet() -> WalletUnlocked {
    let wallet = WalletUnlocked::new_random(None);
    wallet
}

#[tokio::test]
async fn test_mint_and_rewards() {
    let (instance, _id, wallet) = setup_test().await;
    let user = Identity::Address(wallet.address());
    let pool_id = PoolId::default();
    let amount = 100;

    // Test minting
    instance
        .methods()
        .on_mint(user.clone(), pool_id, amount)
        .call()
        .await
        .unwrap();

    // Test getting rewards
    let rewards = instance
        .methods()
        .get_user_rewards(user.clone(), pool_id)
        .call()
        .await
        .unwrap();
    assert_eq!(rewards.value, amount);

    // Test total rewards
    let total = instance
        .methods()
        .get_total_user_rewards(user.clone())
        .call()
        .await
        .unwrap();
    assert_eq!(total.value, amount);

    // Test reward pools
    let pools = instance
        .methods()
        .get_user_reward_pools(user.clone())
        .call()
        .await
        .unwrap();
    assert_eq!(pools.value.len(), 1);
    assert_eq!(pools.value[0], pool_id);
}

#[tokio::test]
async fn test_burn_rewards() {
    let (instance, _id, wallet) = setup_test().await;
    let user = Identity::Address(wallet.address());
    let pool_id = PoolId::default();
    let mint_amount = 100;
    let burn_amount = 30;

    // Initial mint
    instance
        .methods()
        .on_mint(user.clone(), pool_id, mint_amount)
        .call()
        .await
        .unwrap();

    // Test burning
    instance
        .methods()
        .on_burn(user.clone(), pool_id, burn_amount)
        .call()
        .await
        .unwrap();

    // Check remaining rewards
    let rewards = instance
        .methods()
        .get_user_rewards(user.clone(), pool_id)
        .call()
        .await
        .unwrap();
    assert_eq!(rewards.value, mint_amount - burn_amount);
}

#[tokio::test]
async fn test_multiple_pools() {
    let (instance, _id, wallet) = setup_test().await;
    let user = Identity::Address(wallet.address());
    
    // Create two different pool IDs
    let pool_id_1 = PoolId::default();
    let pool_id_2 = PoolId {
        token_0: AssetId::new([2u8; 32]),
        token_1: AssetId::new([3u8; 32]),
        is_stable: false,
    };

    // Add rewards to multiple pools
    instance
        .methods()
        .on_mint(user.clone(), pool_id_1, 100)
        .call()
        .await
        .unwrap();
    instance
        .methods()
        .on_mint(user.clone(), pool_id_2, 200)
        .call()
        .await
        .unwrap();

    // Check individual pool rewards
    let rewards_1 = instance
        .methods()
        .get_user_rewards(user.clone(), pool_id_1)
        .call()
        .await
        .unwrap();
    let rewards_2 = instance
        .methods()
        .get_user_rewards(user.clone(), pool_id_2)
        .call()
        .await
        .unwrap();
    assert_eq!(rewards_1.value, 100);
    assert_eq!(rewards_2.value, 200);

    // Check total rewards
    let total = instance
        .methods()
        .get_total_user_rewards(user.clone())
        .call()
        .await
        .unwrap();
    assert_eq!(total.value, 300);

    // Check reward pools
    let pools = instance
        .methods()
        .get_user_reward_pools(user.clone())
        .call()
        .await
        .unwrap();
    assert_eq!(pools.value.len(), 2);
}

#[tokio::test]
#[should_panic(expected = "Insufficient rewards")]
async fn test_insufficient_rewards() {
    let (instance, _id, wallet) = setup_test().await;
    let user = Identity::Address(wallet.address());
    let pool_id = PoolId::default();

    // Mint initial rewards
    instance
        .methods()
        .on_mint(user.clone(), pool_id, 100)
        .call()
        .await
        .unwrap();

    // Try to burn more than available (should panic)
    instance
        .methods()
        .on_burn(user.clone(), pool_id, 150)
        .call()
        .await
        .unwrap();
}