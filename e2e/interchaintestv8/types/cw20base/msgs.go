/* Code generated by github.com/srdtrk/go-codegen, DO NOT EDIT. */
package cw20base

type InstantiateMsg struct {
	Symbol string `json:"symbol"`
	Decimals int `json:"decimals"`
	InitialBalances []Cw20Coin `json:"initial_balances"`
	Marketing *InstantiateMarketingInfo `json:"marketing,omitempty"`
	Mint *MinterResponse `json:"mint,omitempty"`
	Name string `json:"name"`
}

type ExecuteMsg struct {
	// Transfer is a base message to move tokens to another account without triggering actions
	Transfer *ExecuteMsg_Transfer `json:"transfer,omitempty"`
	// Burn is a base message to destroy tokens forever
	Burn *ExecuteMsg_Burn `json:"burn,omitempty"`
	// Send is a base message to transfer tokens to a contract and trigger an action on the receiving contract.
	Send *ExecuteMsg_Send `json:"send,omitempty"`
	// Only with "approval" extension. Allows spender to access an additional amount tokens from the owner's (env.sender) account. If expires is Some(), overwrites current allowance expiration with this one.
	IncreaseAllowance *ExecuteMsg_IncreaseAllowance `json:"increase_allowance,omitempty"`
	// Only with "approval" extension. Lowers the spender's access of tokens from the owner's (env.sender) account by amount. If expires is Some(), overwrites current allowance expiration with this one.
	DecreaseAllowance *ExecuteMsg_DecreaseAllowance `json:"decrease_allowance,omitempty"`
	// Only with "approval" extension. Transfers amount tokens from owner -> recipient if `env.sender` has sufficient pre-approval.
	TransferFrom *ExecuteMsg_TransferFrom `json:"transfer_from,omitempty"`
	// Only with "approval" extension. Sends amount tokens from owner -> contract if `env.sender` has sufficient pre-approval.
	SendFrom *ExecuteMsg_SendFrom `json:"send_from,omitempty"`
	// Only with "approval" extension. Destroys tokens forever
	BurnFrom *ExecuteMsg_BurnFrom `json:"burn_from,omitempty"`
	// Only with the "mintable" extension. If authorized, creates amount new tokens and adds to the recipient balance.
	Mint *ExecuteMsg_Mint `json:"mint,omitempty"`
	// Only with the "mintable" extension. The current minter may set a new minter. Setting the minter to None will remove the token's minter forever.
	UpdateMinter *ExecuteMsg_UpdateMinter `json:"update_minter,omitempty"`
	// Only with the "marketing" extension. If authorized, updates marketing metadata. Setting None/null for any of these will leave it unchanged. Setting Some("") will clear this field on the contract storage
	UpdateMarketing *ExecuteMsg_UpdateMarketing `json:"update_marketing,omitempty"`
	// If set as the "marketing" role on the contract, upload a new URL, SVG, or PNG for the token
	UploadLogo *ExecuteMsg_UploadLogo `json:"upload_logo,omitempty"`
}

type QueryMsg struct {
	// Returns the current balance of the given address, 0 if unset.
	Balance *QueryMsg_Balance `json:"balance,omitempty"`
	// Returns metadata on the contract - name, decimals, supply, etc.
	TokenInfo *QueryMsg_TokenInfo `json:"token_info,omitempty"`
	// Only with "mintable" extension. Returns who can mint and the hard cap on maximum tokens after minting.
	Minter *QueryMsg_Minter `json:"minter,omitempty"`
	// Only with "allowance" extension. Returns how much spender can use from owner account, 0 if unset.
	Allowance *QueryMsg_Allowance `json:"allowance,omitempty"`
	// Only with "enumerable" extension (and "allowances") Returns all allowances this owner has approved. Supports pagination.
	AllAllowances *QueryMsg_AllAllowances `json:"all_allowances,omitempty"`
	// Only with "enumerable" extension (and "allowances") Returns all allowances this spender has been granted. Supports pagination.
	AllSpenderAllowances *QueryMsg_AllSpenderAllowances `json:"all_spender_allowances,omitempty"`
	// Only with "enumerable" extension Returns all accounts that have balances. Supports pagination.
	AllAccounts *QueryMsg_AllAccounts `json:"all_accounts,omitempty"`
	// Only with "marketing" extension Returns more metadata on the contract to display in the client: - description, logo, project url, etc.
	MarketingInfo *QueryMsg_MarketingInfo `json:"marketing_info,omitempty"`
	// Only with "marketing" extension Downloads the embedded logo data (if stored on chain). Errors if no logo data is stored for this contract.
	DownloadLogo *QueryMsg_DownloadLogo `json:"download_logo,omitempty"`
}

