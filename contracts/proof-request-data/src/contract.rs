use avida_verifier::{
    msg::proof_request_data::{ExecuteMsg, InstantiateMsg, QueryMsg},
    state::proof_request_data::SUB_PROOF_REQ_PARAMS,
    types::SubProofReqParams,
};

#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{Binary, Deps, DepsMut, Env, MessageInfo, Response, StdError, StdResult};
use cw2::set_contract_version;
use std::convert::TryInto;
use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum ContractError {
    #[error("StdError{0}")]
    Std(#[from] StdError),

    #[error("NotImplemented")]
    NotImplemented,

    #[error("Map Error {0}")]
    MapError(String),
}

// version info for migration info
const CONTRACT_NAME: &str = env!("CARGO_PKG_NAME");
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;

    let params = msg
        .req_params
        .iter()
        .map(|e| {
            e.clone()
                .try_into()
                .map_err(|_| ContractError::MapError("Sub Proof Req value".to_string()))
        })
        .collect::<Result<Vec<SubProofReqParams>, _>>()?;

    SUB_PROOF_REQ_PARAMS.save(deps.storage, &params)?;

    Ok(Response::default())
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
pub fn query(_deps: Deps, _env: Env, _msg: QueryMsg) -> StdResult<Binary> {
    Err(StdError::generic_err("not implemented"))
}
