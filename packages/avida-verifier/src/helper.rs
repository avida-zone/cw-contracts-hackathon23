use crate::types::{
    BigNumberBytes, WCredentialPubKey, WCredentialSchema, WNonCredentialSchema, WProof,
    WSubProofReq,
};
use serde_json;
use std::{
    convert::TryFrom,
    fs,
    path::{Path, PathBuf},
};
use ursa::cl::CredentialPublicKey;

pub fn data_dir(dir: &str) -> PathBuf {
    let output = std::process::Command::new(env!("CARGO"))
        .arg("locate-project")
        .arg("--workspace")
        .arg("--message-format=plain")
        .output()
        .unwrap()
        .stdout;
    let cargo_path = Path::new(std::str::from_utf8(&output).unwrap().trim());
    let data_path = cargo_path.parent().unwrap().join("data/").join(dir);
    std::fs::create_dir_all(data_path.clone()).unwrap();
    data_path
}

pub const PROOF_PATH: &str = "/proof.json";
pub const PROOF_NONCE_PATH: &str = "/proof_req_nonce.json";
pub const CRED_SCHEMA_PATH: &str = "/credential_schema.json";
pub const NON_CRED_SCHEMA_PATH: &str = "/non_credential_schema.json";
pub const CRED_PUB_KEY: &str = "/credential_pub_key.json";
pub const CRED_PRI_KEY: &str = "/credential_priv_key.json";
pub const CRED_CORRECTNESS_PATH: &str = "/credential_correctness.json";
pub const SUB_PROOF_REQ_PATH: &str = "/sub_proof_request.json";

//  Limit: We only support 1 subProofReq per issuer
pub fn get_issuer_setup_outputs(
    dir: String,
) -> (
    WCredentialSchema,
    WNonCredentialSchema,
    WCredentialPubKey,
    WSubProofReq,
) {
    let data_dir = data_dir(&dir);
    let path = data_dir.to_str().unwrap();
    let schema_json = fs::read_to_string(format!("{}{}", path, CRED_SCHEMA_PATH)).unwrap();
    let schema: WCredentialSchema = serde_json::from_str(&schema_json).unwrap();

    let non_schema_json = fs::read_to_string(format!("{}{}", path, NON_CRED_SCHEMA_PATH)).unwrap();
    let non_schema: WNonCredentialSchema = serde_json::from_str(&non_schema_json).unwrap();

    let sub_proof_req_json = fs::read_to_string(format!("{}{}", path, SUB_PROOF_REQ_PATH)).unwrap();
    let sub_proof_req: WSubProofReq = serde_json::from_str(&sub_proof_req_json).unwrap();

    // We cannot directly try to make this into WCredentialPubKey because the field `r` is not
    // fixed, i.e. it depends on the requests.
    let pk_json = fs::read_to_string(format!("{}{}", path, CRED_PUB_KEY)).unwrap();
    let pk: CredentialPublicKey = serde_json::from_str(&pk_json).unwrap();
    let wpk = WCredentialPubKey::try_from(pk).unwrap();

    (schema, non_schema, wpk, sub_proof_req)
}

pub fn get_proof() -> (WProof, BigNumberBytes) {
    let data_dir = data_dir("");
    let path = data_dir.to_str().unwrap();

    let proof_json = fs::read_to_string(format!("{}{}", path, PROOF_PATH)).unwrap();
    let proof: WProof = serde_json::from_str(&proof_json).unwrap();

    let nonce_json = fs::read_to_string(format!("{}{}", path, PROOF_NONCE_PATH)).unwrap();
    let nonce: BigNumberBytes = serde_json::from_str(&nonce_json).unwrap();

    (proof, nonce)
}
