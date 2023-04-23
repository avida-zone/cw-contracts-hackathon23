use ursa::bn::BigNumber;
use ursa::cl::issuer::Issuer;
use ursa::cl::new_nonce;
use ursa::cl::prover::Prover;
use ursa::cl::verifier::Verifier;
use ursa_demo::{
    data_dir, issuer_adds_cred_values_and_signs, issuer_set_up, prover_create_credential_req,
};

fn main() {
    env_logger::init();

    let schema_attrs: Vec<&'static str> = vec!["name", "age"];
    let attr_values: Vec<&'static str> = vec![
        "5683357908103347059525661567944931380213200296242705847943670736985289748650",
        "998878473220768490044181134091226568472324611129842329272337785551474",
    ];
    let non_schema_attrs: Vec<&'static str> = vec!["link_secret"];

    let w_schema_attrs: Vec<&'static str> = vec!["wallet_addr", "controller_addr"];
    let controller_addr = "wasm1jcdyqsjyvp86g6tuzwwryfkpvua89fau728ctm";
    // This is dummy
    let wallet_addr = "wasm1jcdyqsjyvp86g6tuzwwryfkpvua89fau728ctm";
    let addr = wallet_addr.as_bytes();
    let controller_num = BigNumber::from_bytes(addr).unwrap().to_dec().unwrap();
    let wallet_num = BigNumber::from_bytes(controller_addr.as_bytes())
        .unwrap()
        .to_dec()
        .unwrap();
    let w_attr_values = vec![wallet_num.as_str(), controller_num.as_str()];

    // Issuers set up schema and credential def
    // - Issuer qualified for DApp
    // - Vectis wallet also stores a credential definition
    let (
        credential_schema,
        non_credential_schema,
        credential_pub_key,
        credential_priv_key,
        credential_key_correctness_proof,
    ) = issuer_set_up(
        schema_attrs.clone(),
        non_schema_attrs.clone(),
        "issuer".into(),
    );

    let (
        w_credential_schema,
        w_non_credential_schema,
        w_credential_pub_key,
        w_credential_priv_key,
        w_credential_key_correctness_proof,
    ) = issuer_set_up(
        w_schema_attrs.clone(),
        non_schema_attrs.clone(),
        "wallet".into(),
    );

    // Prover creates request
    // The prover must create a link_secret, which is that "blinded" as the
    // `blinded_credential_secrets`, factors and correctness proof.
    // ```pub struct CredentialRequest {
    //    pub prover_did: DidValue,
    //    pub cred_def_id: CredentialDefinitionId,
    //    pub blinded_ms: BlindedCredentialSecrets,
    //    pub blinded_ms_correctness_proof: BlindedCredentialSecretsCorrectnessProof,
    //    pub nonce: Nonce,
    //}
    //```
    // Note that indy-sdk only supports 1 blinded attribute in creating the
    // `credential_secrets_blinding_factors`.
    let link_secret = Prover::new_master_secret().unwrap();
    let prover_credential_nonce = new_nonce().unwrap();
    let (
        blinded_credential_secrets,
        credential_secrets_blinding_factors, // This is a secret and should not be shared
        blinded_credential_secrets_correctness_proof,
    ) = prover_create_credential_req(
        non_schema_attrs.clone(),
        vec![link_secret.value().unwrap()],
        credential_pub_key.try_clone().unwrap(),
        credential_key_correctness_proof,
        prover_credential_nonce.try_clone().unwrap(),
    );

    let w_prover_credential_nonce = new_nonce().unwrap();
    let (
        w_blinded_credential_secrets,
        w_credential_secrets_blinding_factors, // This is a secret and should not be shared
        w_blinded_credential_secrets_correctness_proof,
    ) = prover_create_credential_req(
        non_schema_attrs.clone(),
        vec![link_secret.value().unwrap()],
        w_credential_pub_key.try_clone().unwrap(),
        w_credential_key_correctness_proof,
        w_prover_credential_nonce.try_clone().unwrap(),
    );

    // Issuer creates credential
    // - populate the credential values with values required in the schema
    // - signs the values individually and also the blinding factor
    let issuer_issuance_nonce = new_nonce().unwrap();
    let (mut cred_sig, cred_sig_proof) = issuer_adds_cred_values_and_signs(
        "some-did".into(),
        blinded_credential_secrets,
        blinded_credential_secrets_correctness_proof,
        prover_credential_nonce.try_clone().unwrap(),
        issuer_issuance_nonce.try_clone().unwrap(),
        schema_attrs.clone(),
        attr_values.clone(),
        credential_pub_key.try_clone().unwrap(),
        &credential_priv_key,
    );

    let w_issuer_issuance_nonce = new_nonce().unwrap();
    let (mut w_cred_sig, w_cred_sig_proof) = issuer_adds_cred_values_and_signs(
        "some-did".into(),
        w_blinded_credential_secrets,
        w_blinded_credential_secrets_correctness_proof,
        w_prover_credential_nonce.try_clone().unwrap(),
        w_issuer_issuance_nonce.try_clone().unwrap(),
        w_schema_attrs.clone(),
        w_attr_values.clone(),
        w_credential_pub_key.try_clone().unwrap(),
        &w_credential_priv_key,
    );

    // Prover accepts the credential and "unblind" it
    // Now the credential_values have both hidden / known attributes
    let mut cred_values_builder = Issuer::new_credential_values_builder().unwrap();
    for (idx, attr) in schema_attrs.iter().enumerate() {
        cred_values_builder
            .add_dec_known(attr, &attr_values[idx])
            .unwrap();
    }
    // There is only one hidden value
    cred_values_builder
        .add_value_hidden(&non_schema_attrs[0], &link_secret.value().unwrap())
        .unwrap();
    let credential_values = cred_values_builder.finalize().unwrap();

    Prover::process_credential_signature(
        &mut cred_sig,
        &credential_values,
        &cred_sig_proof,
        &credential_secrets_blinding_factors,
        &credential_pub_key,
        &issuer_issuance_nonce,
        None,
        None,
        None,
    )
    .unwrap();

    let mut w_cred_values_builder = Issuer::new_credential_values_builder().unwrap();
    for (idx, attr) in w_schema_attrs.iter().enumerate() {
        w_cred_values_builder
            .add_dec_known(attr, &w_attr_values[idx])
            .unwrap();
    }
    w_cred_values_builder
        .add_value_hidden(&non_schema_attrs[0], &link_secret.value().unwrap())
        .unwrap();
    let w_credential_values = w_cred_values_builder.finalize().unwrap();
    Prover::process_credential_signature(
        &mut w_cred_sig,
        &w_credential_values,
        &w_cred_sig_proof,
        &w_credential_secrets_blinding_factors,
        &w_credential_pub_key,
        &w_issuer_issuance_nonce,
        None,
        None,
        None,
    )
    .unwrap();

    // Verifier creates proof request schema
    let mut sub_proof_request_builder = Verifier::new_sub_proof_request_builder().unwrap();
    for attr in schema_attrs.clone() {
        sub_proof_request_builder.add_revealed_attr(&attr).unwrap();
    }
    let sub_proof_request = sub_proof_request_builder.finalize().unwrap();

    let mut w_sub_proof_request_builder = Verifier::new_sub_proof_request_builder().unwrap();
    for attr in w_schema_attrs.clone() {
        w_sub_proof_request_builder
            .add_revealed_attr(&attr)
            .unwrap();
    }
    let w_sub_proof_request = w_sub_proof_request_builder.finalize().unwrap();

    // Prover provide proof
    // - add the sub proof request from an issuer
    // - add the sub proof request from the self issued credential
    let mut proof_builder = Prover::new_proof_builder().unwrap();
    proof_builder.add_common_attribute("link_secret").unwrap();

    proof_builder
        .add_sub_proof_request(
            &sub_proof_request,
            &credential_schema,
            &non_credential_schema,
            &cred_sig,
            &credential_values,
            &credential_pub_key,
            None,
            None,
        )
        .unwrap();

    proof_builder
        .add_sub_proof_request(
            &w_sub_proof_request,
            &w_credential_schema,
            &w_non_credential_schema,
            &w_cred_sig,
            &w_credential_values,
            &w_credential_pub_key,
            None,
            None,
        )
        .unwrap();

    let issuer_dir = data_dir("issuer");
    let issuer_path = issuer_dir.to_str().unwrap();
    std::fs::write(
        format!("{}{}", issuer_path, "sub_proof_request.json"),
        serde_json::to_string_pretty(&sub_proof_request).unwrap(),
    )
    .unwrap();

    let wallet_dir = data_dir("wallet");
    let wallet_path = wallet_dir.to_str().unwrap();
    std::fs::write(
        format!("{}{}", wallet_path, "sub_proof_request.json"),
        serde_json::to_string_pretty(&w_sub_proof_request).unwrap(),
    )
    .unwrap();

    let proof_request_nonce = new_nonce().unwrap();
    let proof = proof_builder.finalize(&proof_request_nonce).unwrap();

    // Verifier verifies
    // - adds any issuer's proof req
    // - adds wallets self issued proof req
    let mut proof_verifier = Verifier::new_proof_verifier().unwrap();

    proof_verifier
        .add_sub_proof_request(
            &sub_proof_request,
            &credential_schema,
            &non_credential_schema,
            &credential_pub_key,
            None,
            None,
        )
        .unwrap();
    proof_verifier
        .add_sub_proof_request(
            &w_sub_proof_request,
            &w_credential_schema,
            &w_non_credential_schema,
            &w_credential_pub_key,
            None,
            None,
        )
        .unwrap();

    proof_verifier.verify(&proof, &proof_request_nonce).unwrap();

    let dir = "";
    let data_dir = data_dir(&dir);
    let path = data_dir.to_str().unwrap();
    std::fs::write(
        format!("{}/{}", path, "proof_req_nonce.json"),
        serde_json::to_string_pretty(&proof_request_nonce).unwrap(),
    )
    .unwrap();

    std::fs::write(
        format!("{}/{}", path, "proof.json",),
        serde_json::to_string_pretty(&proof).unwrap(),
    )
    .unwrap();
}
