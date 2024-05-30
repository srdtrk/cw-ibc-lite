//! This module defines the state storage of the Contract.

use cosmwasm_std::Addr;
use cw_storage_plus::{Item, Map};

/// `NEXT_CLIENT_NUM` is the item that stores the next client number.
pub const NEXT_CLIENT_NUM: Item<u64> = Item::new("client_num");

/// `CLIENTS` is the map of all client ids to their contract address.
/// The reverse mapping should not be needed as the client should be responding with a reply.
pub const CLIENTS: Map<String, Addr> = Map::new("clients");

/// `COUNTERPARTY` is the map of all client ids to their counterparty client id.
pub const COUNTERPARTY: Map<String, String> = Map::new("counterparty");
