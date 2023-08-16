use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Addr, Decimal, Uint128};
use cw_storage_plus::{Item, Map};
use std::fmt;

/// Mappping from staker address to staker balance.
pub const STAKERS_INFO: Map<Addr, StakerInfoResponse> = Map::new("stakers_info_response");

/// Store the stake info
/// This is the list of validators to stake on
pub const STAKE_INFO: Item<StakeInfo> = Item::new("stake_info");

#[cw_serde]
pub struct StakerInfoResponse {
    pub staker: Addr, // Address of the staker
    pub amount: Uint128, // How many staked tokens the user has provided.
    pub joined_time: u64, // When the user joined.
    pub expired_time: u64, // When the user's stake will expire.
}

// We difine a struct for StakeInfo
#[cw_serde]
pub struct StakeInfo {
    pub validators_list: Vec<Addr>,
}

impl fmt::Display for StakeInfo {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "validators_list: [")?;
        for validator in &self.validators_list {
            write!(f, "{}, ", validator)?;
        }
        write!(f, "]")
    }
}