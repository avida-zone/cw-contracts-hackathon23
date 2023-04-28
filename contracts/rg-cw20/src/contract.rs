#[cfg(not(feature = "library"))]
pub(crate) use cosmwasm_std::entry_point;
pub(crate) use cosmwasm_std::{
    from_binary, to_binary, Binary, Deps, DepsMut, Env, MessageInfo, Reply, Response, StdError,
    StdResult, Uint128,
};

use avida_verifier::types::SubProofReqParams;
use cw_utils::parse_reply_execute_data;

use cw2::set_contract_version;
pub(crate) use cw20::{
    AllAccountsResponse, BalanceResponse, Cw20Coin, Cw20ReceiveMsg, DownloadLogoResponse,
    EmbeddedLogo, Logo, LogoInfo, MarketingInfoResponse, TokenInfoResponse,
};

use crate::state::PENDING_VERIFICATION;
pub(crate) use crate::{
    enumerable::query_all_accounts,
    error::ContractError,
    exec::*,
    marketing::*,
    msg::{ExecuteMsg, InstantiateMsg, QueryMsg},
    query::*,
    state::{
        RgMinterData, TokenInfo, BALANCES, LAUNCHPAD, LOGO, MARKETING_INFO, SUB_PROOF_REQ_PARAMS,
        TOKEN_INFO, VC_NONCE,
    },
    util::*,
    verify_vc_proof::verify_vc_proof,
};
use std::convert::TryInto;

// version info for migration info
const CONTRACT_NAME: &str = "crates.io:cw20-base";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

pub(crate) const VERIFICATION_ID: u64 = 0;
pub(crate) const LOGO_SIZE_CAP: usize = 5 * 1024;

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    mut deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;
    // check valid token info
    msg.validate()?;

    // create initial accounts
    let total_supply = create_accounts(&mut deps, &msg.initial_balances)?;

    if let Some(limit) = msg.get_cap() {
        if total_supply > limit {
            return Err(StdError::generic_err("Initial supply greater than cap").into());
        }
    }

    // Stores the proof request that the zkProof has to satisfy to interact with this contract
    let params = msg
        .req_params
        .iter()
        .map(|e| {
            e.clone()
                .try_into()
                .map_err(|_| ContractError::SubProofReqParams)
        })
        .collect::<Result<Vec<SubProofReqParams>, _>>()?;
    SUB_PROOF_REQ_PARAMS.save(deps.storage, &params)?;

    // store token info
    let data = TokenInfo {
        name: msg.name,
        symbol: msg.symbol,
        decimals: msg.decimals,
        total_supply,
        mint: msg.mint,
    };

    TOKEN_INFO.save(deps.storage, &data)?;
    LAUNCHPAD.save(deps.storage, &info.sender)?;

    if let Some(marketing) = msg.marketing {
        let logo = if let Some(logo) = marketing.logo {
            verify_logo(&logo)?;
            LOGO.save(deps.storage, &logo)?;

            match logo {
                Logo::Url(url) => Some(LogoInfo::Url(url)),
                Logo::Embedded(_) => Some(LogoInfo::Embedded),
            }
        } else {
            None
        };

        let data = MarketingInfoResponse {
            project: marketing.project,
            description: marketing.description,
            marketing: marketing
                .marketing
                .map(|addr| deps.api.addr_validate(&addr))
                .transpose()?,
            logo,
        };
        MARKETING_INFO.save(deps.storage, &data)?;
    }

    Ok(Response::default())
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::UpdateMarketing {
            project,
            description,
            marketing,
        } => execute_update_marketing(deps, env, info, project, description, marketing),
        ExecuteMsg::UploadLogo(logo) => execute_upload_logo(deps, env, info, logo),
        _ => verify_vc_proof(deps, env, info, msg),
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::Balance { address } => to_binary(&query_balance(deps, address)?),
        QueryMsg::ProofNonce { address } => to_binary(&query_nonce(deps, address)?),
        QueryMsg::TokenInfo {} => to_binary(&query_token_info(deps)?),
        QueryMsg::Minter {} => to_binary(&query_minter(deps)?),
        QueryMsg::AllAccounts { start_after, limit } => {
            to_binary(&query_all_accounts(deps, start_after, limit)?)
        }
        QueryMsg::MarketingInfo {} => to_binary(&query_marketing_info(deps)?),
        QueryMsg::DownloadLogo {} => to_binary(&query_download_logo(deps)?),
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn reply(deps: DepsMut, env: Env, reply: Reply) -> Result<Response, ContractError> {
    match reply.id {
        VERIFICATION_ID => {
            let verification_result = parse_reply_execute_data(reply)?;
            let (info, pending_tx) = PENDING_VERIFICATION.load(deps.storage)?;
            let verified: bool = from_binary(&verification_result.data.unwrap())?;
            if verified {
                VC_NONCE.update(deps.storage, &info.sender, |old| {
                    old.unwrap_or_default()
                        .checked_add(1)
                        .ok_or(ContractError::Overflow)
                })?;
                match pending_tx {
                    ExecuteMsg::Transfer {
                        recipient, amount, ..
                    } => execute_transfer(deps, env, info, recipient, amount),
                    ExecuteMsg::Burn { amount, .. } => execute_burn(deps, env, info, amount),
                    ExecuteMsg::Send {
                        contract,
                        amount,
                        msg,
                        ..
                    } => execute_send(deps, env, info, contract, amount, msg),
                    ExecuteMsg::Mint { amount, .. } => execute_mint(deps, env, info, amount),
                    // We handled these cases already because they do not need proofs
                    _ => unreachable!(),
                }
            } else {
                Err(ContractError::VerificationProcessError)
            }
        }
        _ => Err(ContractError::InvalidReplyId),
    }
}
