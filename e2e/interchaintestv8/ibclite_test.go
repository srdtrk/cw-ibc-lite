package main

import (
	"context"
	"encoding/base64"
	"strconv"
	"testing"

	"github.com/cosmos/gogoproto/proto"
	"github.com/stretchr/testify/suite"

	sdkmath "cosmossdk.io/math"

	codectypes "github.com/cosmos/cosmos-sdk/codec/types"

	clienttypes "github.com/cosmos/ibc-go/v8/modules/core/02-client/types"
	commitmenttypes "github.com/cosmos/ibc-go/v8/modules/core/23-commitment/types"
	commitmenttypesv2 "github.com/cosmos/ibc-go/v8/modules/core/23-commitment/types/v2"
	ibcexported "github.com/cosmos/ibc-go/v8/modules/core/exported"
	ibctm "github.com/cosmos/ibc-go/v8/modules/light-clients/07-tendermint"
	ibctesting "github.com/cosmos/ibc-go/v8/testing"

	wasmtypes "github.com/CosmWasm/wasmd/x/wasm/types"

	"github.com/strangelove-ventures/interchaintest/v8"
	"github.com/strangelove-ventures/interchaintest/v8/chain/cosmos"
	"github.com/strangelove-ventures/interchaintest/v8/ibc"

	"github.com/srdtrk/cw-ibc-lite/e2esuite/v8/e2esuite"
	"github.com/srdtrk/cw-ibc-lite/e2esuite/v8/testvalues"
	"github.com/srdtrk/cw-ibc-lite/e2esuite/v8/types/cw20base"
	"github.com/srdtrk/cw-ibc-lite/e2esuite/v8/types/ics02client"
	"github.com/srdtrk/cw-ibc-lite/e2esuite/v8/types/ics07tendermint"
	"github.com/srdtrk/cw-ibc-lite/e2esuite/v8/types/ics20transfer"
	"github.com/srdtrk/cw-ibc-lite/e2esuite/v8/types/ics26router"
)

// IBCLiteTestSuite is a suite of tests that wraps the TestSuite
// and can provide additional functionality
type IBCLiteTestSuite struct {
	e2esuite.TestSuite

	// cw20Base is the cw20 base contract for the tokens to be transferred
	cw20Base *cw20base.Contract

	// This is the admin of all the cw-ibc-lite contracts
	// In production, this should be the chain's governance module
	govAccount ibc.Wallet

	ics26Router     *ics26router.Contract
	ics02Client     *ics02client.Contract
	ics07Tendermint *ics07tendermint.Contract
	ics20Transfer   *ics20transfer.Contract

	// lastest trustedHeight of the ics07Tendermint contract
	trustedHeight clienttypes.Height
	// this line is used by go-codegen # suite/contract
}

