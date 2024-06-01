//! This module defines the state storage of the Contract.

use cosmwasm_std::Addr;
use cw_storage_plus::{Item, Map};
use ibc_client_cw::types::MerklePath;

/// `NEXT_CLIENT_NUMBER` is the item that stores the next client number.
pub const NEXT_CLIENT_NUMBER: Item<u64> = Item::new("client_number");

/// `CLIENTS` is the map of all client ids to their contract address.
/// The reverse mapping should not be needed as the client should be responding with a reply.
pub const CLIENTS: Map<&str, Addr> = Map::new("clients");

/// `COUNTERPARTY` is the map of all client ids to their [`CounterpartyInfo`].
pub const COUNTERPARTY: Map<&str, CounterpartyInfo> = Map::new("counterparty");

/// `CREATORS` is the map of all client ids to their creator address.
pub const CREATORS: Map<&str, Addr> = Map::new("creators");

/// Counterparty client information.
#[cosmwasm_schema::cw_serde]
pub struct CounterpartyInfo {
    /// The client id of the counterparty.
    pub client_id: String,
    /// The merkle path prefix of the counterparty.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub merkle_path_prefix: Option<MerklePath>,
}

/// Contains state storage helpers.
pub mod helpers {
    use cosmwasm_std::{StdResult, Storage};
    use cw_ibc_lite_shared::types::error::ContractError;

    use crate::types::keys;

    /// Generates a new client id and increments the client number.
    ///
    /// # Errors
    /// Returns an error if the client number cannot be loaded or saved.
    pub fn new_client_id(storage: &mut dyn Storage) -> StdResult<String> {
        let client_number = super::NEXT_CLIENT_NUMBER
            .may_load(storage)?
            .unwrap_or_default();
        super::NEXT_CLIENT_NUMBER.save(storage, &(client_number + 1))?;

        Ok(format!("{}{}", keys::CLIENT_ID_PREFIX, client_number))
    }

    /// Asserts that the given creator is the creator of the client.
    ///
    /// # Errors
    /// Returns an error if the creator cannot be loaded or if the creator is not the same as the
    /// given creator.
    pub fn assert_creator(
        storage: &dyn Storage,
        client_id: &str,
        creator: &cosmwasm_std::Addr,
    ) -> Result<(), ContractError> {
        let stored_creator = super::CREATORS.load(storage, client_id)?;
        if stored_creator != creator {
            return Err(ContractError::Unauthorized);
        }
        Ok(())
    }
}
