use cosmwasm_schema::write_api;
use utils::msg::{MessagesExecuteMsg as ExecuteMsg, MessagesInstantiateMsg as InstantiateMsg};
use utils::query::MessagesQueryMsg as QueryMsg;

fn main() {
    write_api! {
        instantiate: InstantiateMsg,
        execute: ExecuteMsg,
        query: QueryMsg,
    }
}