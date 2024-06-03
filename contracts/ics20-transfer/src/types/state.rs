//! This module defines the state storage of the Contract.

use cosmwasm_std::Uint128;
use cw_storage_plus::Map;

/// The item that stores the escrowed tokens per denom.
/// It maps (`channel_id`, `denom`) to the escrowed amount.
pub const ESCROW: Map<(&str, &str), Uint128> = Map::new("escrow");
