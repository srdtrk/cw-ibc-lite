//! This module handles the execution logic of the contract.

use cosmwasm_std::{Binary, Deps, DepsMut, Env, MessageInfo, Response};

use cw_ibc_lite_shared::types::error::ContractError;

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
    match msg {
        ExecuteMsg::CreateClient {
            code_id,
            instantiate_msg,
            counterparty_info,
        } => execute::create_client(deps, env, info, code_id, instantiate_msg, counterparty_info),
        ExecuteMsg::ExecuteClient { client_id, message } => {
            execute::execute_client(deps, env, info, client_id, message)
        }
        ExecuteMsg::MigrateClient {
            subject_client_id,
            substitute_client_id,
        } => execute::migrate_client(deps, env, info, subject_client_id, substitute_client_id),
        ExecuteMsg::ProvideCounterparty {
            client_id,
            counterparty_info,
        } => execute::provide_counterparty(deps, env, info, client_id, counterparty_info),
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
        QueryMsg::ClientInfo { client_id } => query::client_info(deps, client_id),
    }
}

mod execute {
    use super::{state, ContractError, DepsMut, Env, MessageInfo, Response};

    use crate::types::events;

    use cw_ibc_lite_shared::types::clients::helpers;

    #[allow(clippy::needless_pass_by_value)]
    pub fn create_client(
        deps: DepsMut,
        env: Env,
        info: MessageInfo,
        code_id: u64,
        instantiate_msg: cw_ibc_lite_shared::types::clients::msg::InstantiateMsg,
        counterparty_info: Option<state::CounterpartyInfo>,
    ) -> Result<Response, ContractError> {
        let client_id = state::helpers::new_client_id(deps.storage)?;

        state::CREATORS.save(deps.storage, &client_id, &info.sender)?;
        if let Some(counterparty_info) = &counterparty_info {
            state::COUNTERPARTY.save(deps.storage, &client_id, counterparty_info)?;
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
                counterparty_info,
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
        message: cw_ibc_lite_shared::types::clients::msg::ExecuteMsg,
    ) -> Result<Response, ContractError> {
        let client_address = state::CLIENTS.load(deps.storage, &client_id)?;
        let client_contract = helpers::LightClientContract::new(client_address);

        let execute = client_contract.call(message)?;
        Ok(Response::new().add_message(execute))
    }

    #[allow(clippy::needless_pass_by_value)]
    pub fn migrate_client(
        deps: DepsMut,
        _env: Env,
        info: MessageInfo,
        subject_client_id: String,
        substitute_client_id: String,
    ) -> Result<Response, ContractError> {
        cw_ownable::assert_owner(deps.storage, &info.sender)?;
        if !state::CLIENTS.has(deps.storage, &subject_client_id) {
            return Err(ContractError::not_found::<cosmwasm_std::Addr>(
                subject_client_id.into_bytes(),
            ));
        }

        let substitute_client_address = state::CLIENTS.load(deps.storage, &substitute_client_id)?;
        state::CLIENTS.save(deps.storage, &subject_client_id, &substitute_client_address)?;

        Ok(Response::new().add_event(events::migrate_client::success(
            &subject_client_id,
            &substitute_client_id,
            substitute_client_address.as_str(),
        )))
    }

    #[allow(clippy::needless_pass_by_value)]
    pub fn provide_counterparty(
        deps: DepsMut,
        _env: Env,
        info: MessageInfo,
        client_id: String,
        counterparty_info: state::CounterpartyInfo,
    ) -> Result<Response, ContractError> {
        state::helpers::assert_creator(deps.storage, &client_id, &info.sender)?;
        if state::COUNTERPARTY.has(deps.storage, &client_id) {
            return Err(ContractError::CounterpartyAlreadyProvided);
        }
        state::COUNTERPARTY.save(deps.storage, &client_id, &counterparty_info)?;

        Ok(
            Response::new().add_event(events::provide_counterparty::success(
                &client_id,
                counterparty_info,
            )),
        )
    }
}

mod query {
    use super::{state, Binary, ContractError, Deps};

    use crate::types::msg::query_responses;

    /// Returns the address of the client encoded as a JSON binary.
    #[allow(clippy::needless_pass_by_value)]
    pub fn client_info(deps: Deps, client_id: String) -> Result<Binary, ContractError> {
        let address = state::CLIENTS.load(deps.storage, &client_id)?;
        let counterparty_info = state::COUNTERPARTY.may_load(deps.storage, &client_id)?;
        let creator = state::CREATORS.load(deps.storage, &client_id)?;

        Ok(cosmwasm_std::to_json_binary(
            &query_responses::ClientInfo {
                client_id,
                address: address.into_string(),
                counterparty_info,
                creator: creator.into_string(),
            },
        )?)
    }
}
