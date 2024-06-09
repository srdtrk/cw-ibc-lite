package main

import (
	"context"
	"encoding/base64"
	"fmt"
	"testing"

	"github.com/stretchr/testify/suite"

	"github.com/cosmos/cosmos-sdk/client/grpc/cmtservice"
	codectypes "github.com/cosmos/cosmos-sdk/codec/types"

	clienttypes "github.com/cosmos/ibc-go/v8/modules/core/02-client/types"
	commitmenttypes "github.com/cosmos/ibc-go/v8/modules/core/23-commitment/types"
	ibctm "github.com/cosmos/ibc-go/v8/modules/light-clients/07-tendermint"
	ibctesting "github.com/cosmos/ibc-go/v8/testing"

	wasmtypes "github.com/CosmWasm/wasmd/x/wasm/types"

	"github.com/strangelove-ventures/interchaintest/v8/chain/cosmos"

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

	wasmd1, wasmd2 := s.ChainA, s.ChainB

	var codeID string
	s.Require().True(s.Run("StoreCode", func() {
		// Upload the contract to the chain
		proposal, err := types.NewCompressedStoreCodeMsg(ctx, wasmd1, s.UserA, "../../artifacts/cw_ibc_lite_ics07_tendermint.wasm")
		s.Require().NoError(err)
		_, err = s.BroadcastMessages(ctx, wasmd1, s.UserA, 6_000_000, proposal)
		s.Require().NoError(err)

		codeResp, err := e2esuite.GRPCQuery[wasmtypes.QueryCodesResponse](ctx, wasmd1, &wasmtypes.QueryCodesRequest{})
		s.Require().NoError(err)
		s.Require().Len(codeResp.CodeInfos, 1)

		codeID = fmt.Sprintf("%d", codeResp.CodeInfos[0].CodeID)
	}))

	s.Require().True(s.Run("InstantiateContract", func() {
		header, err := s.FetchHeader(ctx, wasmd2)
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
			clientStateBz = clienttypes.MustMarshalClientState(wasmd2.Config().EncodingConfig.Codec, clientState)

			consensusState := ibctm.NewConsensusState(header.Time, commitmenttypes.NewMerkleRoot([]byte(ibctm.SentinelRoot)), header.ValidatorsHash)
			consensusStateBz = clienttypes.MustMarshalConsensusState(wasmd2.Config().EncodingConfig.Codec, consensusState)
		}))

		// Instantiate the contract using contract helpers.
		// This will an error if the instantiate message is invalid.
		s.tendermintContract, err = ics07tendermint.Instantiate(ctx, s.UserA.KeyName(), codeID, "", wasmd1, ics07tendermint.InstantiateMsg{
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
}

// TestUpdateClient is a test that demonstrates updating the ICS-07 Tendermint client.
func (s *ICS07TendermintTestSuite) TestUpdateClient() {
	ctx := context.Background()

	s.SetupSuite(ctx)

	_, wasmd2 := s.ChainA, s.ChainB

	s.UpdateClientContract(ctx, s.tendermintContract, wasmd2)
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

func (s *ICS07TendermintTestSuite) FetchHeader(ctx context.Context, chain *cosmos.CosmosChain) (*cmtservice.Header, error) {
	latestHeight, err := chain.Height(ctx)
	if err != nil {
		return nil, err
	}

	headerResp, err := e2esuite.GRPCQuery[cmtservice.GetBlockByHeightResponse](ctx, chain, &cmtservice.GetBlockByHeightRequest{
		Height: latestHeight,
	})
	if err != nil {
		return nil, err
	}

	return &headerResp.SdkBlock.Header, nil
}
