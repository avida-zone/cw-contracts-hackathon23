#[macro_use]
extern crate rocket;

use std::collections::BTreeMap;
use std::fs;

use log::info;
use rocket::fairing::{Fairing, Info, Kind};
use rocket::http::Header;
use rocket::serde::{json::Json, Serialize};
use rocket::State;
use rocket::{Request, Response};

use ursa::cl::{CredentialPublicKey, SubProofRequest};
use ursa_service::{
    create_issuers, establish_connection, get_issuer, models::Issuer, rg_holder_issuer_set_up,
    schema::issuers::dsl::*,
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

// TODO threads

// Returns sub-proof-req-params from issuers
#[get("/sub-proof-req-params?<issuer>")]
fn get_subproofreq(issuer: Vec<&str>) -> Json<BTreeMap<&str, Option<String>>> {
    let mut connection = establish_connection();
    let mut v = BTreeMap::new();
    for i in issuer {
        let r = get_issuer(&mut connection, i);
        v.insert(i, r);
    }
    Json(v)
}

// Returns self issuer credential pubkey
#[post("/rg-holder-setup/<controller_addr>")]
fn rg_holder_setup(controller_addr: String) -> Json<CredentialPublicKey> {
    let mut connection = establish_connection();
    // self issue credential and store in issuers
    let cred_pubkey = rg_holder_issuer_set_up(&mut connection, controller_addr);
    Json(cred_pubkey)

    // get_issuers, for each, issuer credential and store it
    // return self issue pubkey
}

#[launch]
fn rocket() -> _ {
    let mut connection = establish_connection();

    rocket::build()
        .mount("/", routes![get_subproofreq, rg_holder_setup])
        .attach(CORS)
}
