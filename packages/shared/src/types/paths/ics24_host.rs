//! This module contains types for provable store keys.

use ibc_core_host::types::path::{
    CHANNEL_PREFIX, PACKET_COMMITMENT_PREFIX, PORT_PREFIX, SEQUENCE_PREFIX,
};

// pub struct PacketCommitmentPath(ibc_core_host::types::path::CommitmentPath);

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn packet_commitment_path() {
        let path = PacketCommitmentPath {
            port_id: "transfer".parse().unwrap(),
            channel_id: "channel-0".parse().unwrap(),
            sequence: 1.into(),
        };

        let expected = format!(
            "{}/{}/{}/{}/{}/{}/{}",
            PACKET_COMMITMENT_PREFIX,
            PORT_PREFIX,
            "transfer",
            CHANNEL_PREFIX,
            "channel-0",
            SEQUENCE_PREFIX,
            1
        );

        assert_eq!(format!("{path}"), expected,);
        assert_eq!(path.to_string(), expected);
    }
}
