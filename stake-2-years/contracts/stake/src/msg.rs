use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::{Addr, Uint128};

#[cw_serde]
pub struct InstantiateMsg {
    /// whitelist is a list of addresses that are allowed to undelegate or claim rewards
    pub whitelist: Option<Vec<Addr>>,
}

#[cw_serde]
pub enum ExecuteMsg {
    /// Deposit staked tokens and collect reward tokens (if any)
    Deposit {
        validator: Addr,
        amount: Uint128,
    },
    /// Withdraw staked tokens and collect reward tokens (if any)
    Withdraw {
        validator: Addr,
        amount: Uint128,
    },
    // // Harvest reward tokens
    // Harvest {},
}