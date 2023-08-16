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
    msg::{InstantiateMsg, ExecuteMsg}, state::{OwnerInfo, OWNER_INFO, STAKERS_INFO, WHITELIST}
};

// version info for migration info
const CONTRACT_NAME: &str = "crates.io:stake";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> StdResult<Response> {
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;

    // Init stake info
    let owner = OwnerInfo {
        owner: info.sender,
    };

    // Save stake info
    OWNER_INFO.save(deps.storage, &owner)?;

    // Save whitelist
    if let Some(whitelist) = msg.whitelist {
        WHITELIST.save(deps.storage, &whitelist)?;
    }

    Ok(Response::new().add_attributes([
        ("method", "instantiate"),
        ("owner", &owner.to_string()),
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
        ExecuteMsg::Withdraw { validator, amount } => execute_withdraw(deps, env, info, validator, amount),
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
    let stake_info = OWNER_INFO.load(deps.storage)?;
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
    let delegate = SubMsg::new(CosmosMsg::Staking(StakingMsg::Delegate {
        validator: validator.to_string(),
        amount: coin(amount.u128(), "ueaura"),
    }));


    Ok(res.add_submessage(delegate))
}

pub fn execute_withdraw(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    validator: Addr,
    amount: Uint128,
) -> Result<Response, ContractError> {
    // Get the staker's info
    // let staker_info = STAKERS_INFO.load(deps.storage, &info.sender)?;

    // Get the stake info
    let stake_info = OWNER_INFO.load(deps.storage)?;
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

    // Undelegate from the validator
    let undelegate = SubMsg::new(CosmosMsg::Staking(StakingMsg::Undelegate {
        validator: validator.to_string(),
        amount: coin(amount.u128(), "ueaura"),
    }));

    // Tranfer all the staked tokens to the staker
    let transfer = SubMsg::new(CosmosMsg::Bank(BankMsg::Send {
        to_address: info.sender.to_string(),
        amount: vec![coin(amount.u128(), "ueaura")],
    }));

    Ok(res.add_submessage(undelegate)
            .add_submessage(transfer))
}