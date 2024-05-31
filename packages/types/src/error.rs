//! This module defines [`ContractError`].

use cosmwasm_std::StdError;
use thiserror::Error;

/// `ContractError` is the error type returned by contract's functions.
#[allow(missing_docs)]
#[allow(clippy::module_name_repetitions)]
#[derive(Error, Debug)]
pub enum ContractError {
    #[error("{0}")]
    Std(#[from] StdError),
    #[error("{0}")]
    OwnershipError(#[from] cw_ownable::OwnershipError),

    #[error("unauthorized")]
    Unauthorized,
    // format!("type: {type_name}; key: {:02x?}", key)
    #[error("not found: {type_name} with key {key:?}")]
    NotFound { type_name: String, key: Vec<u8> },
    #[error("try_from failed: {source_type} -> {target_type}")]
    TryFrom {
        source_type: String,
        target_type: String,
    },

    #[error("counterparty already provided")]
    CounterpartyAlreadyProvided,
}

impl ContractError {
    /// Returns a new [`ContractError::NotFound`] with the given type name and key.
    #[must_use]
    pub fn not_found<T>(key: Vec<u8>) -> Self {
        Self::NotFound {
            type_name: std::any::type_name::<T>().to_string(),
            key,
        }
    }

    /// Returns a new [`ContractError::TryFrom`] with the given source and target types.
    #[must_use]
    pub fn try_from<S, T>() -> Self {
        Self::TryFrom {
            source_type: std::any::type_name::<S>().to_string(),
            target_type: std::any::type_name::<T>().to_string(),
        }
    }
}

impl From<ibc_client_cw::types::ContractError> for ContractError {
    fn from(error: ibc_client_cw::types::ContractError) -> Self {
        Self::Std(error.into())
    }
}
