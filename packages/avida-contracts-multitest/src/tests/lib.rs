use avida_contracts_multitest::testsuite::AvidaTest;

#[test]
fn install_plugin_works() {
    let mut suite = AvidaTest::init();
    let identity_plugin = suite.vectis_account_installs_identity_plugin();
}
