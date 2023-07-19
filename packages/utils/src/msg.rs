use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Addr, Binary};
use cw_ownable::cw_ownable_execute;

#[cw_serde]
pub struct ProfilesInstantiateMsg {}

#[cw_serde]
pub struct MessagesInstantiateMsg {
    pub default_query_limit: u64,
    pub max_query_limit: u64,
}

#[cw_ownable_execute]
#[cw_serde]
pub enum ProfilesExecuteMsg {
    CreateProfile {
        address: Addr,
        user_id: String,
        pubkey: String,
    },
    ChangeUserId {
        address: Addr,
        user_id: String,
    },
    ChangePubkey {
        address: Addr,
        pubkey: String,
    },
}

#[cw_ownable_execute]
#[cw_serde]
pub enum MessagesExecuteMsg {
    SendMessage {
        sender: Addr,
        receiver: Addr,
        message: Binary,
    },
    ClaimMessageFunds {
        message_ids: Vec<u64>,
    },
    DeleteMessages {
        message_ids: Vec<u64>
    },
    ChangeConfig {
        default_query_limit: u64,
        max_query_limit: u64,    
    }
}
