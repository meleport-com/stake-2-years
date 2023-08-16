use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::{Addr, Uint128};

#[cw_serde]
pub struct InstantiateMsg {
    /// The list of validators to stake on
    pub validator_list: Vec<Addr>,
}

#[cw_serde]
pub enum ExecuteMsg {
    /// Deposit staked tokens and collect reward tokens (if any)
    Deposit {
        validator: Addr,
        amount: Uint128,
    },
    // /// Withdraw staked tokens and collect reward tokens (if any)
    // Withdraw {
    //     amount: Uint128,
    // },
    // // Harvest reward tokens
    // Harvest {},
}