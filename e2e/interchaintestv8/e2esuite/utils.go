package e2esuite

import (
	"context"
	"fmt"

	"cosmossdk.io/math"

	"github.com/cosmos/cosmos-sdk/client"
	"github.com/cosmos/cosmos-sdk/client/grpc/cmtservice"
	"github.com/cosmos/cosmos-sdk/client/tx"
	sdk "github.com/cosmos/cosmos-sdk/types"

	clienttypes "github.com/cosmos/ibc-go/v8/modules/core/02-client/types"
	ibctm "github.com/cosmos/ibc-go/v8/modules/light-clients/07-tendermint"

	"github.com/strangelove-ventures/interchaintest/v8/chain/cosmos"
	"github.com/strangelove-ventures/interchaintest/v8/ibc"
	"github.com/strangelove-ventures/interchaintest/v8/testutil"

	"github.com/srdtrk/cw-ibc-lite/e2esuite/v8/testvalues"
	"github.com/srdtrk/cw-ibc-lite/e2esuite/v8/types"
)

// FundAddressChainA sends funds to the given address on Chain A.
// The amount sent is 1,000,000,000 of the chain's denom.
func (s *TestSuite) FundAddressChainA(ctx context.Context, address string) {
	s.fundAddress(ctx, s.ChainA, s.UserA.KeyName(), address)
}

// FundAddressChainB sends funds to the given address on Chain B.
// The amount sent is 1,000,000,000 of the chain's denom.
func (s *TestSuite) FundAddressChainB(ctx context.Context, address string) {
	s.fundAddress(ctx, s.ChainB, s.UserB.KeyName(), address)
}

// BroadcastMessages broadcasts the provided messages to the given chain and signs them on behalf of the provided user.
// Once the broadcast response is returned, we wait for two blocks to be created on chain.
func (s *TestSuite) BroadcastMessages(ctx context.Context, chain *cosmos.CosmosChain, user ibc.Wallet, gas uint64, msgs ...sdk.Msg) (*sdk.TxResponse, error) {
	sdk.GetConfig().SetBech32PrefixForAccount(chain.Config().Bech32Prefix, chain.Config().Bech32Prefix+sdk.PrefixPublic)
	sdk.GetConfig().SetBech32PrefixForValidator(
		chain.Config().Bech32Prefix+sdk.PrefixValidator+sdk.PrefixOperator,
		chain.Config().Bech32Prefix+sdk.PrefixValidator+sdk.PrefixOperator+sdk.PrefixPublic,
	)

	broadcaster := cosmos.NewBroadcaster(s.T(), chain)

	broadcaster.ConfigureClientContextOptions(func(clientContext client.Context) client.Context {
		return clientContext.
			WithCodec(chain.Config().EncodingConfig.Codec).
			WithChainID(chain.Config().ChainID).
			WithTxConfig(chain.Config().EncodingConfig.TxConfig)
	})

	broadcaster.ConfigureFactoryOptions(func(factory tx.Factory) tx.Factory {
		return factory.WithGas(gas)
	})

	resp, err := cosmos.BroadcastTx(ctx, broadcaster, user, msgs...)
	if err != nil {
		return nil, err
	}

	// wait for 2 blocks for the transaction to be included
	s.Require().NoError(testutil.WaitForBlocks(ctx, 2, chain))

	return &resp, nil
}

// fundAddress sends funds to the given address on the given chain
func (s *TestSuite) fundAddress(ctx context.Context, chain *cosmos.CosmosChain, keyName, address string) {
	err := chain.SendFunds(ctx, keyName, ibc.WalletAmount{
		Address: address,
		Denom:   chain.Config().Denom,
		Amount:  math.NewInt(1_000_000_000),
	})
	s.Require().NoError(err)

	// wait for 2 blocks for the funds to be received
	err = testutil.WaitForBlocks(ctx, 2, chain)
	s.Require().NoError(err)
}

