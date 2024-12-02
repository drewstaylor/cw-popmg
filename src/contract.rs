#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{to_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult};
use cw2::set_contract_version;

use crate::error::ContractError;
use crate::execute::{execute_add_secret, execute_prove};
use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg};
use crate::query::{query_details, query_secret_ids};
use crate::state::{Config, CONFIG};

// version info for migration info
const CONTRACT_NAME: &str = "popmg";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    let state = Config {
        owner: info.sender.clone(),
        nft_contract: msg.cw721.clone(),
        denom: msg.denom.clone(),
    };
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;
    CONFIG.save(deps.storage, &state)?;

    Ok(Response::new()
        .add_attribute("method", "instantiate")
        .add_attribute("owner", info.sender.to_string())
        .add_attribute("nft_contract", msg.cw721.to_string())
        .add_attribute("denom", msg.denom))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::AddSecret(msg) => execute_add_secret(deps, env, info, msg),
        ExecuteMsg::SubmitProof(msg) => execute_prove(deps, env, info, msg),
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::SecretIds { start_after, limit } => {
            to_binary(&query_secret_ids(deps, start_after, limit)?)
        }
        QueryMsg::Details { id } => to_binary(&query_details(deps, id)?),
    }
}
