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
    // NOTE: [`QueryMsg::VerifyMembership`] and [`QueryMsg::VerifyNonMembership`] are handled by
    // the sudo handler in ibc_client_cw. This is beacuse initial ibc specifications required this.
    // In cw-ibc-lite, we moved these messages to the query handler, this is why we need to handle
    // verify membership and non-membership queries in a different way.
    match msg {
        QueryMsg::Status(_) => query::status(deps, env, msg.try_into()?),
        QueryMsg::ExportMetadata(_) => query::export_metadata(deps, env, msg.try_into()?),
        QueryMsg::TimestampAtHeight(_) => query::timestamp_at_height(deps, env, msg.try_into()?),
        QueryMsg::VerifyClientMessage(_) => {
            query::verify_client_message(deps, env, msg.try_into()?)
        }
        QueryMsg::CheckForMisbehaviour(_) => {
            query::check_for_misbehaviour(deps, env, msg.try_into()?)
        }
        QueryMsg::VerifyMembership(_) | QueryMsg::VerifyNonMembership(_) => {
            query::execute_query(deps, env, msg.try_into()?)
        }
    }
}

mod query {
    use cw_ibc_lite_types::{clients::msg::query_responses, storage::mock_mut::MockMutStorage};

    use ibc_client_cw::types::{QueryMsg as TendermintQueryMsg, QueryResponse};

    use super::{Binary, ContractError, Deps, Env, TendermintContext};

    pub fn status(deps: Deps, env: Env, msg: TendermintQueryMsg) -> Result<Binary, ContractError> {
        tendermint_query(deps, env, msg)
            .and_then(query_responses::Status::try_from)
            .and_then(|qr| cosmwasm_std::to_json_binary(&qr).map_err(ContractError::from))
    }

    pub fn export_metadata(
        deps: Deps,
        env: Env,
        msg: TendermintQueryMsg,
    ) -> Result<Binary, ContractError> {
        tendermint_query(deps, env, msg)
            .and_then(query_responses::ExportMetadata::try_from)
            .and_then(|qr| cosmwasm_std::to_json_binary(&qr).map_err(ContractError::from))
    }

    pub fn timestamp_at_height(
        deps: Deps,
        env: Env,
        msg: TendermintQueryMsg,
    ) -> Result<Binary, ContractError> {
        tendermint_query(deps, env, msg)
            .and_then(query_responses::TimestampAtHeight::try_from)
            .and_then(|qr| cosmwasm_std::to_json_binary(&qr).map_err(ContractError::from))
    }

    pub fn verify_client_message(
        deps: Deps,
        env: Env,
        msg: TendermintQueryMsg,
    ) -> Result<Binary, ContractError> {
        tendermint_query(deps, env, msg)
            .and_then(query_responses::VerifyClientMessage::try_from)
            .and_then(|qr| cosmwasm_std::to_json_binary(&qr).map_err(ContractError::from))
    }

    pub fn check_for_misbehaviour(
        deps: Deps,
        env: Env,
        msg: TendermintQueryMsg,
    ) -> Result<Binary, ContractError> {
        tendermint_query(deps, env, msg)
            .and_then(query_responses::CheckForMisbehaviour::try_from)
            .and_then(|qr| cosmwasm_std::to_json_binary(&qr).map_err(ContractError::from))
    }

    fn tendermint_query(
        deps: Deps,
        env: Env,
        msg: TendermintQueryMsg,
    ) -> Result<QueryResponse, ContractError> {
        let ctx = TendermintContext::new_ref(deps, env)?;
        cosmwasm_std::from_json(ctx.query(msg)?).map_err(ContractError::from)
    }

    #[allow(clippy::needless_pass_by_value, clippy::module_name_repetitions)]
    pub fn execute_query(
        deps: Deps,
        env: Env,
        msg: ibc_client_cw::types::SudoMsg,
    ) -> Result<Binary, ContractError> {
        let mut storage_mut = MockMutStorage::new(deps.storage);
        let deps_mut = storage_mut.to_deps_mut(deps.api, &deps.querier);
        let mut ctx = TendermintContext::new_mut(deps_mut, env)?;

        ctx.sudo(msg).map_err(ContractError::from)
    }
}
