use cosmwasm_schema::cw_serde;

#[cw_serde]
pub struct InstantiateMsg {
    pub code_id_profiles: u64,
    pub code_id_messages: u64,
}