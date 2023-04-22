#[macro_use]
extern crate rocket;

use std::fs;

use log::info;
use rocket::fairing::{Fairing, Info, Kind};
use rocket::http::Header;
use rocket::serde::{json::Json, Serialize};
use rocket::State;
use rocket::{Request, Response};

use serde_json;
use ursa::bn::BigNumber;
use ursa::cl::issuer::Issuer;
use ursa::cl::prover::Prover;
use ursa::cl::verifier::Verifier;
use ursa::cl::{
    new_nonce, BlindedCredentialSecrets, BlindedCredentialSecretsCorrectnessProof,
    CredentialKeyCorrectnessProof, CredentialPrivateKey, CredentialPublicKey, CredentialSchema,
    CredentialSecretsBlindingFactors, CredentialSignature, CredentialValues, MasterSecret,
    NonCredentialSchema, Nonce, Proof, SignatureCorrectnessProof,
};
use ursa_demo::{
    data_dir, get_issuer_setup_outputs, issuer_adds_cred_values_and_signs,
    prover_create_credential_req,
};

pub struct CORS;

#[rocket::async_trait]
impl Fairing for CORS {
    fn info(&self) -> Info {
        Info {
            name: "Add CORS headers to responses",
            kind: Kind::Response,
        }
    }

    async fn on_response<'r>(&self, _request: &'r Request<'_>, response: &mut Response<'r>) {
        response.set_header(Header::new("Access-Control-Allow-Origin", "*"));
        response.set_header(Header::new("Access-Control-Allow-Methods", "POST, GET"));
        response.set_header(Header::new("Access-Control-Allow-Headers", "*"));
        response.set_header(Header::new("Access-Control-Allow-Credentials", "true"));
    }
}

const schema_attrs: &'static [&str] = &["name", "age"];
const attr_values: &'static [&str] = &[
    "5683357908103347059525661567944931380213200296242705847943670736985289748650",
    "998878473220768490044181134091226568472324611129842329272337785551474",
];
const non_schema_attrs: &'static [&str] = &["link_secret"];
const w_schema_attrs: &'static [&str] = &["wallet_addr", "controller_addr"];

pub struct States {
    pub linksecret: MasterSecret,
    pub credential_pub_key: CredentialPublicKey,
    pub credential_priv_key: CredentialPrivateKey,
    pub credential_schema: CredentialSchema,
    pub non_cred_schema: NonCredentialSchema,
    pub credential_key_correctness_proof: CredentialKeyCorrectnessProof,
    pub w_credential_pub_key: CredentialPublicKey,
    pub w_credential_priv_key: CredentialPrivateKey,
    pub w_credential_schema: CredentialSchema,
    pub w_non_cred_schema: NonCredentialSchema,
    pub w_credential_key_correctness_proof: CredentialKeyCorrectnessProof,
    pub prover_credential_nonce: Nonce,
    pub proof_req_nonce: Nonce,
    pub issuer_issuance_nonce: Nonce,
    pub w_issuer_issuance_nonce: Nonce,
}

#[get("/definitions/<issuer>")]
fn credential_definitions(issuer: &str) -> Option<Json<CredentialPublicKey>> {
    match issuer {
        "kyc" => {
            let (_, _, credential_pub_key, _, _) = get_issuer_setup_outputs("issuer".into());

            Some(Json(credential_pub_key))
        }
        "wallet" => {
            let (_, _, w_credential_pub_key, _, _) = get_issuer_setup_outputs("wallet".into());
            Some(Json(w_credential_pub_key))
        }
        _ => None,
    }
}

