use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cosmwasm_std::{to_json_binary, Addr, CosmosMsg, StdResult, WasmMsg};

use crate::msg::ExecuteMsg;

/// CwTemplateContract is a wrapper around Addr that provides a lot of helpers
/// for working with this.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
pub struct CwTemplateContract(pub Addr);

impl CwTemplateContract {
    pub fn addr(&self) -> Addr {
        self.0.clone()
    }

    #[allow(unreachable_code)]
    pub fn call<T: Into<ExecuteMsg>>(&self, msg: T) -> StdResult<CosmosMsg> {
        Ok(WasmMsg::Execute {
            contract_addr: self.addr().into(),
            msg: to_json_binary(&msg.into())?,
            funds: vec![],
        }
        .into())
    }
}
