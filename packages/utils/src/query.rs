use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::{Addr, Coin, Binary};

#[cw_serde]
#[derive(QueryResponses)]
pub enum ProfileQueryMsg {
    #[returns(ProfileInfo)]
    UserInfo { user_id: String },
    #[returns(ProfileInfo)]
    AddressInfo { address: Addr },
}

#[cw_serde]
pub struct ProfileInfo {
    pub address: Addr,
    pub user_id: String,
    pub pubkey: String,
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum MessagesQueryMsg {
    #[returns(MessagesInfo)]
    Messages { from: u64, limit: u64 },
}

#[cw_serde]
pub struct MessagesInfo {
    pub messages: Vec<MessageInfo>,
}

#[cw_serde]
pub struct MessageInfo {
    pub id: u64,
    pub sender: Addr,
    pub content: Binary,
    pub funds: Vec<Coin>,
}