contract;

use std::{
    storage::StorageMap,
    vec::Vec,
};
use interfaces::{
    lp_rewards_hook::LPRewardsHook,
    data_structures::PoolId,
};

storage {
    /// Tracks LP rewards for each user per pool
    lp_rewards: StorageMap<(Identity, PoolId), u64> = StorageMap {},
    /// Tracks all pools where a user has had LP tokens
    user_pools: StorageMap<Identity, Vec<PoolId>> = StorageMap {},
}

impl LPRewardsHook for Contract {
    #[storage(read, write)]
    fn on_mint(user: Identity, pool_id: PoolId, amount: u64) {
        let current = storage.lp_rewards.get((user, pool_id)).try_read().unwrap_or(0);
        storage.lp_rewards.insert((user, pool_id), current + amount);
        
        // Track pool for user if not already tracked
        let mut user_pools = storage.user_pools.get(user).try_read().unwrap_or(Vec::new());
        if !user_pools.contains(pool_id) {
            user_pools.push(pool_id);
            storage.user_pools.insert(user, user_pools);
        }
    }

    #[storage(read, write)]
    fn on_burn(user: Identity, pool_id: PoolId, amount: u64) {
        let current = storage.lp_rewards.get((user, pool_id)).try_read().unwrap_or(0);
        require(current >= amount, "Insufficient rewards");
        storage.lp_rewards.insert((user, pool_id), current - amount);
    }

    #[storage(read)]
    fn get_user_rewards(user: Identity, pool_id: PoolId) -> u64 {
        storage.lp_rewards.get((user, pool_id)).try_read().unwrap_or(0)
    }

    #[storage(read)]
    fn get_total_user_rewards(user: Identity) -> u64 {
        let mut total = 0;
        if let Some(pools) = storage.user_pools.get(user).try_read() {
            let mut i = 0;
            while i < pools.len() {
                if let Some(reward) = storage.lp_rewards.get((user, pools.get(i).unwrap())).try_read() {
                    total += reward;
                }
                i += 1;
            }
        }
        total
    }

    #[storage(read)]
    fn get_user_reward_pools(user: Identity) -> Vec<PoolId> {
        storage.user_pools.get(user).try_read().unwrap_or(Vec::new())
    }
}