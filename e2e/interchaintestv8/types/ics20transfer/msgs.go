/* Code generated by github.com/srdtrk/go-codegen, DO NOT EDIT. */
package ics20transfer

// The message to instantiate the contract.
type InstantiateMsg struct {
	// The contract address allowed to make IBC callbacks.
	Ics26RouterAddress string `json:"ics26_router_address"`
}

// The execute messages supported by the contract.
type ExecuteMsg struct {
	// This accepts a properly-encoded ReceiveMsg from a cw20 contract The wrapped message is expected to be [`TransferMsg`].
	Receive *ExecuteMsg_Receive `json:"receive,omitempty"`
	// The application callback message from `cw-ibc-lite`. The handler for this variant should verify that this message comes from an expected legitimate source.
	ReceiveIbcAppCallback *ExecuteMsg_ReceiveIbcAppCallback `json:"receive_ibc_app_callback,omitempty"`
}

// The query messages supported by the contract.
type QueryMsg struct {
	// The escrowed amount for the given channel and cw20 contract address.
	EscrowAmount *QueryMsg_EscrowAmount `json:"escrow_amount,omitempty"`
	// The list of all escrows for the given channel. Returns (cw20_address, amount) pairs.
	ListEscrows *QueryMsg_ListEscrows `json:"list_escrows,omitempty"`
	// Query the contract's ownership information
	Ownership *QueryMsg_Ownership `json:"ownership,omitempty"`
}

type QueryMsg_ListEscrows struct {
	// The channel identifier.
	Channel string `json:"channel"`
	// limit results to this number
	Limit *int `json:"limit,omitempty"`
	// start pagination after this contract address
	StartAfter *string `json:"start_after,omitempty"`
}

// Information on the escrowed amount for a given channel and cw20 address
type EscrowInfo struct {
	// Amount of funds escrowed
	Amount Uint128 `json:"amount"`
	// The channel identifier of the escrowed amount
	Channel string `json:"channel"`
	// The address of the cw20 token contract
	Cw20Address string `json:"cw20_address"`
}

// The contract's ownership info
type Ownership_for_String struct {
	// The contract's current owner. `None` if the ownership has been renounced.
	Owner *string `json:"owner,omitempty"`
	// The deadline for the pending owner to accept the ownership. `None` if there isn't a pending ownership transfer, or if a transfer exists and it doesn't have a deadline.
	PendingExpiry *Expiration `json:"pending_expiry,omitempty"`
	// The account who has been proposed to take over the ownership. `None` if there isn't a pending ownership transfer.
	PendingOwner *string `json:"pending_owner,omitempty"`
}

/*
A thin wrapper around u128 that is using strings for JSON encoding/decoding, such that the full u128 range can be used for clients that convert JSON numbers to floats, like JavaScript and jq.

# Examples

Use `from` to create instances of this and `u128` to get the value out:

``` # use cosmwasm_std::Uint128; let a = Uint128::from(123u128); assert_eq!(a.u128(), 123);

let b = Uint128::from(42u64); assert_eq!(b.u128(), 42);

let c = Uint128::from(70u32); assert_eq!(c.u128(), 70); ```
*/
type Uint128 string

/*
A thin wrapper around u64 that is using strings for JSON encoding/decoding, such that the full u64 range can be used for clients that convert JSON numbers to floats, like JavaScript and jq.

# Examples

Use `from` to create instances of this and `u64` to get the value out:

``` # use cosmwasm_std::Uint64; let a = Uint64::from(42u64); assert_eq!(a.u64(), 42);

let b = Uint64::from(70u32); assert_eq!(b.u64(), 70); ```
*/
type Uint64 string

// The sequence number of a packet enforces ordering among packets from the same source.
type Sequence int

type PortId string

type QueryMsg_EscrowAmount struct {
	// The channel identifier.
	Channel string `json:"channel"`
	// The cw20 contract address.
	Cw20Address string `json:"cw20_address"`
}

type QueryMsg_Ownership struct{}

// Expiration represents a point in time when some event happens. It can compare with a BlockInfo and will return is_expired() == true once the condition is hit (and for every block in the future)
type Expiration struct {
	// AtHeight will expire when `env.block.height` >= height
	AtHeight *Expiration_AtHeight `json:"at_height,omitempty"`
	// AtTime will expire when `env.block.time` >= time
	AtTime *Expiration_AtTime `json:"at_time,omitempty"`
	// Never will never expire. Used to express the empty variant
	Never *Expiration_Never `json:"never,omitempty"`
}
type ExecuteMsg_ReceiveIbcAppCallback IbcAppCallbackMsg

// In IBC each package must set at least one type of timeout: the timestamp or the block height. Using this rather complex enum instead of two timeout fields we ensure that at least one timeout is set.
type IbcTimeout struct {
	Block *IbcTimeoutBlock `json:"block,omitempty"`
	Timestamp *Timestamp `json:"timestamp,omitempty"`
}

