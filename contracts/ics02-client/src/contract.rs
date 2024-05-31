//! This module handles the execution logic of the contract.

use cosmwasm_std::{Binary, Deps, DepsMut, Env, MessageInfo, Response};

use cw_ibc_lite_types::error::ContractError;

use crate::types::{
    keys,
    msg::{ExecuteMsg, InstantiateMsg, QueryMsg},
    state,
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
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    cw_ownable::assert_owner(deps.storage, &info.sender)?;

    match msg {
        ExecuteMsg::CreateClient {
            code_id,
            instantiate_msg,
            counterparty_id,
        } => execute::create_client(deps, env, info, code_id, instantiate_msg, counterparty_id),
        ExecuteMsg::ExecuteClient { client_id, message } => {
            execute::execute_client(deps, env, info, client_id, message)
        }
        ExecuteMsg::MigrateClient {
            client_id,
            new_client_id,
        } => execute::migrate_client(deps, env, info, client_id, new_client_id),
        ExecuteMsg::ProvideCounterparty {
            client_id,
            counterparty_id,
        } => execute::provide_counterparty(deps, env, info, client_id, counterparty_id),
    }
}

/// Handles the query messages by routing them to the respective handlers.
///
/// # Errors
/// Will return an error if the handler returns an error.
#[allow(clippy::needless_pass_by_value)]
#[cosmwasm_std::entry_point]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> Result<Binary, ContractError> {
    match msg {
        QueryMsg::QueryClient { client_id, query } => query::query_client(deps, client_id, query),
        QueryMsg::ClientInfo { client_id } => query::client_info(deps, client_id),
    }
}

mod execute {
    use super::{state, ContractError, DepsMut, Env, MessageInfo, Response};

    use crate::types::events;

    use cw_ibc_lite_types::clients::helpers;

    #[allow(clippy::needless_pass_by_value)]
    pub fn create_client(
        deps: DepsMut,
        env: Env,
        info: MessageInfo,
        code_id: u64,
        instantiate_msg: cw_ibc_lite_types::clients::msg::InstantiateMsg,
        counterparty_id: Option<String>,
    ) -> Result<Response, ContractError> {
        let client_id = state::helpers::new_client_id(deps.storage)?;

        state::CREATORS.save(deps.storage, &client_id, &info.sender)?;
        if let Some(counterparty_id) = &counterparty_id {
            state::COUNTERPARTY.save(deps.storage, &client_id, counterparty_id)?;
        }

        // Instantiate the light client.
        let client_code = helpers::LightClientCode::new(code_id);
        let (instantiate2, address) = client_code.instantiate2(
            deps.api,
            &deps.querier,
            &env,
            instantiate_msg,
            // TODO: Make sure there is no DOS vector here.
            &client_id,
            Some(&env.contract.address),
            &client_id,
        )?;

        state::CLIENTS.save(deps.storage, &client_id, &address)?;

        Ok(Response::new()
            .add_message(instantiate2)
            .add_event(events::create_client::success(
                &client_id,
                &counterparty_id.unwrap_or_default(),
                info.sender.as_str(),
                address.as_str(),
            )))
    }

    #[allow(clippy::needless_pass_by_value, clippy::module_name_repetitions)]
    pub fn execute_client(
        deps: DepsMut,
        _env: Env,
        _info: MessageInfo,
        client_id: String,
        message: cw_ibc_lite_types::clients::msg::ExecuteMsg,
    ) -> Result<Response, ContractError> {
        let client_address = state::CLIENTS.load(deps.storage, &client_id)?;
        let client_contract = helpers::LightClientContract::new(client_address);

        let execute = client_contract.call(message)?;
        Ok(Response::new().add_message(execute))
    }

    #[allow(clippy::needless_pass_by_value)]
    pub fn migrate_client(
        _deps: DepsMut,
        _env: Env,
        _info: MessageInfo,
        _client_id: String,
        _new_client_id: String,
    ) -> Result<Response, ContractError> {
        todo!()
    }

    #[allow(clippy::needless_pass_by_value)]
    pub fn provide_counterparty(
        deps: DepsMut,
        _env: Env,
        info: MessageInfo,
        client_id: String,
        counterparty_id: String,
    ) -> Result<Response, ContractError> {
        state::helpers::assert_creator(deps.storage, &client_id, &info.sender)?;
        if state::COUNTERPARTY.has(deps.storage, &client_id) {
            return Err(ContractError::CounterpartyAlreadyProvided);
        }
        state::COUNTERPARTY.save(deps.storage, &client_id, &counterparty_id)?;

        Ok(
            Response::new().add_event(events::provide_counterparty::success(
                &client_id,
                &counterparty_id,
            )),
        )
    }
}

mod query {
    use super::{state, Binary, ContractError, Deps};

    use cw_ibc_lite_types::clients::helpers;

    use crate::types::msg::query_responses;

    /// Returns the address of the client encoded as a JSON binary.
    #[allow(clippy::needless_pass_by_value)]
    pub fn client_info(deps: Deps, client_id: String) -> Result<Binary, ContractError> {
        let address = state::CLIENTS.load(deps.storage, &client_id)?;
        let counterparty_id = state::COUNTERPARTY.may_load(deps.storage, &client_id)?;
        let creator = state::CREATORS.load(deps.storage, &client_id)?;

        Ok(cosmwasm_std::to_json_binary(
            &query_responses::ClientInfo {
                client_id,
                address: address.into_string(),
                counterparty_id,
                creator: creator.into_string(),
            },
        )?)
    }

    #[allow(clippy::needless_pass_by_value, clippy::module_name_repetitions)]
    pub fn query_client(
        deps: Deps,
        client_id: String,
        query_msg: cw_ibc_lite_types::clients::msg::QueryMsg,
    ) -> Result<Binary, ContractError> {
        let client_address = state::CLIENTS.load(deps.storage, &client_id)?;
        let client_contract = helpers::LightClientContract::new(client_address);

        Ok(client_contract.query(&deps.querier).smart_raw(query_msg)?)
    }
}
