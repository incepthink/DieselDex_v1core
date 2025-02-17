
use fuels::types::AssetId;

#[derive(Debug, Clone)]
pub struct PoolMetadata {
    pub token_0: AssetId,
    pub token_1: AssetId,
    pub is_stable: bool,
}

pub type PoolId = (AssetId, AssetId, bool);