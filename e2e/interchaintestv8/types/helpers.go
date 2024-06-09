package types

import (
	"bytes"
	"compress/gzip"
	"context"
	"io"
	"os"

	ics23 "github.com/cosmos/ics23/go"

	errorsmod "cosmossdk.io/errors"

	"github.com/cosmos/cosmos-sdk/client/grpc/cmtservice"

	commitmenttypes "github.com/cosmos/ibc-go/v8/modules/core/23-commitment/types"

	wasmtypes "github.com/CosmWasm/wasmd/x/wasm/types"

	"github.com/strangelove-ventures/interchaintest/v8/chain/cosmos"
	"github.com/strangelove-ventures/interchaintest/v8/ibc"
)

// compressFile compresses the file in the given path using gzip
func compressFile(path string) ([]byte, error) {
	file, err := os.Open(path)
	if err != nil {
		return nil, err
	}

	content, err := io.ReadAll(file)
	if err != nil {
		return nil, err
	}

	// compress the wasm file since it is too large to submit as a proposal
	var b bytes.Buffer
	gz := gzip.NewWriter(&b)
	_, err = gz.Write(content)
	if err != nil {
		return nil, err
	}

	gz.Close()

	return b.Bytes(), nil
}

// NewCompressedStoreCodeMsg creates a MsgStoreCode message with the compressed wasm code
func NewCompressedStoreCodeMsg(ctx context.Context, chain *cosmos.CosmosChain, wallet ibc.Wallet, filePath string) (*wasmtypes.MsgStoreCode, error) {
	compressed, err := compressFile(filePath)
	if err != nil {
		return nil, err
	}

	msgStoreCode := &wasmtypes.MsgStoreCode{
		Sender:       wallet.FormattedAddress(),
		WASMByteCode: compressed,
	}

	return msgStoreCode, nil
}

// ConvertProofs converts cmtservice.ProofOps into MerkleProof
func ConvertProofs(tmProof *cmtservice.ProofOps) (commitmenttypes.MerkleProof, error) {
	if tmProof == nil {
		return commitmenttypes.MerkleProof{}, errorsmod.Wrapf(commitmenttypes.ErrInvalidMerkleProof, "tendermint proof is nil")
	}
	// Unmarshal all proof ops to CommitmentProof
	proofs := make([]*ics23.CommitmentProof, len(tmProof.Ops))
	for i, op := range tmProof.Ops {
		var p ics23.CommitmentProof
		err := p.Unmarshal(op.Data)
		if err != nil || p.Proof == nil {
			return commitmenttypes.MerkleProof{}, errorsmod.Wrapf(commitmenttypes.ErrInvalidMerkleProof, "could not unmarshal proof op into CommitmentProof at index %d: %v", i, err)
		}
		proofs[i] = &p
	}
	return commitmenttypes.MerkleProof{
		Proofs: proofs,
	}, nil
}