type ExecuteMsg_BurnFrom struct {
	Amount Uint128 `json:"amount"`
	Owner string `json:"owner"`
}

type QueryMsg_Balance struct {
	Address string `json:"address"`
}

type QueryMsg_TokenInfo struct{}

type QueryMsg_Allowance struct {
	Owner string `json:"owner"`
	Spender string `json:"spender"`
}

type QueryMsg_AllSpenderAllowances struct {
	Limit *int `json:"limit,omitempty"`
	Spender string `json:"spender"`
	StartAfter *string `json:"start_after,omitempty"`
}

type Cw20Coin struct {
	Address string `json:"address"`
	Amount Uint128 `json:"amount"`
}

type ExecuteMsg_DecreaseAllowance struct {
	Amount Uint128 `json:"amount"`
	Expires *Expiration `json:"expires,omitempty"`
	Spender string `json:"spender"`
}

/*
A point in time in nanosecond precision.

This type can represent times from 1970-01-01T00:00:00Z to 2554-07-21T23:34:33Z.

## Examples

``` # use cosmwasm_std::Timestamp; let ts = Timestamp::from_nanos(1_000_000_202); assert_eq!(ts.nanos(), 1_000_000_202); assert_eq!(ts.seconds(), 1); assert_eq!(ts.subsec_nanos(), 202);

let ts = ts.plus_seconds(2); assert_eq!(ts.nanos(), 3_000_000_202); assert_eq!(ts.seconds(), 3); assert_eq!(ts.subsec_nanos(), 202); ```
*/
type Timestamp Uint64

type QueryMsg_AllAllowances struct {
	Limit *int `json:"limit,omitempty"`
	Owner string `json:"owner"`
	StartAfter *string `json:"start_after,omitempty"`
}

// This is used to display logo info, provide a link or inform there is one that can be downloaded from the blockchain itself
type LogoInfo interface {
	Implements_LogoInfo()
}

var _ LogoInfo = (*LogoInfo_Url)(nil)

type LogoInfo_Url string

func (*LogoInfo_Url) Implements_LogoInfo() {}

var _ LogoInfo = (*LogoInfo_Embedded)(nil)

type LogoInfo_Embedded string

// There is an embedded logo on the chain, make another call to download it.
const LogoInfo_Embedded_Value LogoInfo_Embedded = "embedded"

func (*LogoInfo_Embedded) Implements_LogoInfo() {}

// This is used to store the logo on the blockchain in an accepted format. Enforce maximum size of 5KB on all variants.
type EmbeddedLogo struct {
	// Store the Logo as an SVG file. The content must conform to the spec at https://en.wikipedia.org/wiki/Scalable_Vector_Graphics (The contract should do some light-weight sanity-check validation)
	Svg *EmbeddedLogo_Svg `json:"svg,omitempty"`
	// Store the Logo as a PNG file. This will likely only support up to 64x64 or so within the 5KB limit.
	Png *EmbeddedLogo_Png `json:"png,omitempty"`
}

