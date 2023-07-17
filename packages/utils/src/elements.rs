use cosmwasm_schema::cw_serde;

#[cw_serde]
pub struct Profile {
    pub user_id: String,
    pub pubkey: String,
}
