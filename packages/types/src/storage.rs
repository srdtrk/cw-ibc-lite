//! This module defines the `CosmWasm` storage helper types.

use cosmwasm_std::{Addr, CustomQuery, QuerierWrapper, StdResult, Storage};

use crate::error::ContractError;

/// `PureItem` is used to store [`Vec<u8>`] values.
/// This is useful when you want to store a value similar to a [`cw_storage_plus::Item`]
/// but you don't want to use JSON serialization.
pub struct PureItem {
    storage_key: Vec<u8>,
}

impl PureItem {
    /// Creates a new [`PureItem`] with the given storage key.
    #[must_use]
    pub fn new(storage_key: &str) -> Self {
        Self {
            storage_key: storage_key.as_bytes().to_vec(),
        }
    }

    /// Gets the path of the data to use elsewhere
    #[must_use]
    pub fn as_slice(&self) -> &[u8] {
        self.storage_key.as_slice()
    }

    /// save will serialize the model and store, returns an error on serialization issues
    pub fn save(&self, store: &mut dyn Storage, data: &[u8]) {
        store.set(self.as_slice(), data);
    }

    /// remove will remove the data at the key
    pub fn remove(&self, store: &mut dyn Storage) {
        store.remove(self.as_slice());
    }

    /// `load` will return the data stored at the key
    ///
    /// # Errors
    /// Return [`ContractError::NotFound`] if no data is set at the given key
    pub fn load(&self, store: &dyn Storage) -> Result<Vec<u8>, ContractError> {
        self.may_load(store)
            .ok_or_else(|| ContractError::not_found::<Vec<u8>>(self.as_slice().to_vec()))
    }

    /// `may_load` will return the data stored at the key if present, returns [`None`] if no data there.
    pub fn may_load(&self, store: &dyn Storage) -> Option<Vec<u8>> {
        store.get(self.as_slice())
    }

    /// Returns `true` if data is stored at the key, `false` otherwise.
    pub fn exists(&self, store: &dyn Storage) -> bool {
        store.get(self.as_slice()).is_some()
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
        querier.query_wasm_raw(remote_contract, self.as_slice())
    }
}
