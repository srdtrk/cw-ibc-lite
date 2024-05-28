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

    #[error("Unauthorized")]
    Unauthorized {},
    // Add any other custom errors you like here.
    // Look at https://docs.rs/thiserror/1.0.21/thiserror/ for details.
}
