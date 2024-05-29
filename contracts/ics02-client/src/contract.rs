//! This module handles the execution logic of the contract.

use cosmwasm_std::{Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult};

use cw_ibc_lite_types::error::ContractError;

use crate::types::{
    keys,
    msg::{ExecuteMsg, InstantiateMsg, QueryMsg},
};

/// Instantiates a new contract.
///
/// # Errors
/// Will return an error if the instantiation fails.
#[allow(clippy::needless_pass_by_value)]
#[cosmwasm_std::entry_point]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    _msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    cw2::set_contract_version(deps.storage, keys::CONTRACT_NAME, keys::CONTRACT_VERSION)?;
    cw_ownable::initialize_owner(deps.storage, deps.api, Some(info.sender.as_str()))?;

    Ok(Response::default())
}

/// Handles the execution of the contract by routing the messages to the respective handlers.
///
/// # Errors
/// Will return an error if the handler returns an error.
#[allow(clippy::needless_pass_by_value)]
#[cosmwasm_std::entry_point]
pub fn execute(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    cw_ownable::assert_owner(deps.storage, &info.sender)?;

    match msg {
        ExecuteMsg::CreateClient {
            code_id,
            instantiate_msg,
        } => execute::create_client(deps, code_id, instantiate_msg),
        ExecuteMsg::ExecuteClient { client_id, message } => {
            execute::execute_client(deps, client_id, message)
        }
        ExecuteMsg::MigrateClient {
            client_id,
            new_client_id,
        } => execute::migrate_client(deps, client_id, new_client_id),
    }
}

/// Handles the query messages by routing them to the respective handlers.
///
/// # Errors
/// Will return an error if the handler returns an error.
#[allow(clippy::needless_pass_by_value)]
#[cosmwasm_std::entry_point]
pub fn query(_deps: Deps, _env: Env, _msg: QueryMsg) -> StdResult<Binary> {
    unimplemented!()
}

mod execute {
    use super::{ContractError, DepsMut, Response};

    #[allow(clippy::needless_pass_by_value)]
    pub fn create_client(
        _deps: DepsMut,
        _code_id: String,
        _instantiate_msg: cw_ibc_lite_types::clients::InstantiateMsg,
    ) -> Result<Response, ContractError> {
        todo!()
    }

    #[allow(clippy::needless_pass_by_value, clippy::module_name_repetitions)]
    pub fn execute_client(
        _deps: DepsMut,
        _client_id: String,
        _message: cw_ibc_lite_types::clients::ExecuteMsg,
    ) -> Result<Response, ContractError> {
        todo!()
    }

    #[allow(clippy::needless_pass_by_value)]
    pub fn migrate_client(
        _deps: DepsMut,
        _client_id: String,
        _new_client_id: String,
    ) -> Result<Response, ContractError> {
        todo!()
    }
}
