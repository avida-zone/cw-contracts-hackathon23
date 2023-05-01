import {
  MsgBroadcasterWithPk,
  MsgExecuteContract,
  PrivateKey,
} from "@injectivelabs/sdk-ts";
import { getNetworkEndpoints, Network } from "@injectivelabs/networks";
import { accounts } from "./accounts";
import {
  writeToFile,
  extractValueFromEvent,
  QueryService,
  get_plugin_info,
  toCosmosMsg,
} from "./utils";
import {
  WCredentialPubKey,
  InstantiateMsg as IdentityInstMsg,
} from "./interfaces/IdentityPlugin.types";
import { ProxyT, FactoryT } from "@vectis/types";

(async function create_() {
  // Template
  const { user } = accounts;
  const privateKey = PrivateKey.fromMnemonic(user.mnemonic);
  const network = Network.Testnet;
  const endpoints = getNetworkEndpoints(network);
  const client = new MsgBroadcasterWithPk({
    privateKey,
    network,
    simulateTx: true,
  });
  const qs = new QueryService(network, endpoints);

  // We create a proxy
  let msg: FactoryT.ExecuteMsg = {
    create_wallet: {
      create_wallet_msg: {
        controller_addr: user.address,
        guardians: { addresses: [] },
        label: "Plugin for Avida",
        proxy_initial_funds: [],
        relayers: [],
      },
    },
  };

  let executeMsg = MsgExecuteContract.fromJSON({
    contractAddress: "inj1qp6nm4zy3ldx7frtjwrt0rgga5xfzp9u9uyhct",
    sender: user.address,
    msg,
    funds: { denom: "inj", amount: "10" },
  });

  let txResponse = await client.broadcast({
    msgs: executeMsg,
    injectiveAddress: user.address,
  });

  const proxy_addr = extractValueFromEvent(
    txResponse.rawLog,
    "cosmwasm.wasm.v1.EventContractInstantiated",
    "contract_address"
  );

  console.log("proxy_addr: ", proxy_addr);

  // We first must self issue and get the pub key of that credential
  const cred_def: WCredentialPubKey = await get_plugin_info(
    user.address,
    proxy_addr
  );

  let identity_inst: IdentityInstMsg = {
    cred_def,
  };

  let plugin_params: ProxyT.PluginParams = {
    // This key is found as a hardcoded value in  avida-verifier::types
    permissions: [{ query: "anoncreds-pubkey" }],
  };

  let install_plugin: ProxyT.ExecuteMsg = {
    instantiate_plugin: {
      instantiate_msg: toCosmosMsg(identity_inst),
      label: "Plugin for Vectis",
      plugin_params,
      src: { code_id: 991 },
    },
  };

  let pluginExecuteMsg = MsgExecuteContract.fromJSON({
    contractAddress: proxy_addr,
    sender: user.address,
    msg: install_plugin,
    funds: { denom: "inj", amount: "1" },
  });

  let pluginRes = await client.broadcast({
    msgs: pluginExecuteMsg,
    injectiveAddress: user.address,
  });

  console.log("plugin: ", pluginRes);

  const plugin_addr = extractValueFromEvent(
    pluginRes.rawLog,
    "cosmwasm.wasm.v1.EventContractInstantiated",
    "contract_address"
  );

  console.log("plugin: ", plugin_addr);

  writeToFile(
    "./deploy/plugin_account.json",
    JSON.stringify({
      plugin: plugin_addr,
      wallet: proxy_addr,
    })
  );
})();
