#![doc = include_str!("../README.md")]
#![deny(missing_docs)]
#![deny(clippy::nursery, clippy::pedantic, warnings)]

#[cfg(feature = "export")]
pub mod contract;
pub mod types;
