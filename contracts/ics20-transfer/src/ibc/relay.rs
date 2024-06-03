//! This module contains the callback handlers for the IBC module.

use cosmwasm_std::{Binary, DepsMut, Env, MessageInfo, Response};
use cw_ibc_lite_shared::{
    types::{
        error::ContractError,
        ibc,
        transfer::{error::TransferError, packet::Ics20Packet},
    },
    utils,
};

use crate::types::{keys, state};

/// Handles the callback for the `on_send_packet` IBC handler.
///
/// # Errors
/// Will return an error if the sender is not this contract or the balance cannot be recorded.
#[allow(clippy::needless_pass_by_value)]
pub fn on_send_packet(
    deps: DepsMut,
    env: Env,
    _info: MessageInfo,
    packet: ibc::Packet,
    version: String,
    sender: String,
) -> Result<Response, ContractError> {
    // NOTE: We must ensure that the sender is the contract itself because the tokens were received
    // by the contract at an earlier point, on [`crate::types::msg::ExecuteMsg::Receive`].
    if sender != env.contract.address {
        return Err(ContractError::Unauthorized);
    }
    if version != keys::ICS20_VERSION {
        return Err(TransferError::InvalidVersion.into());
    }

    let port_id = utils::apps::contract_port_id(&env.contract.address)?;
    if packet.source_port != port_id {
        return Err(TransferError::unexpected_port_id(port_id, packet.source_port).into());
    }

    let ics20_packet: Ics20Packet = cosmwasm_std::from_json(packet.data)?;
    ics20_packet.validate()?;

    // Add amount to the escrowed balance.
    state::ESCROW.update(
        deps.storage,
        (packet.source_channel.as_str(), &ics20_packet.denom),
        |escrowed_bal| -> Result<_, ContractError> {
            let mut escrowed_bal = escrowed_bal.unwrap_or_default();
            escrowed_bal += ics20_packet.amount;
            Ok(escrowed_bal)
        },
    )?;

    Ok(Response::default())
}

/// Handles the callback for the `on_recv_packet` IBC handler.
///
/// # Errors
/// Will return an error if new balance cannot be transferred to the receiver.
#[allow(clippy::needless_pass_by_value)]
pub fn on_recv_packet(
    _deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    _packet: ibc::Packet,
    _relayer: String,
) -> Result<Response, ContractError> {
    todo!()
}

/// Handles the callback for the `on_acknowledgement_packet` IBC handler.
///
/// # Errors
/// Will return an error if the acknowledgement cannot be processed.
#[allow(clippy::needless_pass_by_value)]
pub fn on_acknowledgement_packet(
    _deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    _packet: ibc::Packet,
    _ack: Binary,
    _relayer: String,
) -> Result<Response, ContractError> {
    todo!()
}

/// Handles the callback for the `on_timeout_packet` IBC handler.
///
/// # Errors
/// Will return an error if the timeout cannot be processed and tokens refunded.
#[allow(clippy::needless_pass_by_value)]
pub fn on_timeout_packet(
    _deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    _packet: ibc::Packet,
    _relayer: String,
) -> Result<Response, ContractError> {
    todo!()
}
