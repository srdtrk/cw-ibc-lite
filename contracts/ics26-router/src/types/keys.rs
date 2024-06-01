//! # Keys
//!
//! Contains key constants definitions for the contract such as version info for migrations.

/// `CONTRACT_NAME` is the name of the contract recorded with [`cw2`]
pub const CONTRACT_NAME: &str = "crates.io:cw-ibc-lite-ics26-router";
/// `CONTRACT_VERSION` is the version of the cargo package.
/// This is also the version of the contract recorded in [`cw2`]
pub const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

/// `PORT_ID_PREFIX` is the prefix of the port id
pub const PORT_ID_PREFIX: &str = "wasm.";

/// `ICS02_CLIENT_SALT` is the salt for the ICS02 client instantiation
pub const ICS02_CLIENT_SALT: &str = "ics02_client";

/// Contains the reply ids for various `SubMsg` replies
pub mod reply {
    /// `ON_SEND_PACKET` is the reply id for the `OnSendPacket` callback reply
    pub const ON_SEND_PACKET: u64 = 1;
}
