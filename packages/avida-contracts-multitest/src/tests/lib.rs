use anoncreds_identity_plugin::contract::InstantiateMsg as PluginInstMsg;
use avida_contracts_multitest::testsuite::AvidaTest;
use avida_verifier::types::PLUGIN_QUERY_KEY;
use cw_multi_test::App;

//fn vectis_account_installs_plugin(app: App) {
//    suite
//        .hub
//        .app
//        .execute_contract(
//            suite.hub.controller.clone(),
//            suite.proxy.clone(),
//            &ProxyExecuteMsg::<Empty>::InstantiatePlugin {
//                src: PluginSource::VectisRegistry(plugin_id),
//                instantiate_msg: to_binary(&PluginInstMsg { cred_def }).unwrap(),
//                plugin_params: PluginParams {
//                    permissions: vec![PluginPermissions::Query(PLUGIN_QUERY_KEY)],
//                },
//                label: "avida-identity-plugin".into(),
//            },
//            &[coin(INSTALL_FEE + 0u128, DENOM)],
//        )
//        .unwrap();
//}

#[test]
fn install_plugin_works() {
    let suite = AvidaTest::init();
}
