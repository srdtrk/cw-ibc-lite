//! This module contains helper functions for working with [`cosmwasm_std::IbcTimeout`].

use cosmwasm_std::{Env, IbcTimeout};

use crate::types::error::ContractError;

/// Validates an [`IbcTimeout`].
///
/// # Errors
/// Returns an error if the timeout is not a timestamp or if the timestamp is in the past.
pub fn validate(env: &Env, timeout: &IbcTimeout) -> Result<(), ContractError> {
    if timeout.block().is_some() {
        return Err(ContractError::InvalidTimeoutHeight);
    }
    timeout
        .timestamp()
        .ok_or(ContractError::EmptyTimestamp)
        .and_then(|ts| {
            if env.block.time >= ts {
                return Err(ContractError::invalid_timeout_timestamp(
                    env.block.time.seconds(),
                    ts.seconds(),
                ));
            }

            Ok(())
        })
}