/*
A thin wrapper around u128 that is using strings for JSON encoding/decoding, such that the full u128 range can be used for clients that convert JSON numbers to floats, like JavaScript and jq.

# Examples

Use `from` to create instances of this and `u128` to get the value out:

``` # use cosmwasm_std::Uint128; let a = Uint128::from(123u128); assert_eq!(a.u128(), 123);

let b = Uint128::from(42u64); assert_eq!(b.u128(), 42);

let c = Uint128::from(70u32); assert_eq!(c.u128(), 70); ```
*/
type Uint128 string

type ExecuteMsg_Transfer struct {
	Amount Uint128 `json:"amount"`
	Recipient string `json:"recipient"`
}

type ExecuteMsg_IncreaseAllowance struct {
	Amount Uint128 `json:"amount"`
	Expires *Expiration `json:"expires,omitempty"`
	Spender string `json:"spender"`
}

type ExecuteMsg_SendFrom struct {
	Contract string `json:"contract"`
	Msg Binary `json:"msg"`
	Owner string `json:"owner"`
	Amount Uint128 `json:"amount"`
}

// Expiration represents a point in time when some event happens. It can compare with a BlockInfo and will return is_expired() == true once the condition is hit (and for every block in the future)
type Expiration struct {
	// AtHeight will expire when `env.block.height` >= height
	AtHeight *Expiration_AtHeight `json:"at_height,omitempty"`
	// AtTime will expire when `env.block.time` >= time
	AtTime *Expiration_AtTime `json:"at_time,omitempty"`
	// Never will never expire. Used to express the empty variant
	Never *Expiration_Never `json:"never,omitempty"`
}

type QueryMsg_AllAccounts struct {
	Limit *int `json:"limit,omitempty"`
	StartAfter *string `json:"start_after,omitempty"`
}

type AllSpenderAllowancesResponse struct {
	Allowances []SpenderAllowanceInfo `json:"allowances"`
}

type InstantiateMarketingInfo struct {
	Description *string `json:"description,omitempty"`
	Logo *Logo `json:"logo,omitempty"`
	Marketing *string `json:"marketing,omitempty"`
	Project *string `json:"project,omitempty"`
}

// This is used for uploading logo data, or setting it in InstantiateData
type Logo struct {
	// A reference to an externally hosted logo. Must be a valid HTTP or HTTPS URL.
	Url *Logo_Url `json:"url,omitempty"`
	// Logo content stored on the blockchain. Enforce maximum size of 5KB on all variants
	Embedded *Logo_Embedded `json:"embedded,omitempty"`
}

type AllowanceResponse struct {
	Allowance Uint128 `json:"allowance"`
	Expires Expiration `json:"expires"`
}
type ExecuteMsg_UploadLogo Logo

type SpenderAllowanceInfo struct {
	Allowance Uint128 `json:"allowance"`
	Expires Expiration `json:"expires"`
	Owner string `json:"owner"`
}

type ExecuteMsg_Burn struct {
	Amount Uint128 `json:"amount"`
}

type ExecuteMsg_Mint struct {
	Amount Uint128 `json:"amount"`
	Recipient string `json:"recipient"`
}

type MinterResponse struct {
	// cap is a hard cap on total supply that can be achieved by minting. Note that this refers to total_supply. If None, there is unlimited cap.
	Cap *Uint128 `json:"cap,omitempty"`
	Minter string `json:"minter"`
}

type MarketingInfoResponse struct {
	// A URL pointing to the project behind this token.
	Project *string `json:"project,omitempty"`
	// A longer description of the token and it's utility. Designed for tooltips or such
	Description *string `json:"description,omitempty"`
	// A link to the logo, or a comment there is an on-chain logo stored
	Logo *LogoInfo `json:"logo,omitempty"`
	// The address (if any) who can update this data structure
	Marketing *Addr `json:"marketing,omitempty"`
}

type BalanceResponse struct {
	Balance Uint128 `json:"balance"`
}

