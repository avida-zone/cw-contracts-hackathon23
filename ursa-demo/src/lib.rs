use std::{
    fs,
    path::{Path, PathBuf},
};

use log::info;
use serde::{Deserialize, Serialize};
use serde_json;
use ursa::cl::issuer::Issuer;
use ursa::cl::prover::Prover;
use ursa::cl::verifier::Verifier;
use ursa::cl::{
    BlindedCredentialSecrets, BlindedCredentialSecretsCorrectnessProof,
    CredentialKeyCorrectnessProof, CredentialPrivateKey, CredentialPublicKey, CredentialSchema,
    CredentialSecretsBlindingFactors, CredentialSignature, NonCredentialSchema, Nonce,
    SignatureCorrectnessProof,
};
use ursa::{bn::BigNumber, cl::SubProofRequest};

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
    CredentialSchema,
    NonCredentialSchema,
    CredentialPublicKey,
    CredentialPrivateKey,
    CredentialKeyCorrectnessProof,
    SubProofRequest,
) {
    let data_dir = data_dir(&dir);
    let path = data_dir.to_str().unwrap();
    let schema_json = fs::read_to_string(format!("{}{}", path, CRED_SCHEMA_PATH)).unwrap();
    let schema: CredentialSchema = serde_json::from_str(&schema_json).unwrap();

    let non_schema_json = fs::read_to_string(format!("{}{}", path, NON_CRED_SCHEMA_PATH)).unwrap();
    let non_schema: NonCredentialSchema = serde_json::from_str(&non_schema_json).unwrap();

    let pk_json = fs::read_to_string(format!("{}{}", path, CRED_PUB_KEY)).unwrap();
    let pk: CredentialPublicKey = serde_json::from_str(&pk_json).unwrap();

    let priv_json = fs::read_to_string(format!("{}{}", path, CRED_PRI_KEY)).unwrap();
    let priv_key: CredentialPrivateKey = serde_json::from_str(&priv_json).unwrap();

    let correctness_json =
        fs::read_to_string(format!("{}{}", path, CRED_CORRECTNESS_PATH)).unwrap();
    let correctness: CredentialKeyCorrectnessProof =
        serde_json::from_str(&correctness_json).unwrap();

    let sub_proof_req_json = fs::read_to_string(format!("{}{}", path, SUB_PROOF_REQ_PATH)).unwrap();
    let sub_proof_req: SubProofRequest = serde_json::from_str(&sub_proof_req_json).unwrap();

    (schema, non_schema, pk, priv_key, correctness, sub_proof_req)
}

pub fn get_issuer_setup_outputs_str(
    dir: String,
) -> (String, String, String, String, String, String) {
    let data_dir = data_dir(&dir);
    let path = data_dir.to_str().unwrap();
    let schema = fs::read_to_string(format!("{}{}", path, CRED_SCHEMA_PATH)).unwrap();

    let non_schema = fs::read_to_string(format!("{}{}", path, NON_CRED_SCHEMA_PATH)).unwrap();

    let pk = fs::read_to_string(format!("{}{}", path, CRED_PUB_KEY)).unwrap();

    let privkey = fs::read_to_string(format!("{}{}", path, CRED_PRI_KEY)).unwrap();

    let correctness = fs::read_to_string(format!("{}{}", path, CRED_CORRECTNESS_PATH)).unwrap();

    let sub_proof_req = fs::read_to_string(format!("{}{}", path, SUB_PROOF_REQ_PATH)).unwrap();

    (schema, non_schema, pk, privkey, correctness, sub_proof_req)
}

