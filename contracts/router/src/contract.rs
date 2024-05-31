//! This module handles the execution logic of the contract.

use cosmwasm_std::{Binary, Deps, DepsMut, Env, MessageInfo, Response};

use cw_ibc_lite_ics02_client as ics02_client;

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
    env: Env,
    _info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    cw2::set_contract_version(deps.storage, keys::CONTRACT_NAME, keys::CONTRACT_VERSION)?;

    let ics02_code = ics02_client::helpers::Ics02ClientCode::new(msg.ics02_client_code_id);
    let (ics02_instantiate, ics02_address) = ics02_code.instantiate2(
        deps.api,
        &deps.querier,
        &env,
        ics02_client::types::msg::InstantiateMsg {},
        // TODO: ensure there is no DOS attack vector here
        format!("{}.{}", keys::ICS02_CLIENT_SALT, env.contract.address),
        None::<String>,
        keys::ICS02_CLIENT_SALT,
    )?;
    state::ICS02_CLIENT_ADDRESS.save(deps.storage, &ics02_address)?;

    Ok(Response::new().add_message(ics02_instantiate))
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
        ExecuteMsg::SendPacket {
            source_channel,
            source_port_id,
            dest_channel,
            dest_port_id,
            data,
            timeout,
        } => execute::send_packet(
            deps,
            env,
            info,
            source_channel,
            source_port_id,
            dest_channel,
            dest_port_id,
            data,
            timeout,
        ),
        ExecuteMsg::RecvPacket {
            packet,
            proof_commitment,
            proof_height,
        } => execute::recv_packet(deps, env, info, packet, proof_commitment, proof_height),
        ExecuteMsg::Acknowledgement {
            packet,
            acknowledgement,
            proof_acked,
            proof_height,
        } => execute::acknowledgement(
            deps,
            env,
            info,
            packet,
            acknowledgement,
            proof_acked,
            proof_height,
        ),
        ExecuteMsg::Timeout {
            packet,
            proof_unreceived,
            proof_height,
            next_sequence_recv,
        } => execute::timeout(
            deps,
            env,
            info,
            packet,
            proof_unreceived,
            proof_height,
            next_sequence_recv,
        ),
        ExecuteMsg::RegisterIbcApp { port_id, address } => {
            execute::register_ibc_app(deps, env, info, port_id, address)
        }
    }
}

/// Handles the query messages by routing them to the respective handlers.
///
/// # Errors
/// Will return an error if the handler returns an error.
#[allow(clippy::needless_pass_by_value)]
#[cosmwasm_std::entry_point]
pub fn query(deps: Deps, env: Env, msg: QueryMsg) -> Result<Binary, ContractError> {
    match msg {
        QueryMsg::PortRouter { port_id } => query::port_router(deps, env, port_id),
    }
}

mod execute {
    use crate::types::events;

    use super::{keys, state, ContractError, DepsMut, Env, MessageInfo, Response};

    use cosmwasm_std::{Binary, IbcTimeout};

    use cw_ibc_lite_types::ibc::{Height, Packet};

    #[allow(clippy::too_many_arguments, clippy::needless_pass_by_value)]
    pub fn send_packet(
        _deps: DepsMut,
        _env: Env,
        _info: MessageInfo,
        _source_channel: String,
        _source_port_id: String,
        _dest_channel: String,
        _dest_port_id: String,
        _data: Binary,
        _timeout: IbcTimeout,
    ) -> Result<Response, ContractError> {
        todo!()
    }

    #[allow(clippy::needless_pass_by_value)]
    pub fn recv_packet(
        _deps: DepsMut,
        _env: Env,
        _info: MessageInfo,
        _packet: Packet,
        _proof_commitment: Binary,
        _proof_height: Height,
    ) -> Result<Response, ContractError> {
        todo!()
    }

    #[allow(clippy::needless_pass_by_value)]
    pub fn acknowledgement(
        _deps: DepsMut,
        _env: Env,
        _info: MessageInfo,
        _packet: Packet,
        _acknowledgement: Binary,
        _proof_acked: Binary,
        _proof_height: Height,
    ) -> Result<Response, ContractError> {
        todo!()
    }

    #[allow(clippy::needless_pass_by_value)]
    pub fn timeout(
        _deps: DepsMut,
        _env: Env,
        _info: MessageInfo,
        _packet: Packet,
        _proof_unreceived: Binary,
        _proof_height: Height,
        _next_sequence_recv: u64,
    ) -> Result<Response, ContractError> {
        todo!()
    }

    #[allow(clippy::needless_pass_by_value)]
    pub fn register_ibc_app(
        deps: DepsMut,
        env: Env,
        info: MessageInfo,
        port_id: Option<String>,
        contract_address: String,
    ) -> Result<Response, ContractError> {
        let contract_address = deps.api.addr_validate(&contract_address)?;
        let port_id = if let Some(port_id) = port_id {
            // NOTE: Only the admin can register an IBC app with a custom port ID.
            state::admin::assert_admin(&env, &deps.querier, &info.sender)?;
            port_id
        } else {
            format!("{}{}", keys::PORT_ID_PREFIX, contract_address)
        };

        state::IBC_APPS.save(deps.storage, &port_id, &contract_address)?;

        Ok(Response::new().add_event(events::register_ibc_app::success(
            &port_id,
            contract_address.as_str(),
            info.sender.as_str(),
        )))
    }
}

mod query {
    use super::{state, Binary, ContractError, Deps, Env};

    #[allow(clippy::needless_pass_by_value)]
    pub fn port_router(deps: Deps, _env: Env, port_id: String) -> Result<Binary, ContractError> {
        Ok(cosmwasm_std::to_json_binary(
            &state::IBC_APPS.load(deps.storage, &port_id)?,
        )?)
    }
}
