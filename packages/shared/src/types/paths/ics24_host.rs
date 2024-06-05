//! This module contains types for provable store keys.

use ibc_core_host::types::path::{
    CHANNEL_PREFIX, PACKET_ACK_PREFIX, PACKET_COMMITMENT_PREFIX, PACKET_RECEIPT_PREFIX,
    PORT_PREFIX, SEQUENCE_PREFIX,
};

use crate::types::{error::ContractError, storage::PureItem};

// Re-export merkle path from `ibc-client-cw`
pub use ibc_client_cw::types::MerklePath;

/// Path for the packet commitment.
#[derive(
    Clone, Debug, PartialEq, Eq, derive_more::Display, serde::Serialize, serde::Deserialize,
)]
#[display(
    fmt = "{PACKET_COMMITMENT_PREFIX}/{PORT_PREFIX}/{port_id}/{CHANNEL_PREFIX}/{channel_id}/{SEQUENCE_PREFIX}/{sequence}"
)]
pub struct PacketCommitmentPath {
    /// Port identifier.
    pub port_id: super::identifiers::PortId,
    /// Channel identifier.
    pub channel_id: super::identifiers::ChannelId,
    /// Sequence number.
    pub sequence: super::identifiers::Sequence,
}

/// Path for the packet acknowledgement.
#[derive(
    Clone, Debug, PartialEq, Eq, derive_more::Display, serde::Serialize, serde::Deserialize,
)]
#[display(
    fmt = "{PACKET_ACK_PREFIX}/{PORT_PREFIX}/{port_id}/{CHANNEL_PREFIX}/{channel_id}/{SEQUENCE_PREFIX}/{sequence}"
)]
pub struct PacketAcknowledgementPath {
    /// Port identifier.
    pub port_id: super::identifiers::PortId,
    /// Channel identifier.
    pub channel_id: super::identifiers::ChannelId,
    /// Sequence number.
    pub sequence: super::identifiers::Sequence,
}

/// Path for the packet receipt.
#[derive(
    Clone, Debug, PartialEq, Eq, derive_more::Display, serde::Serialize, serde::Deserialize,
)]
#[display(
    fmt = "{PACKET_RECEIPT_PREFIX}/{PORT_PREFIX}/{port_id}/{CHANNEL_PREFIX}/{channel_id}/{SEQUENCE_PREFIX}/{sequence}"
)]
pub struct PacketReceiptPath {
    /// Port identifier.
    pub port_id: super::identifiers::PortId,
    /// Channel identifier.
    pub channel_id: super::identifiers::ChannelId,
    /// Sequence number.
    pub sequence: super::identifiers::Sequence,
}

impl PacketCommitmentPath {
    /// Converts the path to a prefixed merkle path.
    /// If a prefix is provided, the path is appended to the prefix.
    ///
    /// # Errors
    /// Returns an error if the prefix is provided and is empty.
    pub fn to_prefixed_merkle_path(
        &self,
        prefix: Option<MerklePath>,
    ) -> Result<MerklePath, ContractError> {
        let mut prefix = prefix.unwrap_or(MerklePath {
            key_path: vec![String::new()],
        });

        prefix
            .key_path
            .last_mut()
            .ok_or(ContractError::EmptyMerklePrefix)?
            .push_str(&self.to_string());

        Ok(prefix)
    }
}

impl PacketAcknowledgementPath {
    /// Converts the path to a prefixed merkle path.
    /// If a prefix is provided, the path is appended to the prefix.
    ///
    /// # Errors
    /// Returns an error if the prefix is provided and is empty.
    pub fn to_prefixed_merkle_path(
        &self,
        prefix: Option<MerklePath>,
    ) -> Result<MerklePath, ContractError> {
        let mut prefix = prefix.unwrap_or(MerklePath {
            key_path: vec![String::new()],
        });

        prefix
            .key_path
            .last_mut()
            .ok_or(ContractError::EmptyMerklePrefix)?
            .push_str(&self.to_string());

        Ok(prefix)
    }
}

impl PacketReceiptPath {
    /// Converts the path to a prefixed merkle path.
    /// If a prefix is provided, the path is appended to the prefix.
    ///
    /// # Errors
    /// Returns an error if the prefix is provided and is empty.
    pub fn to_prefixed_merkle_path(
        &self,
        prefix: Option<MerklePath>,
    ) -> Result<MerklePath, ContractError> {
        let mut prefix = prefix.unwrap_or(MerklePath {
            key_path: vec![String::new()],
        });

        prefix
            .key_path
            .last_mut()
            .ok_or(ContractError::EmptyMerklePrefix)?
            .push_str(&self.to_string());

        Ok(prefix)
    }
}

impl From<PacketCommitmentPath> for PureItem {
    fn from(path: PacketCommitmentPath) -> Self {
        Self::new(&path.to_string())
    }
}
impl From<PacketAcknowledgementPath> for PureItem {
    fn from(path: PacketAcknowledgementPath) -> Self {
        Self::new(&path.to_string())
    }
}
impl From<PacketReceiptPath> for PureItem {
    fn from(path: PacketReceiptPath) -> Self {
        Self::new(&path.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn packet_commitment_path() {
        let path = PacketCommitmentPath {
            port_id: "transfer".parse().unwrap(),
            channel_id: "08-wasm-0".parse().unwrap(),
            sequence: 1.into(),
        };

        let expected = format!(
            "{}/{}/{}/{}/{}/{}/{}",
            PACKET_COMMITMENT_PREFIX,
            PORT_PREFIX,
            "transfer",
            CHANNEL_PREFIX,
            "08-wasm-0",
            SEQUENCE_PREFIX,
            1
        );

        assert_eq!(format!("{path}"), expected,);
        assert_eq!(path.to_string(), expected);
    }
}
