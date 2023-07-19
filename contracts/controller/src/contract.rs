use std::{cmp::Ordering, vec};

use cosmwasm_std::{
    entry_point, to_binary, Addr, BankMsg, Binary, CosmosMsg, Deps, DepsMut, Env, MessageInfo,
    Reply, Response, StdResult, SubMsg, WasmMsg,
};
use cw2::set_contract_version;
use cw_ownable::{assert_owner, initialize_owner};
use cw_utils::{one_coin, parse_reply_instantiate_data};
use utils::{
    msg::{MessagesInstantiateMsg, ProfilesInstantiateMsg, MessagesExecuteMsg, ProfilesExecuteMsg},
    query::{ProfileInfo, ProfilesQueryMsg},
};

use crate::{
    error::ContractError,
    msg::{ExecuteMsg, InstantiateMsg, QueryMsg},
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
            message_max_len: msg.message_max_len,
            message_cost: msg.send_message_cost,
            profile_cost: msg.create_profile_cost,
        },
    )?;

    let wasm_profiles_msg = WasmMsg::Instantiate {
        code_id: msg.code_id_profiles,
        msg: to_binary(&ProfilesInstantiateMsg {})?,
        funds: vec![],
        admin: Some(env.contract.address.clone().into_string()),
        label: format!("PROFILES-INFORMATION--{}", msg.code_id_profiles,),
    };
    let submsg_profiles =
        SubMsg::reply_on_success(wasm_profiles_msg, INSTANTIATE_PROFILES_REPLY_ID);

    let wasm_messages_msg = WasmMsg::Instantiate {
        code_id: msg.code_id_messages,
        msg: to_binary(&MessagesInstantiateMsg {
            default_query_limit: msg.message_query_default_limit,
            max_query_limit: msg.message_query_max_limit,
        })?,
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
        ExecuteMsg::CreateProfile { pubkey, user_id } => {
            create_profile(deps, info, pubkey, user_id)
        }
        ExecuteMsg::ChangeUserId { user_id } => change_user_id(deps, info, user_id),
        ExecuteMsg::ChangePubkey { pubkey } => change_pubkey(deps, info, pubkey),
        ExecuteMsg::SendMessage {
            content,
            dest_address,
            dest_id,
        } => send_message(deps, info, content, dest_address, dest_id),
        ExecuteMsg::ChangeMessagesConfig {
            message_query_default_limit,
            message_query_max_limit,
        } => change_messages_config(
            deps,
            info,
            message_query_default_limit,
            message_query_max_limit,
        ),
        ExecuteMsg::RetrieveFees { receiver } => retrieve_fees(deps, env, info, receiver),
        ExecuteMsg::UpdateOwnership(action) => update_ownership(deps, env, info, action),
    }
}

fn create_profile(
    deps: DepsMut,
    info: MessageInfo,
    pubkey: String,
    user_id: String,
) -> Result<Response, ContractError> {
    let config = CONFIG.load(deps.storage)?;

    if let Some(coin) = config.profile_cost {
        let funds = one_coin(&info)?;
        if funds != coin {
            return Err(ContractError::InvalidFunds {
                funds_required: coin,
            });
        }
    }

    let profile_address = PROFILES_ADDRESS.load(deps.storage)?;
    let create_profile_msg = ProfilesExecuteMsg::CreateProfile {
        address: info.sender.clone(),
        user_id: user_id.clone(),
        pubkey,
    };
    let msg = CosmosMsg::Wasm(WasmMsg::Execute {
        contract_addr: profile_address.to_string(),
        msg: to_binary(&create_profile_msg)?,
        funds: vec![],
    });

    Ok(Response::new()
        .add_message(msg)
        .add_attribute("action", "create_profile")
        .add_attribute("sender", info.sender)
        .add_attribute("user_id", user_id))
}

fn change_user_id(
    deps: DepsMut,
    info: MessageInfo,
    user_id: String,
) -> Result<Response, ContractError> {
    let profile_address = PROFILES_ADDRESS.load(deps.storage)?;

    let change_userid_msg = ProfilesExecuteMsg::ChangeUserId {
        address: info.sender.clone(),
        user_id: user_id.clone(),
    };
    let msg = CosmosMsg::Wasm(WasmMsg::Execute {
        contract_addr: profile_address.to_string(),
        msg: to_binary(&change_userid_msg)?,
        funds: vec![],
    });

    Ok(Response::new()
        .add_message(msg)
        .add_attribute("action", "create_userid")
        .add_attribute("sender", info.sender)
        .add_attribute("user_id", user_id))
}

