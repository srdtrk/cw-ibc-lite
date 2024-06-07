package main

import (
	"context"
	"fmt"
	"testing"

	"github.com/stretchr/testify/suite"

	"github.com/cosmos/cosmos-sdk/client/grpc/cmtservice"

	clienttypes "github.com/cosmos/ibc-go/v8/modules/core/02-client/types"
	commitmenttypes "github.com/cosmos/ibc-go/v8/modules/core/23-commitment/types"
	ibctm "github.com/cosmos/ibc-go/v8/modules/light-clients/07-tendermint"
	ibctesting "github.com/cosmos/ibc-go/v8/testing"

	wasmtypes "github.com/CosmWasm/wasmd/x/wasm/types"

	"github.com/strangelove-ventures/interchaintest/v8/testutil"

	"github.com/srdtrk/cw-ibc-lite/e2esuite/v8/e2esuite"
	"github.com/srdtrk/cw-ibc-lite/e2esuite/v8/types"
	"github.com/srdtrk/cw-ibc-lite/e2esuite/v8/types/ics07tendermint"
)

// TendermintTestSuite is a suite of tests that wraps the TestSuite
// and can provide additional functionality
type TendermintTestSuite struct {
	e2esuite.TestSuite

	tendermintContract *ics07tendermint.Contract
	// this line is used by go-codegen # suite/contract
}

// SetupSuite calls the underlying TendermintTestSuite's SetupSuite method
func (s *TendermintTestSuite) SetupSuite(ctx context.Context) {
	s.TestSuite.SetupSuite(ctx)
}

// TestWithTendermintTestSuite is the boilerplate code that allows the test suite to be run
func TestWithTendermintTestSuite(t *testing.T) {
	suite.Run(t, new(TendermintTestSuite))
}

// TestInstantiate is a test that demonstrates instantiating the ICS-07 Tendermint contract.
func (s *TendermintTestSuite) TestInstantiate() {
	ctx := context.Background()

	s.SetupSuite(ctx)

	wasmd1, wasmd2 := s.ChainA, s.ChainB

	s.Require().True(s.Run("UploadAndInstantiateContract", func() {
		// Compress the code as it is too large
		code, err := types.CompressFile("../../artifacts/cw_ibc_lite_ics07_tendermint.wasm")
		s.Require().NoError(err)

		_, err = s.BroadcastMessages(ctx, wasmd1, s.UserA, 10_000_000, &wasmtypes.MsgStoreCode{
			Sender:       s.UserA.FormattedAddress(),
			WASMByteCode: code,
		})
		s.Require().NoError(err)

		s.Require().NoError(testutil.WaitForBlocks(ctx, 3, wasmd1))

		headerResp, err := e2esuite.GRPCQuery[wasmtypes.QueryCodesResponse](ctx, wasmd1, &wasmtypes.QueryCodesRequest{})
		s.Require().NoError(err)
		s.Require().Len(headerResp.CodeInfos, 1)

		codeID := headerResp.CodeInfos[0].CodeID

		var (
			latestHeight int64
			header       *cmtservice.Header
		)
		s.Require().True(s.Run("fetch header and at the latest height", func() {
			latestHeight, err = wasmd2.Height(ctx)
			s.Require().NoError(err)

			headerResp, err := e2esuite.GRPCQuery[cmtservice.GetBlockByHeightResponse](ctx, wasmd2, &cmtservice.GetBlockByHeightRequest{
				Height: latestHeight,
			})
			s.Require().NoError(err)

			header = &headerResp.SdkBlock.Header
		}))

		var (
			clientStateBz    []byte
			consensusStateBz []byte
		)
		s.Require().True(s.Run("construct the client and consensus state", func() {
			tmConfig := ibctesting.NewTendermintConfig()
			revision := clienttypes.ParseChainID(header.ChainID)
			height := clienttypes.NewHeight(revision, uint64(header.Height))

			clientState := ibctm.NewClientState(
				header.ChainID,
				tmConfig.TrustLevel, tmConfig.TrustingPeriod, tmConfig.UnbondingPeriod, tmConfig.MaxClockDrift,
				height, commitmenttypes.GetSDKSpecs(), ibctesting.UpgradePath,
			)
			clientStateBz = clienttypes.MustMarshalClientState(wasmd2.Config().EncodingConfig.Codec, clientState)

			consensusState := ibctm.NewConsensusState(header.Time, commitmenttypes.NewMerkleRoot([]byte(ibctm.SentinelRoot)), header.ValidatorsHash)
			consensusStateBz = clienttypes.MustMarshalConsensusState(wasmd2.Config().EncodingConfig.Codec, consensusState)
		}))

		// Instantiate the contract using contract helpers.
		// This will an error if the instantiate message is invalid.
		s.tendermintContract, err = ics07tendermint.Instantiate(ctx, s.UserA.KeyName(), fmt.Sprintf("%d", codeID), "", wasmd1, ics07tendermint.InstantiateMsg{
			ClientState:    ics07tendermint.ToBinary(clientStateBz),
			ConsensusState: ics07tendermint.ToBinary(consensusStateBz),
		})
		s.Require().NoError(err)

		s.Require().NotEmpty(s.tendermintContract.Address)
	}))
}

// TestUpdateClient is a test that demonstrates updating the ICS-07 Tendermint client.
func (s *TendermintTestSuite) TestUpdateClient() {
	// WIP
	ctx := context.Background()

	s.SetupSuite(ctx)

	// wasmd1, wasmd2 := s.ChainA, s.ChainB
	//
	// res, err := e2esuite.GRPCQuery[cmtservice.GetValidatorSetByHeightResponse](ctx, wasmd2, &cmtservice.GetValidatorSetByHeightRequest{
	// 	Height: latestHeight,
	// })
	// s.Require().NoError(err)
	// s.Require().NotEmpty(res.Validators)

	// Sort the validators
	// sort.SliceStable(res.Validators, func(i, j int) bool {
	// 	return res.Validators[i].Address < res.Validators[j].Address
	// })
}
