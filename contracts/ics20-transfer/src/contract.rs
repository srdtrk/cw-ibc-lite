//! This module handles the execution logic of the contract.

use cosmwasm_std::{Binary, Deps, DepsMut, Env, MessageInfo, Reply, Response};

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
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    // NOTE: Sender is assumed to be the ics26-router contract.
    // NOTE: Admin is assumed to be gov module address.
    cw2::set_contract_version(deps.storage, keys::CONTRACT_NAME, keys::CONTRACT_VERSION)?;
    cw_ownable::initialize_owner(deps.storage, deps.api, Some(&msg.ics26_router_address))?;

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
        ExecuteMsg::Receive(receive_msg) => execute::receive(deps, env, info, receive_msg),
        ExecuteMsg::ReceiveIbcAppCallback(callback_msg) => {
            execute::receive_ibc_callback(deps, env, info, callback_msg)
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
        keys::reply::ON_RECV_PACKET_CW20_TRANSFER => {
            reply::on_recv_packet_cw20_transfer(deps, env, msg.result, msg.payload)
        }
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
        QueryMsg::EscrowAmount {
            channel,
            cw20_address,
        } => query::escrow_amount(deps, env, channel, cw20_address),
        QueryMsg::ListEscrows {
            channel,
            start_after,
            limit,
        } => query::list_escrow(deps, env, channel, start_after, limit),
        QueryMsg::Ownership {} => query::ownership(deps),
    }
}

mod execute {
    use cosmwasm_std::IbcTimeout;
    use cw_ibc_lite_ics26_router::{
        helpers::IbcLiteRouterContract, types::msg::execute::SendPacketMsg,
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

        let send_packet_msg = SendPacketMsg {
            source_port,
            source_channel: transfer_msg.source_channel,
            dest_port: keys::DEFAULT_PORT_ID.to_string(),
            dest_channel: None, // NOTE: Router will determine the dest channel.
            data: cosmwasm_std::to_json_binary(&packet)?,
            timeout,
            version: keys::ICS20_VERSION.to_string(),
        };
        let ics26_msg = ics26_contract.call(send_packet_msg)?;

        // TODO: Add events, maybe
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

mod reply {
    use crate::types::state::{self, ESCROW};

    use super::{ContractError, DepsMut, Env, Response};

    use cosmwasm_std::{Binary, SubMsgResult};
    use cw_ibc_lite_shared::types::transfer::packet::Ics20Ack;

    /// Handles the reply to
    /// [`cw_ibc_lite_shared::types::apps::callbacks::IbcAppCallbackMsg::OnRecvPacket`].
    /// It writes the acknowledgement and emits the write acknowledgement events.
    #[allow(clippy::needless_pass_by_value)]
    pub fn on_recv_packet_cw20_transfer(
        deps: DepsMut,
        _env: Env,
        result: SubMsgResult,
        payload: Binary,
    ) -> Result<Response, ContractError> {
        match result {
            SubMsgResult::Ok(_) => {
                unreachable!("unexpected response on `SubMsg::reply_on_err`")
            }
            SubMsgResult::Err(err) => {
                let reply_args: state::RecvPacketReplyPayload = cosmwasm_std::from_json(payload)?;
                // undo the escrow reduction
                ESCROW.update(
                    deps.storage,
                    (&reply_args.channel_id, &reply_args.denom),
                    |bal| -> Result<_, ContractError> {
                        let mut bal = bal.unwrap();
                        bal += reply_args.amount;
                        Ok(bal)
                    },
                )?;

                Ok(Response::new().set_data(Ics20Ack::error(err).to_vec()))
            }
        }
    }
}

mod query {
    use cosmwasm_std::{Order, StdResult};
    use cw_storage_plus::Bound;

    use crate::types::{
        msg::responses,
        state::{self, ESCROW},
    };

    use super::{Binary, ContractError, Deps, Env};

    /// Queries the escrowed amount for the given channel and cw20 contract address.
    ///
    /// # Errors
    /// Will return an error if the escrowed amount cannot be loaded or serialized.
    #[allow(clippy::needless_pass_by_value)]
    pub fn escrow_amount(
        deps: Deps,
        _env: Env,
        channel: String,
        cw20_address: String,
    ) -> Result<Binary, ContractError> {
        let amount: cosmwasm_std::Uint128 =
            state::ESCROW.load(deps.storage, (&channel, &cw20_address))?;
        Ok(cosmwasm_std::to_json_binary(&amount)?)
    }

    pub fn ownership(deps: Deps) -> Result<Binary, ContractError> {
        Ok(cosmwasm_std::to_json_binary(&cw_ownable::get_ownership(
            deps.storage,
        )?)?)
    }

    /// Default limit for pagination
    const DEFAULT_LIMIT: u32 = 10;

    #[allow(clippy::needless_pass_by_value)]
    pub fn list_escrow(
        deps: Deps,
        _env: Env,
        channel: String,
        start_after: Option<String>,
        limit: Option<u32>,
    ) -> Result<Binary, ContractError> {
        let limit = limit.unwrap_or(DEFAULT_LIMIT) as usize;
        let start = start_after.as_deref().map(Bound::exclusive);

        let escrows = ESCROW
            .prefix(&channel)
            .range(deps.storage, start, None, Order::Ascending)
            .take(limit)
            .map(|item| {
                item.map(|(cw20_address, amount)| responses::EscrowInfo {
                    channel: channel.clone(),
                    cw20_address,
                    amount,
                })
            })
            .collect::<StdResult<Vec<_>>>()?;
        let response = responses::EscrowList { list: escrows };

        Ok(cosmwasm_std::to_json_binary(&response)?)
    }
}
