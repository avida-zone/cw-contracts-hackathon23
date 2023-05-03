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
import {
  Coin,
  ExecuteMsg as AdapterExecMsg,
} from "./interfaces/Cw20Adapter.types";

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

  let tfdenom =
    "factory/inj16n20mwtmvunl9tdkxajxzezn87z4jv5hgkjn5l/inj1w6ghr4pkladye5x9zj4cmx7lpg7a8tg4x2t63f";

  const current_rg_balances = await qs.queryWasm(
    rg1_transform_address.default,
    {
      balance: { address: wallet },
    }
  );
  console.log("current native balance: ", current_native_balances);
  console.log("current rg balance: ", current_rg_balances);

  let nonce: string = await qs.queryWasm(rg1_transform_address.default, {
    proof_nonce: { address: wallet },
  });
  console.log("proof nonce for unadapt: ", nonce);
  let proof = await generateProof(user.address, wallet, nonce);

  // create the redeem message
  const user_to_adapter: AdapterExecMsg = {
    // default send to itself
    redeem_and_transfer: {},
  };

  let proxy_msg: ProxyT.CosmosMsgForEmpty = {
    wasm: {
      execute: {
        contract_addr: adapter,
        funds: [{ denom: tfdenom, amount: "500" }],
        msg: toCosmosMsg(user_to_adapter),
      },
    },
  };

  // We do not need to send funds here because the wallet has the funds
  let mint = MsgExecuteContract.fromJSON({
    contractAddress: wallet,
    sender: user.address,
    msg: { execute: { msgs: [proxy_msg] } },
  });

  let res = await client.broadcast({
    msgs: mint,
    injectiveAddress: user.address,
  });
  const unadapt_native_balances = await qs.queryBalances(wallet);
  console.log("current balance: ", unadapt_native_balances);

  const unadapt_rg_balances = await qs.queryWasm(
    rg1_transform_address.default,
    {
      balance: { address: wallet },
    }
  );
  console.log("unadapt native balance: ", unadapt_native_balances);
  console.log("unadapt rg balance: ", unadapt_rg_balances);
})();
