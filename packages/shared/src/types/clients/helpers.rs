//! This file contains helper functions for working with ibc-lite light client contracts
//! from external contracts.

use super::msg::{self, query_responses};

use ibc_client_cw::types::{
    CheckForMisbehaviourMsgRaw, ContractResult, ExportMetadataMsg, StatusMsg, TimestampAtHeightMsg,
    VerifyClientMessageRaw, VerifyMembershipMsgRaw, VerifyNonMembershipMsgRaw,
};

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cosmwasm_std::{
    instantiate2_address, to_json_binary, Addr, Api, CosmosMsg, Env, QuerierWrapper, QueryRequest,
    StdError, StdResult, WasmMsg,
};

/// `LightClientContract` is a wrapper around Addr that provides helpers
/// for working with this contract.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
pub struct LightClientContract(pub Addr);

/// `LightClientCode` is a wrapper around u64 that provides helpers for
/// initializing this contract.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
pub struct LightClientCode(pub u64);

/// `LightClientContractQuerier` is a wrapper around [`QuerierWrapper`] that provides
/// helpers for querying this contract.
///
/// This can be constructed by [`LightClientContract::query`] or [`LightClientContractQuerier::new`].
pub struct LightClientContractQuerier<'a> {
    querier: &'a QuerierWrapper<'a>,
    addr: String,
}

impl LightClientContract {
    /// Creates a new [`LightClientContract`]
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

    /// `query` creates a new [`LightClientContractQuerier`] for this contract.
    #[must_use]
    pub fn query<'a>(&self, querier: &'a QuerierWrapper) -> LightClientContractQuerier<'a> {
        LightClientContractQuerier::new(querier, self.addr().into_string())
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

impl LightClientCode {
    /// `new` creates a new [`LightClientCode`]
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

impl<'a> LightClientContractQuerier<'a> {
    /// Creates a new [`LightClientContractQuerier`]
    #[must_use]
    pub const fn new(querier: &'a QuerierWrapper<'a>, addr: String) -> Self {
        Self { querier, addr }
    }

    /// `status` sends a [`msg::QueryMsg::Status`] query to this contract.
    ///
    /// # Errors
    ///
    /// This function returns an error if the query fails
    pub fn status(&self, status_msg: impl Into<StatusMsg>) -> StdResult<query_responses::Status> {
        self.querier
            .query_wasm_smart(&self.addr, &msg::QueryMsg::Status(status_msg.into()))
    }

    /// `export_metadata` sends a [`msg::QueryMsg::ExportMetadata`] query to this contract.
    ///
    /// # Errors
    ///
    /// This function returns an error if the query fails
    pub fn export_metadata(
        &self,
        msg: impl Into<ExportMetadataMsg>,
    ) -> StdResult<query_responses::ExportMetadata> {
        self.querier
            .query_wasm_smart(&self.addr, &msg::QueryMsg::ExportMetadata(msg.into()))
    }

    /// `timestamp_at_height` sends a [`msg::QueryMsg::TimestampAtHeight`] query to this contract.
    ///
    /// # Errors
    /// This function returns an error if the query fails
    pub fn timestamp_at_height(
        &self,
        msg: impl Into<TimestampAtHeightMsg>,
    ) -> StdResult<query_responses::TimestampAtHeight> {
        self.querier
            .query_wasm_smart(&self.addr, &msg::QueryMsg::TimestampAtHeight(msg.into()))
    }

    /// `verify_client_message` sends a [`msg::QueryMsg::VerifyClientMessage`] query to this contract.
    ///
    /// # Errors
    /// This function returns an error if the query fails
    pub fn verify_client_message(
        &self,
        msg: impl Into<VerifyClientMessageRaw>,
    ) -> StdResult<query_responses::VerifyClientMessage> {
        self.querier
            .query_wasm_smart(&self.addr, &msg::QueryMsg::VerifyClientMessage(msg.into()))
    }

    /// `check_for_misbehaviour` sends a [`msg::QueryMsg::CheckForMisbehaviour`] query to this contract.
    ///
    /// # Errors
    /// This function returns an error if the query fails
    pub fn check_for_misbehaviour(
        &self,
        msg: impl Into<CheckForMisbehaviourMsgRaw>,
    ) -> StdResult<query_responses::CheckForMisbehaviour> {
        self.querier
            .query_wasm_smart(&self.addr, &msg::QueryMsg::CheckForMisbehaviour(msg.into()))
    }

    /// `verify_membership` sends a [`msg::QueryMsg::VerifyMembership`] query to this contract.
    ///
    /// # Errors
    /// This function returns an error if the query fails
    pub fn verify_membership(
        &self,
        msg: impl Into<VerifyMembershipMsgRaw>,
    ) -> StdResult<ContractResult> {
        self.querier
            .query_wasm_smart(&self.addr, &msg::QueryMsg::VerifyMembership(msg.into()))
    }

    /// `verify_non_membership` sends a [`msg::QueryMsg::VerifyNonMembership`] query to this contract.
    ///
    /// # Errors
    /// This function returns an error if the query fails
    pub fn verify_non_membership(
        &self,
        msg: impl Into<VerifyNonMembershipMsgRaw>,
    ) -> StdResult<ContractResult> {
        self.querier
            .query_wasm_smart(&self.addr, &msg::QueryMsg::VerifyNonMembership(msg.into()))
    }

    /// `smart_raw` sends a [`msg::QueryMsg`] query to this contract.
    /// It returns the raw [`cosmwasm_std::Binary`] response from the contract.
    ///
    /// # Errors
    /// This function returns an error if the query fails
    pub fn smart_raw(&self, msg: impl Into<msg::QueryMsg>) -> StdResult<cosmwasm_std::Binary> {
        let request: QueryRequest<cosmwasm_std::Empty> = cosmwasm_std::WasmQuery::Smart {
            contract_addr: self.addr.clone(),
            msg: to_json_binary(&msg.into())?,
        }
        .into();

        match self
            .querier
            .raw_query(&cosmwasm_std::to_json_vec(&request)?)
        {
            cosmwasm_std::SystemResult::Err(system_err) => Err(StdError::generic_err(format!(
                "Querier system error: {system_err}"
            ))),
            cosmwasm_std::SystemResult::Ok(cosmwasm_std::ContractResult::Err(contract_err)) => Err(
                StdError::generic_err(format!("Querier contract error: {contract_err}")),
            ),
            cosmwasm_std::SystemResult::Ok(cosmwasm_std::ContractResult::Ok(value)) => Ok(value),
        }
    }
}
