//! # Messages
//!
//! This module defines the messages that this contract receives.

use cosmwasm_schema::{cw_serde, QueryResponses};

/// The message to instantiate the contract.
/// Sender is assumed to be cw-ibc-lite-routee, and becomes the owner of the contract.
#[cw_serde]
pub struct InstantiateMsg {
    /// cw-ibc-lite-ics02-client code id
    ics02_client_code_id: u64,
}

/// The execute messages supported by the contract.
#[cw_serde]
pub enum ExecuteMsg {
    /// Create a new client.
    CreateClient {
        /// Code id of the light client contract code.
        code_id: u64,
        /// Instantiate message for the light client contract.
        instantiate_msg: cw_ibc_lite_types::clients::msg::InstantiateMsg,
        /// The optional counterparty id. If provided, the client will be provided with the counterparty.
        /// If not provided, the counterparty must be provided later using the `ProvideCounterparty` message.
        #[serde(skip_serializing_if = "Option::is_none")]
        counterparty_id: Option<String>,
    },
    /// Execute a message on a client.
    ExecuteClient {
        /// The client id of the client to execute the message on.
        client_id: String,
        /// The message to execute on the client.
        message: cw_ibc_lite_types::clients::msg::ExecuteMsg,
    },
    /// Migrate the underlying client
    MigrateClient {
        /// The client id of the client to migrate.
        client_id: String,
        /// The new client id to migrate to.
        new_client_id: String,
    },
    /// Provide the counterparty for a client.
    ProvideCounterparty {
        /// The client id of the client to provide the counterparty for.
        client_id: String,
        /// The counterparty client id.
        counterparty_id: String,
    },
}

/// The query messages supported by the contract.
#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    /// Execute a query on a client.
    #[returns(query_responses::QueryClient)]
    QueryClient {
        /// The client id of the client to execute the query on.
        client_id: String,
        /// The query to execute on the client.
        query: cw_ibc_lite_types::clients::msg::QueryMsg,
    },
    /// Get the contract address of a client. Returns an error if the client does not exist.
    #[returns(String)]
    ClientAddress {
        /// The client id of the client to get the address of.
        client_id: String,
    },
}

/// Contains the query responses supported by the contract.
pub mod query_responses {
    /// The response to [`super::QueryMsg::QueryClient`].
    #[super::cw_serde]
    pub enum QueryClient {
        /// The response to [`cw_ibc_lite_types::clients::QueryMsg::Status`].
        Status(cw_ibc_lite_types::clients::msg::query_responses::Status),
        /// The response to [`cw_ibc_lite_types::clients::QueryMsg::ExportMetadata`].
        ExportMetadata(cw_ibc_lite_types::clients::msg::query_responses::ExportMetadata),
        /// The response to [`cw_ibc_lite_types::clients::QueryMsg::TimestampAtHeight`].
        TimestampAtHeight(cw_ibc_lite_types::clients::msg::query_responses::TimestampAtHeight),
        /// The response to [`cw_ibc_lite_types::clients::QueryMsg::VerifyClientMessage`].
        VerifyClientMessage(cw_ibc_lite_types::clients::msg::query_responses::VerifyClientMessage),
        /// The response to [`cw_ibc_lite_types::clients::QueryMsg::CheckForMisbehaviour`].
        CheckForMisbehaviour(
            cw_ibc_lite_types::clients::msg::query_responses::CheckForMisbehaviour,
        ),
    }
}
