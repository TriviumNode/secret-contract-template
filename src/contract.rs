use cosmwasm_std::{
    entry_point, to_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response, Storage,
};

use secret_toolkit::permit::{validate, Permit, RevokedPermits, TokenPermissions};
use secret_toolkit::utils::{pad_handle_result, pad_query_result};
use secret_toolkit::viewing_key::{ViewingKey, ViewingKeyStore};

use crate::error::ContractError;
use crate::msg::{ExecuteMsg, InstantiateMsg, QueryAnswer, QueryMsg, QueryWithPermit};
use crate::state::{Config, BLOCK_SIZE, CONFIG_KEY, PREFIX_REVOKED_PERMITS};

#[entry_point]
pub fn instantiate(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    _msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    let config = Config {
        owner: info.sender,
        contract_address: env.contract.address,
    };

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
    let res = match msg {
        ExecuteMsg::CreateViewingKey { entropy } => try_create_key(deps, env, info, entropy),
        ExecuteMsg::SetViewingKey { key, .. } => try_set_key(deps, info, &key),
        ExecuteMsg::RevokePermit { permit_name, .. } => revoke_permit(deps, env, info, permit_name),
    };

    pad_handle_result(res, BLOCK_SIZE)
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

fn revoke_permit(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    permit_name: String,
) -> Result<Response, ContractError> {
    RevokedPermits::revoke_permit(
        deps.storage,
        PREFIX_REVOKED_PERMITS,
        info.sender.as_ref(),
        &permit_name,
    );

    Ok(Response::new())
}

// ---------------------------------------- QUERIES --------------------------------------

#[entry_point]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> Result<Binary, ContractError> {
    let res = match msg {
        QueryMsg::QueryEx {} => query_ex(deps),
        QueryMsg::WithPermit { permit, query } => permit_queries(deps, permit, query),
        _ => viewing_keys_queries(deps, msg),
    };
    pad_query_result(res, BLOCK_SIZE)
}

/// Returns QueryResult from validating a permit and then using its creator's address when
/// performing the specified query
///
/// # Arguments
///
/// * `deps` - a reference to Extern containing all the contract's external dependencies
/// * `permit` - the permit used to authentic the query
/// * `query` - the query to perform
fn permit_queries(
    deps: Deps,
    permit: Permit,
    query: QueryWithPermit,
) -> Result<Binary, ContractError> {
    // Validate permit content
    let config = CONFIG_KEY.load(deps.storage)?;

    let viewer = validate(
        deps,
        PREFIX_REVOKED_PERMITS,
        &permit,
        config.contract_address.to_string(),
        None,
    )?;

    // Permit validated! We can now execute the query.
    match query {
        QueryWithPermit::Permissioned {} => {
            if !permit.check_permission(&TokenPermissions::Balance) {
                return Err(ContractError::Unauthorized {});
            }

            query_permissioned(deps, viewer)
        }
    }
}

pub fn viewing_keys_queries(deps: Deps, msg: QueryMsg) -> Result<Binary, ContractError> {
    let (address, key) = msg.get_validation_params();

    if !is_key_valid(deps.storage, &address, key) {
        Err(ContractError::Unauthorized {})
    } else {
        match msg {
            // Base
            QueryMsg::Permissioned { viewer, key: _ } => query_permissioned(deps, viewer),

            _ => panic!("This query type does not require authentication"),
        }
    }
}

fn query_ex(_deps: Deps) -> Result<Binary, ContractError> {
    Ok(to_binary(&QueryAnswer::QueryExAns {})?)
}

fn query_permissioned(_deps: Deps, _viewer: String) -> Result<Binary, ContractError> {
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
