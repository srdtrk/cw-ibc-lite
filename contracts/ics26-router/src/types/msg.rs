//! # Messages
//!
//! This module defines the messages that this contract receives.

use cosmwasm_schema::{cw_serde, QueryResponses};

/// The message to instantiate the contract.
#[cw_serde]
pub struct InstantiateMsg {
    /// cw-ibc-lite-ics02-client router code id
    pub ics02_client_code_id: u64,
}

/// The execute messages supported by the contract.
#[cw_serde]
pub enum ExecuteMsg {
    /// Send a packet to another client.
    SendPacket(execute::SendPacketMsg),
    /// Receive a packet from another client.
    /// From https://github.com/cosmos/ibc-go/blob/cf191f4ab3ff27a2e68b3dac17c547669f80102c/modules/core/04-channel/types/tx.pb.go#L646
    RecvPacket(execute::RecvPacketMsg),
    /// Acknowledge a packet sent to another client.
    /// From https://github.com/cosmos/ibc-go/blob/cf191f4ab3ff27a2e68b3dac17c547669f80102c/modules/core/04-channel/types/tx.pb.go#L887
    Acknowledgement(execute::AcknowledgementMsg),
    /// Timeout a packet sent to another client.
    /// From https://github.com/cosmos/ibc-go/blob/cf191f4ab3ff27a2e68b3dac17c547669f80102c/modules/core/04-channel/types/tx.pb.go#L725
    Timeout(execute::TimeoutMsg),
    /// Anyone can register an IBC app with this contract.
    /// A custom port ID can only be provided if the caller is the admin of the contract.
    RegisterIbcApp {
        /// The port ID of the IBC app. Can only be provided by the admin of the contract.
        /// If not provided, the contract address is used with the [`super::keys::PORT_ID_PREFIX`]
        /// prefix.
        port_id: Option<String>,
        /// The contract address of the IBC app.
        address: String,
    },
}

/// The query messages supported by the contract.
// TODO: Add pagination query support.
#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    /// PortRouter queries the contract address of the IBC app registered with the given port ID.
    #[returns(String)]
    PortRouter {
        /// The port ID of the router.
        port_id: String,
    },
}

/// Contains the messages wrapped by [`super::ExecuteMsg`].
pub mod execute {
    use super::cw_serde;
    use cosmwasm_std::{Binary, IbcTimeout};
    use cw_ibc_lite_shared::types::ibc::{Height, Packet};

    /// The message to send a packet to another client.
    #[cw_serde]
    pub struct SendPacketMsg {
        /// The source client ID.
        pub source_channel: String,
        /// The source port ID.
        pub source_port: String,
        /// The destination client ID.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub dest_channel: Option<String>,
        /// The destination port ID.
        pub dest_port: String,
        /// The packet data to commit.
        pub data: Binary,
        /// Timeout information for the packet.
        pub timeout: IbcTimeout,
        /// The application version.
        pub version: String,
    }

    /// The message to receive a packet from another client.
    #[cw_serde]
    pub struct RecvPacketMsg {
        /// The packet to receive.
        pub packet: Packet,
        /// The proof of the packet commitment.
        pub proof_commitment: Binary,
        /// The height of the proof.
        pub proof_height: Height,
    }

    /// The message to acknowledge a packet sent to another client.
    #[cw_serde]
    pub struct AcknowledgementMsg {
        /// The packet to acknowledge.
        pub packet: Packet,
        /// The acknowledgement data.
        pub acknowledgement: Binary,
        /// The proof of the acknowledgement.
        pub proof_acked: Binary,
        /// The height of the proof.
        pub proof_height: Height,
    }

    /// The message to timeout a packet sent to another client.
    #[cw_serde]
    pub struct TimeoutMsg {
        /// The packet to timeout.
        pub packet: Packet,
        /// The proof that the packet is unreceived.
        pub proof_unreceived: Binary,
        /// The height of the proof.
        pub proof_height: Height,
        /// The next sequence receive number.
        pub next_sequence_recv: u64,
    }

    impl From<SendPacketMsg> for super::ExecuteMsg {
        fn from(msg: SendPacketMsg) -> Self {
            Self::SendPacket(msg)
        }
    }
    impl From<RecvPacketMsg> for super::ExecuteMsg {
        fn from(msg: RecvPacketMsg) -> Self {
            Self::RecvPacket(msg)
        }
    }
    impl From<AcknowledgementMsg> for super::ExecuteMsg {
        fn from(msg: AcknowledgementMsg) -> Self {
            Self::Acknowledgement(msg)
        }
    }
    impl From<TimeoutMsg> for super::ExecuteMsg {
        fn from(msg: TimeoutMsg) -> Self {
            Self::Timeout(msg)
        }
    }
}
