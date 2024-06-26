/* Code generated by github.com/srdtrk/go-codegen, DO NOT EDIT. */
package ics02client

// The message to instantiate the contract. Sender is assumed to be cw-ibc-lite-router, and becomes the owner of the contract.
type InstantiateMsg struct{}

// The execute messages supported by the contract.
type ExecuteMsg struct {
	// Create a new client.
	CreateClient *ExecuteMsg_CreateClient `json:"create_client,omitempty"`
	// Execute a message on a client.
	ExecuteClient *ExecuteMsg_ExecuteClient `json:"execute_client,omitempty"`
	// Migrate the underlying client
	MigrateClient *ExecuteMsg_MigrateClient `json:"migrate_client,omitempty"`
	// Provide the counterparty for a client.
	ProvideCounterparty *ExecuteMsg_ProvideCounterparty `json:"provide_counterparty,omitempty"`
}

// The query messages supported by the contract.
type QueryMsg struct {
	// Execute a query on a client. Instead of using this message from external contracts, it is recommended to use [`crate::helpers::Ics02ClientContractQuerier::client_querier`] to query the client.
	QueryClient *QueryMsg_QueryClient `json:"query_client,omitempty"`
	// Get the contract address of a client. Returns an error if the client does not exist.
	ClientInfo *QueryMsg_ClientInfo `json:"client_info,omitempty"`
	// Get the counterparty of a client. Returns an error if the client does not have a counterparty or the client does not exist.
	Counterparty *QueryMsg_Counterparty `json:"counterparty,omitempty"`
}

type VerifyClientMessageRaw struct {
	ClientMessage string `json:"client_message"`
}

type VerifyMembershipMsgRaw struct {
	DelayTimePeriod  int        `json:"delay_time_period"`
	Height           Height2    `json:"height"`
	Path             MerklePath `json:"path"`
	Proof            string     `json:"proof"`
	Value            string     `json:"value"`
	DelayBlockPeriod int        `json:"delay_block_period"`
}

// The core IBC height type, which represents the height of a chain, which typically is the number of blocks since genesis (or more generally, since the last revision/hard upgrade).
type Height struct {
	// The height of a block
	RevisionHeight int `json:"revision_height"`
	// Previously known as "epoch"
	RevisionNumber int `json:"revision_number"`
}

// The response to [`super::QueryMsg::QueryClient`].
type QueryClient_2 struct {
	// The response to [`cw_ibc_lite_shared::types::clients::msg::QueryMsg::Status`].
	Status *QueryClient_Status `json:"status,omitempty"`
	// The response to [`cw_ibc_lite_shared::types::clients::msg::QueryMsg::ExportMetadata`].
	ExportMetadata *QueryClient_ExportMetadata `json:"export_metadata,omitempty"`
	// The response to [`cw_ibc_lite_shared::types::clients::msg::QueryMsg::TimestampAtHeight`].
	TimestampAtHeight *QueryClient_TimestampAtHeight `json:"timestamp_at_height,omitempty"`
	// The response to [`cw_ibc_lite_shared::types::clients::msg::QueryMsg::VerifyClientMessage`].
	VerifyClientMessage *QueryClient_VerifyClientMessage `json:"verify_client_message,omitempty"`
	// The response to [`cw_ibc_lite_shared::types::clients::msg::QueryMsg::CheckForMisbehaviour`].
	CheckForMisbehaviour *QueryClient_CheckForMisbehaviour `json:"check_for_misbehaviour,omitempty"`
}

type UpdateStateOnMisbehaviourMsgRaw struct {
	ClientMessage string `json:"client_message"`
}

type QueryMsg_ClientInfo struct {
	// The client id of the client to get the address of.
	ClientId string `json:"client_id"`
}

type QueryMsg_Counterparty struct {
	// The client id of the client to get the counterparty of.
	ClientId string `json:"client_id"`
}

type StatusMsg struct{}

// The response to [`super::QueryMsg::ExportMetadata`]
type ExportMetadata struct {
	// The genesis metadata
	Metadata []GenesisMetadata `json:"metadata"`
}

