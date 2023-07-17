use cosmwasm_schema::cw_serde;
use cosmwasm_std::Addr;
use cw_ownable::cw_ownable_execute;

#[cw_serde]
pub struct InstantiateProfilesMsg {
}

#[cw_serde]
pub struct InstantiateMessagesMsg {
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
    UpdatePubkey{
        address: Addr,
        pubkey: String,
    }
}

