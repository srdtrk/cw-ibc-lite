//! # Messages
//!
//! This module defines the messages that this contract receives.

use cosmwasm_schema::{cw_serde, QueryResponses};

use cw_ibc_lite_shared::types::apps::helpers::ibc_lite_app_callback;

/// The message to instantiate the contract.
#[cw_serde]
pub struct InstantiateMsg {
    /// The contract address allowed to make IBC callbacks.
    pub ics26_router_address: String,
}

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
#[cw_ownable::cw_ownable_query]
#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    /// The escrowed amount for the given channel and cw20 contract address.
    #[returns(cosmwasm_std::Uint128)]
    EscrowAmount {
        /// The channel identifier.
        channel: String,
        /// The cw20 contract address.
        cw20_address: String,
    },
    /// The list of all escrows for the given channel.
    /// Returns (cw20_address, amount) pairs.
    #[returns(responses::EscrowList)]
    ListEscrows {
        /// The channel identifier.
        channel: String,
        /// start pagination after this contract address
        #[serde(skip_serializing_if = "Option::is_none")]
        start_after: Option<String>,
        /// limit results to this number
        #[serde(skip_serializing_if = "Option::is_none")]
        limit: Option<u32>,
    },
}

/// Contains the query responses
pub mod responses {
    use cosmwasm_std::Uint128;

    /// Response to [`super::QueryMsg::ListEscrows`]
    #[super::cw_serde]
    pub struct EscrowList {
        /// List of escrow infos
        pub list: Vec<EscrowInfo>,
    }

    /// Information on the escrowed amount for a given channel and cw20 address
    #[super::cw_serde]
    pub struct EscrowInfo {
        /// The channel identifier of the escrowed amount
        pub channel: String,
        /// The address of the cw20 token contract
        pub cw20_address: String,
        /// Amount of funds escrowed
        pub amount: Uint128,
    }
}
