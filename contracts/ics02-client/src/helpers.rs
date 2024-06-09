//! This file contains helper functions for working with this contract from
//! external contracts.

use cw_ibc_lite_shared::types::clients::helpers::LightClientContractQuerier;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cosmwasm_std::{
    instantiate2_address, to_json_binary, Addr, Api, CosmosMsg, Env, QuerierWrapper, StdError,
    StdResult, WasmMsg,
};

use crate::types::{msg, state::CounterpartyInfo};

/// `Ics02ClientContract` is a wrapper around Addr that provides helpers
/// for working with this contract.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
pub struct Ics02ClientContract(pub Addr);

/// `Ics02ClientCode` is a wrapper around u64 that provides helpers for
/// initializing this contract.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
pub struct Ics02ClientCode(pub u64);

/// `Ics02ClientContractQuerier` is a wrapper around [`QuerierWrapper`] that provides
/// helpers for querying this contract.
///
/// This can be constructed by [`Ics02ClientContract::query`] or [`Ics02ClientContractQuerier::new`].
pub struct Ics02ClientContractQuerier<'a> {
    querier: &'a QuerierWrapper<'a>,
    addr: String,
}

impl Ics02ClientContract {
    /// Creates a new [`Ics02ClientContract`]
    #[must_use]
    pub const fn new(addr: Addr) -> Self {
        Self(addr)
    }

    /// Returns the address of the contract
    #[must_use]
    pub fn addr(&self) -> Addr {
        self.0.clone()
    }

    /// Creates a [`WasmMsg::Execute`] message targeting this contract,
    ///
    /// # Errors
    ///
    /// This function returns an error if the given message cannot be serialized
    pub fn call(&self, msg: impl Into<msg::ExecuteMsg>) -> StdResult<CosmosMsg> {
        let msg = to_json_binary(&msg.into())?;
        Ok(WasmMsg::Execute {
            contract_addr: self.addr().into(),
            msg,
            funds: vec![],
        }
        .into())
    }

    /// `query` creates a new [`Ics02ClientContractQuerier`] for this contract.
    #[must_use]
    pub fn query<'a>(&self, querier: &'a QuerierWrapper) -> Ics02ClientContractQuerier<'a> {
        Ics02ClientContractQuerier::new(querier, self.addr().into_string())
    }

    /// `update_admin` creates a [`WasmMsg::UpdateAdmin`] message targeting this contract
    pub fn update_admin(&self, admin: impl Into<String>) -> CosmosMsg {
        WasmMsg::UpdateAdmin {
            contract_addr: self.addr().into(),
            admin: admin.into(),
        }
        .into()
    }

    /// `clear_admin` creates a [`WasmMsg::ClearAdmin`] message targeting this contract
    #[must_use]
    pub fn clear_admin(&self) -> CosmosMsg {
        WasmMsg::ClearAdmin {
            contract_addr: self.addr().into(),
        }
        .into()
    }
}

impl Ics02ClientCode {
    /// `new` creates a new [`Ics02ClientCode`]
    #[must_use]
    pub const fn new(code_id: u64) -> Self {
        Self(code_id)
    }

    /// `code_id` returns the code id of this code
    #[must_use]
    pub const fn code_id(&self) -> u64 {
        self.0
    }

    /// `instantiate` creates a [`WasmMsg::Instantiate`] message targeting this code
    ///
    /// # Errors
    ///
    /// This function returns an error if the given message cannot be serialized
    pub fn instantiate(
        &self,
        msg: impl Into<msg::InstantiateMsg>,
        label: impl Into<String>,
        admin: Option<impl Into<String>>,
    ) -> StdResult<CosmosMsg> {
        let msg = to_json_binary(&msg.into())?;
        Ok(WasmMsg::Instantiate {
            code_id: self.code_id(),
            msg,
            funds: vec![],
            label: label.into(),
            admin: admin.map(Into::into),
        }
        .into())
    }

