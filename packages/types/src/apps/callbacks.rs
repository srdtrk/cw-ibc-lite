//! # Callbacks
//!
//! This module contains the callbacks message types that IBC applications built with `cw-ibc-lite`
//! must implement.

use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Binary, StdResult};

/// All IBC applications built with `cw-ibc-lite` must handle these callback messages.
#[cw_serde]
pub enum IbcAppCallbackMsg {
    /// Called when a packet is sent to this IBC application.
    /// This callback needs to be responded with [`response::AcknowledgementData`].
    OnRecvPacket {
        /// The packet that was received.
        packet: crate::ibc::Packet,
        /// The relayer address that submitted the packet.
        relayer: String,
    },
    /// Called when a packet to be acknowledged by this IBC application.
    /// This callback need not be responded with a response.
    OnAcknowledgementPacket {
        /// The packet to acknowledge.
        packet: crate::ibc::Packet,
        /// The acknowledgement data.
        acknowledgement: Binary,
        /// The relayer address that submitted the acknowledgement.
        relayer: String,
    },
    /// Called when a packet to be timed out by this IBC application.
    /// This callback need not be responded with a response.
    OnTimeoutPacket {
        /// The packet to timeout.
        packet: crate::ibc::Packet,
        /// The relayer address that submitted the timeout.
        relayer: String,
    },
}

/// This is just a helper to properly serialize [`IbcAppCallbackMsg`].
/// The actual receiver should include this variant in the larger ExecuteMsg enum
#[cw_serde]
enum ReceiverExecuteMsg {
    ReceiveIbcAppCallback(IbcAppCallbackMsg),
}

/// This module lists the response types for the callbacks.
pub mod response {
    /// The response to [`super::IbcAppCallbackMsg::OnRecvPacket`].
    #[super::cw_serde]
    pub enum AcknowledgementData {
        /// The acknowledgement data upon successful processing of the packet.
        Result(super::Binary),
        /// The error message upon failure to process the packet.
        Error(String),
    }
}

impl IbcAppCallbackMsg {
    /// serializes the message
    ///
    /// # Errors
    ///
    /// This function returns an error if the message cannot be serialized.
    pub fn into_json_binary(self) -> StdResult<Binary> {
        let msg = ReceiverExecuteMsg::ReceiveIbcAppCallback(self);
        cosmwasm_std::to_json_binary(&msg)
    }

    /// `into_cosmos_msg` converts this message into a [`CosmosMsg`] message to be sent to
    /// the named contract.
    ///
    /// # Errors
    ///
    /// This function returns an error if the message cannot be serialized.
    pub fn into_cosmos_msg<C>(
        self,
        contract_addr: impl Into<String>,
    ) -> StdResult<cosmwasm_std::CosmosMsg<C>>
    where
        C: Clone + std::fmt::Debug + PartialEq,
    {
        let execute = cosmwasm_std::WasmMsg::Execute {
            contract_addr: contract_addr.into(),
            msg: self.into_json_binary()?,
            funds: vec![],
        };

        Ok(execute.into())
    }
}
