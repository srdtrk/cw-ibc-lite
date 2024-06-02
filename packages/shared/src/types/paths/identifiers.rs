//! This module contains identifier types for `cw-ibc-lite`.

pub use ibc_core_host::types::identifiers::PortId; // Re-export the PortId type from ibc_core_host
pub use ibc_core_host::types::identifiers::Sequence; // Re-export the Sequence type

/// In `cw-ibc-lite`, a `ChannelId` is the same as a `ClientId`
pub type ChannelId = ibc_core_host::types::identifiers::ClientId;
