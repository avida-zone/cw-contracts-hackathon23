use avida_contracts_multitest::testsuite::AvidaTest;

#[test]
fn install_plugin_works() {
    let mut suite = AvidaTest::init();
    let identity_plugin = suite.vectis_account_installs_identity_plugin();
    let plugin_in_vectis = suite
        .vectis
        .query_installed_plugins()
        .unwrap()
        .query_plugins[0]
        .clone();
    println!("vectis account, {} ", suite.vectis.proxy);
    println!("vectis controller,{} ", suite.vectis.hub.controller);
    assert_eq!(identity_plugin, plugin_in_vectis);
}
