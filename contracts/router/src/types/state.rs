//! This module defines the state storage of the Contract.

use cw_ibc_lite_types::storage::PureItem;

use cw_storage_plus::Map;

use ibc_core_host::types::path;

/// The map for the next sequence to send.
/// Maps (`port_id`, `channel_id`) to the next sequence to send.
pub const NEXT_SEQUENCE_SEND: Map<(String, String), u64> = Map::new("next_sequence_send");

/// A collection of methods to access the packet commitment state.
pub mod packet_commitment_item {
    use super::{path, PureItem};

    /// Returns a new [`PureItem`] for the packet commitment state.
    pub fn new(
        port_id: impl Into<String>,
        channel_id: impl Into<String>,
        sequence: u64,
    ) -> PureItem {
        let key = format!(
            "{}/{}/{}/{}/{}/{}/{}",
            path::PACKET_COMMITMENT_PREFIX,
            path::PORT_PREFIX,
            port_id.into(),
            path::CHANNEL_PREFIX,
            channel_id.into(),
            path::SEQUENCE_PREFIX,
            sequence
        );
        PureItem::new(&key)
    }
}

/// A collection of methods to access the packet acknowledgment state.
pub mod packet_ack_item {
    use super::{path, PureItem};

    /// Returns a new [`PureItem`] for the packet acknowledgment state.
    pub fn new(
        port_id: impl Into<String>,
        channel_id: impl Into<String>,
        sequence: u64,
    ) -> PureItem {
        let key = format!(
            "{}/{}/{}/{}/{}/{}/{}",
            path::PACKET_ACK_PREFIX,
            path::PORT_PREFIX,
            port_id.into(),
            path::CHANNEL_PREFIX,
            channel_id.into(),
            path::SEQUENCE_PREFIX,
            sequence
        );
        PureItem::new(&key)
    }
}

/// A collection of methods to access the packet receipt state.
pub mod packet_receipt_item {
    use super::{path, PureItem};

    /// Returns a new [`PureItem`] for the packet receipt state.
    pub fn new(
        port_id: impl Into<String>,
        channel_id: impl Into<String>,
        sequence: u64,
    ) -> PureItem {
        let key = format!(
            "{}/{}/{}/{}/{}/{}/{}",
            path::PACKET_RECEIPT_PREFIX,
            path::PORT_PREFIX,
            port_id.into(),
            path::CHANNEL_PREFIX,
            channel_id.into(),
            path::SEQUENCE_PREFIX,
            sequence
        );
        PureItem::new(&key)
    }
}
