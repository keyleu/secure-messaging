use cosmwasm_schema::write_api;
use utils::msg::{ProfilesExecuteMsg as ExecuteMsg, ProfilesInstantiateMsg as InstantiateMsg};
use utils::query::ProfilesQueryMsg as QueryMsg;

fn main() {
    write_api! {
        instantiate: InstantiateMsg,
        execute: ExecuteMsg,
        query: QueryMsg,
    }
}