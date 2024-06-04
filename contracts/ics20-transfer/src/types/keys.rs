//! # Keys
//!
//! Contains key constants definitions for the contract such as version info for migrations.

/// `CONTRACT_NAME` is the name of the contract recorded with [`cw2`]
pub const CONTRACT_NAME: &str = "crates.io:cw-ibc-lite-ics20-transfer";
/// `CONTRACT_VERSION` is the version of the cargo package.
/// This is also the version of the contract recorded in [`cw2`]
pub const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

/// `ICS20_VERSION` is the version of the ICS20 module used in the contract.
pub const ICS20_VERSION: &str = "ics20-1";
/// `DEFAULT_PORT_ID` is the default port ID used in the counterparty chain.
pub const DEFAULT_PORT_ID: &str = "transfer";
/// `DEFAULT_TIMEOUT_SECONDS` is the default timeout in seconds for the ICS20 module.
pub const DEFAULT_TIMEOUT_SECONDS: u64 = 600;

/// Contains the reply ids for various `SubMsg` replies
pub mod reply {
    /// `ON_RECV_PACKET_CW20_TRANSFER` is the reply id for the `on_recv_packet` reply
    pub const ON_RECV_PACKET_CW20_TRANSFER: u64 = 1;
}
