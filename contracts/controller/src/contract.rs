use cosmwasm_std::{entry_point, to_binary, DepsMut, Env, MessageInfo, Response, SubMsg, WasmMsg};
use cw2::set_contract_version;
use cw_ownable::initialize_owner;
use utils::msg::{InstantiateMessagesMsg, InstantiateProfilesMsg};

use crate::{error::ContractError, msg::InstantiateMsg};

const CONTRACT_NAME: &str = env!("CARGO_PKG_NAME");
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");
const INSTANTIATE_PROFILES_REPLY_ID: u64 = 1;
const INSTANTIATE_MESSAGES_REPLY_ID: u64 = 2;

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;

    initialize_owner(
        deps.storage,
        deps.api,
        Some(&info.sender.clone().into_string()),
    )?;

    let wasm_profiles_msg = WasmMsg::Instantiate {
        code_id: msg.code_id_profiles,
        msg: to_binary(&InstantiateProfilesMsg {})?,
        funds: vec![],
        admin: Some(env.contract.address.clone().into_string()),
        label: format!("PROFILES-INFORMATION--{}", msg.code_id_profiles,),
    };
    let submsg_profiles =
        SubMsg::reply_on_success(wasm_profiles_msg, INSTANTIATE_PROFILES_REPLY_ID);

    let wasm_messages_msg = WasmMsg::Instantiate {
        code_id: msg.code_id_messages,
        msg: to_binary(&InstantiateMessagesMsg {})?,
        funds: vec![],
        admin: Some(env.contract.address.into_string()),
        label: format!("MESSAGE-STORAGE--{}", msg.code_id_messages,),
    };
    let submsg_messages =
        SubMsg::reply_on_success(wasm_messages_msg, INSTANTIATE_MESSAGES_REPLY_ID);

    Ok(Response::new()
        .add_attribute("action", "instantiate")
        .add_attribute("contract_name", CONTRACT_NAME)
        .add_attribute("contract_version", CONTRACT_VERSION)
        .add_attribute("sender", info.sender)
        .add_submessages(vec![submsg_profiles, submsg_messages]))
}

