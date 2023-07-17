use cosmwasm_schema::{QueryResponses, cw_serde};
use cosmwasm_std::Addr;

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