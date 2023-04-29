use crate::schema::{credentials, issuers};
use diesel::prelude::*;

#[derive(Queryable, Debug)]
pub struct Issuer {
    pub id: i32,
    pub name: String,
    pub correnctness: String,
    pub privkey: String,
    pub pubkey: String,
    pub schema: String,
    pub nonschema: String,
    pub subproofreq: String,
}

#[derive(Insertable)]
#[diesel(table_name = issuers)]
pub struct NewIssuer {
    pub name: String,
    pub correctness: String,
    pub privkey: String,
    pub pubkey: String,
    pub schema: String,
    pub nonschema: String,
    pub subproofreq: String,
}

#[derive(Queryable, Debug)]
pub struct Credential {
    pub id: i32,
    pub contractaddr: String,
    pub issuer: String,
    pub walletaddr: String,
    pub credsig: String,
    pub credvalues: String,
}

#[derive(Insertable)]
#[diesel(table_name = credentials)]
pub struct NewCredential {
    pub contractaddr: String,
    pub issuer: String,
    pub walletaddr: String,
    pub credsig: String,
    pub credvalues: String,
}