//! # Messages
//!
//! This module defines the messages that this contract receives.

use cosmwasm_schema::{cw_serde, QueryResponses};

use cw_ibc_lite_shared::types::apps::helpers::ibc_lite_app_callback;

/// The message to instantiate the contract.
#[cw_serde]
pub struct InstantiateMsg {}

/// The execute messages supported by the contract.
#[ibc_lite_app_callback]
#[cw_serde]
pub enum ExecuteMsg {
    /// This accepts a properly-encoded ReceiveMsg from a cw20 contract
    /// The wrapped message is expected to be [`TransferMsg`].
    Receive(cw20::Cw20ReceiveMsg),
}

/// This is the message we accept via [`ExecuteMsg::Receive`].
#[cw_serde]
pub struct TransferMsg {
    /// The local channel to send the packets on
    pub source_channel: String,
    /// The remote address to send to.
    /// Don't use HumanAddress as this will likely have a different Bech32 prefix than we use
    /// and cannot be validated locally
    pub receiver: String,
    /// How long the packet lives in seconds. If not specified, use default_timeout
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timeout: Option<u64>,
    /// An optional memo to add to the IBC transfer
    #[serde(skip_serializing_if = "Option::is_none")]
    pub memo: Option<String>,
}

/// The query messages supported by the contract.
#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {}
