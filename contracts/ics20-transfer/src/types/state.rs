//! This module defines the state storage of the Contract.

use cosmwasm_std::Uint128;
use cw_storage_plus::{Item, Map};

/// The item that stores the escrowed tokens per denom.
/// It maps (`channel_id`, `denom`) to the escrowed amount.
pub const ESCROW: Map<(&str, &str), Uint128> = Map::new("escrow");

/// Used to pass state to the reply handler in `on_recv_packet`.
const RECV_PACKET_REPLY_ARGS: Item<RecvPacketReplyArgs> = Item::new("recv_packet_reply_args");

/// Used to pass state to the reply handler in `on_recv_packet`.
#[cosmwasm_schema::cw_serde]
pub struct RecvPacketReplyArgs {
    /// The channel identifier.
    pub channel_id: String,
    /// The denomination of the transferred tokens.
    pub denom: String,
    /// The amount of tokens transferred.
    pub amount: Uint128,
}

/// Contains state storage helpers.
pub mod helpers {
    use cw_ibc_lite_shared::types::{error::ContractError, transfer::error::TransferError};

    /// Stores the `RecvPacketReplyArgs` in the contract state.
    ///
    /// # Errors
    /// Returns an error if the storage operation fails or there is an existing value.
    pub fn store_recv_packet_reply_args(
        storage: &mut dyn cosmwasm_std::Storage,
        args: &super::RecvPacketReplyArgs,
    ) -> Result<(), ContractError> {
        if super::RECV_PACKET_REPLY_ARGS.exists(storage) {
            return Err(TransferError::Reentrancy.into());
        }

        super::RECV_PACKET_REPLY_ARGS.save(storage, args)?;
        Ok(())
    }

    /// Loads and removes the `RecvPacketReplyArgs` from the contract state.
    ///
    /// # Errors
    /// Returns an error if the load operation fails.
    pub fn load_recv_packet_reply_args(
        storage: &mut dyn cosmwasm_std::Storage,
    ) -> Result<super::RecvPacketReplyArgs, ContractError> {
        let args = super::RECV_PACKET_REPLY_ARGS.load(storage)?;
        super::RECV_PACKET_REPLY_ARGS.remove(storage);
        Ok(args)
    }
}
