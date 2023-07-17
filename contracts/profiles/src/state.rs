use cosmwasm_std::Addr;
use cw_storage_plus::Map;
use utils::elements::Profile;

pub const USERID_TO_ADDRESS: Map<String, Addr> = Map::new("address_mapping");
pub const ADDRESS_TO_PROFILE: Map<Addr, Profile> = Map::new("profile_mapping");