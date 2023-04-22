use cw_multi_test::{App, AppResponse, Contract, ContractWrapper, Executor};
use vc_verifier::{
    contract::{
        execute as vc_verifier_execute, instantiate as vc_verifier_instantiate,
        query as vc_verifier_query,
    },
    msg::InstantiateMsg as VcVerifierInstMsg,
};

use anoncreds_identity_plugin::contract::{
    execute as plugin_execute, instantiate as plugin_instantiate, query as plugin_query,
};
use avida_verifier::types::{WSubProofReq, WSubProofReqParams};
use cosmwasm_std::{Addr, Empty};
use serde;
use serde_json;
use std::{
    env, fs,
    path::{Path, PathBuf},
};
use vectis_contract_tests::common::common::{proxy_exec, DENOM, INSTALL_FEE, REGISTRY_FEE};
use vectis_contract_tests::common::plugins_common::PluginsSuite;

const ISSUER: &str = "Issuer";

pub fn contract_vc_verifier() -> Box<dyn Contract<Empty>> {
    let contract = ContractWrapper::new(
        vc_verifier_execute,
        vc_verifier_instantiate,
        vc_verifier_query,
    );
    Box::new(contract)
}

pub fn contract_identity_plugin() -> Box<dyn Contract<Empty>> {
    let contract = ContractWrapper::new(plugin_execute, plugin_instantiate, plugin_query);
    Box::new(contract)
}

pub struct AvidaTest {
    pub vectis_test_suite: PluginsSuite,
    pub vc_verifier: Addr,
    pub identity_plugin_id: u64,
    //rg_cw20: Addr,
}

fn load_verifier_init_data() -> (WSubProofReq) {
    let data_path = workspace_dir().join("data/");
    println!("data path: {:?}", fs::canonicalize(data_path.clone()));
    let req_params_str =
		// CARGO_MANIFEST_DIR is 
        fs::read_to_string(&data_path).unwrap();

    let req_params: WSubProofReq = serde_json::from_str(&req_params_str).unwrap();
    req_params
}

impl AvidaTest {
    pub fn init() -> Option<Self> {
        let param = load_verifier_init_data();
        println!("parmas {:?}", param);
        None
        // TODO: params required

        //pub req_params: Vec<WSubProofReqParams>,
        //pub wallet_cred_schema: WCredentialSchema,
        //pub wallet_non_cred_schema: WNonCredentialSchema,
        //pub wallet_sub_proof_request: WSubProofReq,

        //       let vc_verifier_inst_msg = VcVerifierInstMsg {
        //           req_params,
        //           wallet_cred_schema,
        //           wallet_non_cred_schema,
        //           wallet_sub_proof_request,
        //       };
        //       let vectis_test_suite = PluginsSuite::init().unwrap();
        //       let code_id = vectis_test_suite.hub.app.store_code(contract_vc_verifier());
        //       let identity_plugin_code_id = vectis_test_suite
        //           .hub
        //           .app
        //           .store_code(contract_identity_plugin());
        //       let vc_verifier = vectis_test_suite
        //           .hub
        //           .app
        //           .instantiate_contract(
        //               code_id,
        //               Addr::unchecked(ISSUER),
        //               &vc_verifier_inst_msg,
        //               &[],
        //               "Anoncreds RG Token verifier",
        //               None,
        //           )
        //           .unwrap();

        //       // Register the identity plugin to the plugin registry
        //       suite
        //           .hub
        //           .app
        //           .execute_contract(
        //               suite.hub.plugin_committee.clone(),
        //               suite.hub.plugin_registry.clone(),
        //               &RegistryExecMsg::RegisterPlugin {
        //                   name: "Identity plugin".into(),
        //                   creator: suite.hub.deployer.to_string(),
        //                   ipfs_hash: "some-hash".into(),
        //                   version: "1.0".to_string(),
        //                   code_id: identity_plugin_code_id,
        //                   checksum: "some-checksum".to_string(),
        //               },
        //               &[coin(REGISTRY_FEE, DENOM)],
        //           )
        //           .unwrap();

        //       let plugines = suite.query_plugins(None, None).unwrap();
        //       let identity_plugin_id = plugines.total;

        //Self {
        //    vectis_test_suite,
        //    vc_verifier,
        //    identity_plugin_id,
        //}
    }
}
