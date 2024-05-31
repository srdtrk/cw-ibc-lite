//! ICS02-Client Event Keys

/// `EVENT_TYPE_CREATE_CLIENT` is the event type for a create client event
pub const EVENT_TYPE_CREATE_CLIENT: &str = "create_client";
/// `EVENT_TYPE_PROVIDE_COUNTERPARTY` is the event type for a provide counterparty event
pub const EVENT_TYPE_PROVIDE_COUNTERPARTY: &str = "provide_counterparty";

/// `ATTRIBUTE_KEY_CLIENT_ID` is the attribute key for the client id
pub const ATTRIBUTE_KEY_CLIENT_ID: &str = "client_id";
/// `ATTRIBUTE_KEY_COUNTERPARTY_ID` is the attribute key for the counterparty id
/// If the value is the empty string, the counterparty was not provided.
pub const ATTRIBUTE_KEY_COUNTERPARTY_ID: &str = "counterparty_id";
/// `ATTRIBUTE_KEY_CREATOR` is the attribute key for the creator address
pub const ATTRIBUTE_KEY_CREATOR: &str = "creator";
/// `ATTRIBUTE_KEY_CONTRACT_ADDRESS` is the attribute key for the contract address
pub const ATTRIBUTE_KEY_CONTRACT_ADDRESS: &str = "contract_address";

/// Contains event messages emitted during [`super::super::msg::ExecuteMsg::CreateClient`] execution.
pub mod create_client {
    use cosmwasm_std::{Attribute, Event};

    /// `create_client` is the event message for a create client event
    #[must_use]
    pub fn success(
        client_id: &str,
        counterparty_id: &str,
        creator: &str,
        contract_address: &str,
    ) -> Event {
        Event::new(super::EVENT_TYPE_CREATE_CLIENT).add_attributes(vec![
            Attribute::new(super::ATTRIBUTE_KEY_CLIENT_ID, client_id),
            Attribute::new(super::ATTRIBUTE_KEY_COUNTERPARTY_ID, counterparty_id),
            Attribute::new(super::ATTRIBUTE_KEY_CREATOR, creator),
            Attribute::new(super::ATTRIBUTE_KEY_CONTRACT_ADDRESS, contract_address),
        ])
    }
}

/// Contains event messages emitted during [`super::super::msg::ExecuteMsg::ProvideCounterparty`]
/// execution.
pub mod provide_counterparty {
    use cosmwasm_std::{Attribute, Event};

    /// `provide_counterparty` is the event message for a provide counterparty event
    #[must_use]
    pub fn success(client_id: &str, counterparty_id: &str) -> Event {
        Event::new(super::EVENT_TYPE_PROVIDE_COUNTERPARTY).add_attributes(vec![
            Attribute::new(super::ATTRIBUTE_KEY_CLIENT_ID, client_id),
            Attribute::new(super::ATTRIBUTE_KEY_COUNTERPARTY_ID, counterparty_id),
        ])
    }
}
