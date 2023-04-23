use crate::types::{
    BigNumberBytes, WCredentialSchema, WNonCredentialSchema, WProof, WSubProofReq,
    WSubProofReqParams,
};
use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::Addr;

#[cw_serde]
pub struct InstantiateMsg {
    pub req_params: Vec<WSubProofReqParams>,
    pub wallet_cred_schema: WCredentialSchema,
    pub wallet_non_cred_schema: WNonCredentialSchema,
    pub wallet_sub_proof_request: WSubProofReq,
}

#[cw_serde]
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

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    /// Returns Admin address of this contract
    #[returns(Addr)]
    Admin {},
}
