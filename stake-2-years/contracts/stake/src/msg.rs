use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::{Addr, Uint128, DistributionQuery, Delegation, FullDelegation};

#[cw_serde]
pub struct InstantiateMsg {
    /// whitelist is a list of addresses that are allowed to undelegate or claim rewards
    pub whitelist: Option<Vec<Addr>>,
    pub denom: String,
}

#[cw_serde]
pub enum ExecuteMsg {
    /// Delegate staked tokens and collect reward tokens (if any)
    Delegate {
        validator: Addr,
        amount: Uint128,
    },
    /// Undelegate staked tokens and collect reward tokens (if any)
    Undelegate {
        validator: Addr,
        amount: Uint128,
    },
    // WithdrawDelegatorReward reward tokens
    WithdrawDelegatorReward {
        validator: Addr,
    },
    // Claim all tokens from the contract to the owner
    Claim {},
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    #[returns(Delegation)]
    Delegation {
        delegator: Addr,
    },
    #[returns(FullDelegation)]
    FullDelegation {
        delegator: Addr,
        validator: Addr,
    },
}