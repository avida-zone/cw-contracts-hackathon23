import assert from "assert";
import {
  MsgInstantiateContract,
  MsgBroadcasterWithPk,
  MsgExecuteContract,
  PrivateKey,
} from "@injectivelabs/sdk-ts";
import { getNetworkEndpoints, Network } from "@injectivelabs/networks";
import { accounts } from "./accounts";
import {
  writeToFile,
  extractValueFromEvent,
  getIssuerSubProofRequestParam,
  ContractsInterface,
  QueryService,
  parseSubProofReqParam,
} from "./utils";

import {
  RgMinterData,
  InstantiateMsg as RgInstMsg,
} from "./interfaces/RgCw20.types";
import {
  WSubProofReqParams,
  Coin,
  LaunchType,
  MintOptions,
  ExecuteMsg as LaunchExecMsg,
} from "./interfaces/Launchpad.types";

(async function create_rg_tokens() {
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

  const { launchpad, vcverifier } = (await import(
    "./deploy/injective-testnet-deployInfo.json"
  )) as ContractsInterface;

  // This gets all 3 issuers, we can pick and choose in demo
  // see doc of this function
  const params = await getIssuerSubProofRequestParam();
  let parsed_params = [];
  for (let p of params) {
    let param = parseSubProofReqParam(p) as WSubProofReqParams;
    parsed_params.push(param);
  }
  console.log("Subproofreq params", parsed_params);

  // ==========================
  //
  // User deploys a NEW token, not a transform from token => rg-token
  //
  // ==========================
  //
  // Defines the mint option,
  // i.e. for anyone who has the credential from all 3 (not always 3, depends on user pick)
  // will be able to mint, if they pay 3inj
  // Remember the cap  takes in decimal for display purpose
  let mint_option: MintOptions = {
    cap: "1000000",
    price: [{ denom: "inj", amount: "3" }],
  };

  // This is new because it is a brand new token, not a transformed one, i.e. must be mint option
  let launchtype: LaunchType = { new: mint_option };
  // For transform do
  // let launchtype: LaunchType = { transform: "denom-to-be-transformed" };

  // Then we can now define the actual rg_cw20 instant message
  let rg20_instant_msg: RgInstMsg = {
    decimals: 3,
    initial_balances: [],
    marketing: null,
    mint: { cap: "1000000", minter: launchpad },
    name: "RG Token 1",
    req_params: parsed_params,
    symbol: "rgHKT",
  };

  // Let launch it!
  let launchMsg: LaunchExecMsg = {
    launch: {
      label: "RG Token 1",
      launch_type: launchtype,
      msg: rg20_instant_msg,
    },
  };

  let executeMsg = MsgExecuteContract.fromJSON({
    contractAddress: launchpad,
    sender: user.address,
    msg: launchMsg,
  });

  let txResponse = await client.broadcast({
    msgs: executeMsg,
    injectiveAddress: user.address,
  });

  console.log("txResponse:", JSON.stringify(txResponse));

  const rg1_address = extractValueFromEvent(
    txResponse.rawLog,
    //fixing on next deploy "wasm-Avida.Launchpad.v1.MsgTokenContractInstanitated",
    "cosmwasm.wasm.v1.EventContractInstantiated",
    "contract_address"
  );

  writeToFile(
    "./deploy/rg1_new_address.json",
    JSON.stringify(rg1_address, null, 2)
  );

  const minter: RgMinterData = await qs.queryWasm(rg1_address, { minter: {} });
  assert.equal(minter.minter, launchpad);
  console.log("Minter: ", minter);

  const rgContracts = await qs.queryWasm(launchpad, {
    registered_contracts: { contract_type: "new" },
    // for transform do
    // registered_contracts: { contract_type: "transform" },
  });
  console.log("Rg20 on Launchpad: ", rgContracts);
})();
