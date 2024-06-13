package main

import (
	"context"
	"encoding/base64"
	"fmt"
	"testing"

	"github.com/stretchr/testify/suite"

	codectypes "github.com/cosmos/cosmos-sdk/codec/types"

	clienttypes "github.com/cosmos/ibc-go/v8/modules/core/02-client/types"
	commitmenttypes "github.com/cosmos/ibc-go/v8/modules/core/23-commitment/types"
	host "github.com/cosmos/ibc-go/v8/modules/core/24-host"
	ibcexported "github.com/cosmos/ibc-go/v8/modules/core/exported"
	ibctm "github.com/cosmos/ibc-go/v8/modules/light-clients/07-tendermint"
	ibctesting "github.com/cosmos/ibc-go/v8/testing"

	wasmtypes "github.com/CosmWasm/wasmd/x/wasm/types"

	"github.com/strangelove-ventures/interchaintest/v8/chain/cosmos"
	"github.com/strangelove-ventures/interchaintest/v8/testutil"

	"github.com/srdtrk/cw-ibc-lite/e2esuite/v8/e2esuite"
	"github.com/srdtrk/cw-ibc-lite/e2esuite/v8/types"
	"github.com/srdtrk/cw-ibc-lite/e2esuite/v8/types/ics07tendermint"
)

// ICS07TendermintTestSuite is a suite of tests that wraps the TestSuite
// and can provide additional functionality
type ICS07TendermintTestSuite struct {
	e2esuite.TestSuite

	trustedHeight clienttypes.Height

	tendermintContract *ics07tendermint.Contract
	// this line is used by go-codegen # suite/contract
}

// SetupSuite calls the underlying ICS07TendermintTestSuite's SetupSuite method
func (s *ICS07TendermintTestSuite) SetupSuite(ctx context.Context) {
	s.TestSuite.SetupSuite(ctx)

	wasmd, simd := s.ChainA, s.ChainB

	var codeID string
	s.Require().True(s.Run("StoreCode", func() {
		// Upload the contract to the chain
		proposal, err := types.NewCompressedStoreCodeMsg(ctx, wasmd, s.UserA, "../../artifacts/cw_ibc_lite_ics07_tendermint.wasm")
		s.Require().NoError(err)
		_, err = s.BroadcastMessages(ctx, wasmd, s.UserA, 6_000_000, proposal)
		s.Require().NoError(err)

		codeResp, err := e2esuite.GRPCQuery[wasmtypes.QueryCodesResponse](ctx, wasmd, &wasmtypes.QueryCodesRequest{})
		s.Require().NoError(err)
		s.Require().Len(codeResp.CodeInfos, 1)

		codeID = fmt.Sprintf("%d", codeResp.CodeInfos[0].CodeID)
	}))

	s.Require().True(s.Run("InstantiateContract", func() {
		header, err := s.FetchHeader(ctx, simd)
		s.Require().NoError(err)

		var (
			height           clienttypes.Height
			clientStateBz    []byte
			consensusStateBz []byte
		)
		s.Require().True(s.Run("construct the client and consensus state", func() {
			tmConfig := ibctesting.NewTendermintConfig()
			revision := clienttypes.ParseChainID(header.ChainID)
			height = clienttypes.NewHeight(revision, uint64(header.Height))

			clientState := ibctm.NewClientState(
				header.ChainID,
				tmConfig.TrustLevel, tmConfig.TrustingPeriod, tmConfig.UnbondingPeriod, tmConfig.MaxClockDrift,
				height, commitmenttypes.GetSDKSpecs(), ibctesting.UpgradePath,
			)
			clientStateBz = clienttypes.MustMarshalClientState(simd.Config().EncodingConfig.Codec, clientState)

			consensusState := ibctm.NewConsensusState(header.Time, commitmenttypes.NewMerkleRoot([]byte(ibctm.SentinelRoot)), header.ValidatorsHash)
			consensusStateBz = clienttypes.MustMarshalConsensusState(simd.Config().EncodingConfig.Codec, consensusState)
		}))

		// Instantiate the contract using contract helpers.
		// This will an error if the instantiate message is invalid.
		s.tendermintContract, err = ics07tendermint.Instantiate(ctx, s.UserA.KeyName(), codeID, "", wasmd, ics07tendermint.InstantiateMsg{
			ClientState:    ics07tendermint.ToBinary(clientStateBz),
			ConsensusState: ics07tendermint.ToBinary(consensusStateBz),
		})
		s.Require().NoError(err)
		s.Require().NotEmpty(s.tendermintContract.Address)

		// Set the trusted height to the height of the header
		s.trustedHeight = height
	}))
}

// TestWithICS07TendermintTestSuite is the boilerplate code that allows the test suite to be run
func TestWithICS07TendermintTestSuite(t *testing.T) {
	suite.Run(t, new(ICS07TendermintTestSuite))
}

// TestInstantiate is a test that demonstrates instantiating the ICS-07 Tendermint contract.
func (s *ICS07TendermintTestSuite) TestInstantiate() {
	ctx := context.Background()

	s.SetupSuite(ctx)

	s.Require().True(s.Run("VerifyClientStatus", func() {
		statusResp, err := s.tendermintContract.QueryClient().Status(ctx, &ics07tendermint.QueryMsg_Status{})
		s.Require().NoError(err)
		s.Require().Equal(statusResp.Status, ibcexported.Active.String())

		// if the height isn't present in the client state, this query would failed
		_, err = s.tendermintContract.QueryClient().TimestampAtHeight(ctx, &ics07tendermint.QueryMsg_TimestampAtHeight{
			Height: ics07tendermint.Height{
				RevisionNumber: int(s.trustedHeight.RevisionNumber),
				RevisionHeight: int(s.trustedHeight.RevisionHeight),
			},
		})
		s.Require().NoError(err)
	}))
}

