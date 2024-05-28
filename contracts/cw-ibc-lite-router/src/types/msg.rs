//! # Messages
//!
//! This module defines the messages that this contract receives.

use cosmwasm_schema::{cw_serde, QueryResponses};

/// The message to instantiate the contract.
#[cw_serde]
pub struct InstantiateMsg {}

/// The execute messages supported by the contract.
#[cw_serde]
pub enum ExecuteMsg {}

/// The query messages supported by the contract.
#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {}
