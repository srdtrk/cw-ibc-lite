package cw20base

import (
	"encoding/base64"
	"encoding/json"
)

// This is the message we accept via [`ExecuteMsg::Receive`].
type MsgTransfer struct {
	// The local channel to send the packets on
	SourceChannel string `json:"source_channel"`
	// The remote address to send to.
	// Don't use HumanAddress as this will likely have a different Bech32 prefix than we use
	// and cannot be validated locally
	Receiver string `json:"receiver"`
	// How long the packet lives in seconds. If not specified, use default_timeout
	Timeout *uint64 `json:"timeout,omitempty"`
	// An optional memo to add to the IBC transfer
	Memo *string `json:"memo,omitempty"`
}

// Converts any object to a base64-encoded JSON binary.
func ToJsonBinary(obj any) Binary {
	bz, err := json.Marshal(obj)
	if err != nil {
		panic(err)
	}

	b64 := base64.StdEncoding.EncodeToString(bz)
	return Binary(b64)
}
