use crate::types::{BigNumberBytes, WCredentialSchema, WNonCredentialSchema, WProof, WSubProofReq};
use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::Addr;

#[cw_serde]
pub struct InstantiateMsg {
    pub vectis_sub_proof_request: WSubProofReq,
    pub vectis_cred_schema: WCredentialSchema,
    pub vectis_non_cred_schema: WNonCredentialSchema,
    pub launchpad: Addr,
}

#[cw_serde]
pub enum ExecuteMsg {
    /// Called by prover where proof has already been generated offchain
    Verify {
        proof: WProof,
        // TODO: move
        proof_req_nonce: BigNumberBytes,
        wallet_addr: Addr,
    },
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {}
