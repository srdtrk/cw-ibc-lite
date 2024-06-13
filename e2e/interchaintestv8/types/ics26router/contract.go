package ics26router

import (
	"context"

	"github.com/strangelove-ventures/interchaintest/v8/chain/cosmos"

	"github.com/srdtrk/cw-ibc-lite/e2esuite/v8/types"
)

// Contract represents a cw-ibc-lite-ics26-router contract on a Chain.
type Contract = types.Contract[InstantiateMsg, ExecuteMsg, QueryMsg, QueryClient]

// Instantiate creates a new cw-ibc-lite-ics26-router contract instance on the chain.
// It also creates a query client for the contract.
// This function is useful when you want to deploy a new contract on the chain.
func Instantiate(ctx context.Context, callerKeyName, codeId, admin string, chain *cosmos.CosmosChain, msg InstantiateMsg, extraExecTxArgs ...string) (*Contract, error) {
	contract, err := types.Instantiate[InstantiateMsg, ExecuteMsg, QueryMsg, QueryClient](ctx, callerKeyName, codeId, admin, chain, msg, extraExecTxArgs...)
	if err != nil {
		return nil, err
	}

	queryClient, err := NewQueryClient(chain.GetHostGRPCAddress(), contract.Address)
	if err != nil {
		return nil, err
	}
	contract.SetQueryClient(queryClient)

	return contract, nil
}

// NewContract creates a Contract from a given given contract address, code id and chain.
// It also creates a query client for the contract.
// This function is useful when you already have a contract deployed on the chain.
func NewContract(address string, codeId string, chain *cosmos.CosmosChain) (*Contract, error) {
	contract := types.Contract[InstantiateMsg, ExecuteMsg, QueryMsg, QueryClient]{
		Address: address,
		CodeID:  codeId,
		Chain:   chain,
	}

	queryClient, err := NewQueryClient(chain.GetHostGRPCAddress(), contract.Address)
	if err != nil {
		return nil, err
	}
	contract.SetQueryClient(queryClient)

	return &contract, nil
}
