//! ICS02-Client Event Keys

/// `EVENT_TYPE_CREATE_CLIENT` is the event type for a create client event
pub const EVENT_TYPE_CREATE_CLIENT: &str = "create_client";
/// `EVENT_TYPE_PROVIDE_COUNTERPARTY` is the event type for a provide counterparty event
pub const EVENT_TYPE_PROVIDE_COUNTERPARTY: &str = "provide_counterparty";
/// `EVENT_TYPE_MIGRATE_CLIENT` is the event type for a migrate client event
pub const EVENT_TYPE_MIGRATE_CLIENT: &str = "migrate_client";

/// `ATTRIBUTE_KEY_CLIENT_ID` is the attribute key for the client id
pub const ATTRIBUTE_KEY_CLIENT_ID: &str = "client_id";
/// `ATTRIBUTE_KEY_COUNTERPARTY_ID` is the attribute key for the counterparty id
/// If the value is the empty string, the counterparty was not provided.
pub const ATTRIBUTE_KEY_COUNTERPARTY_ID: &str = "counterparty_id";
/// `ATTRIBUTE_KEY_COUNTERPARTY_MERKLE_PREFIX` is the attribute key for the counterparty merkle
/// prefix
pub const ATTRIBUTE_KEY_COUNTERPARTY_MERKLE_PREFIX: &str = "counterparty_merkle_prefix";
/// `ATTRIBUTE_KEY_CREATOR` is the attribute key for the creator address
pub const ATTRIBUTE_KEY_CREATOR: &str = "creator";
/// `ATTRIBUTE_KEY_CONTRACT_ADDRESS` is the attribute key for the contract address
pub const ATTRIBUTE_KEY_CONTRACT_ADDRESS: &str = "contract_address";
/// `ATTRIBUTE_KEY_SUBJECT_CLIENT_ID` is the attribute key for the subject client id
pub const ATTRIBUTE_KEY_SUBJECT_CLIENT_ID: &str = "subject_client_id";
/// `ATTRIBUTE_KEY_SUBSTITUTE_CLIENT_ID` is the attribute key for the substitute client id
pub const ATTRIBUTE_KEY_SUBSTITUTE_CLIENT_ID: &str = "substitute_client_id";
/// `ATTRIBUTE_KEY_SUBSTITUTE_CLIENT_ADDRESS` is the attribute key for the substitute client's
/// contract address
pub const ATTRIBUTE_KEY_SUBSTITUTE_CLIENT_ADDRESS: &str = "substitute_client_address";

/// Contains event messages emitted during [`super::super::msg::ExecuteMsg::CreateClient`] execution.
pub mod create_client {
    use cosmwasm_std::{Attribute, Event};

    use crate::types::state::CounterpartyInfo;

    /// `create_client` is the event message for a create client event
    #[must_use]
    pub fn success(
        client_id: &str,
        counterparty_info: Option<CounterpartyInfo>,
        creator: &str,
        contract_address: &str,
    ) -> Event {
        Event::new(super::EVENT_TYPE_CREATE_CLIENT).add_attributes(vec![
            Attribute::new(super::ATTRIBUTE_KEY_CLIENT_ID, client_id),
            Attribute::new(
                super::ATTRIBUTE_KEY_COUNTERPARTY_ID,
                counterparty_info
                    .as_ref()
                    .map_or_else(String::new, |ci| ci.client_id.clone()),
            ),
            Attribute::new(
                super::ATTRIBUTE_KEY_COUNTERPARTY_MERKLE_PREFIX,
                counterparty_info.map_or_else(String::new, |ci| {
                    ci.merkle_path_prefix
                        .map_or_else(String::new, |prefix| format!("{:?}", prefix.key_path))
                }),
            ),
            Attribute::new(super::ATTRIBUTE_KEY_CREATOR, creator),
            Attribute::new(super::ATTRIBUTE_KEY_CONTRACT_ADDRESS, contract_address),
        ])
    }
}

/// Contains event messages emitted during [`super::super::msg::ExecuteMsg::ProvideCounterparty`]
/// execution.
pub mod provide_counterparty {
    use cosmwasm_std::{Attribute, Event};

    use crate::types::state::CounterpartyInfo;

    /// `provide_counterparty` is the event message for a provide counterparty event
    #[must_use]
    pub fn success(client_id: &str, counterparty_info: CounterpartyInfo) -> Event {
        Event::new(super::EVENT_TYPE_PROVIDE_COUNTERPARTY).add_attributes(vec![
            Attribute::new(super::ATTRIBUTE_KEY_CLIENT_ID, client_id),
            Attribute::new(
                super::ATTRIBUTE_KEY_COUNTERPARTY_ID,
                counterparty_info.client_id,
            ),
            Attribute::new(
                super::ATTRIBUTE_KEY_COUNTERPARTY_MERKLE_PREFIX,
                counterparty_info
                    .merkle_path_prefix
                    .map_or_else(String::new, |prefix| format!("{:?}", prefix.key_path)),
            ),
        ])
    }
}

/// Contains event messages emitted during [`super::super::msg::ExecuteMsg::MigrateClient`]
/// execution.
pub mod migrate_client {
    use cosmwasm_std::{Attribute, Event};

    /// `migrate_client` is the event message for a migrate client event
    #[must_use]
    pub fn success(
        subject_client_id: &str,
        substitute_client_id: &str,
        substitute_client_address: &str,
    ) -> Event {
        Event::new(super::EVENT_TYPE_MIGRATE_CLIENT).add_attributes(vec![
            Attribute::new(super::ATTRIBUTE_KEY_SUBJECT_CLIENT_ID, subject_client_id),
            Attribute::new(
                super::ATTRIBUTE_KEY_SUBSTITUTE_CLIENT_ID,
                substitute_client_id,
            ),
            Attribute::new(
                super::ATTRIBUTE_KEY_SUBSTITUTE_CLIENT_ADDRESS,
                substitute_client_address,
            ),
        ])
    }
}