// TestUpdateClient is a test that demonstrates updating the ICS-07 Tendermint client.
func (s *ICS07TendermintTestSuite) TestUpdateClient() {
	ctx := context.Background()

	s.SetupSuite(ctx)

	_, simd := s.ChainA, s.ChainB

	initialHeight := s.trustedHeight

	s.Require().NoError(testutil.WaitForBlocks(ctx, 2, simd))

	s.UpdateClientContract(ctx, s.tendermintContract, simd)

	s.Require().True(s.Run("VerifyClientStatus", func() {
		// The client should be at a higher height
		s.Require().Greater(s.trustedHeight.RevisionHeight, initialHeight.RevisionHeight)
		s.Require().Equal(s.trustedHeight.RevisionNumber, initialHeight.RevisionNumber)

		statusResp, err := s.tendermintContract.QueryClient().Status(ctx, &ics07tendermint.QueryMsg_Status{})
		s.Require().NoError(err)
		s.Require().Equal(ibcexported.Active.String(), statusResp.Status)

		// if the height isn't present in the client state, this query would failed
		_, err = s.tendermintContract.QueryClient().TimestampAtHeight(ctx, &ics07tendermint.QueryMsg_TimestampAtHeight{
			Height: ics07tendermint.Height{
				RevisionNumber: int(s.trustedHeight.RevisionNumber),
				RevisionHeight: int(s.trustedHeight.RevisionHeight),
			},
		})
		s.Require().NoError(err)
	}))
}

// TestVerifyMembership is a test that demonstrates verifying membership in the ICS-07 Tendermint contract.
func (s *ICS07TendermintTestSuite) TestVerifyMembership() {
	ctx := context.Background()

	s.SetupSuite(ctx)

	_, simd := s.ChainA, s.ChainB

	// We will verify the client state of s.UserB on simd
	var (
		proofHeight int64
		proof       []byte
		value       []byte
		merklePath  commitmenttypes.MerklePath
	)
	s.Require().True(s.Run("CreateClientStateProof", func() {
		s.UpdateClientContract(ctx, s.tendermintContract, simd)

		var err error
		key := host.FullClientStateKey(ibctesting.FirstClientID)
		merklePath = commitmenttypes.NewMerklePath(string(key))
		merklePath, err = commitmenttypes.ApplyPrefix(commitmenttypes.NewMerklePrefix([]byte(ibcexported.StoreKey)), merklePath)
		s.Require().NoError(err)

		// Create a proof for the client state
		value, proof, proofHeight, err = s.QueryProofs(ctx, simd, ibcexported.StoreKey, key, int64(s.trustedHeight.RevisionHeight))
		s.Require().NoError(err)
		s.Require().NotEmpty(proof)
		s.Require().NotEmpty(value)
		// s.Require().Equal(expValue, value)
		s.Require().Equal(int64(s.trustedHeight.RevisionHeight), proofHeight)
	}))

	s.Require().True(s.Run("VerifyMembership", func() {
		_, err := s.tendermintContract.QueryClient().VerifyMembership(ctx, &ics07tendermint.QueryMsg_VerifyMembership{
			DelayBlockPeriod: 0,
			DelayTimePeriod:  0,
			Height: ics07tendermint.Height2{
				RevisionNumber: int(s.trustedHeight.RevisionNumber),
				RevisionHeight: int(proofHeight),
			},
			Path: ics07tendermint.MerklePath{
				KeyPath: merklePath.KeyPath,
			},
			Proof: base64.StdEncoding.EncodeToString(proof),
			Value: base64.StdEncoding.EncodeToString(value),
		})
		s.Require().NoError(err)

		// Ensure that proof verification fails if the proof is incorrect
		incorrectValue := []byte("incorrect value")
		_, err = s.tendermintContract.QueryClient().VerifyMembership(ctx, &ics07tendermint.QueryMsg_VerifyMembership{
			DelayBlockPeriod: 0,
			DelayTimePeriod:  0,
			Height: ics07tendermint.Height2{
				RevisionNumber: int(s.trustedHeight.RevisionNumber),
				RevisionHeight: int(proofHeight),
			},
			Path: ics07tendermint.MerklePath{
				KeyPath: merklePath.KeyPath,
			},
			Proof: base64.StdEncoding.EncodeToString(proof),
			Value: base64.StdEncoding.EncodeToString(incorrectValue),
		})
		s.Require().Error(err)
	}))
}

func (s *ICS07TendermintTestSuite) UpdateClientContract(ctx context.Context, tmContract *ics07tendermint.Contract, counterpartyChain *cosmos.CosmosChain) {
	signedHeader, err := s.QuerySignedHeader(ctx, counterpartyChain, s.trustedHeight)
	s.Require().NoError(err)

	anyHeader, err := codectypes.NewAnyWithValue(signedHeader)
	s.Require().NoError(err)

	signedHeaderBz, err := anyHeader.Marshal()
	s.Require().NoError(err)

	b64Header := base64.StdEncoding.EncodeToString(signedHeaderBz)

	updateMsg := ics07tendermint.ExecuteMsg_UpdateState(ics07tendermint.UpdateStateMsgRaw{
		ClientMessage: b64Header,
	})
	execMsg := ics07tendermint.ExecuteMsg{
		UpdateState: &updateMsg,
	}
	_, err = tmContract.Execute(ctx, s.UserA.KeyName(), execMsg)
	s.Require().NoError(err)

	// NOTE: We assume that revision number does not change
	s.trustedHeight.RevisionHeight = uint64(signedHeader.Header.Height)
}