    /// `instantiate2` returns a [`WasmMsg::Instantiate2`] message targeting this code
    /// and the contract address.
    ///
    /// **Warning**: This function won't work on chains which have substantially changed
    /// address generation such as Injective, test carefully.
    ///
    /// # Errors
    ///
    /// This function returns an error if the given message cannot be serialized or
    /// if the contract address cannot be calculated.
    #[allow(clippy::too_many_arguments)]
    pub fn instantiate2(
        &self,
        api: &dyn Api,
        querier: &QuerierWrapper,
        env: &Env,
        msg: impl Into<msg::InstantiateMsg>,
        label: impl Into<String>,
        admin: Option<impl Into<String>>,
        salt: impl Into<String>,
    ) -> StdResult<(CosmosMsg, Addr)> {
        let salt = salt.into();
        let code_info = querier.query_wasm_code_info(self.code_id())?;
        let creator_cannonical = api.addr_canonicalize(env.contract.address.as_str())?;

        let contract_addr = api.addr_humanize(
            &instantiate2_address(
                code_info.checksum.as_slice(),
                &creator_cannonical,
                salt.as_bytes(),
            )
            .map_err(|e| StdError::generic_err(e.to_string()))?,
        )?;

        let instantiate_msg = WasmMsg::Instantiate2 {
            code_id: self.code_id(),
            msg: to_json_binary(&msg.into())?,
            funds: vec![],
            label: label.into(),
            admin: admin.map(Into::into),
            salt: salt.as_bytes().into(),
        };

        Ok((instantiate_msg.into(), contract_addr))
    }
}

impl<'a> Ics02ClientContractQuerier<'a> {
    /// Creates a new [`Ics02ClientContractQuerier`]
    #[must_use]
    pub const fn new(querier: &'a QuerierWrapper<'a>, addr: String) -> Self {
        Self { querier, addr }
    }

    /// `client_querier` creates a new [`LightClientContractQuerier`] for the client with the given
    /// identifier. This should be used to query the client contract rather than [`Ics02ClientContractQuerier::query_client`].
    ///
    /// # Errors
    /// This function returns an error if the client address cannot be loaded.
    pub fn client_querier(
        &self,
        client_id: impl Into<String>,
    ) -> StdResult<LightClientContractQuerier> {
        let client_address = self.client_info(client_id)?.address;
        Ok(LightClientContractQuerier::new(
            self.querier,
            client_address,
        ))
    }

    /// `client_address` sends a [`msg::QueryMsg::ClientAddress`] query to this contract.
    /// It returns the address of the client contract with the given client id.
    ///
    /// # Errors
    ///
    /// This function returns an error if the query fails
    pub fn client_info(
        &self,
        client_id: impl Into<String>,
    ) -> StdResult<msg::query_responses::ClientInfo> {
        self.querier.query_wasm_smart(
            &self.addr,
            &msg::QueryMsg::ClientInfo {
                client_id: client_id.into(),
            },
        )
    }

    /// `query_client` sends a [`msg::QueryMsg::QueryClient`] query to this contract.
    /// It returns the result of the query on the client contract with the given client id.
    ///
    /// # Errors
    ///
    /// This function returns an error if the query fails
    pub fn query_client(
        &self,
        client_id: impl Into<String>,
        query: impl Into<cw_ibc_lite_shared::types::clients::msg::QueryMsg>,
    ) -> StdResult<msg::query_responses::QueryClient> {
        self.querier.query_wasm_smart(
            &self.addr,
            &msg::QueryMsg::QueryClient {
                client_id: client_id.into(),
                query: query.into(),
            },
        )
    }

    /// `counterparty` sends a [`msg::QueryMsg::Counterparty`] query to this contract.
    /// It returns the counterparty of the client contract with the given client id.
    ///
    /// # Errors
    /// This function returns an error if the query fails
    pub fn counterparty(&self, client_id: impl Into<String>) -> StdResult<CounterpartyInfo> {
        self.querier.query_wasm_smart(
            &self.addr,
            &msg::QueryMsg::Counterparty {
                client_id: client_id.into(),
            },
        )
    }
}
