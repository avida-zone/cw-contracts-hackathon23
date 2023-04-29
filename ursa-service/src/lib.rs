pub mod models;
pub mod schema;
use serde::{Deserialize, Serialize};
use serde_json;

// ursa_demo
use ursa::cl::issuer::Issuer as UrsaIssuer;
use ursa::cl::prover::Prover;
use ursa::cl::verifier::Verifier;
use ursa::cl::{
    BlindedCredentialSecrets, BlindedCredentialSecretsCorrectnessProof,
    CredentialKeyCorrectnessProof, CredentialPrivateKey, CredentialPublicKey, CredentialSchema,
    CredentialSecretsBlindingFactors, CredentialSignature, NonCredentialSchema, Nonce,
    RevocationKeyPublic, RevocationRegistry, SignatureCorrectnessProof,
};
use ursa::{bn::BigNumber, cl::SubProofRequest};
use ursa_demo::get_issuer_setup_outputs;

// diesel
use diesel::pg::PgConnection;
use diesel::prelude::*;
use diesel::result::Error;
use dotenvy::dotenv;

// local
use models::{Issuer, NewIssuer};

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
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

pub fn get_issuers(conn: &mut PgConnection) -> Vec<Issuer> {
    use crate::schema::issuers::dsl::*;
    let results = issuers
        .limit(10)
        .load::<Issuer>(conn)
        .expect("Error loading posts");
    results
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
        credential_schema_builder.add_attr(&a).unwrap();
    }
    let credential_schema = credential_schema_builder.finalize().unwrap();

    let mut non_credential_schema_builder =
        UrsaIssuer::new_non_credential_schema_builder().unwrap();
    for n in non_schema_attrs {
        non_credential_schema_builder.add_attr(&n).unwrap();
    }
    let non_credential_schema = non_credential_schema_builder.finalize().unwrap();

    // Credential definition
    let (credential_pub_key, credential_priv_key, cred_key_correctness_proof) =
        UrsaIssuer::new_credential_def(&credential_schema, &non_credential_schema, false).unwrap();

    let mut sub_proof_request_builder = Verifier::new_sub_proof_request_builder().unwrap();
    for attr in schema_attrs.clone() {
        sub_proof_request_builder.add_revealed_attr(&attr).unwrap();
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
