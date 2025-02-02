library;

use ::data_structures::{PoolId};

abi LPRewardsHook {
    /// Called when LP tokens are minted
    #[storage(read, write)]
    fn on_mint(user: Identity, pool_id: PoolId, amount: u64);

    /// Called when LP tokens are burned
    #[storage(read, write)]
    fn on_burn(user: Identity, pool_id: PoolId, amount: u64);

    /// Get rewards for a specific user and pool
    #[storage(read)]
    fn get_user_rewards(user: Identity, pool_id: PoolId) -> u64;

    /// Get total rewards across all pools for a user
    #[storage(read)]
    fn get_total_user_rewards(user: Identity) -> u64;

    /// Get all pools where user has rewards
    #[storage(read)]
    fn get_user_reward_pools(user: Identity) -> Vec<PoolId>;
}