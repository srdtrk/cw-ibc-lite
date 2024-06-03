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
    // NOTE: Contract admin is assumed to be the ics26-router contract.
    cw2::set_contract_version(deps.storage, keys::CONTRACT_NAME, keys::CONTRACT_VERSION)?;

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
    use cw_ibc_lite_shared::types::{apps::callbacks::IbcAppCallbackMsg, error::TransferError};

    use crate::types::{msg::TransferMsg, state};

    use super::{ContractError, DepsMut, Env, MessageInfo, Response};

    #[allow(clippy::needless_pass_by_value)]
    pub fn receive(
        _deps: DepsMut,
        _env: Env,
        info: MessageInfo,
        msg: cw20::Cw20ReceiveMsg,
    ) -> Result<Response, ContractError> {
        if !info.funds.is_empty() {
            return Err(TransferError::UnexpectedNativeToken.into());
        }

        // NOTE: We use the sender contract address as the denom.
        let _denom = info.sender.as_str();
        let _transfer_msg: TransferMsg = cosmwasm_std::from_json(msg.msg)?;
        todo!()
    }

    #[allow(clippy::needless_pass_by_value)]
    pub fn receive_ibc_callback(
        deps: DepsMut,
        env: Env,
        info: MessageInfo,
        msg: IbcAppCallbackMsg,
    ) -> Result<Response, ContractError> {
        state::admin::assert_admin(&env, &deps.querier, &info.sender)?;

        match msg {
            IbcAppCallbackMsg::OnSendPacket { .. } => todo!(),
            IbcAppCallbackMsg::OnRecvPacket { .. } => todo!(),
            IbcAppCallbackMsg::OnAcknowledgementPacket { .. } => todo!(),
            IbcAppCallbackMsg::OnTimeoutPacket { .. } => todo!(),
        }
    }
}