// Packet defines a type that carries data across different chains through IBC
type Packet struct {
	// identifies the port on the receiving chain.
	DestinationPort PortId `json:"destination_port"`
	// number corresponds to the order of sends and receives, where a Packet with an earlier sequence number must be sent and received before a Packet with a later sequence number.
	Sequence Sequence `json:"sequence"`
	// identifies the channel end on the sending chain.
	SourceChannel ClientId `json:"source_channel"`
	// identifies the port on the sending chain.
	SourcePort PortId `json:"source_port"`
	// block height after which the packet times out
	Timeout IbcTimeout `json:"timeout"`
	// actual opaque bytes transferred directly to the application module
	Data Binary `json:"data"`
	// identifies the channel end on the receiving chain.
	DestinationChannel ClientId `json:"destination_channel"`
}

// IBCTimeoutHeight Height is a monotonically increasing data type that can be compared against another Height for the purposes of updating and freezing clients. Ordering is (revision_number, timeout_height)
type IbcTimeoutBlock struct {
	// block height after which the packet times out. the height within the given revision
	Height int `json:"height"`
	// the version that the client is currently on (e.g. after resetting the chain this could increment 1 as height drops to 0)
	Revision int `json:"revision"`
}
type ExecuteMsg_Receive Cw20ReceiveMsg

type ClientId string

// Cw20ReceiveMsg should be de/serialized under `Receive()` variant in a ExecuteMsg
type Cw20ReceiveMsg struct {
	Amount Uint128 `json:"amount"`
	Msg Binary `json:"msg"`
	Sender string `json:"sender"`
}

// Response to [`super::QueryMsg::ListEscrows`]
type EscrowList struct {
	// List of escrow infos
	List []EscrowInfo `json:"list"`
}

/*
Binary is a wrapper around Vec<u8> to add base64 de/serialization with serde. It also adds some helper methods to help encode inline.

This is only needed as serde-json-{core,wasm} has a horrible encoding for Vec<u8>. See also <https://github.com/CosmWasm/cosmwasm/blob/main/docs/MESSAGE_TYPES.md>.
*/
type Binary string

/*
A point in time in nanosecond precision.

This type can represent times from 1970-01-01T00:00:00Z to 2554-07-21T23:34:33Z.

## Examples

``` # use cosmwasm_std::Timestamp; let ts = Timestamp::from_nanos(1_000_000_202); assert_eq!(ts.nanos(), 1_000_000_202); assert_eq!(ts.seconds(), 1); assert_eq!(ts.subsec_nanos(), 202);

let ts = ts.plus_seconds(2); assert_eq!(ts.nanos(), 3_000_000_202); assert_eq!(ts.seconds(), 3); assert_eq!(ts.subsec_nanos(), 202); ```
*/
type Timestamp Uint64

// All IBC applications built with `cw-ibc-lite` must handle these callback messages.
type IbcAppCallbackMsg struct {
	// OnSendPacket is called when a packet send request is received by the `cw-ibc-lite` router. The packet send is cancelled if the callback response is an error.
	OnSendPacket *IbcAppCallbackMsg_OnSendPacket `json:"on_send_packet,omitempty"`
	// Called when a packet is sent to this IBC application. This callback needs to be responded with [`response::AcknowledgementData`].
	OnRecvPacket *IbcAppCallbackMsg_OnRecvPacket `json:"on_recv_packet,omitempty"`
	// Called when a packet to be acknowledged by this IBC application. This callback need not be responded with data.
	OnAcknowledgementPacket *IbcAppCallbackMsg_OnAcknowledgementPacket `json:"on_acknowledgement_packet,omitempty"`
	// Called when a packet to be timed out by this IBC application. This callback need not be responded with data.
	OnTimeoutPacket *IbcAppCallbackMsg_OnTimeoutPacket `json:"on_timeout_packet,omitempty"`
}

type IbcAppCallbackMsg_OnTimeoutPacket struct {
	// The packet to timeout.
	Packet Packet `json:"packet"`
	// The relayer address that submitted the timeout.
	Relayer string `json:"relayer"`
}

type Expiration_AtHeight int
type Expiration_AtTime Timestamp

type Expiration_Never struct{}

type IbcAppCallbackMsg_OnSendPacket struct {
	// Sender address of the packet.
	Sender string `json:"sender"`
	// The version string of the packet for the IBC application.
	Version string `json:"version"`
	// The packet to be sent.
	Packet Packet `json:"packet"`
}

type IbcAppCallbackMsg_OnRecvPacket struct {
	// The packet that was received.
	Packet Packet `json:"packet"`
	// The relayer address that submitted the packet.
	Relayer string `json:"relayer"`
}

type IbcAppCallbackMsg_OnAcknowledgementPacket struct {
	// The acknowledgement data.
	Acknowledgement Binary `json:"acknowledgement"`
	// The packet to acknowledge.
	Packet Packet `json:"packet"`
	// The relayer address that submitted the acknowledgement.
	Relayer string `json:"relayer"`
}
