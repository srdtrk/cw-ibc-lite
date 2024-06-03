//! Shared utils for IBC applications written for `cw-ibc-lite`.

use std::str::FromStr;

use crate::types::{error::ContractError, paths::identifiers::PortId};
use cosmwasm_std::Addr;

/// `PORT_ID_PREFIX` is the prefix of the port id
pub const PORT_ID_PREFIX: &str = "wasm.";

/// Extracts the port identifier from the given address.
///
/// # Errors
/// Returns an error if [`PortId::from_str`] fails.
pub fn contract_port_id(address: &Addr) -> Result<PortId, ContractError> {
    Ok(PortId::from_str(&format!(
        "{}{}",
        PORT_ID_PREFIX,
        address.as_str()
    ))?)
}
