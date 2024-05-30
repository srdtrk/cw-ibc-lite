//! This module handles the execution logic of the contract.

use cosmwasm_std::{Binary, Deps, DepsMut, Env, MessageInfo, Response};
use cw_ibc_lite_types::error::ContractError;

use crate::types::{
    keys,
    msg::{ExecuteMsg, InstantiateMsg, QueryMsg},
    state,
};

/// Tendermint context for ibc-rs.
pub type TendermintContext<'a> = ibc_client_cw::context::Context<'a, state::TendermintClient>;

/// Instantiates a new contract.
///
/// # Errors
/// Will return an error if the instantiation fails.
#[allow(clippy::needless_pass_by_value)]
#[cosmwasm_std::entry_point]
pub fn instantiate(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    cw2::set_contract_version(deps.storage, keys::CONTRACT_NAME, keys::CONTRACT_VERSION)?;
    cw_ownable::initialize_owner(deps.storage, deps.api, Some(info.sender.as_str()))?;

    let mut ctx = TendermintContext::new_mut(deps, env)?;
    let data = ctx.instantiate(msg.into())?;

    Ok(Response::new().set_data(data))
}

/// Handles the execution of the contract by routing the messages to the respective handlers.
///
/// # Errors
/// Will return an error if the handler returns an error.
#[allow(clippy::needless_pass_by_value)]
#[cosmwasm_std::entry_point]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    cw_ownable::assert_owner(deps.storage, &info.sender)?;

    let mut ctx = TendermintContext::new_mut(deps, env)?;
    let data = ctx.sudo(msg.into())?;

    Ok(Response::new().set_data(data))
}

/// Handles the query messages by routing them to the respective handlers.
///
/// # Errors
/// Will return an error if the handler returns an error.
#[allow(clippy::needless_pass_by_value)]
#[cosmwasm_std::entry_point]
pub fn query(deps: Deps, env: Env, msg: QueryMsg) -> Result<Binary, ContractError> {
    let ctx = TendermintContext::new_ref(deps, env)?;
    ctx.query(msg.into()).map_err(ContractError::from)
}