// ExtractValueFromEvents extracts the value of an attribute from a list of events.
// If the attribute is not found, the function returns an empty string and false.
// If the attribute is found, the function returns the value and true.
func (*TestSuite) ExtractValueFromEvents(events sdk.StringEvents, eventType, attrKey string) (string, bool) {
	for _, event := range events {
		if event.Type != eventType {
			continue
		}

		for _, attr := range event.Attributes {
			if attr.Key != attrKey {
				continue
			}

			return attr.Value, true
		}
	}

	return "", false
}

// QuerySignedHeader queries the signed header from the chain
func (s *TestSuite) QuerySignedHeader(
	ctx context.Context, chain *cosmos.CosmosChain, trustedHeight clienttypes.Height,
) (*ibctm.Header, error) {
	cmd := []string{"ibc", "client", "header"}
	stdout, _, err := chain.GetNode().ExecQuery(ctx, cmd...)
	if err != nil {
		return nil, err
	}

	result := &ibctm.Header{}
	err = chain.Config().EncodingConfig.Codec.UnmarshalJSON(stdout, result)
	if err != nil {
		return nil, err
	}

	// NOTE: We assume that the trusted validators are the same as the current validator set
	result.TrustedValidators = result.ValidatorSet
	result.TrustedHeight = trustedHeight

	return result, nil
}

// QueryProofs queries the proofs from the chain for the given key
func (*TestSuite) QueryProofs(
	ctx context.Context, chain *cosmos.CosmosChain,
	storeKey string, key []byte, height int64,
) ([]byte, []byte, int64, error) {
	resp, err := GRPCQuery[cmtservice.ABCIQueryResponse](ctx, chain, &cmtservice.ABCIQueryRequest{
		Path:   fmt.Sprintf("store/%s/key", storeKey),
		Height: height - 1, // Copied from ibc-go test
		Data:   key,
		Prove:  true,
	})
	if err != nil {
		return nil, nil, 0, err
	}

	merkleProof, err := types.ConvertProofs(resp.ProofOps)
	if err != nil {
		return nil, nil, 0, err
	}

	proof, err := chain.Config().EncodingConfig.Codec.Marshal(&merkleProof)
	if err != nil {
		return nil, nil, 0, err
	}

	// proof height + 1 is returned as the proof created corresponds to the height the proof
	// was created in the IAVL tree. Tendermint and subsequently the clients that rely on it
	// have heights 1 above the IAVL tree. Thus we return proof height + 1
	return resp.Value, proof, resp.Height + 1, nil
}

func (s *TestSuite) FetchHeader(ctx context.Context, chain *cosmos.CosmosChain) (*cmtservice.Header, error) {
	latestHeight, err := chain.Height(ctx)
	if err != nil {
		return nil, err
	}

	headerResp, err := GRPCQuery[cmtservice.GetBlockByHeightResponse](ctx, chain, &cmtservice.GetBlockByHeightRequest{
		Height: latestHeight,
	})
	if err != nil {
		return nil, err
	}

	return &headerResp.SdkBlock.Header, nil
}

// GetRelayerUsers returns two ibc.Wallet instances which can be used for the relayer users
// on the two chains.
func (s *TestSuite) GetRelayerUsers(ctx context.Context) (ibc.Wallet, ibc.Wallet) {
	chainA, chainB := s.ChainA, s.ChainB
	chainAAccountBytes, err := chainA.GetAddress(ctx, testvalues.ChainARelayerName)
	s.Require().NoError(err)

	chainBAccountBytes, err := chainB.GetAddress(ctx, testvalues.ChainBRelayerName)
	s.Require().NoError(err)

	chainARelayerUser := cosmos.NewWallet(testvalues.ChainARelayerName, chainAAccountBytes, "", chainA.Config())
	chainBRelayerUser := cosmos.NewWallet(testvalues.ChainBRelayerName, chainBAccountBytes, "", chainB.Config())

	return chainARelayerUser, chainBRelayerUser
}
