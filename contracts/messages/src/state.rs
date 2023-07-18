use cosmwasm_std::Addr;
use cw_storage_plus::{Map, Item};
use utils::elements::Message;
use cosmwasm_schema::cw_serde;

pub const CONFIG: Item<Config> = Item::new("config");
pub const USER_MESSAGES: Map<Addr, Vec<Message>> = Map::new("user_messages");

#[cw_serde]
pub struct Config {
    pub default_query_limit: u64,
    pub max_query_limit: u64,
}