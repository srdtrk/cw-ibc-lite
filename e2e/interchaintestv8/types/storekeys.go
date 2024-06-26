package types

import (
	"cosmossdk.io/collections"

	sdk "github.com/cosmos/cosmos-sdk/types"
	banktypes "github.com/cosmos/cosmos-sdk/x/bank/types"

	commitmenttypes "github.com/cosmos/ibc-go/v8/modules/core/23-commitment/types"
	commitmenttypesv2 "github.com/cosmos/ibc-go/v8/modules/core/23-commitment/types/v2"
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

func ConvertToMerklePath(prefix []byte, key []byte) (*commitmenttypesv2.MerklePath, error) {
	merklePath := commitmenttypes.NewMerklePath(key)
	merklePrefix := commitmenttypes.NewMerklePrefix(prefix)
	path, err := commitmenttypes.ApplyPrefix(merklePrefix, merklePath)
	if err != nil {
		return nil, err
	}

	return &path, nil
}

func ToLegacyMerklePath(path *commitmenttypesv2.MerklePath) *commitmenttypes.MerklePath {
	legacyPath := commitmenttypes.MerklePath{}
	for _, key := range path.KeyPath {
		legacyPath.KeyPath = append(legacyPath.KeyPath, string(key))
	}

	return &legacyPath
}
