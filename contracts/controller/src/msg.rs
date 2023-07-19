use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::{Addr, Coin, Binary};
use cw_ownable::cw_ownable_execute;

use crate::state::Config;

#[cw_serde]
pub struct InstantiateMsg {
    pub code_id_profiles: u64,
    pub code_id_messages: u64,
    pub message_max_len: u64,
    pub message_query_default_limit: u64,
    pub message_query_max_limit: u64,
    pub create_profile_cost: Option<Coin>,
    pub send_message_cost: Option<Coin>,
}

#[cw_ownable_execute]
#[cw_serde]
pub enum ExecuteMsg {
    CreateProfile {
        user_id: String,
        pubkey: String,
    },
    ChangeUserId {
        user_id: String,
    },
    ChangePubkey {
        pubkey: String,
    },
    SendMessage {
        content: Binary,
        dest_address: Option<Addr>,
        dest_id: Option<String>,
    },
    ChangeMessagesConfig {
        message_query_default_limit: u64,
        message_query_max_limit: u64,
    },
    RetrieveFees {
        receiver: Option<Addr>,
    },
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    #[returns(Config)]
    Config {},
}
