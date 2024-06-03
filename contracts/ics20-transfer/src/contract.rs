//! This module handles the execution logic of the contract.

use cosmwasm_std::{Binary, Deps, DepsMut, Env, MessageInfo, Response};

use cw_ibc_lite_shared::types::error::ContractError;

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
    // NOTE: Sender is assumed to be the ics26-router contract.
    // NOTE: Admin is assumed to be gov module address.
    cw2::set_contract_version(deps.storage, keys::CONTRACT_NAME, keys::CONTRACT_VERSION)?;
    cw_ownable::initialize_owner(deps.storage, deps.api, None)?;

    todo!()
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
        ExecuteMsg::Receive(receive_msg) => execute::receive(deps, env, info, receive_msg),
        ExecuteMsg::ReceiveIbcAppCallback(callback_msg) => {
            execute::receive_ibc_callback(deps, env, info, callback_msg)
        }
    }
}

/// Handles the query messages by routing them to the respective handlers.
///
/// # Errors
/// Will return an error if the handler returns an error.
#[allow(clippy::needless_pass_by_value)]
#[cosmwasm_std::entry_point]
pub fn query(_deps: Deps, _env: Env, _msg: QueryMsg) -> Result<Binary, ContractError> {
    todo!()
}

mod execute {
    use cosmwasm_std::IbcTimeout;
    use cw_ibc_lite_ics26_router::{
        helpers::IbcLiteRouterContract, types::msg::ExecuteMsg as Ics26ExecuteMsg,
    };
    use cw_ibc_lite_shared::{
        types::{
            apps::callbacks::IbcAppCallbackMsg,
            transfer::{error::TransferError, packet::Ics20Packet},
        },
        utils,
    };

    use crate::{ibc, types::msg::TransferMsg};

    use super::{keys, ContractError, DepsMut, Env, MessageInfo, Response};

    #[allow(clippy::needless_pass_by_value)]
    pub fn receive(
        deps: DepsMut,
        env: Env,
        info: MessageInfo,
        msg: cw20::Cw20ReceiveMsg,
    ) -> Result<Response, ContractError> {
        if !info.funds.is_empty() {
            return Err(TransferError::UnexpectedNativeToken.into());
        }

        let ics26_address = cw_ownable::get_ownership(deps.storage)?
            .owner
            .ok_or(ContractError::Unauthorized)?;
        let ics26_contract = IbcLiteRouterContract::new(ics26_address);

        let transfer_msg: TransferMsg = cosmwasm_std::from_json(msg.msg)?;
        let denom = info.sender.to_string(); // NOTE: We use the sender contract address as the denom.
        let source_port = utils::apps::contract_port_id(&env.contract.address)?.into();
        let timeout_seconds = transfer_msg
            .timeout
            .unwrap_or(keys::DEFAULT_TIMEOUT_SECONDS);
        let timeout = IbcTimeout::with_timestamp(env.block.time.plus_seconds(timeout_seconds));

        let packet = Ics20Packet::try_new(
            msg.amount,
            denom,
            transfer_msg.receiver,
            msg.sender,
            transfer_msg.memo,
        )?;

        let send_packet_msg = Ics26ExecuteMsg::SendPacket {
            source_port,
            source_channel: transfer_msg.source_channel,
            dest_port: keys::DEFAULT_PORT_ID.to_string(),
            dest_channel: None, // NOTE: Router will determine the dest channel.
            data: cosmwasm_std::to_json_binary(&packet)?,
            timeout,
        };
        let ics26_msg = ics26_contract.call(send_packet_msg)?;

        // TODO: Add events
        Ok(Response::new().add_message(ics26_msg))
    }

    #[allow(clippy::needless_pass_by_value)]
    pub fn receive_ibc_callback(
        deps: DepsMut,
        env: Env,
        info: MessageInfo,
        msg: IbcAppCallbackMsg,
    ) -> Result<Response, ContractError> {
        cw_ownable::assert_owner(deps.storage, &info.sender)?;
        match msg {
            IbcAppCallbackMsg::OnSendPacket {
                packet,
                version,
                sender,
            } => ibc::relay::on_send_packet(deps, env, info, packet, version, sender),
            IbcAppCallbackMsg::OnRecvPacket { packet, relayer } => {
                ibc::relay::on_recv_packet(deps, env, info, packet, relayer)
            }
            IbcAppCallbackMsg::OnAcknowledgementPacket {
                packet,
                acknowledgement,
                relayer,
            } => ibc::relay::on_acknowledgement_packet(
                deps,
                env,
                info,
                packet,
                acknowledgement,
                relayer,
            ),
            IbcAppCallbackMsg::OnTimeoutPacket { packet, relayer } => {
                ibc::relay::on_timeout_packet(deps, env, info, packet, relayer)
            }
        }
    }
}
