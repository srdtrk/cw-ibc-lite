//! This module defines the types used by all light client contracts for ibc-lite

use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::Binary;
use ibc_client_cw::types::{
    CheckForMisbehaviourMsgRaw, ExportMetadataMsg, StatusMsg, TimestampAtHeightMsg,
    UpdateStateMsgRaw, UpdateStateOnMisbehaviourMsgRaw, VerifyClientMessageRaw,
    VerifyMembershipMsgRaw, VerifyNonMembershipMsgRaw, VerifyUpgradeAndUpdateStateMsgRaw,
};

/// Instantiate message for all light client contracts in ibc-lite
#[cw_serde]
pub struct InstantiateMsg {
    /// The initial client state.
    pub client_state: Binary,
    /// The initial consensus state.
    pub consensus_state: Binary,
}

/// Execute messages supported by all light client contracts in ibc-lite
#[cw_serde]
pub enum ExecuteMsg {
    /// Update the client state
    UpdateState(UpdateStateMsgRaw),
    /// Update the client state on misbehaviour
    UpdateStateOnMisbehaviour(UpdateStateOnMisbehaviourMsgRaw),
    /// Verify upgrade and update the client state
    VerifyUpgradeAndUpdateState(VerifyUpgradeAndUpdateStateMsgRaw),
    /// Verify membership
    // TODO: Move this to QueryMsg
    VerifyMembership(VerifyMembershipMsgRaw),
    /// Verify non-membership
    // TODO: Move this to QueryMsg
    VerifyNonMembership(VerifyNonMembershipMsgRaw),
}

/// Query messages supported by all light client contracts in ibc-lite
#[derive(QueryResponses)]
#[cw_serde]
pub enum QueryMsg {
    /// Get the status of the client
    #[returns(query_responses::Status)]
    Status(StatusMsg),
    /// Export the metadata
    #[returns(query_responses::ExportMetadata)]
    ExportMetadata(ExportMetadataMsg),
    /// Get the timestamp at the given height
    #[returns(query_responses::TimestampAtHeight)]
    TimestampAtHeight(TimestampAtHeightMsg),
    /// Verify the client message
    #[returns(query_responses::VerifyClientMessage)]
    VerifyClientMessage(VerifyClientMessageRaw),
    /// Check for misbehaviour
    #[returns(query_responses::CheckForMisbehaviour)]
    CheckForMisbehaviour(CheckForMisbehaviourMsgRaw),
}

/// Contains the query responses supported by all light client contracts in ibc-lite
pub mod query_responses {
    use super::cw_serde;
    use ibc_client_cw::types::GenesisMetadata;

    /// The response to [`super::QueryMsg::Status`]
    #[cw_serde]
    pub struct Status {
        /// The status of the client
        pub status: String,
    }
    /// The response to [`super::QueryMsg::ExportMetadata`]
    #[cw_serde]
    pub struct ExportMetadata {
        /// The genesis metadata
        pub metadata: Vec<GenesisMetadata>,
    }
    /// The response to [`super::QueryMsg::TimestampAtHeight`]
    #[cw_serde]
    pub struct TimestampAtHeight {
        /// The timestamp at the given height
        pub timestamp: u64,
    }
    /// The response to [`super::QueryMsg::VerifyClientMessage`]
    #[cw_serde]
    pub struct VerifyClientMessage {
        /// Whether the client message is valid
        pub is_valid: bool,
    }
    /// The response to [`super::QueryMsg::CheckForMisbehaviour`]
    #[cw_serde]
    pub struct CheckForMisbehaviour {
        /// Whether misbehaviour was found
        pub found_misbehaviour: bool,
    }
}
