package main

import (
	"context"
	"encoding/base64"
	"encoding/hex"
	"errors"
	"fmt"
	"strconv"
	"strings"
	"testing"
	"time"

	"github.com/cosmos/gogoproto/proto"
	"github.com/stretchr/testify/suite"

	sdkmath "cosmossdk.io/math"

	codectypes "github.com/cosmos/cosmos-sdk/codec/types"
	sdk "github.com/cosmos/cosmos-sdk/types"
	banktypes "github.com/cosmos/cosmos-sdk/x/bank/types"

	abci "github.com/cometbft/cometbft/abci/types"

	transfertypes "github.com/cosmos/ibc-go/v8/modules/apps/transfer/types"
	clienttypes "github.com/cosmos/ibc-go/v8/modules/core/02-client/types"
	channeltypes "github.com/cosmos/ibc-go/v8/modules/core/04-channel/types"
	commitmenttypes "github.com/cosmos/ibc-go/v8/modules/core/23-commitment/types"
	commitmenttypesv2 "github.com/cosmos/ibc-go/v8/modules/core/23-commitment/types/v2"
	host "github.com/cosmos/ibc-go/v8/modules/core/24-host"
	ibcexported "github.com/cosmos/ibc-go/v8/modules/core/exported"
	ibctm "github.com/cosmos/ibc-go/v8/modules/light-clients/07-tendermint"
	ibctesting "github.com/cosmos/ibc-go/v8/testing"

	wasmtypes "github.com/CosmWasm/wasmd/x/wasm/types"

	"github.com/strangelove-ventures/interchaintest/v8"
	"github.com/strangelove-ventures/interchaintest/v8/chain/cosmos"
	"github.com/strangelove-ventures/interchaintest/v8/ibc"
	"github.com/strangelove-ventures/interchaintest/v8/testutil"

	"github.com/srdtrk/cw-ibc-lite/e2esuite/v8/e2esuite"
	"github.com/srdtrk/cw-ibc-lite/e2esuite/v8/testvalues"
	"github.com/srdtrk/cw-ibc-lite/e2esuite/v8/types"
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

		// ics02Client is instantiated by ics26Router, so we need to fetch it from the response
		// and cast it to the correct type
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
						KeyPath: []string{ibcexported.StoreKey, ""},
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

		clientInfo, err := s.ics02Client.QueryClient().ClientInfo(ctx, &ics02client.QueryMsg_ClientInfo{ClientId: testvalues.FirstWasmClientID})
		s.Require().NoError(err)
		s.Require().Equal(clientInfo.ClientId, testvalues.FirstWasmClientID)
		s.Require().Equal(clientInfo.CounterpartyInfo.ClientId, ibctesting.FirstClientID)
		s.Require().Equal(clientInfo.CounterpartyInfo.MerklePathPrefix.KeyPath, []string{ibcexported.StoreKey, ""})
		s.Require().Equal(clientInfo.Address, s.ics07Tendermint.Address)
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

		contractAddr, err := s.ics26Router.QueryClient().PortRouter(ctx, &ics26router.QueryMsg_PortRouter{
			PortId: s.ics20Transfer.Port(),
		})
		s.Require().NoError(err)
		s.Require().Equal(s.ics20Transfer.Address, *contractAddr)
	}))

	s.Require().True(s.Run("Instantiate CW20", func() {
		var err error
		s.cw20Base, err = cw20base.Instantiate(ctx, s.UserA.KeyName(), cw20CodeId, s.UserA.FormattedAddress(), wasmd, cw20base.InstantiateMsg{
			Name:     "IBC Lite E2E Test Token",
			Symbol:   "ATOM",
			Decimals: 6,
			InitialBalances: []cw20base.Cw20Coin{{
				Address: s.UserA.FormattedAddress(),
				Amount:  cw20base.Uint128(strconv.FormatInt(testvalues.StartingTokenAmount, 10)),
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
			CounterpartyId:   testvalues.FirstWasmClientID,
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

// TestCW20Transfer tests the transfer of CW20 tokens
func (s *IBCLiteTestSuite) TestCW20Transfer() {
	ctx := context.Background()
	s.SetupSuite(ctx)

	wasmd, simd := s.ChainA, s.ChainB

	// Transfer some tokens from UserA to UserB
	const sendAmount = 1_000_000
	var packet channeltypes.Packet
	s.Require().True(s.Run("SendPacket", func() {
		transferMsg := cw20base.MsgTransfer{
			SourceChannel: testvalues.FirstWasmClientID,
			Receiver:      s.UserB.FormattedAddress(),
		}
		cw20SendMsg := cw20base.ExecuteMsg{
			Send: &cw20base.ExecuteMsg_Send{
				Amount:   cw20base.Uint128(strconv.FormatInt(sendAmount, 10)),
				Contract: s.ics20Transfer.Address,
				Msg:      cw20base.ToJsonBinary(transferMsg),
			},
		}

		res, err := s.cw20Base.Execute(ctx, s.UserA.KeyName(), cw20SendMsg, "--gas", "500000")
		s.Require().NoError(err)

		packet, err = s.ExtractPacketFromEvents(res.Events)
		s.Require().NoError(err)
	}))

	s.Require().NoError(s.Relayer.UpdateClients(ctx, s.ExecRep, s.PathName))
	s.Require().NoError(testutil.WaitForBlocks(ctx, 3, simd))

	var (
		clientState *ibctm.ClientState
		proofHeight int64
		proof       []byte
		value       []byte
		merklePath  commitmenttypesv2.MerklePath
	)
	s.Require().True(s.Run("Generate Packet Proof", func() {
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
		packetKey := host.PacketCommitmentPath(packet.SourcePort, packet.SourceChannel, packet.Sequence)
		key := cloneAppend(prefixStoreKey, []byte(packetKey))
		merklePath = commitmenttypes.NewMerklePath(key)
		merklePath, err = commitmenttypes.ApplyPrefix(commitmenttypes.NewMerklePrefix([]byte(wasmtypes.StoreKey)), merklePath)
		s.Require().NoError(err)

		value, proof, proofHeight, err = s.QueryProofs(ctx, wasmd, wasmtypes.StoreKey, key, int64(clientState.LatestHeight.RevisionHeight))
		s.Require().NoError(err)
		s.Require().NotEmpty(proof)
		s.Require().NotEmpty(value)
		s.Require().Equal(int64(clientState.LatestHeight.RevisionHeight), proofHeight)
		expCommitment := channeltypes.CommitLitePacket(simd.Config().EncodingConfig.Codec, packet)
		s.Require().Equal(expCommitment, value)
	}))

	var (
		acknowledgement []byte
		simdCoin        *sdk.Coin
	)
	s.Require().True(s.Run("RecvPacket", func() {
		recvMsg := &channeltypes.MsgRecvPacket{
			Packet:          packet,
			ProofCommitment: proof,
			ProofHeight:     clientState.LatestHeight,
			Signer:          s.UserB.FormattedAddress(),
		}

		txResp, err := s.BroadcastMessages(ctx, simd, s.UserB, 200_000, recvMsg)
		s.Require().NoError(err)

		s.Require().True(s.Run("Check balances", func() {
			ibcDenom := transfertypes.ParseDenomTrace(
				fmt.Sprintf("%s/%s/%s", transfertypes.PortID, ibctesting.FirstClientID, s.cw20Base.Address),
			).IBCDenom()

			// Check the balance of UserB
			resp, err := e2esuite.GRPCQuery[banktypes.QueryBalanceResponse](ctx, simd, &banktypes.QueryBalanceRequest{
				Address: s.UserB.FormattedAddress(),
				Denom:   ibcDenom,
			})
			s.Require().NoError(err)
			s.Require().NotNil(resp.Balance)
			s.Require().Equal(int64(sendAmount), resp.Balance.Amount.Int64())
			s.Require().Equal(ibcDenom, resp.Balance.Denom)
			simdCoin = resp.Balance

			// Check the balance of UserA
			cw20Resp, err := s.cw20Base.QueryClient().Balance(ctx, &cw20base.QueryMsg_Balance{Address: s.UserA.FormattedAddress()})
			s.Require().NoError(err)
			s.Require().Equal(strconv.FormatInt(testvalues.StartingTokenAmount-sendAmount, 10), string(cw20Resp.Balance))
		}))

		ackHex, found := s.ExtractValueFromEvents(txResp.Events, channeltypes.EventTypeWriteAck, channeltypes.AttributeKeyAckHex)
		s.Require().True(found)

		acknowledgement, err = hex.DecodeString(ackHex)
		s.Require().NoError(err)
	}))

	s.UpdateClientContract(ctx, s.ics07Tendermint, simd)

	s.Require().True(s.Run("Generate ack proof", func() {
		var err error
		key := host.PacketAcknowledgementKey(packet.DestinationPort, packet.DestinationChannel, packet.Sequence)
		merklePath = commitmenttypes.NewMerklePath(key)
		merklePath, err = commitmenttypes.ApplyPrefix(commitmenttypes.NewMerklePrefix([]byte(ibcexported.StoreKey)), merklePath)
		s.Require().NoError(err)

		commitmentBz := channeltypes.CommitAcknowledgement(acknowledgement)
		value, proof, proofHeight, err = s.QueryProofs(ctx, simd, ibcexported.StoreKey, key, int64(s.trustedHeight.RevisionHeight))
		s.Require().NoError(err)
		s.Require().NotEmpty(proof)
		s.Require().NotEmpty(value)
		s.Require().Equal(int64(s.trustedHeight.RevisionHeight), proofHeight)
		s.Require().Equal(commitmentBz, value)
	}))

	s.Require().True(s.Run("Verify ack proof", func() {
		_, err := s.ics07Tendermint.QueryClient().VerifyMembership(ctx, &ics07tendermint.QueryMsg_VerifyMembership{
			DelayBlockPeriod: 0,
			DelayTimePeriod:  0,
			Height: ics07tendermint.Height2{
				RevisionNumber: int(s.trustedHeight.RevisionNumber),
				RevisionHeight: int(proofHeight),
			},
			Path: ics07tendermint.MerklePath{
				KeyPath: types.ToLegacyMerklePath(&merklePath).KeyPath,
			},
			Proof: base64.StdEncoding.EncodeToString(proof),
			Value: base64.StdEncoding.EncodeToString(value),
		})
		s.Require().NoError(err)
	}))

	s.Require().True(s.Run("AckPacket", func() {
		ackMsg := ics26router.ExecuteMsg{
			Acknowledgement: &ics26router.ExecuteMsg_Acknowledgement{
				Acknowledgement: ics26router.ToBinary(acknowledgement),
				ProofAcked:      ics26router.ToBinary(proof),
				ProofHeight: ics26router.Height{
					RevisionHeight: int(s.trustedHeight.RevisionHeight),
					RevisionNumber: int(s.trustedHeight.RevisionNumber),
				},
				Packet: ics26router.ToPacket(packet),
			},
		}

		_, err := s.ics26Router.Execute(ctx, s.UserA.KeyName(), ackMsg, "--gas", "500000")
		s.Require().NoError(err)
	}))

	// Now we send the packet back
	var packet2 channeltypes.Packet
	s.Require().True(s.Run("SendPacket2", func() {
		msgTransfer := transfertypes.MsgTransfer{
			SourcePort:       transfertypes.PortID,
			SourceChannel:    ibctesting.FirstClientID,
			Token:            *simdCoin,
			Sender:           s.UserB.FormattedAddress(),
			Receiver:         s.UserA.FormattedAddress(),
			DestPort:         s.ics20Transfer.Port(),
			DestChannel:      testvalues.FirstWasmClientID,
			TimeoutTimestamp: uint64(time.Now().Add(10 * time.Minute).UnixNano()),
		}

		txResp, err := s.BroadcastMessages(ctx, simd, s.UserB, 200_000, &msgTransfer)
		s.Require().NoError(err)

		packet2, err = s.ExtractPacketFromEvents(txResp.Events)
		s.Require().NoError(err)
		s.Require().Equal(s.ics20Transfer.Port(), packet2.DestinationPort)
		s.Require().Equal(testvalues.FirstWasmClientID, packet2.DestinationChannel)
		s.Require().Equal(transfertypes.PortID, packet2.SourcePort)
		s.Require().Equal(ibctesting.FirstClientID, packet2.SourceChannel)
	}))

	s.UpdateClientContract(ctx, s.ics07Tendermint, simd)

	s.Require().True(s.Run("Generate Packet2 Proof", func() {
		var err error
		key := host.PacketCommitmentPath(packet2.SourcePort, packet2.SourceChannel, packet2.Sequence)
		merklePath = commitmenttypes.NewMerklePath([]byte(key))
		merklePath, err = commitmenttypes.ApplyPrefix(commitmenttypes.NewMerklePrefix([]byte(ibcexported.StoreKey)), merklePath)
		s.Require().NoError(err)

		value, proof, proofHeight, err = s.QueryProofs(ctx, simd, ibcexported.StoreKey, []byte(key), int64(s.trustedHeight.RevisionHeight))
		s.Require().NoError(err)
		s.Require().NotEmpty(proof)
		s.Require().NotEmpty(value)
		s.Require().Equal(int64(s.trustedHeight.RevisionHeight), proofHeight)
		expCommitment := channeltypes.CommitLitePacket(simd.Config().EncodingConfig.Codec, packet2)
		s.Require().Equal(expCommitment, value)
	}))

	var acknowledgement2 []byte
	s.Require().True(s.Run("RecvPacket2", func() {
		recvMsg := ics26router.ExecuteMsg{
			RecvPacket: &ics26router.ExecuteMsg_RecvPacket{
				Packet:          ics26router.ToPacket(packet2),
				ProofCommitment: ics26router.ToBinary(proof),
				ProofHeight: ics26router.Height{
					RevisionHeight: int(s.trustedHeight.RevisionHeight),
					RevisionNumber: int(s.trustedHeight.RevisionNumber),
				},
			},
		}

		txResp, err := s.ics26Router.Execute(ctx, s.UserA.KeyName(), recvMsg, "--gas", "700000")
		s.Require().NoError(err)

		s.Require().True(s.Run("Check balances", func() {
			// Check the balance of UserB
			resp, err := e2esuite.GRPCQuery[banktypes.QueryBalanceResponse](ctx, simd, &banktypes.QueryBalanceRequest{
				Address: s.UserB.FormattedAddress(),
				Denom:   simdCoin.Denom,
			})
			s.Require().NoError(err)
			s.Require().NotNil(resp.Balance)
			s.Require().Equal(int64(0), resp.Balance.Amount.Int64())

			// Check the balance of UserA
			cw20Resp, err := s.cw20Base.QueryClient().Balance(ctx, &cw20base.QueryMsg_Balance{Address: s.UserA.FormattedAddress()})
			s.Require().NoError(err)
			s.Require().Equal(strconv.FormatInt(testvalues.StartingTokenAmount, 10), string(cw20Resp.Balance))
		}))

		ackHex, found := s.ExtractValueFromEvents(txResp.Events, wasmtypes.CustomContractEventPrefix+channeltypes.EventTypeWriteAck, channeltypes.AttributeKeyAckHex)
		s.Require().True(found)

		acknowledgement2, err = hex.DecodeString(ackHex)
		s.Require().NoError(err)
		s.Require().Equal([]byte(`{"result":"AQ=="}`), acknowledgement2)
	}))

	s.Require().NoError(s.Relayer.UpdateClients(ctx, s.ExecRep, s.PathName))
	s.Require().NoError(testutil.WaitForBlocks(ctx, 3, simd))

	s.Require().True(s.Run("Generate ack proof", func() {
		resp, err := e2esuite.GRPCQuery[clienttypes.QueryClientStateResponse](ctx, simd, &clienttypes.QueryClientStateRequest{
			ClientId: ibctesting.FirstClientID,
		})
		s.Require().NoError(err)

		err = proto.Unmarshal(resp.ClientState.Value, clientState)
		s.Require().NoError(err)

		contractAddr, err := s.ics26Router.AccAddress()
		s.Require().NoError(err)

		prefixStoreKey := wasmtypes.GetContractStorePrefix(contractAddr)
		packetKey := host.PacketAcknowledgementKey(packet2.DestinationPort, packet2.DestinationChannel, packet2.Sequence)
		key := cloneAppend(prefixStoreKey, packetKey)
		merklePath = commitmenttypes.NewMerklePath(key)
		merklePath, err = commitmenttypes.ApplyPrefix(commitmenttypes.NewMerklePrefix([]byte(wasmtypes.StoreKey)), merklePath)
		s.Require().NoError(err)

		commitmentBz := channeltypes.CommitAcknowledgement(acknowledgement2)
		value, proof, proofHeight, err = s.QueryProofs(ctx, wasmd, wasmtypes.StoreKey, key, int64(clientState.LatestHeight.RevisionHeight))
		s.Require().NoError(err)
		s.Require().NotEmpty(proof)
		s.Require().NotEmpty(value)
		s.Require().Equal(int64(clientState.LatestHeight.RevisionHeight), proofHeight)
		s.Require().Equal(commitmentBz, value)
	}))

	s.Require().True(s.Run("AckPacket2", func() {
		ackMsg := channeltypes.MsgAcknowledgement{
			Packet:          packet2,
			Acknowledgement: acknowledgement2,
			ProofAcked:      proof,
			ProofHeight:     clientState.LatestHeight,
			Signer:          s.UserB.FormattedAddress(),
		}

		_, err := s.BroadcastMessages(ctx, simd, s.UserB, 200_000, &ackMsg)
		s.Require().NoError(err)
	}))
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

func (s *IBCLiteTestSuite) ExtractPacketFromEvents(events []abci.Event) (channeltypes.Packet, error) {
	var (
		err        error
		sourceCh   string
		destCh     string
		sourcePort string
		destPort   string
		data       []byte
		sequence   uint64
		timeout    uint64
	)
	for _, event := range events {
		if !strings.HasPrefix(event.Type, wasmtypes.CustomContractEventPrefix) && event.Type != channeltypes.EventTypeSendPacket {
			continue
		}

		for _, attr := range event.Attributes {
			switch attr.Key {
			case channeltypes.AttributeKeySrcChannel:
				sourceCh = attr.Value
			case channeltypes.AttributeKeyDstChannel:
				destCh = attr.Value
			case channeltypes.AttributeKeySrcPort:
				sourcePort = attr.Value
			case channeltypes.AttributeKeyDstPort:
				destPort = attr.Value
			case channeltypes.AttributeKeySequence:
				sequence, err = strconv.ParseUint(attr.Value, 10, 64)
				s.Require().NoError(err)
			case channeltypes.AttributeKeyDataHex:
				data, err = hex.DecodeString(attr.Value)
				s.Require().NoError(err)
			case channeltypes.AttributeKeyTimeoutTimestamp:
				timeout, err = strconv.ParseUint(attr.Value, 10, 64)
				s.Require().NoError(err)
			default:
				continue
			}
		}
	}

	if sourceCh == "" || destCh == "" || sourcePort == "" || destPort == "" || len(data) == 0 || timeout == 0 || sequence == 0 {
		return channeltypes.Packet{}, errors.New("packet not found in wasm events")
	}

	return channeltypes.Packet{
		Sequence:           sequence,
		SourcePort:         sourcePort,
		SourceChannel:      sourceCh,
		DestinationPort:    destPort,
		DestinationChannel: destCh,
		Data:               data,
		TimeoutTimestamp:   timeout,
	}, nil
}
