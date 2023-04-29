#[macro_use]
extern crate rocket;

use std::collections::BTreeMap;

use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};

use rocket::fairing::{Fairing, Info, Kind};
use rocket::http::Header;
use rocket::serde::json::Json;

use rocket::{Request, Response};

use ursa_service::{
    create_credentials, create_issuers_from_files, establish_connection, get_creds_dev, get_issuer,
    rg_holder_issuer_set_up,
};

pub struct CORS;
// pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!("../../migrations/postgresql");

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
fn get_subproofreqparams(issuer: Vec<&str>) -> Json<BTreeMap<&str, Option<String>>> {
    let mut connection = establish_connection();
    let mut v = BTreeMap::new();
    for i in issuer {
        let r = get_issuer(&mut connection, i);
        v.insert(i, r);
    }
    Json(v)
}

// DEV ONLY
// Returns sub-proof-req-params from issuers
#[get("/creds/<controller_addr>")]
fn get_creds(controller_addr: String) -> Json<Vec<String>> {
    let mut connection = establish_connection();
    let r = get_creds_dev(&mut connection, &controller_addr);
    Json(r)
}

// Returns self issuer credential pubkey
#[post("/rg-holder-setup/<controller_addr>/<wallet_addr>")]
fn rg_holder_setup(controller_addr: String, wallet_addr: String) -> Json<String> {
    let mut connection = establish_connection();
    // self issue credential and store in issuers
    rg_holder_issuer_set_up(&mut connection, controller_addr.clone());

    let issuers_list = &["gayadeed", "infocert", "identrust"];
    create_credentials(
        &mut connection,
        &controller_addr,
        &wallet_addr,
        issuers_list,
    );

    let r = get_issuer(&mut connection, &controller_addr).unwrap();
    Json(r)
}

#[launch]
fn rocket() -> _ {
    let mut connection = establish_connection();
    let setup_issuers = ["gayadeed", "infocert", "identrust"];
    for i in setup_issuers {
        create_issuers_from_files(&mut connection, i);
    }

    rocket::build()
        .mount(
            "/",
            routes![get_subproofreqparams, rg_holder_setup, get_creds],
        )
        .attach(CORS)
}
