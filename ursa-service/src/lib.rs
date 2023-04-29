pub mod models;
pub mod schema;
use serde::{Deserialize, Serialize};


// ursa
use ursa::bn::BigNumber;
use ursa::cl::issuer::Issuer as UrsaIssuer;
use ursa::cl::prover::Prover;
use ursa::cl::verifier::Verifier;
use ursa::cl::{
    new_nonce, CredentialKeyCorrectnessProof, CredentialPrivateKey, CredentialPublicKey,
    CredentialSchema, NonCredentialSchema, RevocationKeyPublic, RevocationRegistry,
    SubProofRequest,
};

// ursa_demo
use ursa_demo::{
    get_issuer_setup_outputs, issuer_adds_cred_values_and_signs, prover_create_credential_req,
};

// diesel
use diesel::pg::PgConnection;
use diesel::prelude::*;

use dotenvy::dotenv;

// local
use models::{Issuer, NewCredential, NewIssuer};

// env
use std::env;

// avida
#[derive(Serialize, Deserialize)]
pub struct SubProofReqParams {
    pub sub_proof_request: SubProofRequest,
    pub credential_schema: CredentialSchema,
    pub non_credential_schema: NonCredentialSchema,
    pub credential_pub_key: CredentialPublicKey,
    pub rev_key_pub: Option<RevocationKeyPublic>,
    pub rev_reg: Option<RevocationRegistry>,
}

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {database_url}"))
}

pub fn get_creds_dev(conn: &mut PgConnection, query_name: &str) -> Vec<String> {
    use crate::schema::credentials::dsl::{contractaddr, credentials, credvalues};
    credentials
        .filter(contractaddr.eq(query_name))
        .select(credvalues)
        .load(conn)
        .expect("error loading creds")
}

pub fn get_issuer(conn: &mut PgConnection, query_name: &str) -> Option<String> {
    use crate::schema::issuers::dsl::{issuers, name, subproofreqparams};
    issuers
        .filter(name.eq(query_name))
        .select(subproofreqparams)
        .first(conn)
        .ok()
}

pub fn create_issuers_from_files(conn: &mut PgConnection, issuer: &str) -> Vec<Issuer> {
    use crate::schema::issuers;

    // These are pre-generated materials
    let (schema, nonschema, pubkey, privkey, correctness, subproofreq) =
        get_issuer_setup_outputs(issuer.into());

    let sub_proof_request_params = SubProofReqParams {
        sub_proof_request: subproofreq,
        credential_schema: schema,
        non_credential_schema: nonschema,
        credential_pub_key: pubkey,
        rev_reg: None,
        rev_key_pub: None,
    };

    // Serialize not expected to fail
    let new_issuer = NewIssuer {
        name: issuer.into(),
        correctness: serde_json::to_string(&correctness).unwrap(),
        privkey: serde_json::to_string(&privkey).unwrap(),
        subproofreqparams: serde_json::to_string(&sub_proof_request_params).unwrap(),
    };

    let new_issuers = vec![new_issuer];

    diesel::insert_into(issuers::dsl::issuers)
        .values(&new_issuers)
        .get_results(conn)
        .expect("Error saving new post")
}

// We mock the device environmen there
// All of this should be done in the holders own dervice
pub fn rg_holder_issuer_set_up(conn: &mut PgConnection, controller_addr: String) {
    // These are static for the Avida framework
    let non_schema_attrs: Vec<&'static str> = vec!["link_secret"];
    let schema_attrs: Vec<&'static str> = vec!["wallet_addr", "controller_addr"];

    // Credential schema and Non credential schema
    let mut credential_schema_builder = UrsaIssuer::new_credential_schema_builder().unwrap();
    for a in &schema_attrs {
        credential_schema_builder.add_attr(a).unwrap();
    }
    let credential_schema = credential_schema_builder.finalize().unwrap();

    let mut non_credential_schema_builder =
        UrsaIssuer::new_non_credential_schema_builder().unwrap();
    for n in non_schema_attrs {
        non_credential_schema_builder.add_attr(n).unwrap();
    }
    let non_credential_schema = non_credential_schema_builder.finalize().unwrap();

    // Credential definition
    let (credential_pub_key, credential_priv_key, cred_key_correctness_proof) =
        UrsaIssuer::new_credential_def(&credential_schema, &non_credential_schema, false).unwrap();

    let mut sub_proof_request_builder = Verifier::new_sub_proof_request_builder().unwrap();
    for attr in schema_attrs.clone() {
        sub_proof_request_builder.add_revealed_attr(attr).unwrap();
    }
    let sub_proof_request = sub_proof_request_builder.finalize().unwrap();

    let params = SubProofReqParams {
        sub_proof_request,
        credential_schema,
        non_credential_schema,
        credential_pub_key,
        rev_reg: None,
        rev_key_pub: None,
    };

    let issuer = NewIssuer {
        name: controller_addr,
        correctness: serde_json::to_string(&cred_key_correctness_proof).unwrap(),
        privkey: serde_json::to_string(&credential_priv_key).unwrap(),
        subproofreqparams: serde_json::to_string(&params).unwrap(),
    };

    let new_issuers = vec![issuer];

    use crate::schema::issuers;
    diesel::insert_into(issuers::dsl::issuers)
        .values(&new_issuers)
        .execute(conn)
        .expect("Error saving new post");
}

