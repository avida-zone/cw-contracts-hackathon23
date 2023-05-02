use crate::{
    contract::*,
    error::ContractError,
    state::{LAUNCHPAD, PENDING_VERIFICATION, VC_NONCE},
};
use avida_verifier::{
    msg::vc_verifier::ExecuteMsg as VcVerifierExecMsg, state::launchpad::VERIFIER,
    types::BigNumberBytes,
};
use cosmwasm_std::{to_binary, CosmosMsg, MessageInfo, SubMsg};

/// Calls the vc-verifier contract
pub fn verify_vc_proof(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    PENDING_VERIFICATION.save(deps.storage, &(info.clone(), msg.clone()))?;
    let (proof, wallet_addr) = match msg.clone() {
        ExecuteMsg::Burn { proof, .. } => (proof, info.sender),
        ExecuteMsg::Send { proof, .. } => (proof, info.sender),
        ExecuteMsg::Mint {
            proof, recipient, ..
        } => (proof, recipient),
        ExecuteMsg::Transfer { proof, .. } => (proof, info.sender),
        _ => unreachable!(),
    };

    // Get the nonce of the wallet
    let nonce = VC_NONCE
        .may_load(deps.storage, &wallet_addr)?
        .unwrap_or_default()
        .to_string();
    let proof_req_nonce = BigNumberBytes(nonce);

    let verification_msg = VcVerifierExecMsg::Verify {
        proof,
        proof_req_nonce,
        wallet_addr,
    };
    let verifier = VERIFIER.query(&deps.querier, LAUNCHPAD.load(deps.storage)?)?;

    let sub_msg = SubMsg::reply_on_success(
        CosmosMsg::Wasm(cosmwasm_std::WasmMsg::Execute {
            contract_addr: verifier.to_string(),
            msg: to_binary(&verification_msg)?,
            // This is currently no set but it will likely be set to a reasonable price
            funds: vec![],
        }),
        VERIFICATION_ID,
    );

    Ok(Response::new().add_submessage(sub_msg))
}
