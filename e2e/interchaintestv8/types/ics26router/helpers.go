package ics26router

import (
	"encoding/base64"
	"strconv"

	channeltypes "github.com/cosmos/ibc-go/v8/modules/core/04-channel/types"
)

func ToBinary(bz []byte) Binary {
	b64 := base64.StdEncoding.EncodeToString(bz)
	return Binary(b64)
}

func ToPacket(packet channeltypes.Packet) Packet {
	timestamp := Timestamp(strconv.FormatUint(packet.TimeoutTimestamp, 10))
	return Packet{
		SourcePort:         PortId(packet.SourcePort),
		SourceChannel:      ClientId(packet.SourceChannel),
		DestinationPort:    PortId(packet.DestinationPort),
		DestinationChannel: ClientId(packet.DestinationChannel),
		Data:               ToBinary(packet.Data),
		Sequence:           Sequence(int(packet.Sequence)),
		Timeout: IbcTimeout{
			Block: &IbcTimeoutBlock{
				Height:   int(packet.TimeoutHeight.RevisionHeight),
				Revision: int(packet.TimeoutHeight.RevisionNumber),
			},
			Timestamp: &timestamp,
		},
	}
}
