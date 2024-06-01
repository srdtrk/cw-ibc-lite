//! # Messages
//!
//! This module defines the messages that this contract receives.

use cosmwasm_schema::{cw_serde, QueryResponses};

/// The message to instantiate the contract.
/// Sender is assumed to be cw-ibc-lite-routee, and becomes the owner of the contract.
#[cw_serde]
pub struct InstantiateMsg {}

/// The execute messages supported by the contract.
#[cw_serde]
pub enum ExecuteMsg {
    /// Create a new client.
    CreateClient {
        /// Code id of the light client contract code.
        code_id: u64,
        /// Instantiate message for the light client contract.
        instantiate_msg: cw_ibc_lite_shared::types::clients::msg::InstantiateMsg,
        /// The optional counterparty info. If provided, the client will be provided with the counterparty.
        /// If not provided, the counterparty must be provided later using the `ProvideCounterparty` message.
        #[serde(skip_serializing_if = "Option::is_none")]
        counterparty_info: Option<super::state::CounterpartyInfo>,
    },
    /// Execute a message on a client.
    ExecuteClient {
        /// The client id of the client to execute the message on.
        client_id: String,
        /// The message to execute on the client.
        message: cw_ibc_lite_shared::types::clients::msg::ExecuteMsg,
    },
    /// Migrate the underlying client
    MigrateClient {
        /// Identifier of the client to migrate.
        subject_client_id: String,
        /// Identifier of the client with the new contract.
        substitute_client_id: String,
    },
    /// Provide the counterparty for a client.
    ProvideCounterparty {
        /// The client id of the client to provide the counterparty for.
        client_id: String,
        /// Counterparty client information.
        counterparty_info: super::state::CounterpartyInfo,
    },
}

/// The query messages supported by the contract.
#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    /// Execute a query on a client.
    /// Instead of using this message from external contracts, it is recommended to use
    /// [`crate::helpers::Ics02ClientContractQuerier::client_querier`] to query the client.
    #[returns(query_responses::QueryClient)]
    QueryClient {
        /// The client id of the client to execute the query on.
        client_id: String,
        /// The query to execute on the client.
        query: cw_ibc_lite_shared::types::clients::msg::QueryMsg,
    },
    /// Get the contract address of a client. Returns an error if the client does not exist.
    #[returns(query_responses::ClientInfo)]
    ClientInfo {
        /// The client id of the client to get the address of.
        client_id: String,
    },
    /// Get the counterparty of a client. Returns an error if the client does not have a
    /// counterparty or the client does not exist.
    #[returns(super::state::CounterpartyInfo)]
    Counterparty {
        /// The client id of the client to get the counterparty of.
        client_id: String,
    },
}

/// Contains the query responses supported by the contract.
pub mod query_responses {
    /// The response to [`super::QueryMsg::QueryClient`].
    #[super::cw_serde]
    pub enum QueryClient {
        /// The response to [`cw_ibc_lite_shared::types::clients::msg::QueryMsg::Status`].
        Status(cw_ibc_lite_shared::types::clients::msg::query_responses::Status),
        /// The response to [`cw_ibc_lite_shared::types::clients::msg::QueryMsg::ExportMetadata`].
        ExportMetadata(cw_ibc_lite_shared::types::clients::msg::query_responses::ExportMetadata),
        /// The response to [`cw_ibc_lite_shared::types::clients::msg::QueryMsg::TimestampAtHeight`].
        TimestampAtHeight(
            cw_ibc_lite_shared::types::clients::msg::query_responses::TimestampAtHeight,
        ),
        /// The response to [`cw_ibc_lite_shared::types::clients::msg::QueryMsg::VerifyClientMessage`].
        VerifyClientMessage(
            cw_ibc_lite_shared::types::clients::msg::query_responses::VerifyClientMessage,
        ),
        /// The response to [`cw_ibc_lite_shared::types::clients::msg::QueryMsg::CheckForMisbehaviour`].
        CheckForMisbehaviour(
            cw_ibc_lite_shared::types::clients::msg::query_responses::CheckForMisbehaviour,
        ),
    }

    /// The response to [`super::QueryMsg::ClientInfo`].
    #[super::cw_serde]
    pub struct ClientInfo {
        /// The client identifier.
        pub client_id: String,
        /// The contract address of the client.
        pub address: String,
        /// The counterparty client info.
        /// None if the counterparty is not provided.
        pub counterparty_info: Option<super::super::state::CounterpartyInfo>,
        /// The creator address of the client.
        pub creator: String,
    }
}
