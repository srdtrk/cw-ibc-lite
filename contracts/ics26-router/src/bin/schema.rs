use cosmwasm_schema::write_api;

use cw_ibc_lite_ics26_router::types::msg::{ExecuteMsg, InstantiateMsg, QueryMsg};

fn main() {
    write_api! {
        instantiate: InstantiateMsg,
        execute: ExecuteMsg,
        query: QueryMsg,
    }
}
