import assert from "assert";
import {
  MsgBroadcasterWithPk,
  MsgExecuteContract,
  PrivateKey,
} from "@injectivelabs/sdk-ts";
import { getNetworkEndpoints, Network } from "@injectivelabs/networks";
import { accounts } from "./accounts";
import {
  ContractsInterface,
  QueryService,
  WalletPlugin,
  toCosmosMsg,
  generateProof,
} from "./utils";
import { ExecuteMsg as LaunchPadMsg } from "./interfaces/Launchpad.types";
import { ProxyT } from "@vectis/types";

(async function create_mint() {
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

  const { launchpad } = (await import(
    "./deploy/injective-testnet-deployInfo.json"
  )) as ContractsInterface;

  const { wallet } = (await import(
    "./deploy/plugin_account.json"
  )) as WalletPlugin;

  const rg1_transform_addr = await import(
    "./deploy/rg1_transform_address.json"
  );

  let nonce: string = await qs.queryWasm(rg1_transform_addr.default, {
    proof_nonce: { address: wallet },
  });
  console.log("nonce: ", nonce);

  const rgContracts = await qs.queryWasm(launchpad, {
    registered_contracts: { contract_type: "transform" },
    // for transform do
    // registered_contracts: { contract_type: "transform" },
  });
  console.log("Rg20 on Launchpad: ", JSON.stringify(rgContracts));

  let proof = await generateProof(user.address, wallet, nonce);

  let transform_msg: LaunchPadMsg = {
    transform: {
      proof,
      rg_token_addr: rg1_transform_addr.default,
    },
  };

  let proxy_msg: ProxyT.CosmosMsgForEmpty = {
    wasm: {
      execute: {
        contract_addr: launchpad,
        // this means we should get 1INJ of the balance of rgINJ
        funds: [{ denom: "inj", amount: "2000000000000000000" }],
        msg: toCosmosMsg(transform_msg),
      },
    },
  };

  let mint = MsgExecuteContract.fromJSON({
    contractAddress: wallet,
    sender: user.address,
    msg: { execute: { msgs: [proxy_msg] } },
    funds: { denom: "inj", amount: "2000000000000000000" },
  });

  let res = await client.broadcast({
    msgs: mint,
    injectiveAddress: user.address,
  });

  console.log("res: ", res);

  let balance: string = await qs.queryWasm(rg1_transform_addr.default, {
    balance: { address: wallet },
  });

  let new_nonce: string = await qs.queryWasm(rg1_transform_addr.default, {
    proof_nonce: { address: wallet },
  });

  console.log("balance: ", balance);
  console.log("new nonce: ", new_nonce);
})();
