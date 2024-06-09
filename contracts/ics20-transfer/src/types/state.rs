//! This module defines the state storage of the Contract.

use cosmwasm_std::Uint128;
use cw_storage_plus::Map;

/// The item that stores the escrowed tokens per denom.
/// It maps (`channel_id`, `denom`) to the escrowed amount.
pub const ESCROW: Map<(&str, &str), Uint128> = Map::new("escrow");

/// Used to pass state to the reply handler in `on_recv_packet`.
#[cosmwasm_schema::cw_serde]
pub struct RecvPacketReplyPayload {
    /// The channel identifier.
    pub channel_id: String,
    /// The denomination of the transferred tokens.
    pub denom: String,
    /// The amount of tokens transferred.
    pub amount: Uint128,
}
