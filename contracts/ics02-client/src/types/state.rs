//! This module defines the state storage of the Contract.

use cw_storage_plus::Item;

/// `NEXT_CLIENT_NUM` is the item that stores the next client number.
pub const NEXT_CLIENT_NUM: Item<u64> = Item::new("client_num");
