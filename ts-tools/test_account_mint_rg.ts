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
  const rg1_new_addr = await import("./deploy/rg1_new_address.json");

  let nonce: string = await qs.queryWasm(rg1_new_addr.default, {
    proof_nonce: { address: wallet },
  });
  console.log("nonce: ", nonce);

  const rgContracts = await qs.queryWasm(launchpad, {
    registered_contracts: { contract_type: "new" },
  });
  console.log("Rg20 on Launchpad: ", JSON.stringify(rgContracts));

  const results = await qs.queryWasm(launchpad, {
    verifier: {},
  });
  console.log("verifier on launchpad", JSON.stringify(results));

  const viaRgToken = await qs.queryWasm(rg1_new_addr.default, {
    token_info: {},
  });
  console.log("verifier on launchpad", JSON.stringify(viaRgToken));

  // AGAIN, here we assumed ALL 3 issuers are on the rg-cw20 address
  // defined when we launched the token in test-create-rgtokens.ts
  //
  // In reality, it can be 1 / 2 / 3 issuers
  let proof = await generateProof(user.address, wallet, nonce);

  let mint_msg: LaunchPadMsg = {
    mint: {
      // 100000000000000000 inj each from the price
      amount: "3",
      proof,
      rg_token_addr: rg1_new_addr.default,
    },
  };

  let proxy_msg: ProxyT.CosmosMsgForEmpty = {
    wasm: {
      execute: {
        contract_addr: launchpad,
        funds: [{ denom: "inj", amount: "300000000000000000" }],
        msg: toCosmosMsg(mint_msg),
      },
    },
  };

  let mint = MsgExecuteContract.fromJSON({
    contractAddress: wallet,
    sender: user.address,
    msg: { execute: { msgs: [proxy_msg] } },
    funds: { denom: "inj", amount: "300000000000000000" },
  });

  let res = await client.broadcast({
    msgs: mint,
    injectiveAddress: user.address,
  });

  console.log("res: ", res);

  let balance: string = await qs.queryWasm(rg1_new_addr.default, {
    balance: { address: wallet },
  });

  let new_nonce: string = await qs.queryWasm(rg1_new_addr.default, {
    proof_nonce: { address: wallet },
  });

  console.log("balance: ", balance);
  console.log("new nonce: ", new_nonce);
})();
