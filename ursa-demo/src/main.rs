use std::fs;
use ursa::bn::BigNumber;
use ursa::cl::issuer::Issuer;
use ursa::cl::new_nonce;
use ursa::cl::prover::Prover;
use ursa::cl::verifier::Verifier;
use ursa_demo::{
    create_sub_proof_request, data_dir, issuer_adds_cred_values_and_signs, issuer_set_up,
    prover_create_credential_req,
};

fn main() {
    env_logger::init();

    // Issuers set up schema and credential def
    // - Issuer qualified for DApp
    let issuers_str = fs::read_to_string("./setup_data/issuers.json").unwrap();
    let issuers: Vec<String> = serde_json::from_str(&issuers_str).unwrap();

    for issuer in issuers {
        let (
            credential_schema,
            non_credential_schema,
            credential_pub_key,
            credential_priv_key,
            credential_key_correctness_proof,
        ) = issuer_set_up(&issuer);
        create_sub_proof_request(&issuer)
    }
}
