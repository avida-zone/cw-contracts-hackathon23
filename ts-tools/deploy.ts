import {
  MsgBroadcasterWithPk,
  MsgInstantiateContract,
  MsgExecuteContract,
  PrivateKey,
} from "@injectivelabs/sdk-ts";
import { getNetworkEndpoints, Network } from "@injectivelabs/networks";
import { accounts } from "./accounts";
import {
  writeToFile,
  extractValueFromEvent,
  getSubProofReq,
  getNonCredentialSchema,
  getCredentialSchema,
  ContractsInterface,
} from "./utils";
import { InstantiateMsg as VcVerifierInstMsg } from "./interfaces/VcVerifier.types";
import { InstantiateMsg as AdapterInstMsg } from "./interfaces/Cw20Adapter.types";
import { ExecuteMsg as LaunchpadExecMsg } from "./interfaces/Launchpad.types";

interface CodeIds {
  avidaIdentityPluginCodeId: number;
  rgCw20CodeId: number;
  vcVerifierCodeId: number;
  avidaLaunchpadCodeId: number;
  adapterCodeId: number;
}

(async function deploy() {
  const { admin } = accounts;
  const endpoints = getNetworkEndpoints(Network.TestnetK8s);

  console.log("deploying to: ", endpoints);

  const {
    avidaLaunchpadCodeId,
    rgCw20CodeId,
    vcVerifierCodeId,
    adapterCodeId,
  } = (await import("./deploy/injective-testnet-uploadInfo.json")) as CodeIds;

  const adminClient = new MsgBroadcasterWithPk({
    privateKey: PrivateKey.fromMnemonic(admin.mnemonic),
    network: Network.Testnet,
    endpoints,
    simulateTx: true,
  });

  // ========================================
  //
  //  Instntiate launchpad
  //
  //  ========================================
  let msg = MsgInstantiateContract.fromJSON({
    sender: admin.address,
    codeId: avidaLaunchpadCodeId,
    label: "AVIPAD",
    admin: admin.address,
    msg: { rg_cw20_code_id: rgCw20CodeId },
  });

  let txResponse = await adminClient.broadcast({
    msgs: msg,
    injectiveAddress: admin.address,
  });

  const launchpadAddr = extractValueFromEvent(
    txResponse.rawLog,
    "cosmwasm.wasm.v1.EventContractInstantiated",
    "contract_address"
  );
  console.log("1. Instantiated launchpad Addr: ", launchpadAddr);

  /// ========================================
  //
  //  Instntiate vc-verifier
  //
  //  ========================================
  //  dev:
  //  These are static for vc verifier
  let vcVerifierInsMsg: VcVerifierInstMsg = {
    vectis_sub_proof_request: getSubProofReq(
      "./registry_info/wallet_sub_proof_request.json"
    ),
    vectis_cred_schema: getCredentialSchema(
      "./registry_info/wallet_credential_schema.json"
    ),
    vectis_non_cred_schema: getNonCredentialSchema(
      "./registry_info/wallet_non_credential_schema.json"
    ),
    launchpad: launchpadAddr,
  };

  let verifierMsg = MsgInstantiateContract.fromJSON({
    sender: admin.address,
    codeId: vcVerifierCodeId,
    label: "AVIDA VC Verifier",
    admin: admin.address,
    msg: vcVerifierInsMsg,
  });

  txResponse = await adminClient.broadcast({
    msgs: verifierMsg,
    injectiveAddress: admin.address,
  });

  const vcVerifierAddr = extractValueFromEvent(
    txResponse.rawLog,
    "cosmwasm.wasm.v1.EventContractInstantiated",
    "contract_address"
  );
  console.log("2. Instantiated VcVerifier Addr: ", vcVerifierAddr);

  // =========================================
  //
  // Instantiate adapter
  //
  // =========================================
  //
  let adapterInstMsg: AdapterInstMsg = {
    launchpad: launchpadAddr,
  };

  let adapterMsg = MsgInstantiateContract.fromJSON({
    sender: admin.address,
    codeId: adapterCodeId,
    label: "AVIDA rgToken Adapter",
    admin: admin.address,
    msg: adapterInstMsg,
  });

  txResponse = await adminClient.broadcast({
    msgs: adapterMsg,
    injectiveAddress: admin.address,
  });

  const adapterAddr = extractValueFromEvent(
    txResponse.rawLog,
    "cosmwasm.wasm.v1.EventContractInstantiated",
    "contract_address"
  );
  console.log("3. Instantiated Adapter Addr: ", adapterAddr);

  const contracts: ContractsInterface = {
    launchpad: launchpadAddr,
    vcverifier: vcVerifierAddr,
    adapter: adapterAddr,
  };

  writeToFile(
    `./deploy/injective-testnet-deployInfo.json`,
    JSON.stringify(contracts, null, 2)
  );

  //===========================================
  //
  // UPDATE launchpad with verifier and adapter
  //
  // ==========================================

  //const {
  //  launchpad: launchpadAddr,
  //  vcverifier: vcVerifierAddr,
  //  adapter: adapterAddr,
  //} = (await import(
  //  "./deploy/injective-testnet-deployInfo.json"
  //)) as ContractsInterface;

  let update_verifier_msg: LaunchpadExecMsg = {
    update_verifier: {
      address: vcVerifierAddr,
    },
  };

  let update = MsgExecuteContract.fromJSON({
    contractAddress: launchpadAddr,
    sender: admin.address,
    msg: update_verifier_msg,
  });

  let res = await adminClient.broadcast({
    msgs: update,
    injectiveAddress: admin.address,
  });

  console.log("UPDATE VERFIER: \n ", res);

  let update_adapter_msg: LaunchpadExecMsg = {
    update_adapter: {
      address: adapterAddr,
    },
  };

  update = MsgExecuteContract.fromJSON({
    contractAddress: launchpadAddr,
    sender: admin.address,
    msg: update_adapter_msg,
  });

  res = await adminClient.broadcast({
    msgs: update,
    injectiveAddress: admin.address,
  });

  console.log("UPDATE ADAPTER: \n ", res);

  console.log(res);
})();