type ExecuteMsg_CreateClient struct {
	// Code id of the light client contract code.
	CodeId int `json:"code_id"`
	// The optional counterparty info. If provided, the client will be provided with the counterparty. If not provided, the counterparty must be provided later using the `ProvideCounterparty` message.
	CounterpartyInfo *CounterpartyInfo `json:"counterparty_info,omitempty"`
	// Instantiate message for the light client contract.
	InstantiateMsg InstantiateMsg_2 `json:"instantiate_msg"`
}

type ExportMetadataMsg struct{}

type QueryMsg_QueryClient struct {
	// The client id of the client to execute the query on.
	ClientId string `json:"client_id"`
	// The query to execute on the client.
	Query QueryMsg_2 `json:"query"`
}

// Counterparty client information.
type CounterpartyInfo struct {
	// The merkle path prefix of the counterparty.
	MerklePathPrefix *MerklePath `json:"merkle_path_prefix,omitempty"`
	// The client id of the counterparty.
	ClientId string `json:"client_id"`
}

type VerifyNonMembershipMsgRaw struct {
	DelayBlockPeriod int        `json:"delay_block_period"`
	DelayTimePeriod  int        `json:"delay_time_period"`
	Height           Height2    `json:"height"`
	Path             MerklePath `json:"path"`
	Proof            string     `json:"proof"`
}

/*
Height is a monotonically increasing data type that can be compared against another Height for the purposes of updating and freezing clients

Normally the RevisionHeight is incremented at each height while keeping RevisionNumber the same. However some consensus algorithms may choose to reset the height in certain conditions e.g. hard forks, state-machine breaking changes In these cases, the RevisionNumber is incremented so that height continues to be monitonically increasing even as the RevisionHeight gets reset
*/
type Height2 struct {
	// the height within the given revision
	RevisionHeight int `json:"revision_height"`
	// the revision that the client is currently on
	RevisionNumber int `json:"revision_number"`
}

type GenesisMetadata struct {
	Key   []int `json:"key"`
	Value []int `json:"value"`
}

type VerifyUpgradeAndUpdateStateMsgRaw struct {
	ProofUpgradeClient         string `json:"proof_upgrade_client"`
	ProofUpgradeConsensusState string `json:"proof_upgrade_consensus_state"`
	UpgradeClientState         string `json:"upgrade_client_state"`
	UpgradeConsensusState      string `json:"upgrade_consensus_state"`
}

type ExecuteMsg_ProvideCounterparty struct {
	// The client id of the client to provide the counterparty for.
	ClientId string `json:"client_id"`
	// Counterparty client information.
	CounterpartyInfo CounterpartyInfo `json:"counterparty_info"`
}

type TimestampAtHeightMsg struct {
	Height Height `json:"height"`
}

// Counterparty client information.
type CounterpartyInfo_2 struct {
	// The client id of the counterparty.
	ClientId string `json:"client_id"`
	// The merkle path prefix of the counterparty.
	MerklePathPrefix *MerklePath `json:"merkle_path_prefix,omitempty"`
}

// The response to [`super::QueryMsg::Status`]
type Status struct {
	// The status of the client
	Status string `json:"status"`
}

// The response to [`super::QueryMsg::TimestampAtHeight`]
type TimestampAtHeight struct {
	// The timestamp at the given height
	Timestamp int `json:"timestamp"`
}

type ExecuteMsg_MigrateClient struct {
	// Identifier of the client to migrate.
	SubjectClientId string `json:"subject_client_id"`
	// Identifier of the client with the new contract.
	SubstituteClientId string `json:"substitute_client_id"`
}

// The response to [`super::QueryMsg::CheckForMisbehaviour`]
type CheckForMisbehaviour struct {
	// Whether misbehaviour was found
	FoundMisbehaviour bool `json:"found_misbehaviour"`
}

// Execute messages supported by all light client contracts in ibc-lite
type ExecuteMsg_2 struct {
	// Update the client state
	UpdateState *ExecuteMsg_UpdateState `json:"update_state,omitempty"`
	// Update the client state on misbehaviour
	UpdateStateOnMisbehaviour *ExecuteMsg_UpdateStateOnMisbehaviour `json:"update_state_on_misbehaviour,omitempty"`
	// Verify upgrade and update the client state
	VerifyUpgradeAndUpdateState *ExecuteMsg_VerifyUpgradeAndUpdateState `json:"verify_upgrade_and_update_state,omitempty"`
}

