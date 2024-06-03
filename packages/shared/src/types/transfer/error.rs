//! Defines shared error types for ICS-20 transfer application.

/// `TransferError` is the error type returned by the ics20 transfer contract.
#[allow(missing_docs, clippy::module_name_repetitions)]
#[non_exhaustive]
#[derive(thiserror::Error, Debug)]
pub enum TransferError {
    #[error("unexpected native token")]
    UnexpectedNativeToken,
    #[error("zero amount")]
    ZeroAmount,
    #[error("empty denom")]
    EmptyDenom,
    #[error("amount overflow")]
    AmountOverflow,
    #[error("receiver cannot be empty")]
    EmptyReceiver,
    #[error("empty sender")]
    EmptySender,
    #[error("invalid ICS-20 version")]
    InvalidVersion,
    #[error("invalid port ID: expected {expected}, actual {actual}")]
    UnexpectedPortId { expected: String, actual: String },
}

impl TransferError {
    /// Creates [`TransferError::UnexpectedPortId`] error.
    pub fn unexpected_port_id(expected: impl Into<String>, actual: impl Into<String>) -> Self {
        Self::UnexpectedPortId {
            expected: expected.into(),
            actual: actual.into(),
        }
    }
}
