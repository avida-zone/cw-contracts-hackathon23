use crate::test_data::*;
use avida_verifier::types::{
    BigNumberBytes, WCredentialPubKey, WCredentialSchema, WNonCredentialSchema, WProof,
    WSubProofReq,
};
use serde_json;
use std::convert::TryFrom;
use ursa::cl::CredentialPublicKey;

//  Limit: We only support 1 subProofReq per issuer
pub fn get_issuer_setup_outputs(
    issuer: &str,
) -> (
    WCredentialSchema,
    WNonCredentialSchema,
    WCredentialPubKey,
    WSubProofReq,
) {
    println!("issuer: {}", issuer);
    match issuer {
        "trusted_issuer" => {
            let pk: CredentialPublicKey = serde_json::from_str(&PUB_KEY).unwrap();
            let wpk = WCredentialPubKey::try_from(pk).unwrap();
            (
                serde_json::from_str(&CRED_SCHEMA).unwrap(),
                serde_json::from_str(&NON_CRED_SCHEMA).unwrap(),
                wpk,
                serde_json::from_str(&SUB_PROOF_REQ).unwrap(),
            )
        }
        "self_issued" => {
            let pk: CredentialPublicKey = serde_json::from_str(&SELF_PUB_KEY).unwrap();
            let wpk = WCredentialPubKey::try_from(pk).unwrap();
            (
                serde_json::from_str(&SELF_CRED_SCHEMA).unwrap(),
                serde_json::from_str(&SELF_NON_CRED_SCHEMA).unwrap(),
                wpk,
                serde_json::from_str(&SELF_SUB_PROOF_REQ).unwrap(),
            )
        }
        _ => panic!("not supported"),
    }
}

pub fn get_proof() -> (WProof, BigNumberBytes) {
    let proof: WProof = serde_json::from_str(&PROOF).unwrap();
    let nonce: BigNumberBytes = serde_json::from_str(&PROOF_REQ_NONCE).unwrap();
    (proof, nonce)
}
