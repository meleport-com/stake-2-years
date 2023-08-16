use std::env;

#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{
    coin, has_coins, to_binary, Addr, BalanceResponse, BankMsg, BankQuery, Binary, Coin, CosmosMsg,
    Decimal, Deps, DepsMut, Env, MessageInfo, QuerierWrapper, QueryRequest, Response, StdError,
    StdResult, SubMsg, Uint128, WasmMsg, WasmQuery, Delegation, StakingMsg, DistributionMsg,
};

use cw2::set_contract_version;
use cw20::{BalanceResponse as Cw20BalanceResponse, Cw20ExecuteMsg, Cw20QueryMsg};

use crate::{
    error::ContractError,
    msg::{InstantiateMsg, ExecuteMsg, QueryMsg}, state::{ContractInfo, CONTRACT_INFO, STAKERS_INFO, WHITELIST}
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
    let contract_info = ContractInfo {
        owner: info.sender,
        denom: msg.denom,
    };

    // Save stake info
    CONTRACT_INFO.save(deps.storage, &contract_info)?;

    // Save whitelist
    if let Some(whitelist) = msg.whitelist {
        WHITELIST.save(deps.storage, &whitelist)?;
    }

    Ok(Response::new().add_attributes([
        ("method", "instantiate"),
        ("owner", &contract_info.owner.to_string()),
        ("denom", &contract_info.denom),
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
        ExecuteMsg::Delegate { validator, amount } => execute_delegate(deps, env, info, validator, amount),
        ExecuteMsg::Undelegate { validator, amount } => execute_undelegate(deps, env, info, validator, amount),
        ExecuteMsg::WithdrawDelegatorReward { validator } => execute_withdraw_delegator_reward(deps, env, info, validator),
        ExecuteMsg::Claim {} => execute_claim(deps, env, info),
    }
}

pub fn execute_delegate(
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
    let contract_info = CONTRACT_INFO.load(deps.storage)?;
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
        amount: coin(amount.u128(), contract_info.denom),
    }));


    Ok(res.add_submessage(delegate))
}

pub fn execute_undelegate(
    deps: DepsMut,
    env: Env,
    _info: MessageInfo,
    validator: Addr,
    amount: Uint128,
) -> Result<Response, ContractError> {
    // Get the staker's info
    // let staker_info = STAKERS_INFO.load(deps.storage, &info.sender)?;

    // Get the stake info
    let contract_info = CONTRACT_INFO.load(deps.storage)?;
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
        amount: coin(amount.u128(), contract_info.denom),
    }));

    Ok(res.add_submessage(undelegate))
}

pub fn execute_withdraw_delegator_reward(
    deps: DepsMut,
    env: Env,
    _info: MessageInfo,
    validator: Addr,
) -> Result<Response, ContractError> {
    // Get the staker's info
    // let staker_info = STAKERS_INFO.load(deps.storage, &info.sender)?;

    // Get the stake info
    let contract_info = CONTRACT_INFO.load(deps.storage)?;
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

    // Withdraw delegator reward from the validator
    let withdraw_delegator_reward = SubMsg::new(CosmosMsg::Distribution(DistributionMsg::WithdrawDelegatorReward {
        validator: validator.to_string(),
    }));

    Ok(res.add_submessage(withdraw_delegator_reward))
}

pub fn execute_claim(
    deps: DepsMut,
    env: Env,
    _info: MessageInfo,
) -> Result<Response, ContractError> {
    // Get the staker's info
    // let staker_info = STAKERS_INFO.load(deps.storage, &info.sender)?;

    // Get the stake info
    let contract_info = CONTRACT_INFO.load(deps.storage)?;
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

    // Query the balance of the contract
    let balance = deps.querier.query_balance(&env.contract.address, &contract_info.denom)?;

    // Claim rewards by call the bank send message
    let claim = SubMsg::new(CosmosMsg::Bank(BankMsg::Send {
        to_address: contract_info.owner.clone().to_string(),
        amount: vec![coin(balance.amount.into(), contract_info.denom)],
    }));

    Ok(res.add_submessage(claim))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::TotalDelegatorReward { delegator } => Ok(to_binary(&query_total_delegator_reward(deps, delegator.to_string())?)?),
    }
}

pub fn query_total_delegator_reward(deps: Deps, delegator: String) -> Result<Vec<Delegation>, StdError> {

    let res: Vec<Delegation> = deps.querier.query_all_delegations(&delegator)?;

    Ok(res)
}
