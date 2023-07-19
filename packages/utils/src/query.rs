use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::Addr;

use crate::elements::Message;

#[cw_serde]
#[derive(QueryResponses)]
pub enum ProfilesQueryMsg {
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
    #[returns(MessagesResponse)]
    Messages {
        address: Addr,
        from: u64,
        limit: Option<u64>,
    },
    #[returns(TotalMessagesResponse)]
    TotalMessages { address: Addr },
}

#[cw_serde]
pub struct MessagesResponse {
    pub messages: Vec<MessageResponse>,
}

#[cw_serde]
pub struct MessageResponse {
    pub id: u64,
    pub message: Message,
}

#[cw_serde]
pub struct TotalMessagesResponse {
    pub total: u64,
}