type QueryMsg_MarketingInfo struct{}

type AllAccountsResponse struct {
	Accounts []string `json:"accounts"`
}

type ExecuteMsg_UpdateMinter struct {
	NewMinter *string `json:"new_minter,omitempty"`
}

type MinterResponse_2 struct {
	// cap is a hard cap on total supply that can be achieved by minting. Note that this refers to total_supply. If None, there is unlimited cap.
	Cap *Uint128 `json:"cap,omitempty"`
	Minter string `json:"minter"`
}

type AllowanceInfo struct {
	Expires Expiration `json:"expires"`
	Spender string `json:"spender"`
	Allowance Uint128 `json:"allowance"`
}

type AllAllowancesResponse struct {
	Allowances []AllowanceInfo `json:"allowances"`
}

/*
Binary is a wrapper around Vec<u8> to add base64 de/serialization with serde. It also adds some helper methods to help encode inline.

This is only needed as serde-json-{core,wasm} has a horrible encoding for Vec<u8>. See also <https://github.com/CosmWasm/cosmwasm/blob/main/docs/MESSAGE_TYPES.md>.
*/
type Binary string

type ExecuteMsg_TransferFrom struct {
	Amount Uint128 `json:"amount"`
	Owner string `json:"owner"`
	Recipient string `json:"recipient"`
}

/*
A thin wrapper around u64 that is using strings for JSON encoding/decoding, such that the full u64 range can be used for clients that convert JSON numbers to floats, like JavaScript and jq.

# Examples

Use `from` to create instances of this and `u64` to get the value out:

``` # use cosmwasm_std::Uint64; let a = Uint64::from(42u64); assert_eq!(a.u64(), 42);

let b = Uint64::from(70u32); assert_eq!(b.u64(), 70); ```
*/
type Uint64 string

type QueryMsg_Minter struct{}

type QueryMsg_DownloadLogo struct{}

type TokenInfoResponse struct {
	Decimals int `json:"decimals"`
	Name string `json:"name"`
	Symbol string `json:"symbol"`
	TotalSupply Uint128 `json:"total_supply"`
}

// When we download an embedded logo, we get this response type. We expect a SPA to be able to accept this info and display it.
type DownloadLogoResponse struct {
	Data Binary `json:"data"`
	MimeType string `json:"mime_type"`
}

/*
A human readable address.

In Cosmos, this is typically bech32 encoded. But for multi-chain smart contracts no assumptions should be made other than being UTF-8 encoded and of reasonable length.

This type represents a validated address. It can be created in the following ways 1. Use `Addr::unchecked(input)` 2. Use `let checked: Addr = deps.api.addr_validate(input)?` 3. Use `let checked: Addr = deps.api.addr_humanize(canonical_addr)?` 4. Deserialize from JSON. This must only be done from JSON that was validated before such as a contract's state. `Addr` must not be used in messages sent by the user because this would result in unvalidated instances.

This type is immutable. If you really need to mutate it (Really? Are you sure?), create a mutable copy using `let mut mutable = Addr::to_string()` and operate on that `String` instance.
*/
type Addr string

type ExecuteMsg_Send struct {
	Amount Uint128 `json:"amount"`
	Contract string `json:"contract"`
	Msg Binary `json:"msg"`
}

type ExecuteMsg_UpdateMarketing struct {
	// The address (if any) who can update this data structure
	Marketing *string `json:"marketing,omitempty"`
	// A URL pointing to the project behind this token.
	Project *string `json:"project,omitempty"`
	// A longer description of the token and it's utility. Designed for tooltips or such
	Description *string `json:"description,omitempty"`
}
type EmbeddedLogo_Svg Binary
type EmbeddedLogo_Png Binary

type Expiration_AtHeight int
type Expiration_AtTime Timestamp

type Expiration_Never struct{}

type Logo_Url string
type Logo_Embedded EmbeddedLogo