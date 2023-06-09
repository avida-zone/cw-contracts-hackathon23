use crate::state::INST_FEE;
pub(crate) use crate::{
    error::ContractError,
    exec::{
        exec_mint, exec_revert, exec_transform, exec_update_adapter, exec_update_code_id,
        exec_update_fee, exec_update_verifier, instantiate_rg_cw20,
    },
    msg::{ContractResponse, ContractType, ExecuteMsg, InstantiateMsg, LaunchType, QueryMsg},
    state::{
        LaunchpadOptions, ADAPTER, DEPLOYER, FEE, PENDING_INST, RG_CONTRACTS, RG_CW_20_CODE_ID,
        RG_TRANSFORM,
    },
};

use avida_verifier::state::launchpad::VERIFIER;
#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
pub(crate) use cosmwasm_std::{
    to_binary, Addr, Binary, Coin, Deps, DepsMut, Env, Event, MessageInfo, Order, Reply, Response,
    StdResult, SubMsg, Uint128, WasmMsg,
};
use cw2::set_contract_version;
use cw20_adapter::msg::ExecuteMsg as AdapterMsg;
use cw_storage_plus::Bound;
use cw_utils::parse_reply_instantiate_data;

// version info for migration info
const CONTRACT_NAME: &str = env!("CARGO_PKG_NAME");
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");
pub const INST_REPLY_ID: u64 = u64::MIN;
pub const TRANS_REPLY_ID: u64 = u64::MIN + 1;
// There is not additional fee at this time, only that required for creating denom
pub const DEFAULT_FEE: u128 = 10000000000000000000;

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;
    FEE.save(deps.storage, &Uint128::from(DEFAULT_FEE))?;
    factory_instantiate(deps, env, info, msg)
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::Launch {
            msg,
            label,
            launch_type,
        } => instantiate_rg_cw20(deps, env, info, msg, label, launch_type),
        ExecuteMsg::Transform {
            rg_token_addr,
            proof,
        } => exec_transform(deps, info, rg_token_addr, proof),
        ExecuteMsg::Revert { amount, recipient } => exec_revert(deps, info, amount, recipient),
        ExecuteMsg::Mint {
            rg_token_addr,
            amount,
            proof,
        } => exec_mint(deps, info, rg_token_addr, amount, proof),
        ExecuteMsg::UpdateVerifier { address } => exec_update_verifier(deps, info, address),
        ExecuteMsg::UpdateAdapter { address } => exec_update_adapter(deps, info, address),
        ExecuteMsg::UpdateFee { fee } => exec_update_fee(deps, info, fee),
        ExecuteMsg::UpdateRgTokenCodeId { id } => exec_update_code_id(deps, info, id),
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn reply(deps: DepsMut, _env: Env, reply: Reply) -> Result<Response, ContractError> {
    match reply.id {
        INST_REPLY_ID | TRANS_REPLY_ID => {
            let map = if reply.id == INST_REPLY_ID {
                RG_CONTRACTS
            } else {
                RG_TRANSFORM
            };
            let result = parse_reply_instantiate_data(reply)?;
            let pending = PENDING_INST.load(deps.storage)?;
            PENDING_INST.remove(deps.storage);
            let validated_addr = deps.api.addr_validate(&result.contract_address)?;
            map.save(deps.storage, validated_addr.clone(), &pending)?;
            let msg = WasmMsg::Execute {
                contract_addr: ADAPTER.load(deps.storage)?.to_string(),
                msg: to_binary(&AdapterMsg::RegisterRG {
                    addr: validated_addr,
                })?,
                funds: vec![Coin {
                    denom: "inj".to_string(),
                    amount: FEE.load(deps.storage)?,
                }],
            };
            let event = Event::new("Avida.Launchpad.v1.MsgTokenContractInstantiated")
                .add_attribute("contract_address", result.contract_address);
            Ok(Response::new().add_event(event).add_message(msg))
        }
        _ => Err(ContractError::NotImplemented),
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::RegisteredContracts {
            start_after,
            limit,
            contract_type,
        } => to_binary(&query_contracts(deps, start_after, limit, contract_type)?),
        QueryMsg::RegisteredContract { address } => to_binary(&query_contract(deps, address)?),
        QueryMsg::Fee {} => to_binary(&query_fee(deps)?),
        QueryMsg::RgCodeId {} => to_binary(&query_code_id(deps)?),
        QueryMsg::Verifier {} => to_binary(&query_verifier(deps)?),
        QueryMsg::Adapter {} => to_binary(&query_adapter(deps)?),
    }
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
    let event = Event::new("Avida.Launchpad.v1.MsgInstantiate")
        .add_attribute("contract_address", env.contract.address);
    Ok(Response::new().add_event(event))
}

pub fn query_fee(deps: Deps) -> StdResult<Coin> {
    INST_FEE.load(deps.storage)
}

pub fn query_code_id(deps: Deps) -> StdResult<u64> {
    RG_CW_20_CODE_ID.load(deps.storage)
}

pub fn query_verifier(deps: Deps) -> StdResult<Addr> {
    VERIFIER.load(deps.storage)
}

pub fn query_adapter(deps: Deps) -> StdResult<Addr> {
    ADAPTER.load(deps.storage)
}

pub fn query_contract(deps: Deps, address: String) -> StdResult<ContractResponse> {
    let valid_addr = deps.api.addr_validate(&address)?;
    let options = if let Some(options) = RG_CONTRACTS.may_load(deps.storage, valid_addr.clone())? {
        options
    } else {
        RG_TRANSFORM.load(deps.storage, valid_addr.clone())?
    };
    Ok(ContractResponse {
        contract_address: valid_addr,
        options,
    })
}

pub const DEFAULT_LIMIT: u64 = 20;
pub const MAX_LIMIT: u64 = 100;
pub fn query_contracts(
    deps: Deps,
    start_after: Option<String>,
    limit: Option<u64>,
    contract_type: ContractType,
) -> StdResult<Vec<ContractResponse>> {
    let limit = limit.unwrap_or(DEFAULT_LIMIT).min(MAX_LIMIT) as usize;
    let start = match start_after {
        Some(s) => {
            let rg_address = deps.api.addr_validate(&s)?;
            Some(Bound::exclusive(rg_address))
        }
        None => None,
    };
    let map = match contract_type {
        ContractType::New => RG_CONTRACTS,
        ContractType::Transform => RG_TRANSFORM,
    };
    let contracts: StdResult<Vec<ContractResponse>> = map
        .prefix(())
        .range(deps.storage, start, None, Order::Ascending)
        .take(limit)
        .map(|w| {
            let result = w?;
            Ok(ContractResponse {
                contract_address: result.0,
                options: result.1,
            })
        })
        .collect();

    Ok(contracts?)
}
