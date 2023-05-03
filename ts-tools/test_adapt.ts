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
import { ProxyT } from "@vectis/types";

import { ExecuteMsg as RgCw20ExecMsg } from "./interfaces/RgCw20.types";
import { ExecuteMsg as AdapterExecMsg } from "./interfaces/Cw20Adapter.types";

(async function adapt_tf() {
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

  const { adapter } = (await import(
    "./deploy/injective-testnet-deployInfo.json"
  )) as ContractsInterface;

  const { wallet } = (await import(
    "./deploy/plugin_account.json"
  )) as WalletPlugin;
  const rg1_transform_address = await import(
    "./deploy/rg1_transform_address.json"
  );

  const current_native_balances = await qs.queryBalances(wallet);
  console.log("current balance: ", current_native_balances);

  const current_rg_balances = await qs.queryWasm(
    rg1_transform_address.default,
    {
      balance: { address: wallet },
    }
  );
  console.log("current native balance: ", current_native_balances);
  console.log("current rg balance: ", current_rg_balances);

  // ======================================
  //
  // Do Adapt
  //
  // ======================================
  //

  // First create the send message on rg20 for the adapter
  let nonce: string = await qs.queryWasm(rg1_transform_address.default, {
    proof_nonce: { address: wallet },
  });
  console.log("proof nonce for adapt: ", nonce);
  let proof = await generateProof(user.address, wallet, nonce);
  let rg_cw20_send_msg: RgCw20ExecMsg = {
    send: {
      amount: "1000",
      contract: adapter,
      // msg is ignored by the cw20-adapter
      msg: "",
      proof,
    },
  };

  let proxy_msg: ProxyT.CosmosMsgForEmpty = {
    wasm: {
      execute: {
        contract_addr: rg1_transform_address.default,
        funds: [],
        msg: toCosmosMsg(rg_cw20_send_msg),
      },
    },
  };

  let adapt = MsgExecuteContract.fromJSON({
    contractAddress: wallet,
    sender: user.address,
    msg: { execute: { msgs: [proxy_msg] } },
  });

  let res = await client.broadcast({
    msgs: adapt,
    injectiveAddress: user.address,
  });

  console.log("adapt msg ", res);

  const after_native_balances = await qs.queryBalances(wallet);
  const after_rg_balances = await qs.queryWasm(rg1_transform_address.default, {
    balance: { address: wallet },
  });
  console.log("after native balance: ", after_native_balances);
  console.log("after rg balance: ", after_rg_balances);
})();
