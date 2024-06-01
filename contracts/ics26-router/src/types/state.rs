//! This module defines the state storage of the Contract.

use cosmwasm_std::Addr;
use cw_ibc_lite_shared::types::storage::PureItem;

use cw_storage_plus::{Item, Map};

use ibc_core_host::types::path;

/// The map for the next sequence to send.
/// Maps (`port_id`, `channel_id`) to the next sequence to send.
pub const NEXT_SEQUENCE_SEND: Map<(&str, &str), u64> = Map::new("next_sequence_send");

/// The map from port IDs to their associated contract addresses.
/// For now, the port ID is the same as the contract address with the
/// [`super::keys::PORT_ID_PREFIX`] prefix.
pub const IBC_APPS: Map<&str, Addr> = Map::new("ibc_apps");

/// The item for storing the ics02-client router contract address.
pub const ICS02_CLIENT_ADDRESS: Item<Addr> = Item::new("ics02_client_address");

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

/// A collection of methods to access the admin of the contract.
pub mod admin {
    use cosmwasm_std::{Addr, Env, QuerierWrapper};
    use cw_ibc_lite_shared::types::error::ContractError;

    /// Asserts that the given address is the admin of the contract.
    ///
    /// # Errors
    /// Returns an error if the given address is not the admin of the contract or the contract
    /// doesn't have an admin.
    #[allow(clippy::module_name_repetitions)]
    pub fn assert_admin(
        env: &Env,
        querier: &QuerierWrapper,
        addr: &Addr,
    ) -> Result<(), ContractError> {
        let admin = querier
            .query_wasm_contract_info(&env.contract.address)?
            .admin
            .ok_or(ContractError::Unauthorized)?;

        if admin != addr.as_str() {
            return Err(ContractError::Unauthorized);
        }

        Ok(())
    }
}

/// Contains state storage helpers.
pub mod helpers {
    use cosmwasm_std::{StdResult, Storage};
    use cw_ibc_lite_shared::types::ibc::Packet;

    /// Generates a new sequence number for sending packets.
    ///
    /// # Errors
    /// Returns an error if the sequence number cannot be loaded or saved.
    pub fn new_sequence_send(
        storage: &mut dyn Storage,
        port_id: &str,
        channel_id: &str,
    ) -> StdResult<u64> {
        let next_sequence = super::NEXT_SEQUENCE_SEND
            .may_load(storage, (port_id, channel_id))?
            .unwrap_or_default();
        super::NEXT_SEQUENCE_SEND.save(storage, (port_id, channel_id), &(next_sequence + 1))?;
        Ok(next_sequence)
    }

    /// Commits a packet to the provable packet commitment store.
    pub fn commit_packet(storage: &mut dyn Storage, packet: &Packet) {
        let item = super::packet_commitment_item::new(
            &packet.source_port,
            &packet.source_channel,
            packet.sequence,
        );

        item.save(storage, &packet.to_commitment_bytes());
    }
}
