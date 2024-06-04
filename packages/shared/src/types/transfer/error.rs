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
    #[error("unexpected channel ID: expected {expected}, actual {actual}")]
    UnexpectedChannelId { expected: String, actual: String },
    #[error("no foreign tokens allowed")]
    NoForeignTokens,
    #[error("insufficient funds in escrow")]
    InsufficientFundsInEscrow { escrowed: String, requested: String },
    #[error("reentrancy safeguard")]
    Reentrancy,
    #[error("unknown acknowledgement: {0}")]
    UnknownAcknowledgement(String),
}

impl TransferError {
    /// Creates [`TransferError::UnexpectedPortId`] error.
    pub fn unexpected_port_id(expected: impl Into<String>, actual: impl Into<String>) -> Self {
        Self::UnexpectedPortId {
            expected: expected.into(),
            actual: actual.into(),
        }
    }

    /// Creates [`TransferError::UnexpectedChannelId`] error.
    pub fn unexpected_channel_id(expected: impl Into<String>, actual: impl Into<String>) -> Self {
        Self::UnexpectedChannelId {
            expected: expected.into(),
            actual: actual.into(),
        }
    }

    /// Creates [`TransferError::InsufficientFundsInEscrow`] error.
    pub fn insufficient_funds_in_escrow(
        escrowed: impl Into<String>,
        requested: impl Into<String>,
    ) -> Self {
        Self::InsufficientFundsInEscrow {
            escrowed: escrowed.into(),
            requested: requested.into(),
        }
    }
}
