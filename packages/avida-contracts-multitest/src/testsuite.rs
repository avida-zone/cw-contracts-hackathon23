use crate::helper::{get_issuer_setup_outputs, get_proof};
use avida_identity_plugin::contract::{
    execute as plugin_execute, instantiate as plugin_instantiate, query as plugin_query,
    InstantiateMsg as PluginInstMsg,
};
use avida_verifier::{
    msg::{
        launchpad as launchpadMsg,
        vc_verifier::{ExecuteMsg as VcVerifierExecMsg, InstantiateMsg as VcVerifierInstMsg},
    },
    types::{WSubProofReq, WSubProofReqParams, PLUGIN_QUERY_KEY},
};
use cosmwasm_std::{coin, to_binary, Addr, CosmosMsg, Empty, WasmMsg};
use cw_multi_test::{App, AppResponse, Contract, ContractWrapper, Executor};
use rg_cw20::contract::{
    execute as rg_execute, instantiate as rg_instantiate, query as rg_query, reply as rg_reply,
};
use serde;
use serde_json;
use std::convert::TryInto;
use vc_verifier::contract::{
    execute as vc_verifier_execute, instantiate as vc_verifier_instantiate,
    query as vc_verifier_query,
};

use vectis_contract_tests::common::common::{
    proxy_exec, PRegistryExecMsg, DENOM, INSTALL_FEE, REGISTRY_FEE,
};
use vectis_contract_tests::common::plugins_common::PluginsSuite;
use vectis_wallet::{PluginParams, PluginPermissions, PluginSource, ProxyExecuteMsg};

const ISSUER: &str = "Issuer";

pub fn contract_rg() -> Box<dyn Contract<Empty>> {
    let contract = ContractWrapper::new(rg_execute, rg_instantiate, rg_query).with_reply(rg_reply);
    Box::new(contract)
}

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
    pub vectis: PluginsSuite,
    pub vc_verifier: Addr,
    pub identity_plugin_id: u64,
    //rg_cw20: Addr,
}

pub fn load_verifier_init_data(issuer: &str) -> WSubProofReqParams {
    let (credential_schema, non_credential_schema, credential_pub_key, sub_proof_request) =
        get_issuer_setup_outputs(issuer);

    WSubProofReqParams {
        sub_proof_request,
        credential_schema,
        non_credential_schema,
        credential_pub_key,
        rev_key_pub: None,
        rev_reg: None,
    }
}

impl AvidaTest {
    pub fn init() -> Self {
        let wallet_param = load_verifier_init_data("trusted_issuer");

        let vc_verifier_inst_msg = VcVerifierInstMsg {
            launchpad: Addr::unchecked("launchpad"),
            vectis_cred_schema: wallet_param.credential_schema,
            vectis_non_cred_schema: wallet_param.non_credential_schema,
            vectis_sub_proof_request: wallet_param.sub_proof_request,
        };

        let mut vectis = PluginsSuite::init().unwrap();
        let vc_verifier_code_id = vectis.hub.app.store_code(contract_vc_verifier());
        let identity_plugin_code_id = vectis.hub.app.store_code(contract_identity_plugin());

        // instantiates vc_verifier
        let vc_verifier = vectis
            .hub
            .app
            .instantiate_contract(
                vc_verifier_code_id,
                Addr::unchecked(ISSUER),
                &vc_verifier_inst_msg,
                &[],
                "Anoncreds RG Token verifier",
                None,
            )
            .unwrap();

        // Register the identity plugin to the plugin registry
        vectis
            .hub
            .app
            .execute_contract(
                vectis.hub.plugin_committee.clone(),
                vectis.hub.plugin_registry.clone(),
                &PRegistryExecMsg::RegisterPlugin {
                    name: "Identity plugin".into(),
                    creator: vectis.hub.deployer.to_string(),
                    ipfs_hash: "some-hash".into(),
                    version: "1.0".to_string(),
                    code_id: identity_plugin_code_id,
                    checksum: "some-checksum".to_string(),
                },
                &[coin(REGISTRY_FEE, DENOM)],
            )
            .unwrap();

        let plugines = vectis.query_plugins(None, None).unwrap();
        let identity_plugin_id = plugines.total;

        Self {
            vectis,
            vc_verifier,
            identity_plugin_id,
        }
    }

    pub fn vectis_account_installs_identity_plugin(&mut self) -> Addr {
        let wallet = load_verifier_init_data("self_issued");
        self.vectis
            .hub
            .app
            .execute_contract(
                self.vectis.hub.controller.clone(),
                self.vectis.proxy.clone(),
                &ProxyExecuteMsg::<Empty>::InstantiatePlugin {
                    src: PluginSource::VectisRegistry(self.identity_plugin_id),
                    instantiate_msg: to_binary(&PluginInstMsg {
                        cred_def: wallet.credential_pub_key,
                    })
                    .unwrap(),
                    plugin_params: PluginParams {
                        permissions: vec![PluginPermissions::Query(PLUGIN_QUERY_KEY.into())],
                    },
                    label: "avida-identity-plugin".into(),
                },
                &[coin(INSTALL_FEE + 0u128, DENOM)],
            )
            .unwrap();

        self.vectis.query_installed_plugins().unwrap().query_plugins[0].clone()
    }

    // TODO: impl TryFrom<Proof> for WProof
    pub fn vectis_verifier_verifies(&mut self) {
        let (proof, proof_req_nonce) = get_proof();
        self.vectis
            .hub
            .proxy_execute(
                &self.vectis.proxy,
                vec![CosmosMsg::Wasm(WasmMsg::Execute {
                    contract_addr: self.vc_verifier.to_string(),
                    msg: to_binary(&VcVerifierExecMsg::Verify {
                        proof,
                        proof_req_nonce,
                        wallet_addr: self.vectis.proxy.clone(),
                    })
                    .unwrap(),
                    funds: vec![],
                })],
                vec![],
            )
            .unwrap();
    }
}
