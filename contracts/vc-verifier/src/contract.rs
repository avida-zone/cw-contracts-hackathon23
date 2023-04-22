use crate::error::ContractError;
use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg};
use crate::state::{
    ADMIN, SUB_PROOF_REQ_PARAMS, WALLET_CRED_SCHEMA, WALLET_NON_CRED_SCHEMA, WALLET_SUB_PROOF_REQ,
};
use avida_verifier::types::PLUGIN_QUERY_KEY;

use avida_verifier::{
    plugin_state::SELF_ISSUED_CRED_DEF,
    types::{BigNumberBytes, SubProofReqParams, WProof},
};

use cw_storage_plus::KeyDeserialize;
use ursa::cl::{
    verifier::{ProofVerifier, Verifier},
    CredentialPublicKey, Proof,
};
use vectis_wallet::{CONTROLLER, QUERY_PLUGINS};

#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{
    to_binary, Addr, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdError, StdResult,
    Storage,
};
use cw2::set_contract_version;
use std::convert::TryInto;

// version info for migration info
const CONTRACT_NAME: &str = "crates.io:proof-verifier";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    mut deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;

    ADMIN.set(deps.branch(), Some(info.sender))?;

    // THIS IS THE ISSUE!
    //
    // On Instantiation of this onchain verifier,
    // we set what this verifier wants to verify.
    let params = msg
        .req_params
        .iter()
        .map(|e| {
            e.clone()
                .try_into()
                .map_err(|_| ContractError::Conversion("Sub Proof Req value".to_string()))
        })
        .collect::<Result<Vec<SubProofReqParams>, _>>()?;

    SUB_PROOF_REQ_PARAMS.save(deps.storage, &params)?;
    WALLET_CRED_SCHEMA.save(deps.storage, &msg.wallet_cred_schema.try_into()?)?;
    WALLET_NON_CRED_SCHEMA.save(deps.storage, &msg.wallet_non_cred_schema.try_into()?)?;
    WALLET_SUB_PROOF_REQ.save(deps.storage, &msg.wallet_sub_proof_request.try_into()?)?;

    Ok(Response::default())
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::Verify {
            proof,
            proof_req_nonce,
            wallet_addr,
        } => execute_proof_verify(deps, proof, proof_req_nonce, wallet_addr),
        ExecuteMsg::UpdateAdmin { new_admin } => execute_update_admin(deps, info, new_admin),
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::Admin {} => to_binary(&query_admin(deps)?),
    }
}

/// This is the verification of a proof from the proof request
///
/// This function directly use ursa CL signature module.
///
/// In libindy/src/services/anoncreds/verifiers.rs there is `Verifier` which
/// 1. checks inputs, which includes std::collection::Hashmap
/// 2. creates CryptoVerifier aka Ursa::cl::UrsaVerifier
/// 3. add sub proofs by iterating `full_proof` and `add_sub_proof_request` to the UrsaVerifier
/// 4. calls `verify()`
///
/// In step 1, the service `Verifier` takes in 6 args
///   full_proof: &Proof,
///   proof_req: &ProofRequestPayload,
///   schemas: &HashMap<SchemaId, SchemaV1>,
///   cred_defs: &HashMap<CredentialDefinitionId, CredentialDefinitionV1>,
///   rev_reg_defs: &HashMap<RevocationRegistryId, RevocationRegistryDefinitionV1>,
///   rev_regs: &HashMap<RevocationRegistryId, HashMap<u64, RevocationRegistryV1>>) -> IndyResult<bool> {
///
/// The verifier does not use any randomness
pub fn execute_proof_verify(
    deps: DepsMut,
    s_proof: WProof,
    s_proof_req_nonce: BigNumberBytes,
    wallet_addr: Addr,
) -> Result<Response, ContractError> {
    let controller = CONTROLLER.query(&deps.querier, wallet_addr.clone())?;
    let identity_pluging = QUERY_PLUGINS
        .query(&deps.querier, wallet_addr, PLUGIN_QUERY_KEY)?
        .ok_or(ContractError::NoIdentityPlugin)?;
    let wallet_cred_pub_key =
        SELF_ISSUED_CRED_DEF.query(&deps.querier, deps.api.addr_humanize(&identity_pluging)?)?;

    let verified = proof_verify(
        deps.storage,
        s_proof,
        s_proof_req_nonce,
        wallet_cred_pub_key.try_into()?,
        deps.api.addr_humanize(&controller.addr)?,
    )?;

    Ok(Response::default().add_attribute("verified", verified.to_string()))
}

fn proof_verify(
    storage: &dyn Storage,
    s_proof: WProof,
    s_proof_req_nonce: BigNumberBytes,
    wallet_cred_pub_key: CredentialPublicKey,
    controller_addr: Addr,
) -> Result<bool, ContractError> {
    let proof: Proof = s_proof.try_into()?;

    // TODO: add the actually wallet address in the credential def as well
    let sub_proof = proof
        .proofs
        .iter()
        .find(|sub_proof| {
            sub_proof
                .primary_proof
                .eq_proof
                .revealed_attrs
                .get("controller_addr")
                .is_some()
        })
        .ok_or(ContractError::MissingWalletAttr {})?;

    // TODO: We should use canonical addr for this
    let user = Addr::from_vec(
        sub_proof
            .primary_proof
            .eq_proof
            .revealed_attrs
            .get("controller_addr")
            .unwrap()
            .to_bytes()?,
    )?;
    if user != controller_addr {
        return Err(ContractError::InvalidWalletProof {});
    }

    let mut proof_verifier: ProofVerifier = Verifier::new_proof_verifier()
        .map_err(|_| ContractError::UrsaCryptoError("New Verifier failed".to_string()))?;

    let sub_proof_requests = SUB_PROOF_REQ_PARAMS.load(storage)?;
    for req in sub_proof_requests {
        proof_verifier.add_sub_proof_request(
            &req.sub_proof_request,
            &req.credential_schema,
            &req.non_credential_schema,
            &req.credential_pub_key.try_into()?,
            req.rev_key_pub.as_ref(),
            req.rev_reg.as_ref(),
        )?;
    }

    // add wallet sub proof request
    proof_verifier.add_sub_proof_request(
        &WALLET_SUB_PROOF_REQ.load(storage)?,
        &WALLET_CRED_SCHEMA.load(storage)?,
        &WALLET_NON_CRED_SCHEMA.load(storage)?,
        &wallet_cred_pub_key,
        None,
        None,
    )?;

    proof_verifier
        .verify(&proof, &s_proof_req_nonce.try_into()?)
        .map_err(|e| ContractError::CannotExecuteVerify(e.to_string()))
}

pub fn execute_update_admin(
    deps: DepsMut,
    info: MessageInfo,
    new_admin: Option<String>,
) -> Result<Response, ContractError> {
    ADMIN.assert_admin(deps.as_ref(), &info.sender)?;

    let admin = match new_admin.clone() {
        Some(addr) => Some(deps.api.addr_validate(&addr)?),
        None => None,
    };
    ADMIN.set(deps, admin)?;
    Ok(Response::default()
        .add_attribute("action", "admin updated")
        .add_attribute("new admin", new_admin.unwrap_or_else(|| "None".to_string())))
}

pub fn query_admin(deps: Deps) -> StdResult<Addr> {
    ADMIN.get(deps)?.ok_or(StdError::NotFound {
        kind: "admin".into(),
    })
}
