use avida_verifier::{
    state::plugin::{SELF_ISSUED_CRED_DEF, VECTIS_ACCOUNT},
    types::WCredentialPubKey,
};
use cosmwasm_schema::{cw_serde, QueryResponses};
#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{
    to_binary, Binary, Event, Deps, DepsMut, Env, MessageInfo, Response, StdError, StdResult,
};
use cw2::set_contract_version;
use thiserror::Error;

// version info for migration info
const CONTRACT_NAME: &str = "crates.io:launchpad";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

#[derive(Error, Debug, PartialEq)]
pub enum ContractError {
    #[error("Identity Plugin Inst Failed")]
    IdentityPluginInstFailed,
    #[error("StdError {0}")]
    Std(#[from] StdError),
    #[error("Not implemented")]
    NotImplemented,
}

#[cw_serde]
pub struct InstantiateMsg {
    pub rg_cw20_code_id: u64,
}

#[cw_serde]
pub struct ExecuteMsg {}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    #[returns(WCredentialPubKey)]
    CredentialPubKey,
}

pub fn factory_instantiate(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    let admin_addr = deps.api.addr_canonicalize(info.sender.as_ref())?;

    DEPLOYER.save(deps.storage, &admin_addr)?;
    RG_CW_20_CODE_ID.save(deps.storage, &msg.rg_cw20_code_id)?;

    let event = Event::new("vectis.factory.v1.MsgInstantiate")
        .add_attribute("contract_address", env.contract.address);

    Ok(Response::new().add_event(event))
}

pub fn instantiate_rg_cw20(
    deps: DepsMut,
    info: MessageInfo,
    env: Env) -> Result<Response, ContractError> {
    // The wasm message containing the `wallet_proxy` instantiation message
    let instantiate_msg = WasmMsg::Instantiate {
        admin: Some(env.contract.address.to_string()),
        code_id: PROXY_CODE_ID.load(deps.storage)?,
        msg: to_binary(&ProxyInstantiateMsg {
            multisig_code_id: PROXY_MULTISIG_CODE_ID.load(deps.storage)?,
            create_wallet_msg,
            code_id: PROXY_CODE_ID.load(deps.storage)?,
        })?,
        funds,
        label: "Wallet-Proxy".into(),
    };
    let msg = SubMsg::reply_on_success(instantiate_msg);

    let event = Event::new("vectis.factory.v1.MsgInstantiate_rg_cw20");

    let res = Response::new().add_submessage(msg).add_event(event);

    Ok(res)
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;
    factory_instantiate(deps, env, info, msg)
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    _deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    _msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    Err(ContractError::NotImplemented)
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::CredentialPubKey => to_binary(&query_cred_pub_key(deps)?),
    }
}

fn query_cred_pub_key(deps: Deps) -> StdResult<WCredentialPubKey> {
    SELF_ISSUED_CRED_DEF.load(deps.storage)
}
