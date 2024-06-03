//! This module defines the packet data structure for the ICS20 transfer protocol.

use cosmwasm_std::Uint128;

use super::error::TransferError;

/// The format for sending an ics20 packet.
/// Proto defined here: https://github.com/cosmos/cosmos-sdk/blob/v0.42.0/proto/ibc/applications/transfer/v1/transfer.proto#L11-L20
/// This is compatible with the JSON serialization
#[allow(clippy::module_name_repetitions)]
#[cosmwasm_schema::cw_serde]
pub struct Ics20Packet {
    /// amount of tokens to transfer is encoded as a string, but limited to u64 max
    pub amount: Uint128,
    /// the token denomination to be transferred
    pub denom: String,
    /// the recipient address on the destination chain
    pub receiver: String,
    /// the sender address
    pub sender: String,
    /// optional memo for the IBC transfer
    #[serde(skip_serializing_if = "Option::is_none")]
    pub memo: Option<String>,
}

impl Ics20Packet {
    /// Creates and validates a new ICS20 packet with the given data.
    ///
    /// # Errors
    /// Returns an error if [`Self::validate`] fails.
    pub fn try_new(
        amount: Uint128,
        denom: String,
        receiver: String,
        sender: String,
        memo: Option<String>,
    ) -> Result<Self, TransferError> {
        let packet = Self {
            amount,
            denom,
            receiver,
            sender,
            memo,
        };
        packet.validate()?;
        Ok(packet)
    }

    /// Validates the ICS20 packet.
    ///
    /// # Errors
    /// Returns an error if the amount is zero, the amount overflows u64, the denom is empty, the
    /// receiver is empty, or the sender is empty.
    pub fn validate(&self) -> Result<(), TransferError> {
        if self.amount.is_zero() {
            return Err(TransferError::ZeroAmount);
        }
        if self.amount.u128() > u64::MAX.into() {
            return Err(TransferError::AmountOverflow);
        }
        if self.denom.is_empty() {
            return Err(TransferError::EmptyDenom);
        }
        if self.receiver.is_empty() {
            return Err(TransferError::EmptyReceiver);
        }
        if self.sender.is_empty() {
            return Err(TransferError::EmptySender);
        }

        Ok(())
    }
}