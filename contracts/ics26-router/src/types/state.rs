//! This module defines the state storage of the Contract.

use cosmwasm_std::Addr;
use cw_ibc_lite_shared::types::ibc;

use cw_storage_plus::{Item, Map};

/// The map for the next sequence to send.
/// Maps (`port_id`, `channel_id`) to the next sequence to send.
pub const NEXT_SEQUENCE_SEND: Map<(&str, &str), u64> = Map::new("next_sequence_send");

/// The map from port IDs to their associated contract addresses.
/// For now, the port ID is the same as the contract address with the
/// [`super::keys::PORT_ID_PREFIX`] prefix.
pub const IBC_APPS: Map<&str, Addr> = Map::new("ibc_apps");

/// The item for storing the ics02-client router contract address.
pub const ICS02_CLIENT_ADDRESS: Item<Addr> = Item::new("ics02_client_address");

/// The item for storing [`ibc::Packet`] to the temporary store for replies.
// TODO: remove this in CosmWasm v2 since it introduces the ability to add custom data to reply.
pub const PACKET_TEMP_STORE: Item<ibc::Packet> = Item::new("recv_packet_temp_store");

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
    use cw_ibc_lite_shared::types::{
        error::ContractError,
        ibc,
        paths::ics24_host::{PacketAcknowledgementPath, PacketCommitmentPath, PacketReceiptPath},
        storage::PureItem,
    };

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
    ///
    /// # Errors
    /// Returns an error if the packet has already been committed.
    pub fn commit_packet(
        storage: &mut dyn Storage,
        packet: &ibc::Packet,
    ) -> Result<(), ContractError> {
        let item: PureItem = PacketCommitmentPath {
            port_id: packet.source_port.clone(),
            channel_id: packet.source_channel.clone(),
            sequence: packet.sequence,
        }
        .into();

        if item.exists(storage) {
            return Err(ContractError::packet_already_commited(
                item.as_slice().to_vec(),
            ));
        }

        item.save(storage, &packet.to_commitment_bytes());
        Ok(())
    }

    /// Deletes a packet commitment from the provable packet commitment store.
    ///
    /// # Errors
    /// Returns an error if the packet commitment cannot be found.
    pub fn delete_packet_commitment(
        storage: &mut dyn Storage,
        packet: &ibc::Packet,
    ) -> Result<(), ContractError> {
        let item: PureItem = PacketCommitmentPath {
            port_id: packet.source_port.clone(),
            channel_id: packet.source_channel.clone(),
            sequence: packet.sequence,
        }
        .into();

        // NOTE: These consume extra gas indeed. We can remove these if this is an issue.
        if !item.exists(storage) {
            return Err(ContractError::packet_commitment_not_found(
                item.as_slice().to_vec(),
            ));
        }

        item.remove(storage);
        Ok(())
    }

    /// Sets the packet receipt in the provable packet receipt store.
    /// This is used to prevent replay.
    ///
    /// # Errors
    /// Returns an error if the receipt has already been committed.
    pub fn set_packet_receipt(
        storage: &mut dyn Storage,
        packet: &ibc::Packet,
    ) -> Result<(), ContractError> {
        let item: PureItem = PacketReceiptPath {
            port_id: packet.destination_port.clone(),
            channel_id: packet.destination_channel.clone(),
            sequence: packet.sequence,
        }
        .into();

        if item.exists(storage) {
            return Err(ContractError::packet_already_commited(
                item.as_slice().to_vec(),
            ));
        }

        item.save(storage, &[1]);
        Ok(())
    }

    /// Commits an acknowledgment to the provable packet acknowledgment store.
    /// This is used to prove the `AcknowledgementPacket` in the counterparty chain.
    ///
    /// # Errors
    /// Returns an error if the acknowledgment has already been committed.
    pub fn commit_packet_ack(
        storage: &mut dyn Storage,
        packet: &ibc::Packet,
        ack: &ibc::Acknowledgement,
    ) -> Result<(), ContractError> {
        let item: PureItem = PacketAcknowledgementPath {
            port_id: packet.destination_port.clone(),
            channel_id: packet.destination_channel.clone(),
            sequence: packet.sequence,
        }
        .into();

        if item.exists(storage) {
            return Err(ContractError::packet_already_commited(
                item.as_slice().to_vec(),
            ));
        }

        item.save(storage, &ack.to_commitment_bytes());
        Ok(())
    }

    /// Saves the packet to [`super::PACKET_TEMP_STORE`].
    ///
    /// # Errors
    /// Returns an error if the packet has already been committed.
    pub fn save_packet_temp_store(
        storage: &mut dyn Storage,
        packet: &ibc::Packet,
    ) -> Result<(), ContractError> {
        if super::PACKET_TEMP_STORE.exists(storage) {
            return Err(ContractError::packet_already_commited(
                super::PACKET_TEMP_STORE.as_slice().to_vec(),
            ));
        }

        Ok(super::PACKET_TEMP_STORE.save(storage, packet)?)
    }

    /// Loads and removes the packet from the temporary store for the reply to
    ///
    /// # Errors
    /// Returns an error if the packet identifier cannot be loaded.
    pub fn remove_packet_temp_store(
        storage: &mut dyn Storage,
    ) -> Result<ibc::Packet, ContractError> {
        let packet = super::PACKET_TEMP_STORE.load(storage)?;
        super::PACKET_TEMP_STORE.remove(storage);
        Ok(packet)
    }
}