// Create credentials
// mocks the process of issuing credential
pub fn create_credentials(
    conn: &mut PgConnection,
    controller_addr: &str,
    wallet_addr: &str,
    issuers_list: &[&str],
) {
    use crate::schema::issuers::dsl::{issuers, name};
    // we only create credentials once for this demo so we dont store these
    let link_secret = Prover::new_master_secret().unwrap();
    issuers_list.to_vec().push(controller_addr);

    for i in issuers_list.iter() {
        let prover_credential_nonce = new_nonce().unwrap();
        let issuer_issuance_nonce = new_nonce().unwrap();

        // Load issuer info
        let _issuer: Issuer = issuers.filter(name.eq(i)).first(conn).unwrap();
        let issuer_params: SubProofReqParams =
            serde_json::from_str(&_issuer.subproofreqparams).unwrap();

        let issuer_correctness_proof: CredentialKeyCorrectnessProof =
            serde_json::from_str(&_issuer.correctness).unwrap();

        let issuer_credential_priv_key: CredentialPrivateKey =
            serde_json::from_str(&_issuer.privkey).unwrap();

        // Mock credential values
        let mut attr_values = vec![];
        let schema_attrs: Vec<String> = issuer_params
            .credential_schema
            .attrs
            .clone()
            .into_iter()
            .collect();
        for attr in &schema_attrs {
            let attr_string = format!("example-{attr}");
            let attr = BigNumber::from_bytes(attr_string.as_bytes())
                .unwrap()
                .to_dec()
                .unwrap();
            attr_values.push(attr);
        }

        // Holder setup cred req
        let (
            blinded_credential_secrets,
            credential_secrets_blinding_factors, // This is a secret and should not be shared
            blinded_credential_secrets_correctness_proof,
        ) = prover_create_credential_req(
            vec!["link_secret"],
            vec![link_secret.value().unwrap()],
            issuer_params.credential_pub_key.try_clone().unwrap(),
            issuer_correctness_proof,
            prover_credential_nonce.try_clone().unwrap(),
        );

        // Issuer issues credential
        let (mut cred_sig, cred_sig_proof) = issuer_adds_cred_values_and_signs(
            _issuer.name.clone(),
            blinded_credential_secrets,
            blinded_credential_secrets_correctness_proof,
            prover_credential_nonce,
            issuer_issuance_nonce.try_clone().unwrap(),
            schema_attrs.clone(),
            attr_values.clone(),
            issuer_params.credential_pub_key.try_clone().unwrap(),
            &issuer_credential_priv_key,
        );

        // Prover accepts the credential and "unblind" it
        // Now the credential_values have both hidden / known attributes
        let mut cred_values_builder = UrsaIssuer::new_credential_values_builder().unwrap();
        for (idx, attr) in schema_attrs.iter().enumerate() {
            cred_values_builder
                .add_dec_known(attr, &attr_values[idx])
                .unwrap();
        }
        // There is only one hidden value
        cred_values_builder
            .add_value_hidden("link_secret", &link_secret.value().unwrap())
            .unwrap();
        let credential_values = cred_values_builder.finalize().unwrap();

        Prover::process_credential_signature(
            &mut cred_sig,
            &credential_values,
            &cred_sig_proof,
            &credential_secrets_blinding_factors,
            &issuer_params.credential_pub_key,
            &issuer_issuance_nonce,
            None,
            None,
            None,
        )
        .unwrap();

        let new_cred = NewCredential {
            contractaddr: controller_addr.into(),
            issuer: _issuer.name,
            walletaddr: wallet_addr.into(),
            credsig: serde_json::to_string(&cred_sig).unwrap(),
            credvalues: serde_json::to_string(&credential_values).unwrap(),
        };

        use crate::schema::credentials;
        diesel::insert_into(credentials::dsl::credentials)
            .values(&new_cred)
            .execute(conn)
            .expect("Error saving new credential");
    }
}
