//! `cw-ibc-lite-ics26-router` Event Keys

/// `EVENT_TYPE_REGISTER_IBC_APP` is the event type for a register IBC app event
pub const EVENT_TYPE_REGISTER_IBC_APP: &str = "register_ibc_app";

/// `ATTRIBUTE_KEY_CONTRACT_ADDRESS` is the attribute key for the contract address
pub const ATTRIBUTE_KEY_CONTRACT_ADDRESS: &str = "contract_address";
/// `ATTRIBUTE_KEY_PORT_ID` is the attribute key for the port id
pub const ATTRIBUTE_KEY_PORT_ID: &str = "port_id";
/// `ATTRIBUTE_KEY_SENDER` is the attribute key for the sender
pub const ATTRIBUTE_KEY_SENDER: &str = "sender";

/// Contains event messages emitted during [`super::super::msg::ExecuteMsg::RegisterIbcApp`]
pub mod register_ibc_app {
    use cosmwasm_std::{Attribute, Event};

    /// `register_ibc_app` is the event message for a register IBC app event
    #[must_use]
    pub fn success(port_id: &str, contract_address: &str, sender: &str) -> Event {
        Event::new(super::EVENT_TYPE_REGISTER_IBC_APP).add_attributes(vec![
            Attribute::new(super::ATTRIBUTE_KEY_CONTRACT_ADDRESS, contract_address),
            Attribute::new(super::ATTRIBUTE_KEY_PORT_ID, port_id),
            Attribute::new(super::ATTRIBUTE_KEY_SENDER, sender),
        ])
    }
}
