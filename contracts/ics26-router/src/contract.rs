//! This module handles the execution logic of the contract.

use cosmwasm_std::{Binary, Deps, DepsMut, Env, MessageInfo, Reply, Response};

use cw_ibc_lite_ics02_client as ics02_client;

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
            source_port,
            dest_channel,
            dest_port,
            data,
            timeout,
        } => execute::send_packet(
            deps,
            env,
            info,
            source_channel,
            source_port,
            dest_channel,
            dest_port,
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

/// Handles the replies to the submessages.
///
/// # Errors
/// Will return an error if the handler returns an error.
#[cosmwasm_std::entry_point]
#[allow(clippy::needless_pass_by_value)]
pub fn reply(deps: DepsMut, env: Env, msg: Reply) -> Result<Response, ContractError> {
    match msg.id {
        // TODO: Ensure that events are emitted for all the replies.
        keys::reply::ON_RECV_PACKET => reply::write_acknowledgement(deps, env, msg.result),
        _ => Err(ContractError::UnknownReplyId(msg.id)),
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
    use std::str::FromStr;

    use crate::types::events;

    use super::{keys, state, ContractError, DepsMut, Env, MessageInfo, Response};

    use cosmwasm_std::{Binary, IbcTimeout, SubMsg};

    use cw_ibc_lite_ics02_client as ics02_client;
    use cw_ibc_lite_shared::{
        types::{
            apps, ibc,
            paths::{ics24_host, identifiers},
            storage::PureItem,
        },
        utils,
    };

    use ibc_client_cw::types::VerifyMembershipMsgRaw;

    #[allow(clippy::too_many_arguments, clippy::needless_pass_by_value)]
    pub fn send_packet(
        deps: DepsMut,
        env: Env,
        info: MessageInfo,
        source_channel: String,
        source_port: String,
        dest_channel: String,
        dest_port: String,
        data: Binary,
        timeout: IbcTimeout,
    ) -> Result<Response, ContractError> {
        let source_channel = identifiers::ChannelId::from_str(&source_channel)?;
        let source_port = identifiers::PortId::from_str(&source_port)?;
        let dest_channel = identifiers::ChannelId::from_str(&dest_channel)?;
        let dest_port = identifiers::PortId::from_str(&dest_port)?;

        let ics02_address = state::ICS02_CLIENT_ADDRESS.load(deps.storage)?;
        let ics02_contract = ics02_client::helpers::Ics02ClientContract::new(ics02_address);

        let ibc_app_address = state::IBC_APPS.load(deps.storage, source_port.as_str())?;
        let ibc_app_contract = apps::helpers::IbcApplicationContract::new(ibc_app_address);

        // Ensure the counterparty is the destination channel.
        let counterparty_id = ics02_contract
            .query(&deps.querier)
            .counterparty(source_channel.as_str())?
            .client_id;
        if counterparty_id != dest_channel.as_str() {
            return Err(ContractError::invalid_counterparty(
                counterparty_id,
                dest_channel.into(),
            ));
        }

        // Ensure the timeout is valid.
        utils::timeout::validate(&env, &timeout)?;

        // Construct the packet.
        let sequence: identifiers::Sequence = state::helpers::new_sequence_send(
            deps.storage,
            source_port.as_str(),
            source_channel.as_str(),
        )?
        .into();
        let packet = ibc::Packet {
            sequence,
            source_channel,
            source_port,
            destination_channel: dest_channel,
            destination_port: dest_port,
            data,
            timeout,
        };

        // TODO: Ensure it is ok to commit packet and emit events before the callback.
        state::helpers::commit_packet(deps.storage, &packet)?;

        let send_packet_event = events::send_packet::success(&packet);
        let callback_msg = apps::callbacks::IbcAppCallbackMsg::OnSendPacket {
            packet,
            version: keys::CONTRACT_VERSION.to_string(),
            sender: info.sender.into(),
        };
        let send_packet_callback = ibc_app_contract.call(callback_msg)?;

        // TODO: Ensure event emission is reverted if the callback fails.
        Ok(Response::new()
            .add_message(send_packet_callback)
            .add_event(send_packet_event))
    }

    #[allow(clippy::needless_pass_by_value)]
    pub fn recv_packet(
        deps: DepsMut,
        _env: Env,
        info: MessageInfo,
        packet: ibc::Packet,
        proof_commitment: Binary,
        proof_height: ibc::Height,
    ) -> Result<Response, ContractError> {
        let ics02_address = state::ICS02_CLIENT_ADDRESS.load(deps.storage)?;
        let ics02_contract = ics02_client::helpers::Ics02ClientContract::new(ics02_address);

        let ibc_app_address =
            state::IBC_APPS.load(deps.storage, packet.destination_port.as_str())?;
        let ibc_app_contract = apps::helpers::IbcApplicationContract::new(ibc_app_address);

        // Verify the counterparty.
        let counterparty = ics02_contract
            .query(&deps.querier)
            .counterparty(packet.destination_channel.as_str())?;
        if counterparty.client_id != packet.source_channel.as_str() {
            return Err(ContractError::invalid_counterparty(
                counterparty.client_id,
                packet.source_channel.into(),
            ));
        }

        // NOTE: Verify the packet commitment.
        // TODO: Use the merkle prefix in counterparty
        let counterparty_commitment_path = ics24_host::PacketCommitmentPath {
            port_id: packet.source_port.clone(),
            channel_id: packet.source_channel.clone(),
            sequence: packet.sequence,
        }
        .into();
        let verify_membership_msg = VerifyMembershipMsgRaw {
            proof: proof_commitment.into(),
            path: counterparty_commitment_path,
            value: packet.to_commitment_bytes(),
            height: proof_height.into(),
            delay_time_period: 0,
            delay_block_period: 0,
        };
        let _ = ics02_contract
            .query(&deps.querier)
            .client_querier(packet.destination_channel.as_str())?
            .verify_membership(verify_membership_msg)?;

        state::helpers::set_packet_receipt(deps.storage, &packet)?;
        state::helpers::save_packet_temp_store(deps.storage, &packet)?;

        let event = events::recv_packet::success(&packet);
        // NOTE: We must retreive a reply from the IBC app to set the acknowledgement.
        let callback_msg = apps::callbacks::IbcAppCallbackMsg::OnRecvPacket {
            packet,
            relayer: info.sender.into(),
        };
        let recv_packet_callback = SubMsg::reply_on_success(
            ibc_app_contract.call(callback_msg)?,
            keys::reply::ON_RECV_PACKET,
        );

        // TODO: Ensure event emission is reverted if the callback fails.
        Ok(Response::new()
            .add_submessage(recv_packet_callback)
            .add_event(event))
    }

    #[allow(clippy::needless_pass_by_value)]
    pub fn acknowledgement(
        deps: DepsMut,
        _env: Env,
        _info: MessageInfo,
        packet: ibc::Packet,
        _acknowledgement: Binary,
        _proof_acked: Binary,
        _proof_height: ibc::Height,
    ) -> Result<Response, ContractError> {
        let ics02_address = state::ICS02_CLIENT_ADDRESS.load(deps.storage)?;
        let ics02_contract = ics02_client::helpers::Ics02ClientContract::new(ics02_address);

        let ibc_app_address = state::IBC_APPS.load(deps.storage, packet.source_channel.as_str())?;
        let _ibc_app_contract = apps::helpers::IbcApplicationContract::new(ibc_app_address);

        // Verify the counterparty.
        let counterparty = ics02_contract
            .query(&deps.querier)
            .counterparty(packet.source_channel.as_str())?;
        if counterparty.client_id != packet.destination_channel.as_str() {
            return Err(ContractError::invalid_counterparty(
                counterparty.client_id,
                packet.destination_channel.into(),
            ));
        }

        // NOTE: If commitment cannot be loaded, this indicates that the acknowledgement has already
        // been relayed or there is a misconfigured relayer attempting to prove an acknowledgement
        // for a packet never sent. IBC Go treats this error as a no-op in order to prevent an entire
        // relay transaction from failing and consuming unnecessary fees. We don't do this here.
        let stored_packet_commitment = PureItem::from(ics24_host::PacketCommitmentPath {
            port_id: packet.source_port.clone(),
            channel_id: packet.source_channel.clone(),
            sequence: packet.sequence,
        })
        .load(deps.storage)?;
        if stored_packet_commitment != packet.to_commitment_bytes() {
            return Err(ContractError::packet_commitment_mismatch(
                stored_packet_commitment,
                packet.to_commitment_bytes(),
            ));
        }

        todo!()
    }

    #[allow(clippy::needless_pass_by_value)]
    pub fn timeout(
        _deps: DepsMut,
        _env: Env,
        _info: MessageInfo,
        _packet: ibc::Packet,
        _proof_unreceived: Binary,
        _proof_height: ibc::Height,
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
            // TODO: Add restrictions to the custom port ID. Such as not using `/`.
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

mod reply {
    use cosmwasm_std::SubMsgResult;
    use cw_ibc_lite_shared::types::ibc;

    use crate::types::events;

    use super::{state, ContractError, DepsMut, Env, Response};

    /// Handles the reply to
    /// [`cw_ibc_lite_shared::types::apps::callbacks::IbcAppCallbackMsg::OnRecvPacket`].
    /// It writes the acknowledgement and emits the write acknowledgement events.
    #[allow(clippy::needless_pass_by_value)]
    pub fn write_acknowledgement(
        deps: DepsMut,
        _env: Env,
        result: SubMsgResult,
    ) -> Result<Response, ContractError> {
        match result {
            SubMsgResult::Ok(resp) => {
                let ack: ibc::Acknowledgement = resp
                    .data
                    .ok_or(ContractError::RecvPacketCallbackNoResponse)?
                    .try_into()?;
                let packet = state::helpers::remove_packet_temp_store(deps.storage)?;

                state::helpers::commit_packet_ack(deps.storage, &packet, &ack)?;
                Ok(
                    Response::new()
                        .add_event(events::write_acknowledgement::success(&packet, &ack)),
                )
            }
            SubMsgResult::Err(err) => {
                unreachable!("unexpected `SubMsg::reply_on_success`, error: {err}")
            }
        }
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
