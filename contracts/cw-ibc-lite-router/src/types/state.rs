//! This module defines the state storage of the Contract.

use cw_storage_plus::{Item, Map};

use ibc_core_host::types::path;

/// The map for the next sequence to send.
/// Maps (`port_id`, `channel_id`) to the next sequence to send.
pub const NEXT_SEQUENCE_SEND: Map<(String, String), u64> = Map::new("next_sequence_send");

/// Returns the packet commitment store for a packet
#[must_use]
pub fn packet_commitment(port_id: &str, channel_id: &str, sequence: u64) -> Item<u64> {
    let key = format!(
        "{}/{}/{}/{}/{}/{}/{}",
        path::PACKET_COMMITMENT_PREFIX,
        path::PORT_PREFIX,
        port_id,
        path::CHANNEL_PREFIX,
        channel_id,
        path::SEQUENCE_PREFIX,
        sequence
    );

    Item::new_dyn(key)
}
