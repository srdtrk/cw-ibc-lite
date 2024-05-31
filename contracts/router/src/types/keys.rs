//! # Keys
//!
//! Contains key constants definitions for the contract such as version info for migrations.

/// `CONTRACT_NAME` is the name of the contract recorded with [`cw2`]
pub const CONTRACT_NAME: &str = "crates.io:cw-ibc-lite-router";
/// `CONTRACT_VERSION` is the version of the cargo package.
/// This is also the version of the contract recorded in [`cw2`]
pub const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

/// `PORT_ID_PREFIX` is the prefix of the port id
pub const PORT_ID_PREFIX: &str = "wasm.";
