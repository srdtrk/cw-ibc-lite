//! Shared utils for ICS-20 transfer.

use crate::types::transfer::error::TransferError;

/// Returns local denom if the denom is an encoded voucher from the expected endpoint
///
/// # Errors
/// Returns an error if the voucher denom is not in the expected format.
pub fn parse_voucher_denom<'a>(
    voucher_denom: &'a str,
    port_id: &str,
    channel_id: &str,
) -> Result<&'a str, TransferError> {
    let split_denom: Vec<&str> = voucher_denom.splitn(3, '/').collect();
    if split_denom.len() != 3 {
        return Err(TransferError::NoForeignTokens);
    }
    if split_denom[0] != port_id {
        return Err(TransferError::unexpected_port_id(port_id, split_denom[0]));
    }
    if split_denom[1] != channel_id {
        return Err(TransferError::unexpected_channel_id(
            channel_id,
            split_denom[1],
        ));
    }

    Ok(split_denom[2])
}