// SetupSuite calls the underlying ICS07TendermintTestSuite's SetupSuite method
func (s *IBCLiteTestSuite) SetupSuite(ctx context.Context) {
	s.TestSuite.SetupSuite(ctx)
	wasmd, simd := s.ChainA, s.ChainB

	var (
		ics26CodeId string
		ics02CodeId string
		ics07CodeId string
		ics20CodeId string
		cw20CodeId  string
	)
	s.Require().True(s.Run("UploadCodes", func() {
		var err error
		ics26CodeId, err = wasmd.StoreContract(ctx, s.UserA.KeyName(), "../../artifacts/cw_ibc_lite_ics26_router.wasm")
		s.Require().NoError(err)
		s.Require().NotEmpty(ics26CodeId)
		ics02CodeId, err = wasmd.StoreContract(ctx, s.UserA.KeyName(), "../../artifacts/cw_ibc_lite_ics02_client.wasm")
		s.Require().NoError(err)
		s.Require().NotEmpty(ics02CodeId)
		ics20CodeId, err = wasmd.StoreContract(ctx, s.UserA.KeyName(), "../../artifacts/cw_ibc_lite_ics20_transfer.wasm")
		s.Require().NoError(err)
		s.Require().NotEmpty(ics20CodeId)
		ics07CodeId, err = wasmd.StoreContract(ctx, s.UserA.KeyName(), "../../artifacts/cw_ibc_lite_ics07_tendermint.wasm")
		s.Require().NoError(err)
		s.Require().NotEmpty(ics07CodeId)
		cw20CodeId, err = wasmd.StoreContract(ctx, s.UserA.KeyName(), "./testdata/cw20_base.wasm")
		s.Require().NoError(err)
		s.Require().NotEmpty(cw20CodeId)
	}))

	s.Require().True(s.Run("Instantiate ICS26 and ICS02", func() {
		s.govAccount = interchaintest.GetAndFundTestUsers(
			s.T(), ctx, s.T().Name(), sdkmath.NewInt(testvalues.StartingTokenAmount), wasmd,
		)[0]

		ics02CodeInt, err := strconv.ParseInt(ics02CodeId, 10, 64)
		s.Require().NoError(err)
		s.ics26Router, err = ics26router.Instantiate(ctx, s.UserA.KeyName(), ics26CodeId, s.govAccount.FormattedAddress(), wasmd, ics26router.InstantiateMsg{
			Ics02ClientCodeId: int(ics02CodeInt),
			Owner:             s.govAccount.FormattedAddress(),
		}, "--gas", "500000")
		s.Require().NoError(err)

		// This should also instantiate the ics02Client contract
		resp, err := e2esuite.GRPCQuery[wasmtypes.QueryContractsByCodeResponse](ctx, wasmd, &wasmtypes.QueryContractsByCodeRequest{
			CodeId: uint64(ics02CodeInt),
		})
		s.Require().NoError(err)
		s.Require().Len(resp.Contracts, 1)
		s.Require().NotEmpty(resp.Contracts[0])

		s.ics02Client, err = ics02client.NewContract(resp.Contracts[0], ics02CodeId, wasmd)
		s.Require().NoError(err)
	}))

	s.Require().True(s.Run("Instantiate ICS07", func() {
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

		ics07CodeInt, err := strconv.ParseInt(ics07CodeId, 10, 64)
		s.Require().NoError(err)
		_, err = s.ics02Client.Execute(ctx, s.UserA.KeyName(), ics02client.ExecuteMsg{
			CreateClient: &ics02client.ExecuteMsg_CreateClient{
				CodeId: int(ics07CodeInt),
				CounterpartyInfo: &ics02client.CounterpartyInfo{
					ClientId: ibctesting.FirstClientID,
					MerklePathPrefix: &ics02client.MerklePath{
						// TODO: make sure this is correct
						KeyPath: []string{ibcexported.StoreKey},
					},
				},
				InstantiateMsg: ics02client.InstantiateMsg_2{
					ClientState:    ics02client.ToBinary(clientStateBz),
					ConsensusState: ics02client.ToBinary(consensusStateBz),
				},
			},
		}, "--gas", "500000")
		s.Require().NoError(err)

		// This should instantiate the ics07Tendermint contract
		resp, err := e2esuite.GRPCQuery[wasmtypes.QueryContractsByCodeResponse](ctx, wasmd, &wasmtypes.QueryContractsByCodeRequest{
			CodeId: uint64(ics07CodeInt),
		})
		s.Require().NoError(err)
		s.Require().Len(resp.Contracts, 1)
		s.Require().NotEmpty(resp.Contracts[0])

		s.ics07Tendermint, err = ics07tendermint.NewContract(resp.Contracts[0], ics07CodeId, wasmd)
		s.Require().NoError(err)

		s.trustedHeight = height
	}))

	s.Require().True(s.Run("Instantiate ICS20", func() {
		var err error
		s.ics20Transfer, err = ics20transfer.Instantiate(ctx, s.UserA.KeyName(), ics20CodeId, s.govAccount.FormattedAddress(), wasmd, ics20transfer.InstantiateMsg{
			Ics26RouterAddress: s.ics26Router.Address,
		})
		s.Require().NoError(err)

		// Register transfer app to ICS26Router
		_, err = s.ics26Router.Execute(ctx, s.UserA.KeyName(), ics26router.ExecuteMsg{
			RegisterIbcApp: &ics26router.ExecuteMsg_RegisterIbcApp{
				Address: s.ics20Transfer.Address,
			},
		})
		s.Require().NoError(err)
	}))

	s.Require().True(s.Run("Instantiate CW20", func() {
		var err error
		s.cw20Base, err = cw20base.Instantiate(ctx, s.UserA.KeyName(), cw20CodeId, s.UserA.FormattedAddress(), wasmd, cw20base.InstantiateMsg{
			Name:     "IBC Lite E2E Test Token",
			Symbol:   "ATOM",
			Decimals: 6,
			InitialBalances: []cw20base.Cw20Coin{{
				Address: s.UserA.FormattedAddress(),
				Amount:  cw20base.Uint128("100000000"),
			}},
		})
		s.Require().NoError(err)
	}))

	s.Require().True(s.Run("Register counterparty for go client", func() {
		_, simdRelayerUser := s.GetRelayerUsers(ctx)

		contractAddr, err := s.ics26Router.AccAddress()
		s.Require().NoError(err)
		prefixStoreKey := wasmtypes.GetContractStorePrefix(contractAddr)
		merklePathPrefix := commitmenttypes.NewMerklePath([]byte(wasmtypes.StoreKey), prefixStoreKey)

		_, err = s.BroadcastMessages(ctx, simd, simdRelayerUser, 200_000, &clienttypes.MsgProvideCounterparty{
			ClientId:         ibctesting.FirstClientID,
			CounterpartyId:   "08-wasm-0",
			MerklePathPrefix: &merklePathPrefix,
			Signer:           simdRelayerUser.FormattedAddress(),
		})
		s.Require().NoError(err)
	}))
}

