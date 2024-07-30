//! This module contains helper functions for working with [`cosmwasm_std::IbcTimeout`].

use std::str::FromStr;

use cosmwasm_std::{Env, IbcTimeout};
use ibc_core_host::types::identifiers::ChainId;

use crate::types::error::ContractError;

/// Validates an [`IbcTimeout`].
///
/// # Errors
/// Returns an error if the timeout is not a timestamp or if the timestamp is in the past.
pub fn validate(env: &Env, timeout: &IbcTimeout) -> Result<(), ContractError> {
    timeout.block().map_or(Ok(()), |b| {
        if env.block.height >= b.height {
            return Err(ContractError::invalid_timeout_block(
                env.block.height,
                b.height,
            ));
        }

        if ChainId::from_str(&env.block.chain_id)?.revision_number() > b.revision {
            return Err(ContractError::invalid_revision_number(
                env.block.height,
                b.revision,
            ));
        }

        Ok(())
    })?;
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
