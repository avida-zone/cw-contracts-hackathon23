import path from "path";

const contractPaths = path.join(__dirname, "../artifacts");

export const codePaths: { [index: string]: string } = {
  avidaIdentityPluginCodePath: path.join(
    contractPaths,
    "avida_identity_plugin.wasm"
  ),
  adapterCodePath: path.join(contractPaths, "cw20_adapter.wasm"),
  rgCw20CodePath: path.join(contractPaths, "rg_cw20.wasm"),
  vcVerifierCodePath: path.join(contractPaths, "vc_verifier.wasm"),
  avidaLaunchpadCodePath: path.join(contractPaths, "avida_launchpad.wasm"),
};
