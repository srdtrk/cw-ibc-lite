package e2esuite

import (
	"context"
	"fmt"

	"cosmossdk.io/math"

	"github.com/cosmos/cosmos-sdk/client"
	"github.com/cosmos/cosmos-sdk/client/tx"
	sdk "github.com/cosmos/cosmos-sdk/types"

	clienttypes "github.com/cosmos/ibc-go/v8/modules/core/02-client/types"
	ibctm "github.com/cosmos/ibc-go/v8/modules/light-clients/07-tendermint"

	"github.com/strangelove-ventures/interchaintest/v8/chain/cosmos"
	"github.com/strangelove-ventures/interchaintest/v8/ibc"
	"github.com/strangelove-ventures/interchaintest/v8/testutil"
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
	ctx context.Context, chain ibc.Chain, trustedHeight clienttypes.Height,
) (*ibctm.Header, error) {
	cosmosChain, ok := chain.(*cosmos.CosmosChain)
	if !ok {
		return nil, fmt.Errorf("QueryTxsByEvents must be passed a cosmos.CosmosChain")
	}

	cmd := []string{"ibc", "client", "header"}
	stdout, _, err := cosmosChain.GetNode().ExecQuery(ctx, cmd...)
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
