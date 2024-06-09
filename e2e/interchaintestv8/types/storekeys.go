package types

import (
	"cosmossdk.io/collections"

	sdk "github.com/cosmos/cosmos-sdk/types"
	banktypes "github.com/cosmos/cosmos-sdk/x/bank/types"

	commitmenttypes "github.com/cosmos/ibc-go/v8/modules/core/23-commitment/types"
)

/*
This package contains helpers to interact with storage keys
*/

func GetBankBalanceKey(address sdk.AccAddress, denom string) ([]byte, error) {
	keyCodec := collections.PairKeyCodec(sdk.AccAddressKey, collections.StringKey)
	key, err := collections.EncodeKeyWithPrefix(banktypes.BalancesPrefix, keyCodec, collections.Join(address, denom))
	if err != nil {
		return nil, err
	}

	return key, nil
}

func ConvertToMerklePath(prefix []byte, key []byte) (*commitmenttypes.MerklePath, error) {
	merklePath := commitmenttypes.NewMerklePath(string(key))
	merklePrefix := commitmenttypes.NewMerklePrefix(prefix)
	path, err := commitmenttypes.ApplyPrefix(merklePrefix, merklePath)
	if err != nil {
		return nil, err
	}

	return &path, nil
}