#[get("/credential/<issuer>/<wallet>/<user>")]
fn cred_req(
    issuer: &str,
    wallet: &str,
    user: &str,
    states: &State<States>,
    // returns (blinded secret, unblinded cred sig, cred values )
) -> String {
    let link_secret = states.linksecret.value().unwrap();
    let mut wallet_cred_values: Vec<&str> = Vec::new();
    let mut wallet_addr_num = String::new();
    let mut user_addr_num = String::new();

    let (cred_pubkey, cred_privkey, correctness, nonce, attrs, cred_values) = match issuer {
        "kyc" => (
            states.credential_pub_key.try_clone().unwrap(),
            &states.credential_priv_key,
            states.credential_key_correctness_proof.try_clone().unwrap(),
            states.prover_credential_nonce.try_clone().unwrap(),
            schema_attrs,
            attr_values,
        ),
        "wallet" => {
            let wallet_addr = wallet.as_bytes();
            wallet_addr_num = BigNumber::from_bytes(wallet_addr)
                .unwrap()
                .to_dec()
                .unwrap();
            let user_addr = user.as_bytes();
            user_addr_num = BigNumber::from_bytes(user_addr).unwrap().to_dec().unwrap();
            wallet_cred_values = vec![&wallet_addr_num, &user_addr_num];
            (
                states.w_credential_pub_key.try_clone().unwrap(),
                &states.w_credential_priv_key,
                states
                    .w_credential_key_correctness_proof
                    .try_clone()
                    .unwrap(),
                states.prover_credential_nonce.try_clone().unwrap(),
                w_schema_attrs,
                wallet_cred_values.as_slice(),
            )
        }
        _ => panic!("no such issuer"),
    };

    let (
        blinded_credential_secrets,
        credential_secrets_blinding_factors, // This is a secret and should not be shared
        blinded_credential_secrets_correctness_proof,
    ) = prover_create_credential_req(
        non_schema_attrs.to_vec(),
        vec![link_secret.try_clone().unwrap()],
        cred_pubkey.try_clone().unwrap(),
        correctness,
        nonce,
    );

    let issuer_issuance_nonce = states.issuer_issuance_nonce.try_clone().unwrap();
    let (mut cred_sig, cred_sig_proof) = issuer_adds_cred_values_and_signs(
        "some-did".into(),
        blinded_credential_secrets.try_clone().unwrap(),
        blinded_credential_secrets_correctness_proof,
        states.prover_credential_nonce.try_clone().unwrap(),
        issuer_issuance_nonce.try_clone().unwrap(),
        attrs.to_vec(),
        cred_values.to_vec(),
        cred_pubkey.try_clone().unwrap(),
        cred_privkey,
    );

    // Prover accepts the credential and "unblind" it
    // Now the credential_values have both hidden / known attributes
    let mut cred_values_builder = Issuer::new_credential_values_builder().unwrap();
    for (idx, attr) in attrs.iter().enumerate() {
        cred_values_builder
            .add_dec_known(attr, cred_values[idx])
            .unwrap();
    }
    // There is only one hidden value
    cred_values_builder
        .add_value_hidden(non_schema_attrs[0], &link_secret)
        .unwrap();
    let credential_values = cred_values_builder.finalize().unwrap();

    Prover::process_credential_signature(
        &mut cred_sig,
        &credential_values,
        &cred_sig_proof,
        &credential_secrets_blinding_factors,
        &cred_pubkey,
        &issuer_issuance_nonce,
        None,
        None,
        None,
    )
    .unwrap();

    let data_dir = data_dir(&issuer);
    let path = data_dir.to_str().unwrap();

    std::fs::write(
        format!("{}/credential_sig.json", path),
        serde_json::to_string_pretty(&cred_sig).unwrap(),
    )
    .unwrap();

    std::fs::write(
        format!("{}/credential_values.json", path),
        format!("{}_credential_values.json", issuer),
        serde_json::to_string_pretty(&credential_values).unwrap(),
    )
    .unwrap();

    serde_json::to_string_pretty(&blinded_credential_secrets).unwrap()
}

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
struct ProofData {
    proof: Proof,
    nonce: Nonce,
}

