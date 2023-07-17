use cosmwasm_std::{
    entry_point, to_binary, Addr, DepsMut, Env, MessageInfo, Reply, Response, SubMsg, WasmMsg,
};
use cw2::set_contract_version;
use cw_ownable::initialize_owner;
use cw_utils::parse_reply_instantiate_data;
use utils::msg::{InstantiateMessagesMsg, InstantiateProfilesMsg};

use crate::{
    error::ContractError,
    msg::{ExecuteMsg, InstantiateMsg},
    state::{Config, CONFIG, MESSAGES_ADDRESS, PROFILES_ADDRESS},
};

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

    CONFIG.save(
        deps.storage,
        &Config {
            message_cost: msg.send_message_cost,
            profile_cost: msg.create_profile_cost,
        },
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

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::CreateProfile {
            pub_key,
            user_id,
        } => todo!(),
        ExecuteMsg::SendMessage {
            content,
            dest_address,
            dest_id,
        } => todo!(),
        ExecuteMsg::UpdateOwnership(action) => update_ownership(deps, env, info, action),
    }
}

fn update_ownership(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    action: cw_ownable::Action,
) -> Result<Response, ContractError> {
    let ownership = cw_ownable::update_ownership(deps, &env.block, &info.sender, action)?;
    Ok(Response::new().add_attributes(ownership.into_attributes()))
}

// Reply callback triggered from instantiation of profiles and messages contract.
#[cfg_attr(not(feature = "library"), entry_point)]
pub fn reply(deps: DepsMut, _env: Env, msg: Reply) -> Result<Response, ContractError> {
    let reply = parse_reply_instantiate_data(msg.clone());
    match reply {
        Ok(res) => match msg.id {
            INSTANTIATE_MESSAGES_REPLY_ID => {
                MESSAGES_ADDRESS
                    .save(deps.storage, &Addr::unchecked(res.contract_address.clone()))?;
                Ok(Response::default()
                    .add_attribute("action", "instantiate_messages_reply")
                    .add_attribute("contract_address", res.contract_address))
            }
            INSTANTIATE_PROFILES_REPLY_ID => {
                PROFILES_ADDRESS
                    .save(deps.storage, &Addr::unchecked(res.contract_address.clone()))?;
                Ok(Response::default()
                    .add_attribute("action", "instantiate_profiles_reply")
                    .add_attribute("contract_address", res.contract_address))
            }
            _ => Err(ContractError::InvalidReplyID {}),
        },
        Err(_) => Err(ContractError::InstantiateError {}),
    }
}
