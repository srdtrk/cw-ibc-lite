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
    _info: MessageInfo,
    _msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    cw2::set_contract_version(deps.storage, keys::CONTRACT_NAME, keys::CONTRACT_VERSION)?;
    unimplemented!()
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
    use super::{ContractError, DepsMut, Env, MessageInfo, Response};

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
}
