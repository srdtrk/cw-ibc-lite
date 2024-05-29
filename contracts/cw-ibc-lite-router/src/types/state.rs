//! This module defines the state storage of the Contract.

use super::ContractError;

use cosmwasm_std::Storage;
use cw_storage_plus::Map;

use ibc_core_host::types::path;

pub use helpers::PureItem;

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
        PureItem::new_dyn(key)
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
        PureItem::new_dyn(key)
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
        PureItem::new_dyn(key)
    }
}

mod helpers {
    use super::{ContractError, Storage};

    use cosmwasm_std::{Addr, CustomQuery, QuerierWrapper, StdResult};
    use cw_storage_plus::Namespace;

    /// `PureItem` is used to store [`Vec<u8>`] values.
    /// This is useful when you want to store a value similar to a [`cw_storage_plus::Item`]
    /// but you don't want to use JSON serialization.
    pub struct PureItem {
        storage_key: Namespace,
    }

    impl PureItem {
        /// Creates a new [`Item`] with the given storage key. This is a const fn only suitable
        /// when you have a static string slice.
        #[must_use]
        pub const fn new(storage_key: &'static str) -> Self {
            Self {
                storage_key: Namespace::from_static_str(storage_key),
            }
        }

        /// Creates a new [`Item`] with the given storage key. Use this if you might need to handle
        /// a dynamic string. Otherwise, you might prefer [`Item::new`].
        pub fn new_dyn(storage_key: impl Into<Namespace>) -> Self {
            Self {
                storage_key: storage_key.into(),
            }
        }

        /// Gets the path of the data to use elsewhere
        #[must_use]
        pub fn as_slice(&self) -> &[u8] {
            self.storage_key.as_slice()
        }

        /// save will serialize the model and store, returns an error on serialization issues
        pub fn save(&self, store: &mut dyn Storage, data: &[u8]) {
            store.set(self.storage_key.as_slice(), data);
        }

        /// remove will remove the data at the key
        pub fn remove(&self, store: &mut dyn Storage) {
            store.remove(self.storage_key.as_slice());
        }

        /// `load` will return the data stored at the key
        ///
        /// # Errors
        /// Return [`ContractError::NotFound`] if no data is set at the given key
        pub fn load(&self, store: &dyn Storage) -> Result<Vec<u8>, ContractError> {
            self.may_load(store).ok_or_else(|| {
                ContractError::not_found::<Vec<u8>>(self.storage_key.as_slice().to_vec())
            })
        }

        /// `may_load` will return the data stored at the key if present, returns [`None`] if no data there.
        pub fn may_load(&self, store: &dyn Storage) -> Option<Vec<u8>> {
            store.get(self.storage_key.as_slice())
        }

        /// Returns `true` if data is stored at the key, `false` otherwise.
        pub fn exists(&self, store: &dyn Storage) -> bool {
            store.get(self.storage_key.as_slice()).is_some()
        }

        /// If you import [`PureItem`] from the remote contract, this will let you read the data
        /// from a remote contract using [`WasmQuery::Raw`]. Returns `Ok(None)` if no data is set.
        ///
        /// # Errors
        /// It only returns error on some runtime issue, not on any data cases.
        pub fn query<Q: CustomQuery>(
            &self,
            querier: &QuerierWrapper<Q>,
            remote_contract: Addr,
        ) -> StdResult<Option<Vec<u8>>> {
            querier.query_wasm_raw(remote_contract, self.storage_key.as_slice())
        }
    }
}