#[get("/generateproof")]
fn gen_proof(states: &State<States>) -> Json<ProofData> {
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

    let issuer_sig_json =
        fs::read_to_string("./kyc_credential_sig.json").expect("Unable to read file");
    let issuer_sig: CredentialSignature =
        serde_json::from_str(&issuer_sig_json).expect("Unable to parse");

    let issuer_v_json =
        fs::read_to_string("./kyc_credential_values.json").expect("Unable to read file");
    let issuer_v: CredentialValues = serde_json::from_str(&issuer_v_json).expect("Unable to parse");

    let wallet_sig_json =
        fs::read_to_string("./wallet_credential_sig.json").expect("Unable to read file");
    let wallet_sig: CredentialSignature =
        serde_json::from_str(&wallet_sig_json).expect("Unable to parse");

    let wallet_v_json =
        fs::read_to_string("./wallet_credential_values.json").expect("Unable to read file");
    let wallet_v: CredentialValues = serde_json::from_str(&wallet_v_json).expect("Unable to parse");

    // Prover provide proof
    // - add the sub proof request from an issuer
    // - add the sub proof request from the self issued credential
    let mut proof_builder = Prover::new_proof_builder().unwrap();
    proof_builder.add_common_attribute("link_secret").unwrap();

    proof_builder
        .add_sub_proof_request(
            &sub_proof_request,
            &states.credential_schema,
            &states.non_cred_schema,
            &issuer_sig,
            &issuer_v,
            &states.credential_pub_key,
            None,
            None,
        )
        .unwrap();

    proof_builder
        .add_sub_proof_request(
            &w_sub_proof_request,
            &states.w_credential_schema,
            &states.w_non_cred_schema,
            &wallet_sig,
            &wallet_v,
            &states.w_credential_pub_key,
            None,
            None,
        )
        .unwrap();

    let nonce = states.proof_req_nonce.try_clone().unwrap();
    let proof = proof_builder.finalize(&nonce).unwrap();

    // To verify
    let mut proof_verifier = Verifier::new_proof_verifier().unwrap();

    proof_verifier
        .add_sub_proof_request(
            &sub_proof_request,
            &states.credential_schema,
            &states.non_cred_schema,
            &states.credential_pub_key,
            None,
            None,
        )
        .unwrap();
    proof_verifier
        .add_sub_proof_request(
            &w_sub_proof_request,
            &states.w_credential_schema,
            &states.w_non_cred_schema,
            &states.w_credential_pub_key,
            None,
            None,
        )
        .unwrap();

    let verified = proof_verifier.verify(&proof, &nonce).unwrap();

    print!("verified: {}", verified);

    Json(ProofData { proof, nonce })
}

#[launch]
fn rocket() -> _ {
    let (
        credential_schema,
        non_cred_schema,
        credential_pub_key,
        credential_priv_key,
        credential_key_correctness_proof,
    ) = get_issuer_setup_outputs("issuer".into());
    let (
        w_credential_schema,
        w_non_cred_schema,
        w_credential_pub_key,
        w_credential_priv_key,
        w_credential_key_correctness_proof,
    ) = get_issuer_setup_outputs("wallet".into());

    let prover_credential_nonce = new_nonce().unwrap();
    let proof_req_nonce = new_nonce().unwrap();
    let issuer_issuance_nonce = new_nonce().unwrap();
    let w_issuer_issuance_nonce = new_nonce().unwrap();

    let states = States {
        linksecret: Prover::new_master_secret().unwrap(),
        credential_schema,
        non_cred_schema,
        credential_pub_key,
        credential_priv_key,
        credential_key_correctness_proof,
        w_credential_schema,
        w_non_cred_schema,
        w_credential_pub_key,
        w_credential_priv_key,
        w_credential_key_correctness_proof,
        prover_credential_nonce,
        issuer_issuance_nonce,
        w_issuer_issuance_nonce,
        proof_req_nonce,
    };

    rocket::build()
        .mount("/", routes![credential_definitions, cred_req, gen_proof])
        .attach(CORS)
}
