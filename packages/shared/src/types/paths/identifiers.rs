//! This module contains identifier types for `cw-ibc-lite`.

// use crate::types::error::ContractError;

// Re-export identifiers from `ibc-core-host`
use ibc_core_host::types::identifiers::ClientId;
pub use ibc_core_host::types::identifiers::PortId;
pub use ibc_core_host::types::identifiers::Sequence;

/// In `cw-ibc-lite`, a `ChannelId` is the same as a `ClientId`
pub type ChannelId = ClientId;

// impl std::str::FromStr for ChannelId {
//     type Err = ContractError;
//
//     fn from_str(s: &str) -> Result<Self, Self::Err> {
//         ClientId::from_str(s)
//     }
// }
