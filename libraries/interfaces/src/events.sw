library;

use ::data_structures::{Asset, PoolId};

pub struct CreatePoolEvent {
    pub pool_id: PoolId,
    pub decimals_0: u8,
    pub decimals_1: u8,
}

pub struct MintEvent {
    pub pool_id: PoolId,
    pub recipient: Identity,
    pub liquidity: Asset,
    pub asset_0_in: u64,
    pub asset_1_in: u64,
}
pub struct LPRewardEvent {
    user: Identity,
    pool_id: PoolId,
    amount: u64,
    is_add: bool,
    new_balance: u64,
}

// impl LPRewardEvent {
//     pub fn new(user: Identity, pool_id: PoolId, amount: u64, is_add: bool, new_balance: u64) -> Self {
//         Self {
//             user,
//             pool_id,
//             amount,
//             is_add,
//             new_balance,
//         }
//     }
// }

// Helper function to log the event
pub fn log_lp_reward_event(user: Identity, pool_id: PoolId, amount: u64, is_add: bool, new_balance: u64) {
    log(LPRewardEvent {
        user,
        pool_id,
        amount,
        is_add,
        new_balance,
    });
}
pub struct BurnEvent {
    pub pool_id: PoolId,
    pub recipient: Identity,
    pub liquidity: Asset,
    pub asset_0_out: u64,
    pub asset_1_out: u64,
}

pub struct SwapEvent {
    pub pool_id: PoolId,
    pub recipient: Identity,
    pub asset_0_in: u64,
    pub asset_1_in: u64,
    pub asset_0_out: u64,
    pub asset_1_out: u64,
}
