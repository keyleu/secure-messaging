use crate::error::ContractError;
use crate::state::{Config, CONFIG, USER_MESSAGES};
use cosmwasm_std::{
    entry_point, to_binary, BankMsg, Coin, Deps, DepsMut, Env, MessageInfo, StdResult,
};
use cosmwasm_std::{Addr, Binary, Response};
use cw2::set_contract_version;
use cw_ownable::{assert_owner, initialize_owner};
use utils::elements::Message;
use utils::msg::MessagesInstantiateMsg as InstantiateMsg;
use utils::msg::MessagesExecuteMsg as ExecuteMsg;
use utils::query::{
    MessageResponse, MessagesQueryMsg as QueryMsg, MessagesResponse, TotalMessagesResponse,
};

const CONTRACT_NAME: &str = env!("CARGO_PKG_NAME");
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
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
            default_query_limit: msg.default_query_limit,
            max_query_limit: msg.max_query_limit,
        },
    )?;

    Ok(Response::new()
        .add_attribute("action", "instantiate")
        .add_attribute("contract_name", CONTRACT_NAME)
        .add_attribute("contract_version", CONTRACT_VERSION)
        .add_attribute("sender", info.sender))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::SendMessage {
            sender,
            receiver,
            message,
        } => send_message(deps, info, sender, receiver, message),
        ExecuteMsg::ClaimMessageFunds { message_ids } => {
            claim_message_funds(deps, info, message_ids)
        }
        ExecuteMsg::DeleteMessages { message_ids } => delete_messages(deps, info, message_ids),
        ExecuteMsg::ChangeConfig {
            default_query_limit,
            max_query_limit,
        } => change_config(deps, info, default_query_limit, max_query_limit),
        ExecuteMsg::UpdateOwnership(action) => update_ownership(deps, env, info, action),
    }
}

fn send_message(
    deps: DepsMut,
    info: MessageInfo,
    sender: Addr,
    receiver: Addr,
    message: Binary,
) -> Result<Response, ContractError> {
    assert_owner(deps.storage, &info.sender)?;
    let message = Message {
        sender: sender.clone(),
        content: message,
        funds: info.funds,
    };

    let mut current_messages = match USER_MESSAGES.has(deps.storage, receiver.clone()) {
        true => USER_MESSAGES.load(deps.storage, receiver.clone())?,
        false => vec![],
    };

    current_messages.push(message);
    USER_MESSAGES.save(deps.storage, receiver, &current_messages)?;

    Ok(Response::new()
        .add_attribute("action", "store_message")
        .add_attribute("sender", sender))
}

fn claim_message_funds(
    deps: DepsMut,
    info: MessageInfo,
    message_ids: Vec<u64>,
) -> Result<Response, ContractError> {
    let mut messages = USER_MESSAGES.load(deps.storage, info.sender.clone())?;
    let mut funds_to_send = vec![];
    create_funds_array(&mut funds_to_send, &mut messages, message_ids)?;

    let bank_msg = BankMsg::Send {
        to_address: info.sender.clone().to_string(),
        amount: funds_to_send,
    };

    USER_MESSAGES.save(deps.storage, info.sender.clone(), &messages)?;

    Ok(Response::new()
        .add_message(bank_msg)
        .add_attribute("action", "claim_message_funds")
        .add_attribute("sender", info.sender))
}

fn delete_messages(
    deps: DepsMut,
    info: MessageInfo,
    message_ids: Vec<u64>,
) -> Result<Response, ContractError> {
    let mut messages = USER_MESSAGES.load(deps.storage, info.sender.clone())?;
    let mut funds_to_send = vec![];
    create_funds_array(&mut funds_to_send, &mut messages, message_ids.clone())?;

    let bank_msg = BankMsg::Send {
        to_address: info.sender.clone().to_string(),
        amount: funds_to_send,
    };

    let mut idx = 0;
    messages.retain(|_| {
        let current = idx;
        idx += 1;
        !message_ids.contains(&current)
    });

    Ok(Response::new()
        .add_message(bank_msg)
        .add_attribute("action", "delete_messages")
        .add_attribute("sender", info.sender))
}

fn create_funds_array(
    funds: &mut Vec<Coin>,
    messages: &mut [Message],
    message_ids: Vec<u64>,
) -> Result<(), ContractError> {
    for index in message_ids {
        let message = messages.get(index as usize);
        match message {
            Some(m) => {
                if !m.funds.is_empty() {
                    for c in m.funds.clone() {
                        match funds.iter().position(|f| f.denom == c.denom) {
                            Some(index) => {
                                funds[index].amount += c.amount;
                            }
                            None => {
                                funds.push(c);
                            }
                        }
                    }

                    messages[index as usize].funds = vec![];
                }
            }
            None => return Err(ContractError::NoMessage {}),
        }
    }
    Ok(())
}

fn change_config(
    deps: DepsMut,
    info: MessageInfo,
    default_query_limit: u64,
    max_query_limit: u64,
) -> Result<Response, ContractError> {
    assert_owner(deps.storage, &info.sender)?;
    let mut config = CONFIG.load(deps.storage)?;
    config.default_query_limit = default_query_limit;
    config.max_query_limit = max_query_limit;
    CONFIG.save(deps.storage, &config)?;

    Ok(Response::new()
        .add_attribute("action", "change_config")
        .add_attribute("sender", info.sender))
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
        QueryMsg::Messages {
            address,
            from,
            limit,
        } => to_binary(&query_messages(deps, address, from, limit)?),
        QueryMsg::TotalMessages { address } => to_binary(&query_total_messages(deps, address)?),
    }
}

fn query_messages(
    deps: Deps,
    address: Addr,
    from: u64,
    limit: Option<u64>,
) -> StdResult<MessagesResponse> {
    let messages = USER_MESSAGES.load(deps.storage, address)?;
    let config = CONFIG.load(deps.storage)?;

    let query_limit = limit
        .unwrap_or(config.default_query_limit)
        .min(config.max_query_limit);
    let mut return_messages = vec![];

    for index in ((from.min(messages.len().try_into().unwrap()) - query_limit).max(0)..from).rev() {
        return_messages.push(MessageResponse {
            id: index,
            message: messages.get(index as usize).unwrap().clone(),
        })
    }

    Ok(MessagesResponse {
        messages: return_messages,
    })
}

fn query_total_messages(deps: Deps, address: Addr) -> StdResult<TotalMessagesResponse> {
    let messages = USER_MESSAGES.may_load(deps.storage, address)?;

    let total = match messages {
        Some(messages) => messages.len(),
        None => 0,
    };

    Ok(TotalMessagesResponse {
        total: total.try_into().unwrap(),
    })
}
