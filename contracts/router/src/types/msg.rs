//! # Messages
//!
//! This module defines the messages that this contract receives.

use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::{Binary, IbcTimeout};

use super::ibc::{Height, Packet};

/// The message to instantiate the contract.
#[cw_serde]
pub struct InstantiateMsg {}

/// The execute messages supported by the contract.
#[cw_serde]
pub enum ExecuteMsg {
    /// Send a packet to another client.
    /// From https://github.com/cosmos/ibc-go/blob/cf191f4ab3ff27a2e68b3dac17c547669f80102c/modules/core/04-channel/types/tx.pb.go#L563
    SendPacket {
        /// The source client ID.
        source_channel: String,
        /// The source port ID.
        source_port_id: String,
        /// The destination client ID.
        dest_channel: String,
        /// The destination port ID.
        dest_port_id: String,
        /// The packet data to commit.
        data: Binary,
        /// Timeout information for the packet.
        timeout: IbcTimeout,
    },
    /// Receive a packet from another client.
    /// From https://github.com/cosmos/ibc-go/blob/cf191f4ab3ff27a2e68b3dac17c547669f80102c/modules/core/04-channel/types/tx.pb.go#L646
    RecvPacket {
        /// The packet to receive.
        packet: Packet,
        /// The proof of the packet commitment.
        proof_commitment: Binary,
        /// The height of the proof.
        proof_height: Height,
    },
    /// Acknowledge a packet sent to another client.
    /// From https://github.com/cosmos/ibc-go/blob/cf191f4ab3ff27a2e68b3dac17c547669f80102c/modules/core/04-channel/types/tx.pb.go#L887
    Acknowledgement {
        /// The packet to acknowledge.
        packet: Packet,
        /// The acknowledgement data.
        acknowledgement: Binary,
        /// The proof of the acknowledgement.
        proof_acked: Binary,
        /// The height of the proof.
        proof_height: Height,
    },
    /// Timeout a packet sent to another client.
    /// From https://github.com/cosmos/ibc-go/blob/cf191f4ab3ff27a2e68b3dac17c547669f80102c/modules/core/04-channel/types/tx.pb.go#L725
    Timeout {
        /// The packet to timeout.
        packet: Packet,
        /// The proof that the packet is unreceived.
        proof_unreceived: Binary,
        /// The height of the proof.
        proof_height: Height,
        /// The next sequence receive number.
        next_sequence_recv: u64,
    },
}

/// The query messages supported by the contract.
#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {}
