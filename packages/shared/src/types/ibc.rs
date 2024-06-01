//! Defines the IBC types used by the contract.

use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Binary, IbcTimeout};

/// Packet defines a type that carries data across different chains through IBC
#[cw_serde]
pub struct Packet {
    /// number corresponds to the order of sends and receives, where a Packet
    /// with an earlier sequence number must be sent and received before a Packet
    /// with a later sequence number.
    pub sequence: u64,
    /// identifies the port on the sending chain.
    pub source_port: String,
    /// identifies the channel end on the sending chain.
    pub source_channel: String,
    /// identifies the port on the receiving chain.
    pub destination_port: String,
    /// identifies the channel end on the receiving chain.
    pub destination_channel: String,
    /// actual opaque bytes transferred directly to the application module
    pub data: Binary,
    /// block height after which the packet times out
    pub timeout: IbcTimeout,
}

/// Height is a monotonically increasing data type
/// that can be compared against another Height for the purposes of updating and
/// freezing clients
///
/// Normally the `RevisionHeight` is incremented at each height while keeping
/// `RevisionNumber` the same. However some consensus algorithms may choose to
/// reset the height in certain conditions e.g. hard forks, state-machine
/// breaking changes In these cases, the `RevisionNumber` is incremented so that
/// height continues to be monitonically increasing even as the `RevisionHeight`
/// gets reset
#[cw_serde]
pub struct Height {
    /// the revision that the client is currently on
    pub revision_number: u64,
    /// the height within the given revision
    pub revision_height: u64,
}

impl Packet {
    /// `to_commitment_bytes` serializes the packet to commitment bytes as per [ibc-lite go implementation](https://github.com/cosmos/ibc-go/blob/2b40562bcd59ce820ddd7d6732940728487cf94e/modules/core/04-channel/types/packet.go#L38)
    #[must_use]
    pub fn to_commitment_bytes(&self) -> Vec<u8> {
        todo!()
    }
}