type CheckForMisbehaviourMsgRaw struct {
	ClientMessage string `json:"client_message"`
}

type ExecuteMsg_ExecuteClient struct {
	// The client id of the client to execute the message on.
	ClientId string `json:"client_id"`
	// The message to execute on the client.
	Message ExecuteMsg_2 `json:"message"`
}

type MerklePath struct {
	KeyPath []string `json:"key_path"`
}

type UpdateStateMsgRaw struct {
	ClientMessage string `json:"client_message"`
}

/*
Binary is a wrapper around Vec<u8> to add base64 de/serialization with serde. It also adds some helper methods to help encode inline.

This is only needed as serde-json-{core,wasm} has a horrible encoding for Vec<u8>. See also <https://github.com/CosmWasm/cosmwasm/blob/main/docs/MESSAGE_TYPES.md>.
*/
type Binary string

// Query messages supported by all light client contracts in ibc-lite
type QueryMsg_2 struct {
	// Get the status of the client
	Status *QueryMsg_Status `json:"status,omitempty"`
	// Export the metadata
	ExportMetadata *QueryMsg_ExportMetadata `json:"export_metadata,omitempty"`
	// Get the timestamp at the given height
	TimestampAtHeight *QueryMsg_TimestampAtHeight `json:"timestamp_at_height,omitempty"`
	// Verify the client message
	VerifyClientMessage *QueryMsg_VerifyClientMessage `json:"verify_client_message,omitempty"`
	// Check for misbehaviour
	CheckForMisbehaviour *QueryMsg_CheckForMisbehaviour `json:"check_for_misbehaviour,omitempty"`
	// Verify membership
	VerifyMembership *QueryMsg_VerifyMembership `json:"verify_membership,omitempty"`
	// Verify non-membership
	VerifyNonMembership *QueryMsg_VerifyNonMembership `json:"verify_non_membership,omitempty"`
	// Get the owner of the contract
	Owner *QueryMsg_Owner `json:"owner,omitempty"`
}

// The response to [`super::QueryMsg::VerifyClientMessage`]
type VerifyClientMessage struct {
	// Whether the client message is valid
	IsValid bool `json:"is_valid"`
}

// The response to [`super::QueryMsg::ClientInfo`].
type ClientInfo struct {
	// The contract address of the client.
	Address string `json:"address"`
	// The client identifier.
	ClientId string `json:"client_id"`
	// The counterparty client info. None if the counterparty is not provided.
	CounterpartyInfo *CounterpartyInfo `json:"counterparty_info,omitempty"`
	// The creator address of the client.
	Creator string `json:"creator"`
}

// Instantiate message for all light client contracts in ibc-lite
type InstantiateMsg_2 struct {
	// The initial client state.
	ClientState Binary `json:"client_state"`
	// The initial consensus state.
	ConsensusState Binary `json:"consensus_state"`
}
type QueryClient_Status Status
type QueryMsg_ExportMetadata ExportMetadataMsg
type QueryClient_ExportMetadata ExportMetadata
type ExecuteMsg_VerifyUpgradeAndUpdateState VerifyUpgradeAndUpdateStateMsgRaw

type QueryMsg_Owner struct{}
type QueryMsg_Status StatusMsg
type QueryMsg_TimestampAtHeight TimestampAtHeightMsg
type QueryClient_CheckForMisbehaviour CheckForMisbehaviour
type ExecuteMsg_UpdateStateOnMisbehaviour UpdateStateOnMisbehaviourMsgRaw
type QueryMsg_VerifyClientMessage VerifyClientMessageRaw
type QueryMsg_CheckForMisbehaviour CheckForMisbehaviourMsgRaw
type QueryMsg_VerifyNonMembership VerifyNonMembershipMsgRaw
type QueryClient_TimestampAtHeight TimestampAtHeight
type QueryClient_VerifyClientMessage VerifyClientMessage
type QueryMsg_VerifyMembership VerifyMembershipMsgRaw
type ExecuteMsg_UpdateState UpdateStateMsgRaw