pub fn issuer_set_up(
    issuer: &str,
) -> (
    CredentialSchema,
    NonCredentialSchema,
    CredentialPublicKey,
    CredentialPrivateKey,
    CredentialKeyCorrectnessProof,
) {
    let schema_json = fs::read_to_string(format!("./setup_data/{}.json", issuer)).unwrap();
    let credential_schema: CredentialSchema = serde_json::from_str(&schema_json).unwrap();

    let non_schema_attrs: Vec<&'static str> = vec!["link_secret"];
    let mut non_credential_schema_builder = Issuer::new_non_credential_schema_builder().unwrap();
    for n in non_schema_attrs {
        non_credential_schema_builder.add_attr(&n).unwrap();
    }
    let non_credential_schema = non_credential_schema_builder.finalize().unwrap();
    info!("non credential schema {:?}", non_credential_schema);

    let data_dir = data_dir(issuer);
    let path = data_dir.to_str().unwrap();

    // Credential definition
    let (credential_pub_key, credential_priv_key, cred_key_correctness_proof) =
        Issuer::new_credential_def(&credential_schema, &non_credential_schema, false).unwrap();
    info!("credential pub key {:?}", credential_pub_key);

    std::fs::write(
        format!("{}{}", path, CRED_SCHEMA_PATH),
        serde_json::to_string_pretty(&credential_schema).unwrap(),
    )
    .unwrap();
    std::fs::write(
        format!("{}{}", path, NON_CRED_SCHEMA_PATH),
        serde_json::to_string_pretty(&non_credential_schema).unwrap(),
    )
    .unwrap();

    std::fs::write(
        format!("{}{}", path, CRED_PUB_KEY),
        serde_json::to_string_pretty(&credential_pub_key).unwrap(),
    )
    .unwrap();

    std::fs::write(
        format!("{}{}", path, CRED_PRI_KEY),
        serde_json::to_string_pretty(&credential_priv_key).unwrap(),
    )
    .unwrap();

    std::fs::write(
        format!("{}{}", path, CRED_CORRECTNESS_PATH),
        serde_json::to_string_pretty(&cred_key_correctness_proof).unwrap(),
    )
    .unwrap();

    (
        credential_schema,
        non_credential_schema,
        credential_pub_key,
        credential_priv_key,
        cred_key_correctness_proof,
    )
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Attrs {
    pub attrs: Vec<String>,
}

// This assumes all attributes are used and revealed with no predicate
pub fn create_sub_proof_request(issuer: &str) {
    let schema_json = fs::read_to_string(format!("./setup_data/{}.json", issuer)).unwrap();
    let schema: Attrs = serde_json::from_str(&schema_json).unwrap();

    let mut sub_proof_request_builder = Verifier::new_sub_proof_request_builder().unwrap();
    for attr in schema.attrs.clone() {
        sub_proof_request_builder.add_revealed_attr(&attr).unwrap();
    }
    let sub_proof_request = sub_proof_request_builder.finalize().unwrap();

    let issuer_dir = data_dir(issuer);
    let issuer_path = issuer_dir.to_str().unwrap();
    std::fs::write(
        format!("{}{}", issuer_path, "/sub_proof_request.json"),
        serde_json::to_string_pretty(&sub_proof_request).unwrap(),
    )
    .unwrap();
}

// this mimics the credential request that indy-sdk builds
pub fn prover_create_credential_req(
    non_schema_attrs: Vec<&str>,
    non_shared_values: Vec<BigNumber>,
    credential_pub_key: CredentialPublicKey,
    cred_key_correctness_proof: CredentialKeyCorrectnessProof,
    prover_credential_nonce: Nonce,
) -> (
    BlindedCredentialSecrets,
    CredentialSecretsBlindingFactors,
    BlindedCredentialSecretsCorrectnessProof,
) {
    // Credential Values: combines secret values
    let mut prover_credential_values_builder = Issuer::new_credential_values_builder().unwrap();

    for (idx, hidden_attr) in non_schema_attrs.iter().enumerate() {
        prover_credential_values_builder
            .add_value_hidden(hidden_attr, &non_shared_values[idx])
            .unwrap();
    }

    let hidden_credential_values = prover_credential_values_builder.finalize().unwrap();

    Prover::blind_credential_secrets(
        &credential_pub_key,
        &cred_key_correctness_proof,
        &hidden_credential_values,
        &prover_credential_nonce,
    )
    .unwrap()
}

pub fn issuer_adds_cred_values_and_signs(
    rev_idx: String,
    blinded_credential_secrets: BlindedCredentialSecrets,
    blinded_credential_secrets_correctness_proof: BlindedCredentialSecretsCorrectnessProof,
    prover_credential_nonce: Nonce,
    issuer_issuance_nonce: Nonce,
    cred_schema_attrs: Vec<&str>,
    cred_values: Vec<&str>,
    credential_pub_key: CredentialPublicKey,
    credential_priv_key: &CredentialPrivateKey,
) -> (CredentialSignature, SignatureCorrectnessProof) {
    // Issuer signs
    let mut credential_values_builder = Issuer::new_credential_values_builder().unwrap();
    for (idx, attr) in cred_schema_attrs.iter().enumerate() {
        credential_values_builder
            .add_dec_known(attr, cred_values[idx])
            .unwrap();
    }
    let credential_values = credential_values_builder.finalize().unwrap();

    Issuer::sign_credential(
        &rev_idx,
        &blinded_credential_secrets,
        &blinded_credential_secrets_correctness_proof,
        &prover_credential_nonce,
        &issuer_issuance_nonce,
        &credential_values,
        &credential_pub_key,
        credential_priv_key,
    )
    .unwrap()
}
