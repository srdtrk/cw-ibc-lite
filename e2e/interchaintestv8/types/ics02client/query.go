/* Code generated by github.com/srdtrk/go-codegen, DO NOT EDIT. */
package ics02client

import (
	"context"
	"encoding/json"
	wasmtypes "github.com/CosmWasm/wasmd/x/wasm/types"
	grpc "google.golang.org/grpc"
	insecure "google.golang.org/grpc/credentials/insecure"
)

// QueryClient is the client API for Query service.
//
// For semantics around ctx use and closing/ending streaming RPCs, please refer to https://godoc.org/google.golang.org/grpc#ClientConn.NewStream.
type QueryClient interface {
	// ClientInfo is the client API for the QueryMsg_ClientInfo query message
	ClientInfo(ctx context.Context, req *QueryMsg_ClientInfo, opts ...grpc.CallOption) (*ClientInfo, error)
	// Counterparty is the client API for the QueryMsg_Counterparty query message
	Counterparty(ctx context.Context, req *QueryMsg_Counterparty, opts ...grpc.CallOption) (*CounterpartyInfo_2, error)
	// QueryClient is the client API for the QueryMsg_QueryClient query message
	QueryClient(ctx context.Context, req *QueryMsg_QueryClient, opts ...grpc.CallOption) (*QueryClient_2, error)
}

type queryClient struct {
	cc      *grpc.ClientConn
	address string
}

var _ QueryClient = (*queryClient)(nil)

// NewQueryClient creates a new QueryClient
func NewQueryClient(gRPCAddress, contractAddress string, opts ...grpc.DialOption) (QueryClient, error) {
	if len(opts) == 0 {
		opts = append(opts, grpc.WithTransportCredentials(insecure.NewCredentials()))
	}

	// Create a connection to the gRPC server
	grpcConn, err := grpc.Dial(gRPCAddress, opts...)
	if err != nil {
		return nil, err
	}

	return &queryClient{
		address: contractAddress,
		cc:      grpcConn,
	}, nil
}

// Close closes the gRPC connection to the server
func (q *queryClient) Close() error {
	return q.cc.Close()
}

// queryContract is a helper function to query the contract with raw query data
func (q *queryClient) queryContract(ctx context.Context, rawQueryData []byte, opts ...grpc.CallOption) ([]byte, error) {
	in := &wasmtypes.QuerySmartContractStateRequest{
		Address:   q.address,
		QueryData: rawQueryData,
	}
	out := new(wasmtypes.QuerySmartContractStateResponse)
	err := q.cc.Invoke(ctx, "/cosmwasm.wasm.v1.Query/SmartContractState", in, out, opts...)
	if err != nil {
		return nil, err
	}
	return out.Data, nil
}

func (q *queryClient) ClientInfo(ctx context.Context, req *QueryMsg_ClientInfo, opts ...grpc.CallOption) (*ClientInfo, error) {
	rawQueryData, err := json.Marshal(&QueryMsg{ClientInfo: req})
	if err != nil {
		return nil, err
	}

	rawResponseData, err := q.queryContract(ctx, rawQueryData, opts...)
	if err != nil {
		return nil, err
	}

	var response ClientInfo
	if err := json.Unmarshal(rawResponseData, &response); err != nil {
		return nil, err
	}

	return &response, nil
}

func (q *queryClient) Counterparty(ctx context.Context, req *QueryMsg_Counterparty, opts ...grpc.CallOption) (*CounterpartyInfo_2, error) {
	rawQueryData, err := json.Marshal(&QueryMsg{Counterparty: req})
	if err != nil {
		return nil, err
	}

	rawResponseData, err := q.queryContract(ctx, rawQueryData, opts...)
	if err != nil {
		return nil, err
	}

	var response CounterpartyInfo_2
	if err := json.Unmarshal(rawResponseData, &response); err != nil {
		return nil, err
	}

	return &response, nil
}

func (q *queryClient) QueryClient(ctx context.Context, req *QueryMsg_QueryClient, opts ...grpc.CallOption) (*QueryClient_2, error) {
	rawQueryData, err := json.Marshal(&QueryMsg{QueryClient: req})
	if err != nil {
		return nil, err
	}

	rawResponseData, err := q.queryContract(ctx, rawQueryData, opts...)
	if err != nil {
		return nil, err
	}

	var response QueryClient_2
	if err := json.Unmarshal(rawResponseData, &response); err != nil {
		return nil, err
	}

	return &response, nil
}
