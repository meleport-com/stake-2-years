use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Addr, Decimal, Uint128};
use cw_storage_plus::{Item, Map};
use std::fmt;

/// Mappping from staker address to staker balance.
pub const STAKERS_INFO: Map<Addr, StakerInfoResponse> = Map::new("stakers_info_response");

/// Store the owenr info
pub const OWNER_INFO: Item<OwnerInfo> = Item::new("owner_info");

/// Store the whitelist
pub const WHITELIST: Item<Vec<Addr>> = Item::new("whitelist");

#[cw_serde]
pub struct StakerInfoResponse {
    pub staker: Addr, // Address of the staker
    pub amount: Uint128, // How many staked tokens the user has provided.
    pub joined_time: u64, // When the user joined.
    pub expired_time: u64, // When the user's stake will expire.
}

// We difine a struct for Owner
#[cw_serde]
pub struct OwnerInfo {
    pub owner: Addr,
}

impl fmt::Display for OwnerInfo {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.owner)
    }
}