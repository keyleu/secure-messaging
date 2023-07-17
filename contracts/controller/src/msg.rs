use cosmwasm_schema::cw_serde;
use cosmwasm_std::Coin;
use cw_ownable::cw_ownable_execute;

#[cw_serde]
pub struct InstantiateMsg {
    pub code_id_profiles: u64,
    pub code_id_messages: u64,
    pub create_profile_cost: Option<Coin>,
    pub send_message_cost: Option<Coin>,
}

#[cw_ownable_execute]
#[cw_serde]
pub enum ExecuteMsg {
    CreateProfile {
        user_id: String,
        pub_key: String,
    },
    SendMessage {
        content: String,
        dest_address: Option<String>,
        dest_id: Option<String>,
    }
}