// TestWithICS07TendermintTestSuite is the boilerplate code that allows the test suite to be run
func TestWithIBCLiteTestSuite(t *testing.T) {
	suite.Run(t, new(IBCLiteTestSuite))
}

// TestIBCLiteSetup tests the setup of the IBC Lite test suite
// TODO: remove this once there are actual tests
func (s *IBCLiteTestSuite) TestIBCLiteSetup() {
	ctx := context.Background()
	s.SetupSuite(ctx)
}

// This is a test to verify that go clients can prove the state of cosmwasm contracts
func (s *IBCLiteTestSuite) TestWasmProofs() {
	ctx := context.Background()
	s.SetupSuite(ctx)

	wasmd, simd := s.ChainA, s.ChainB

	s.Require().NoError(s.Relayer.UpdateClients(ctx, s.ExecRep, s.PathName))

	// During the setup, we have already committed some state into some contracts.
	// Our goal is to prove the ICS02_CLIENT_ADDRESS state in ics26Router contract
	var (
		clientState *ibctm.ClientState
		proofHeight int64
		proof       []byte
		value       []byte
		merklePath  commitmenttypesv2.MerklePath
	)
	s.Require().True(s.Run("Generate wasm proof", func() {
		resp, err := e2esuite.GRPCQuery[clienttypes.QueryClientStateResponse](ctx, simd, &clienttypes.QueryClientStateRequest{
			ClientId: ibctesting.FirstClientID,
		})
		s.Require().NoError(err)

		clientState = &ibctm.ClientState{}
		err = proto.Unmarshal(resp.ClientState.Value, clientState)
		s.Require().NoError(err)

		contractAddr, err := s.ics26Router.AccAddress()
		s.Require().NoError(err)

		prefixStoreKey := wasmtypes.GetContractStorePrefix(contractAddr)
		ics02AddrKey := "ics02_client_address"
		key := cloneAppend(prefixStoreKey, []byte(ics02AddrKey))
		merklePath = commitmenttypes.NewMerklePath(key)
		merklePath, err = commitmenttypes.ApplyPrefix(commitmenttypes.NewMerklePrefix([]byte(wasmtypes.StoreKey)), merklePath)
		s.Require().NoError(err)

		value, proof, proofHeight, err = s.QueryProofs(ctx, wasmd, wasmtypes.StoreKey, key, int64(clientState.LatestHeight.RevisionHeight))
		s.Require().NoError(err)
		s.Require().NotEmpty(proof)
		s.Require().NotEmpty(value)
		s.Require().Equal(int64(clientState.LatestHeight.RevisionHeight), proofHeight)
		s.Require().Equal(value, []byte(`"`+s.ics02Client.Address+`"`))
	}))

	s.Require().True(s.Run("Verify wasm proof", func() {
		resp, err := e2esuite.GRPCQuery[clienttypes.QueryVerifyMembershipResponse](ctx, simd, &clienttypes.QueryVerifyMembershipRequest{
			ClientId:    ibctesting.FirstClientID,
			Proof:       proof,
			Value:       value,
			MerklePath:  merklePath,
			ProofHeight: clientState.LatestHeight,
		})
		s.Require().NoError(err)
		s.Require().True(resp.Success)
	}))
}

func (s *IBCLiteTestSuite) UpdateClientContract(ctx context.Context, tmContract *ics07tendermint.Contract, counterpartyChain *cosmos.CosmosChain) {
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

func cloneAppend(bz []byte, tail []byte) (res []byte) {
	res = make([]byte, len(bz)+len(tail))
	copy(res, bz)
	copy(res[len(bz):], tail)
	return
}
