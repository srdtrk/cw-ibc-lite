//! Defines the IBC types used by the contract.

use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Binary, IbcTimeout};
use sha2::Digest;

use super::{error::ContractError, paths::identifiers};

/// Packet defines a type that carries data across different chains through IBC
#[cw_serde]
pub struct Packet {
    /// number corresponds to the order of sends and receives, where a Packet
    /// with an earlier sequence number must be sent and received before a Packet
    /// with a later sequence number.
    pub sequence: identifiers::Sequence,
    /// identifies the port on the sending chain.
    pub source_port: identifiers::PortId,
    /// identifies the channel end on the sending chain.
    pub source_channel: identifiers::ChannelId,
    /// identifies the port on the receiving chain.
    pub destination_port: identifiers::PortId,
    /// identifies the channel end on the receiving chain.
    pub destination_channel: identifiers::ChannelId,
    /// actual opaque bytes transferred directly to the application module
    pub data: Binary,
    /// block height after which the packet times out
    pub timeout: IbcTimeout,
}

/// Acknowledgement is the data returned by an IBC application after processing a packet.
/// It is opaque to the relayer.
pub struct Acknowledgement(Vec<u8>);

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

// /// `MerklePath` is the path used to verify commitment proofs, which can be an
// /// arbitrary structured object (defined by a commitment type).
// /// `MerklePath` is represented from root-to-leaf
// pub struct MerklePath {
//     pub key_path: Vec<String>,
// }

impl Packet {
    /// `to_commitment_bytes` serializes the packet to commitment bytes as per [ibc-lite go implementation](https://github.com/cosmos/ibc-go/blob/2b40562bcd59ce820ddd7d6732940728487cf94e/modules/core/04-channel/types/packet.go#L38)
    ///
    /// # Panics
    /// Panics if the timeout timestamp is not set.
    #[must_use]
    pub fn to_commitment_bytes(&self) -> Vec<u8> {
        let mut buf: Vec<u8> = vec![];
        // timeout timestep should be validated before calling this function
        let timeout_nanoseconds = self.timeout.timestamp().unwrap().nanos();
        // TODO: make sure that revision_number and revision_height can be ignored
        let revision_number = 0_u64;
        let revision_height = 0_u64;
        let data_hash = sha2::Sha256::digest(self.data.as_slice());

        buf.extend_from_slice(&timeout_nanoseconds.to_be_bytes());
        buf.extend_from_slice(&revision_number.to_be_bytes());
        buf.extend_from_slice(&revision_height.to_be_bytes());
        buf.extend_from_slice(&data_hash);
        buf.extend_from_slice(self.destination_port.as_bytes());
        buf.extend_from_slice(self.destination_channel.as_bytes());

        sha2::Sha256::digest(&buf).to_vec()
    }
}

impl Acknowledgement {
    /// Creates a new acknowledgement from the given bytes.
    #[must_use]
    pub fn new(data: Vec<u8>) -> Self {
        Self(data)
    }

    /// Returns the acknowledgement data as a slice.
    #[must_use]
    pub fn as_slice(&self) -> &[u8] {
        &self.0
    }

    /// Returns the serialized commitment bytes of the acknowledgement.
    #[must_use]
    pub fn to_commitment_bytes(&self) -> Vec<u8> {
        sha2::Sha256::digest(&self.0).to_vec()
    }
}

impl TryFrom<cosmwasm_std::Binary> for Acknowledgement {
    type Error = ContractError;

    fn try_from(data: cosmwasm_std::Binary) -> Result<Self, Self::Error> {
        data.0.try_into()
    }
}

impl TryFrom<Vec<u8>> for Acknowledgement {
    type Error = ContractError;

    fn try_from(data: Vec<u8>) -> Result<Self, Self::Error> {
        if data.is_empty() {
            return Err(ContractError::RecvPacketCallbackNoResponse);
        }

        Ok(Self(data))
    }
}

impl From<Acknowledgement> for Vec<u8> {
    fn from(ack: Acknowledgement) -> Self {
        ack.0
    }
}

impl From<Height> for ibc_proto::ibc::core::client::v1::Height {
    fn from(height: Height) -> Self {
        Self {
            revision_number: height.revision_number,
            revision_height: height.revision_height,
        }
    }
}
