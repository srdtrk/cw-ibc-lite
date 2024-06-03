//! # Keys
//!
//! Contains key constants definitions for the contract such as version info for migrations.

/// `CONTRACT_NAME` is the name of the contract recorded with [`cw2`]
pub const CONTRACT_NAME: &str = "crates.io:cw-ibc-lite-ics26-router";
/// `CONTRACT_VERSION` is the version of the cargo package.
/// This is also the version of the contract recorded in [`cw2`]
pub const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

/// `ICS02_CLIENT_SALT` is the salt for the ICS02 client instantiation
pub const ICS02_CLIENT_SALT: &str = "ics02_client";

/// Contains the reply ids for various `SubMsg` replies
pub mod reply {
    /// `ON_RECV_PACKET` is the reply id for the `on_recv_packet` reply
    pub const ON_RECV_PACKET: u64 = 1;
}
