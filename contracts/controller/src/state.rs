use cosmwasm_std::{Addr, Coin};
use cw_storage_plus::Item;
use cosmwasm_schema::cw_serde;

#[cw_serde]
pub struct Config {
    pub message_cost: Option<Coin>,
    pub profile_cost: Option<Coin>,
}

pub const CONFIG: Item<Config> = Item::new("config");

/// This is saved after handling a reply in instantiation. It's the address of the profiles contract.
pub const PROFILES_ADDRESS: Item<Addr> = Item::new("profiles_address");

/// This is saved after handling a reply in instantiation. It's the address of the messages contract.
pub const MESSAGES_ADDRESS: Item<Addr> = Item::new("messages_address");