fn change_pubkey(
    deps: DepsMut,
    info: MessageInfo,
    pubkey: String,
) -> Result<Response, ContractError> {
    let profile_address = PROFILES_ADDRESS.load(deps.storage)?;

    let change_pubkey_msg = ProfilesExecuteMsg::ChangePubkey {
        address: info.sender.clone(),
        pubkey: pubkey.clone(),
    };
    let msg = CosmosMsg::Wasm(WasmMsg::Execute {
        contract_addr: profile_address.to_string(),
        msg: to_binary(&change_pubkey_msg)?,
        funds: vec![],
    });

    Ok(Response::new()
        .add_message(msg)
        .add_attribute("action", "create_pubkey")
        .add_attribute("sender", info.sender)
        .add_attribute("pubkey", pubkey))
}

fn send_message(
    deps: DepsMut,
    info: MessageInfo,
    content: Binary,
    dest_address: Option<Addr>,
    dest_id: Option<String>,
) -> Result<Response, ContractError> {
    let config = CONFIG.load(deps.storage)?;
    let profile_address = PROFILES_ADDRESS.load(deps.storage)?;

    if content.len() > config.message_max_len.try_into().unwrap() {
        return Err(ContractError::MessageTooLong {});
    }

    if dest_address.is_none() && dest_id.is_none() {
        return Err(ContractError::NoDestination {});
    }

    let destination = match dest_address {
        Some(address) => deps.api.addr_validate(address.as_ref())?,
        None => {
            let profile_info: ProfileInfo = deps.querier.query_wasm_smart(
                profile_address,
                &ProfilesQueryMsg::UserInfo {
                    user_id: dest_id.unwrap(),
                },
            )?;
            profile_info.address
        }
    };

    let mut funds_to_send = info.funds;

    if let Some(cost) = config.message_cost {
        match funds_to_send.iter().position(|c| c.denom == cost.denom) {
            Some(index) => {
                match funds_to_send[index].amount.cmp(&cost.amount) {
                    Ordering::Less => return Err(ContractError::NotEnoughFundsForMessage {}),
                    Ordering::Equal => {
                        funds_to_send.remove(index);
                    }
                    Ordering::Greater => funds_to_send[index].amount -= cost.amount,
                };
            }
            None => return Err(ContractError::NotEnoughFundsForMessage {}),
        }
    }

    let message_address = MESSAGES_ADDRESS.load(deps.storage)?;
    let create_send_msg = MessagesExecuteMsg::SendMessage {
        sender: info.sender.clone(),
        receiver: destination.clone(),
        message: content,
    };
    let msg = CosmosMsg::Wasm(WasmMsg::Execute {
        contract_addr: message_address.to_string(),
        msg: to_binary(&create_send_msg)?,
        funds: funds_to_send,
    });

    Ok(Response::new()
        .add_message(msg)
        .add_attribute("action", "send_message")
        .add_attribute("sender", info.sender)
        .add_attribute("destination", destination))
}

fn change_messages_config(
    deps: DepsMut,
    info: MessageInfo,
    message_query_default_limit: u64,
    message_query_max_limit: u64,
) -> Result<Response, ContractError> {
    assert_owner(deps.storage, &info.sender)?;

    let messages_address = MESSAGES_ADDRESS.load(deps.storage)?;

    let change_pubkey_msg = MessagesExecuteMsg::ChangeConfig {
        default_query_limit: message_query_default_limit,
        max_query_limit: message_query_max_limit,
    };

    let msg = CosmosMsg::Wasm(WasmMsg::Execute {
        contract_addr: messages_address.to_string(),
        msg: to_binary(&change_pubkey_msg)?,
        funds: vec![],
    });

    Ok(Response::new()
        .add_message(msg)
        .add_attribute("action", "change_messages_config")
        .add_attribute("sender", info.sender))
}

fn retrieve_fees(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    receiver: Option<Addr>,
) -> Result<Response, ContractError> {
    assert_owner(deps.storage, &info.sender)?;
    let contract_balances = deps.querier.query_all_balances(env.contract.address)?;
    let receiver_address = deps
        .api
        .addr_validate(receiver.unwrap_or(info.sender).as_ref())?;

    let bank_msg = BankMsg::Send {
        to_address: receiver_address.to_string(),
        amount: contract_balances,
    };

    Ok(Response::new()
        .add_message(bank_msg)
        .add_attribute("recipient", receiver_address))
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

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::Config {} => to_binary(&query_config(deps)?),
    }
}

fn query_config(deps: Deps) -> StdResult<Config> {
    let config = CONFIG.load(deps.storage)?;

    Ok(config)
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
