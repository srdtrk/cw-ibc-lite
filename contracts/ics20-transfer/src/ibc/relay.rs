//! This module contains the callback handlers for the IBC module.

use cosmwasm_std::{Binary, CosmosMsg, DepsMut, Env, MessageInfo, Response, SubMsg, WasmMsg};
use cw_ibc_lite_shared::{
    types::{
        error::ContractError,
        ibc,
        transfer::{
            error::TransferError,
            packet::{Ics20Ack, Ics20Packet},
        },
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
    if sender != env.contract.address.as_str() {
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
    deps: DepsMut,
    env: Env,
    _info: MessageInfo,
    packet: ibc::Packet,
    _relayer: String,
) -> Result<Response, ContractError> {
    let port_id = utils::apps::contract_port_id(&env.contract.address)?;
    if packet.destination_port != port_id {
        return Err(TransferError::unexpected_port_id(port_id, packet.destination_port).into());
    }

    let ics20_packet: Ics20Packet = cosmwasm_std::from_json(packet.data)?;
    let base_denom = utils::transfer::parse_voucher_denom(
        &ics20_packet.denom,
        packet.source_port.as_str(),
        packet.source_channel.as_str(),
    )?;

    // Subtract amount from the escrowed balance.
    state::ESCROW.update(
        deps.storage,
        (packet.destination_channel.as_str(), base_denom),
        |escrowed_bal| -> Result<_, ContractError> {
            let mut escrowed_bal = escrowed_bal.unwrap_or_default();
            escrowed_bal = escrowed_bal.checked_sub(ics20_packet.amount).map_err(|_| {
                TransferError::insufficient_funds_in_escrow(escrowed_bal, ics20_packet.amount)
            })?;
            Ok(escrowed_bal)
        },
    )?;

    // TODO: move to helper
    let cw20_msg: CosmosMsg = WasmMsg::Execute {
        contract_addr: base_denom.to_string(),
        msg: cosmwasm_std::to_json_binary(&cw20::Cw20ExecuteMsg::Transfer {
            recipient: ics20_packet.receiver,
            amount: ics20_packet.amount,
        })?,
        funds: vec![],
    }
    .into();
    let reply_args = state::RecvPacketReplyPayload {
        channel_id: packet.destination_channel.into(),
        denom: base_denom.to_string(),
        amount: ics20_packet.amount,
    };
    let cw20_submsg = SubMsg::reply_on_error(cw20_msg, keys::reply::ON_RECV_PACKET_CW20_TRANSFER)
        .with_payload(cosmwasm_std::to_json_binary(&reply_args)?);

    // NOTE: The success acknowledgement will be overwritten by the SubMsg reply in case of error.
    Ok(Response::new()
        .add_submessage(cw20_submsg)
        .set_data(Ics20Ack::success().to_vec()))
}

/// Handles the callback for the `on_acknowledgement_packet` IBC handler.
///
/// # Errors
/// Will return an error if the acknowledgement cannot be processed.
pub fn on_acknowledgement_packet(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    packet: ibc::Packet,
    ack: Binary,
    relayer: String,
) -> Result<Response, ContractError> {
    match cosmwasm_std::from_json(ack)? {
        Ics20Ack::Result(result) => {
            on_acknowledgement_packet::success(deps, env, info, packet, result, relayer)
        }
        Ics20Ack::Error(err) => {
            on_acknowledgement_packet::error(deps, env, info, packet, err, relayer)
        }
    }
}

/// Handles the callback for the `on_timeout_packet` IBC handler.
///
/// # Errors
/// Will return an error if the timeout cannot be processed and tokens refunded.
pub fn on_timeout_packet(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    packet: ibc::Packet,
    relayer: String,
) -> Result<Response, ContractError> {
    on_acknowledgement_packet::error(deps, env, info, packet, "timeout".to_string(), relayer)
}

mod on_acknowledgement_packet {
    use cosmwasm_std::HexBinary;

    use super::{
        ibc, state, utils, Binary, ContractError, CosmosMsg, DepsMut, Env, Ics20Ack, Ics20Packet,
        MessageInfo, Response, TransferError, WasmMsg,
    };

    #[allow(clippy::needless_pass_by_value)]
    pub fn success(
        _deps: DepsMut,
        _env: Env,
        _info: MessageInfo,
        _packet: ibc::Packet,
        result: Binary,
        _relayer: String,
    ) -> Result<Response, ContractError> {
        if result.as_slice() != Ics20Ack::SUCCESS_BYTES {
            return Err(
                TransferError::UnknownAcknowledgement(HexBinary::from(result).to_hex()).into(),
            );
        }

        Ok(Response::default())
    }

    #[allow(clippy::needless_pass_by_value)]
    pub fn error(
        deps: DepsMut,
        env: Env,
        _info: MessageInfo,
        packet: ibc::Packet,
        _err: String,
        _relayer: String,
    ) -> Result<Response, ContractError> {
        let ics20_packet: Ics20Packet = cosmwasm_std::from_json(packet.data)?;

        let port_id = utils::apps::contract_port_id(&env.contract.address)?;
        if packet.source_port != port_id {
            return Err(TransferError::unexpected_port_id(port_id, packet.source_port).into());
        }

        // Refund the escrowed balance.
        state::ESCROW.update(
            deps.storage,
            (packet.source_channel.as_str(), &ics20_packet.denom),
            |escrowed_bal| -> Result<_, ContractError> {
                let mut escrowed_bal = escrowed_bal.unwrap_or_default();
                escrowed_bal = escrowed_bal.checked_sub(ics20_packet.amount).map_err(|_| {
                    TransferError::insufficient_funds_in_escrow(escrowed_bal, ics20_packet.amount)
                })?;
                Ok(escrowed_bal)
            },
        )?;

        let cw20_msg: CosmosMsg = WasmMsg::Execute {
            contract_addr: ics20_packet.denom,
            msg: cosmwasm_std::to_json_binary(&cw20::Cw20ExecuteMsg::Transfer {
                recipient: ics20_packet.sender,
                amount: ics20_packet.amount,
            })?,
            funds: vec![],
        }
        .into();

        // NOTE: If the refund fails, the packet cannot be acknowledged and must be retried.
        Ok(Response::new().add_message(cw20_msg))
    }
}
