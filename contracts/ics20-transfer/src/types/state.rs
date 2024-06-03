//! This module defines the state storage of the Contract.

/// A collection of methods to access the admin of the contract.
pub mod admin {
    use cosmwasm_std::{Addr, Env, QuerierWrapper};
    use cw_ibc_lite_shared::types::error::ContractError;

    /// Asserts that the given address is the admin of the contract.
    ///
    /// # Errors
    /// Returns an error if the given address is not the admin of the contract or the contract
    /// doesn't have an admin.
    #[allow(clippy::module_name_repetitions)]
    pub fn assert_admin(
        env: &Env,
        querier: &QuerierWrapper,
        addr: &Addr,
    ) -> Result<(), ContractError> {
        let admin = querier
            .query_wasm_contract_info(&env.contract.address)?
            .admin
            .ok_or(ContractError::Unauthorized)?;

        if admin != addr.as_str() {
            return Err(ContractError::Unauthorized);
        }

        Ok(())
    }
}
