use cosmwasm_std::Addr;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::error::ContractError;
use vectis_verifier::types::{
    BigNumberBytes, WCredentialSchema, WNonCredentialSchema, WProof, WSubProofReq,
    WSubProofReqParams,
};

pub type ConversionResult<T> = std::result::Result<T, ContractError>;

#[derive(Serialize, Deserialize, JsonSchema, Debug, Clone, PartialEq)]
pub struct InstantiateMsg {
    pub req_params: Vec<WSubProofReqParams>,
    pub wallet_cred_schema: WCredentialSchema,
    pub wallet_non_cred_schema: WNonCredentialSchema,
    pub wallet_sub_proof_request: WSubProofReq,
}

#[derive(Serialize, Deserialize, JsonSchema, Debug, Clone, PartialEq)]
pub enum ExecuteMsg {
    /// Called by prover where proof has already been generated offchain
    /// This does not change state but will emit an event
    Verify {
        proof: WProof,
        proof_req_nonce: BigNumberBytes,
        wallet_addr: Addr,
    },
    UpdateAdmin {
        new_admin: Option<String>,
    },
}

#[derive(Serialize, Deserialize, JsonSchema, Debug, Clone, PartialEq)]
pub enum QueryMsg {
    /// Returns Admin address of this contract
    Admin {},
    /// Called by other contracts that requires to verify proof that has already been generated offchain
    /// External Queries may not be able to use it, depending on node query gas limit,
    /// use `ExecuteMsg::Verify` instead
    Verify {
        proof: WProof,
        proof_req_nonce: BigNumberBytes,
        addr: Addr,
    },
}
