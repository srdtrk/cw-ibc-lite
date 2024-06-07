//! Exposes the Tendermint client and consensus states.

use ibc_client_cw::api::ClientType;
use ibc_client_tendermint::client_state::ClientState;
use ibc_client_tendermint::consensus_state::ConsensusState;

/// Tendermint client type as defined in ibc-rs.
#[derive(Clone, Debug)]
pub struct TendermintClient;

impl<'a> ClientType<'a> for TendermintClient {
    type ClientState = ClientState;
    type ConsensusState = ConsensusState;
}

/// Contain the owner key and helper functions to interact with it.
// TODO: Switch to cw_ownable, it was removed since otherwise this contract exceeded the
// maximum allowed contract size by wasmd. This is a temporary solution until the contract is
// smaller.
pub mod owner {
    use cw_ibc_lite_shared::types::error::ContractError;

    /// The key to store the owner of the contract.
    pub const OWNER_KEY: &[u8] = b"owner";

    /// Set the owner of the contract.
    pub fn set(storage: &mut dyn cosmwasm_std::Storage, owner: &str) {
        storage.set(OWNER_KEY, owner.as_bytes());
    }

    /// Get the owner of the contract.
    ///
    /// # Errors
    /// Will return an error if the owner is not set or if the owner is not valid UTF-8.
    pub fn get(storage: &dyn cosmwasm_std::Storage) -> Result<String, ContractError> {
        storage
            .get(OWNER_KEY)
            .ok_or(ContractError::Unauthorized)
            .and_then(|v| String::from_utf8(v).map_err(ContractError::from))
    }

    /// Assert the owner of the contract.
    ///
    /// # Errors
    /// Will return an error if the owner is not set or if the owner is not valid UTF-8.
    /// Will return an error if the address is not the owner.
    pub fn assert(storage: &dyn cosmwasm_std::Storage, address: &str) -> Result<(), ContractError> {
        let owner = get(storage)?;
        if owner != address {
            return Err(ContractError::Unauthorized);
        }

        Ok(())
    }
}
