use crate::error::ContractError;
use avida_verifier::{
    msg::vc_verifier::{ExecuteMsg, InstantiateMsg, QueryMsg},
    state::{
        launchpad::{RG_CONTRACTS, RG_TRANSFORM},
        plugin::SELF_ISSUED_CRED_DEF,
        proof_request_data::{
            SUB_PROOF_REQ_PARAMS, VECTIS_CRED_SCHEMA, VECTIS_NON_CRED_SCHEMA, VECTIS_SUB_PROOF_REQ,
        },
    },
    types::{BigNumberBytes, SubProofReqParams, WProof, PLUGIN_QUERY_KEY},
};

use cw_storage_plus::{Item, KeyDeserialize};
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
const CONTRACT_NAME: &str = env!("CARGO_PKG_NAME");
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

const LAUNCHPAD: Item<Addr> = Item::new("launchpad");

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;

    LAUNCHPAD.save(deps.storage, &msg.launchpad)?;
    // Standard minimum proof request is to proof that the user controls the controller of the
    // smart contract wallet, which is pretty trivial.
    //
    // However, this is what the `link secret`  will be anchored to
    VECTIS_SUB_PROOF_REQ.save(deps.storage, &msg.vectis_sub_proof_request.try_into()?)?;
    VECTIS_CRED_SCHEMA.save(deps.storage, &msg.vectis_cred_schema.try_into()?)?;
    VECTIS_NON_CRED_SCHEMA.save(deps.storage, &msg.vectis_non_cred_schema.try_into()?)?;
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
        } => execute_proof_verify(deps, info, proof, proof_req_nonce, wallet_addr),
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(_deps: Deps, _env: Env, _msg: QueryMsg) -> StdResult<Binary> {
    Err(StdError::generic_err("not implemented"))
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
///
/// The format of the proof to be verified is dependent on the `proof-request-data`
pub fn execute_proof_verify(
    deps: DepsMut,
    info: MessageInfo,
    s_proof: WProof,
    s_proof_req_nonce: BigNumberBytes,
    wallet_addr: Addr,
) -> Result<Response, ContractError> {
    let launchpad = LAUNCHPAD.load(deps.storage)?;
    let rgtoken = RG_CONTRACTS.query(&deps.querier, launchpad.clone(), info.sender.clone())?;
    let rgtransform = RG_TRANSFORM.query(&deps.querier, launchpad, info.sender.clone())?;
    if rgtoken.is_none() && rgtransform.is_none() {
        return Err(ContractError::NotAvida);
    }
    let controller = CONTROLLER.query(&deps.querier, wallet_addr.clone())?;
    let identity_pluging = QUERY_PLUGINS
        .query(&deps.querier, wallet_addr, PLUGIN_QUERY_KEY)?
        .ok_or(ContractError::NoIdentityPlugin)?;
    let wallet_cred_pub_key =
        SELF_ISSUED_CRED_DEF.query(&deps.querier, deps.api.addr_humanize(&identity_pluging)?)?;

    let sub_proof_requests = SUB_PROOF_REQ_PARAMS.query(&deps.querier, info.sender)?;

    let verified = proof_verify(
        deps.storage,
        s_proof,
        s_proof_req_nonce,
        wallet_cred_pub_key.try_into()?,
        deps.api.addr_humanize(&controller.addr)?,
        sub_proof_requests,
    )?;

    Ok(Response::default()
        .set_data(to_binary(&verified)?) // set this for caller
        .add_attribute("verified", verified.to_string()))
}

fn proof_verify(
    storage: &dyn Storage,
    s_proof: WProof,
    s_proof_req_nonce: BigNumberBytes,
    wallet_cred_pub_key: CredentialPublicKey,
    controller_addr: Addr,
    sub_proof_requests: Vec<SubProofReqParams>,
) -> Result<bool, ContractError> {
    let proof: Proof = s_proof.try_into()?;

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
        &VECTIS_SUB_PROOF_REQ.load(storage)?,
        &VECTIS_CRED_SCHEMA.load(storage)?,
        &VECTIS_NON_CRED_SCHEMA.load(storage)?,
        &wallet_cred_pub_key,
        None,
        None,
    )?;

    proof_verifier
        .verify(&proof, &s_proof_req_nonce.try_into()?)
        .map_err(|e| ContractError::CannotExecuteVerify(e.to_string()))
}
