//! This module contains the callback handlers for the IBC module.

use cosmwasm_std::{Binary, DepsMut, Env, MessageInfo, Response};
use cw_ibc_lite_shared::types::{error::ContractError, ibc};

/// Handles the callback for the `on_send_packet` IBC handler.
///
/// # Errors
/// Will return an error if the sender is not this contract or the balance cannot be recorded.
#[allow(clippy::needless_pass_by_value)]
pub fn on_send_packet(
    _deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    _packet: ibc::Packet,
    _version: String,
    _sender: String,
) -> Result<Response, ContractError> {
    todo!()
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
