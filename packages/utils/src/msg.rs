use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Addr, Binary};
use cw_ownable::cw_ownable_execute;

#[cw_serde]
pub struct InstantiateProfilesMsg {}

#[cw_serde]
pub struct InstantiateMessagesMsg {
    pub default_query_limit: u64,
    pub max_query_limit: u64,
}

#[cw_ownable_execute]
#[cw_serde]
pub enum ProfileExecuteMsg {
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
pub enum MessageExecuteMsg {
    SendMessage {
        sender: Addr,
        receiver: Addr,
        message: Binary,
    },
    ClaimMessageFunds {
        message_id: u64,
    }
}
