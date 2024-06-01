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
    #[error("{0}")]
    FromUTF8Error(#[from] std::string::FromUtf8Error),
    #[error("{0}")]
    UTF8Error(#[from] std::str::Utf8Error),

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
    #[error("unknown reply id: {0}")]
    UnknownReplyId(u64),

    #[error("counterparty already provided")]
    CounterpartyAlreadyProvided,
    #[error("invalid counterparty: expected {expected}, actual {actual}")]
    InvalidCounterparty { expected: String, actual: String },
    #[error("this contract does not accept block height for timeout, use timestamp")]
    InvalidTimeoutHeight,
    #[error(
        "invalid timeout timestamp: current {current}, timestamp {timestamp} (seconds since epoch)"
    )]
    InvalidTimeoutTimestamp { current: u64, timestamp: u64 },
    #[error("empty timestamp")]
    EmptyTimestamp,
    #[error("packet already commited: key: {:02x?}", key)]
    PacketAlreadyCommited { key: Vec<u8> },
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

    /// Returns a new [`ContractError::InvalidCounterparty`] with the given expected and actual
    /// values.
    #[must_use]
    pub const fn invalid_counterparty(expected: String, actual: String) -> Self {
        Self::InvalidCounterparty { expected, actual }
    }

    /// Returns a new [`ContractError::InvalidTimeoutTimestamp`] with the given current and
    /// timestamp values.
    #[must_use]
    pub const fn invalid_timeout_timestamp(current: u64, timestamp: u64) -> Self {
        Self::InvalidTimeoutTimestamp { current, timestamp }
    }

    /// Returns a new [`ContractError::PacketAlreadyCommited`] with the given key.
    #[must_use]
    pub const fn packet_already_commited(key: Vec<u8>) -> Self {
        Self::PacketAlreadyCommited { key }
    }
}

impl From<ibc_client_cw::types::ContractError> for ContractError {
    fn from(error: ibc_client_cw::types::ContractError) -> Self {
        Self::Std(error.into())
    }
}
