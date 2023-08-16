use std::env;

#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{
    coin, has_coins, to_binary, Addr, BalanceResponse, BankMsg, BankQuery, Binary, Coin, CosmosMsg,
    Decimal, Deps, DepsMut, Env, MessageInfo, QuerierWrapper, QueryRequest, Response, StdError,
    StdResult, SubMsg, Uint128, WasmMsg, WasmQuery,
};

use cw2::set_contract_version;
use cw20::{BalanceResponse as Cw20BalanceResponse, Cw20ExecuteMsg, Cw20QueryMsg};

use crate::{msg::InstantiateMsg, state::{StakeInfo, STAKE_INFO}};

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