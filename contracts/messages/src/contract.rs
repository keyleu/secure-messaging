use crate::error::ContractError;
use crate::state::{Config, CONFIG, USER_MESSAGES};
use cosmwasm_std::{entry_point, BankMsg, DepsMut, Env, MessageInfo};
use cosmwasm_std::{Addr, Binary, Response};
use cw2::set_contract_version;
use cw_ownable::{assert_owner, initialize_owner};
use utils::elements::Message;
use utils::msg::InstantiateMessagesMsg as InstantiateMsg;
use utils::msg::MessageExecuteMsg as ExecuteMsg;

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
        ExecuteMsg::ClaimMessageFunds { message_id } => claim_message_funds(deps, info, message_id),
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
    message_id: u64,
) -> Result<Response, ContractError> {
    let mut messages = USER_MESSAGES.load(deps.storage, info.sender.clone())?;

    let message = messages.get(message_id as usize);
    match message {
        Some(m) => {
            if m.funds.is_empty() {
                return Err(ContractError::MessageNoFunds { id: message_id });
            } else {
                let bank_msg = BankMsg::Send {
                    to_address: info.sender.clone().to_string(),
                    amount: m.funds.clone(),
                };

                messages[message_id as usize].funds = vec![];

                USER_MESSAGES.save(deps.storage, info.sender.clone(), &messages)?;

                Ok(Response::new()
                    .add_message(bank_msg)
                    .add_attribute("action", "claim_message_funds")
                    .add_attribute("sender", info.sender)
                    .add_attribute("message_id", message_id.to_string()))
            }
        }
        None => return Err(ContractError::NoMessage {}),
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
