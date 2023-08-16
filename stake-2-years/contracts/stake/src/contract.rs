use std::env;

#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{
    coin, has_coins, to_binary, Addr, BalanceResponse, BankMsg, BankQuery, Binary, Coin, CosmosMsg,
    Decimal, Deps, DepsMut, Env, MessageInfo, QuerierWrapper, QueryRequest, Response, StdError,
    StdResult, SubMsg, Uint128, WasmMsg, WasmQuery, Delegation, StakingMsg,
};

use cw2::set_contract_version;
use cw20::{BalanceResponse as Cw20BalanceResponse, Cw20ExecuteMsg, Cw20QueryMsg};

use crate::{
    error::ContractError,
    msg::{InstantiateMsg, ExecuteMsg}, state::{StakeInfo, STAKE_INFO, STAKERS_INFO}
};

// version info for migration info
const CONTRACT_NAME: &str = "crates.io:stake";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    msg: InstantiateMsg,
) -> StdResult<Response> {
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;

    // Init stake info
    let stake_info = StakeInfo {
        validators_list: msg.validator_list,
    };

    // Save stake info
    STAKE_INFO.save(deps.storage, &stake_info)?;

    Ok(Response::new().add_attributes([
        ("method", "instantiate"),
        ("stake_info", &stake_info.to_string()),
    ]))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::Deposit { validator, amount } => execute_deposit(deps, env, info, validator, amount),
        // ExecuteMsg::Withdraw { amount } => execute_withdraw(deps, env, info, amount),
        // ExecuteMsg::Harvest {} => execute_harvest(deps, env, info),
    }
}

pub fn execute_deposit(
    deps: DepsMut,
    env: Env,
    _info: MessageInfo,
    validator: Addr,
    amount: Uint128,
) -> Result<Response, ContractError> {
    // Check if the staker has enough balance
    // if !has_coins(&info.sent_funds, &[Coin::new(amount.u128(), "uluna")]) {
    //     return Err(ContractError::InsufficientFunds {
    //         amount: amount.u128(),
    //     });
    // }

    // Get the staker's info
    // let staker_info = STAKERS_INFO.load(deps.storage, &info.sender)?;

    // Get the stake info
    let stake_info = STAKE_INFO.load(deps.storage)?;
    let mut res = Response::new();
    // Check if the validator is in the list
    // if !stake_info.validators_list.contains(&validator) {
    //     return Err(ContractError::ValidatorNotInList {});
    // }

    // Get the current time
    let now = env.block.time;

    // Save the staker's info
    // let staker_info = StakerInfoResponse {
    //     staker: info.sender.clone(),
    //     amount: amount,
    //     joined_time: now,
    //     expired_time: now + 2 * 365 * 24 * 60 * 60,
    // };
    // STAKERS_INFO.save(deps.storage, &staker_info)?;

    // Stake to the validator
    let stake = SubMsg::new(CosmosMsg::Staking(StakingMsg::Delegate {
        validator: validator.to_string(),
        amount: coin(amount.u128(), "ueaura"),
    }));


    Ok(res.add_submessage(stake))
}