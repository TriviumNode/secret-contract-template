use cosmwasm_std::{
    entry_point, to_binary, Addr, Api, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdError,
    Storage,
};

use secret_toolkit::viewing_key::{ViewingKey, ViewingKeyStore};

use crate::error::ContractError;
use crate::msg::{ExecuteMsg, InstantiateMsg, QueryAnswer, QueryMsg};
use crate::state::{Config, CONFIG_KEY};

#[entry_point]
pub fn instantiate(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    let config = Config { owner: info.sender };

    // Save data to storage
    CONFIG_KEY.save(deps.storage, &config)?;

    Ok(Response::new())
}

//-------------------------------------------- HANDLES ---------------------------------

#[entry_point]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::CreateViewingKey { entropy } => try_create_key(deps, env, info, entropy),
        ExecuteMsg::SetViewingKey { key, .. } => try_set_key(deps, info, &key),
    }
}

/// Returns Result<Response, ContractError>
///
/// create a viewing key
///
/// # Arguments
///
/// * `deps`    - DepsMut containing all the contract's external dependencies
/// * `env`     - Env of contract's environment
/// * `info`    - Carries the info of who sent the message and how much native funds were sent along
/// * `entropy` - string to be used as an entropy source for randomization
fn try_create_key(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    entropy: String,
) -> Result<Response, ContractError> {
    let key = ViewingKey::create(
        deps.storage,
        &info,
        &env,
        info.sender.as_str(),
        entropy.as_bytes(),
    );

    Ok(Response::new().add_attribute("viewing_key", key))
}

/// Returns Result<Response, ContractError>
///
/// sets the viewing key
///
/// # Arguments
///
/// * `deps` - DepsMut containing all the contract's external dependencies
/// * `info` - Carries the info of who sent the message and how much native funds were sent along
/// * `key`  - string slice to be used as the viewing key
fn try_set_key(deps: DepsMut, info: MessageInfo, key: &str) -> Result<Response, ContractError> {
    ViewingKey::set(deps.storage, info.sender.as_str(), key);

    Ok(Response::new().add_attribute("viewing_key", key))
}

// ---------------------------------------- QUERIES --------------------------------------

#[entry_point]
pub fn query(deps: Deps, msg: QueryMsg) -> Result<Binary, ContractError> {
    match msg {
        QueryMsg::QueryEx {} => query_ex(deps),
    }
}

fn query_ex(deps: Deps) -> Result<Binary, ContractError> {
    Ok(to_binary(&QueryAnswer::QueryExAns {})?)
}

//----------------------------------------- Helper functions----------------------------------

/// Returns bool result of validating an address' viewing key
///
/// # Arguments
///
/// * `storage`     - a reference to the contract's storage
/// * `account`     - a reference to the str whose key should be validated
/// * `viewing_key` - String key used for authentication
fn is_key_valid(storage: &dyn Storage, account: &str, viewing_key: String) -> bool {
    ViewingKey::check(storage, account, &viewing_key).is_ok()
}
