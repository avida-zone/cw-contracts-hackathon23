use std::{
    fs,
    path::{Path, PathBuf},
};

use log::info;
use serde_json;
use ursa::bn::BigNumber;
use ursa::cl::issuer::Issuer;
use ursa::cl::prover::Prover;
use ursa::cl::{
    BlindedCredentialSecrets, BlindedCredentialSecretsCorrectnessProof,
    CredentialKeyCorrectnessProof, CredentialPrivateKey, CredentialPublicKey, CredentialSchema,
    CredentialSecretsBlindingFactors, CredentialSignature, NonCredentialSchema, Nonce,
    SignatureCorrectnessProof,
};

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

const CRED_SCHEMA_PATH: &str = "/credential_schema.json";
const NON_CRED_SCHEMA_PATH: &str = "/non_credential_schema.json";
const CRED_PUB_KEY: &str = "/credential_pub_key.json";
const CRED_PRI_KEY: &str = "/credential_priv_key.json";
const CRED_CORRECTNESS_PATH: &str = "/credential_correctness.json";

pub fn get_issuer_setup_outputs(
    dir: String,
) -> (
    CredentialSchema,
    NonCredentialSchema,
    CredentialPublicKey,
    CredentialPrivateKey,
    CredentialKeyCorrectnessProof,
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

    (schema, non_schema, pk, priv_key, correctness)
}

pub fn issuer_set_up(
    schema_attrs: Vec<&str>,
    non_schema_attrs: Vec<&str>,
    dir: String,
) -> (
    CredentialSchema,
    NonCredentialSchema,
    CredentialPublicKey,
    CredentialPrivateKey,
    CredentialKeyCorrectnessProof,
) {
    // Credential schema and Non credential schema
    let mut credential_schema_builder = Issuer::new_credential_schema_builder().unwrap();
    for a in schema_attrs {
        credential_schema_builder.add_attr(&a).unwrap();
    }
    let credential_schema = credential_schema_builder.finalize().unwrap();
    info!("credential schema {:?}", credential_schema);

    let mut non_credential_schema_builder = Issuer::new_non_credential_schema_builder().unwrap();
    for n in non_schema_attrs {
        non_credential_schema_builder.add_attr(&n).unwrap();
    }
    let non_credential_schema = non_credential_schema_builder.finalize().unwrap();
    info!("non credential schema {:?}", non_credential_schema);

    let data_dir = data_dir(&dir);
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
