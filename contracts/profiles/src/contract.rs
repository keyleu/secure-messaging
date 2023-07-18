use cosmwasm_std::{entry_point, to_binary, DepsMut, Env, MessageInfo, Response};
use cosmwasm_std::{Addr, Binary, Deps, StdResult};
use cw2::set_contract_version;
use cw_ownable::{assert_owner, initialize_owner};
use utils::elements::Profile;
use utils::msg::InstantiateProfilesMsg as InstantiateMsg;
use utils::msg::ProfileExecuteMsg as ExecuteMsg;
use utils::query::{ProfileInfo, ProfileQueryMsg as QueryMsg};

use crate::error::ContractError;
use crate::state::{ADDRESS_TO_PROFILE, USERID_TO_ADDRESS};

const CONTRACT_NAME: &str = env!("CARGO_PKG_NAME");
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    _msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;

    initialize_owner(
        deps.storage,
        deps.api,
        Some(&info.sender.clone().into_string()),
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
        ExecuteMsg::CreateProfile {
            address,
            user_id,
            pubkey,
        } => create_profile(deps, info, address, user_id, pubkey),
        ExecuteMsg::ChangeUserId { address, user_id } => {
            change_user_id(deps, info, address, user_id)
        }
        ExecuteMsg::ChangePubkey { address, pubkey } => change_pubkey(deps, info, address, pubkey),
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

fn create_profile(
    deps: DepsMut,
    info: MessageInfo,
    address: Addr,
    user_id: String,
    pubkey: String,
) -> Result<Response, ContractError> {
    assert_owner(deps.storage, &info.sender)?;

    if USERID_TO_ADDRESS.has(deps.storage, user_id.clone()) {
        return Err(ContractError::UserIdAlreadyExists {});
    }

    if ADDRESS_TO_PROFILE.has(deps.storage, address.clone()) {
        return Err(ContractError::AddressHasProfile {});
    }

    USERID_TO_ADDRESS.save(deps.storage, user_id.clone(), &address)?;
    ADDRESS_TO_PROFILE.save(deps.storage, address.clone(), &Profile { user_id, pubkey })?;

    Ok(Response::new()
        .add_attribute("action", "create_profile")
        .add_attribute("address", address))
}

fn change_pubkey(
    deps: DepsMut,
    info: MessageInfo,
    address: Addr,
    pubkey: String,
) -> Result<Response, ContractError> {
    assert_owner(deps.storage, &info.sender)?;

    let mut profile = ADDRESS_TO_PROFILE.load(deps.storage, address.clone())?;
    profile.pubkey = pubkey;
    ADDRESS_TO_PROFILE.save(deps.storage, address.clone(), &profile)?;

    Ok(Response::new()
        .add_attribute("action", "update_pubkey")
        .add_attribute("address", address))
}

fn change_user_id(
    deps: DepsMut,
    info: MessageInfo,
    address: Addr,
    user_id: String,
) -> Result<Response, ContractError> {
    assert_owner(deps.storage, &info.sender)?;

    let mut profile = ADDRESS_TO_PROFILE.load(deps.storage, address.clone())?;
    if USERID_TO_ADDRESS.has(deps.storage, user_id.clone()) {
        return Err(ContractError::UserIdAlreadyExists {});
    }
    profile.user_id = user_id;
    ADDRESS_TO_PROFILE.save(deps.storage, address.clone(), &profile)?;

    Ok(Response::new()
        .add_attribute("action", "change_user_id")
        .add_attribute("address", address))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::UserInfo { user_id } => to_binary(&query_user_info(deps, user_id)?),
        QueryMsg::AddressInfo { address } => to_binary(&query_address_info(deps, address)?),
    }
}

fn query_user_info(deps: Deps, user_id: String) -> StdResult<ProfileInfo> {
    let address = USERID_TO_ADDRESS.load(deps.storage, user_id.clone())?;
    let profile = ADDRESS_TO_PROFILE.load(deps.storage, address.clone())?;

    Ok(ProfileInfo {
        address,
        user_id,
        pubkey: profile.pubkey,
    })
}

fn query_address_info(deps: Deps, address: Addr) -> StdResult<ProfileInfo> {
    let profile = ADDRESS_TO_PROFILE.load(deps.storage, address.clone())?;

    Ok(ProfileInfo {
        address,
        user_id: profile.user_id,
        pubkey: profile.pubkey,
    })
}
