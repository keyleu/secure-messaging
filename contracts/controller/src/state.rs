use cosmwasm_std::Addr;
use cw_storage_plus::Item;

/// This is saved after handling a reply in instantiation. It's the address of the profiles contract.
pub const PROFILES_ADDRESS: Item<Addr> = Item::new("profiles_address");

/// This is saved after handling a reply in instantiation. It's the address of the messages contract.
pub const MESSAGES_ADDRESS: Item<Addr> = Item::new("messages_address");