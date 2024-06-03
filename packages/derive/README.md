# `cw-ibc-lite-derive` - CosmWasm IBC Lite Derive

This crate provides a derive macro for contracts `receiving cw-ibc-lite` callback messages.
This crate's macros are not intended to be used directly, but rather as a dependency of the
`cw-ibc-lite` crate where it is re-exported under the `cw_ibc_lite_shared::types::apps::callbacks`.

This allows the users of the `cw-ica-controller` crate to easily merge the required callback
message enum variant into their `ExecuteMsg` enum.

## Usage

```rust
use cosmwasm_schema::{cw_serde, QueryResponses};
use cw_ibc_lite_shared::types::apps::callbacks::ibc_lite_callback;

#[cw_serde]
pub struct InstantiateMsg {}

#[ibc_lite_callback]
#[cw_serde]
pub enum ExecuteMsg {}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    /// GetCallbackCounter returns the callback counter.
    #[returns(crate::state::CallbackCounter)]
    GetCallbackCounter {},
}
```
