import assert from "assert";
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
  getIssuerSubProofRequestParam,
  ContractsInterface,
  QueryService,
  parseSubProofReqParam,
} from "./utils";

import {
  RgMinterData,
  InstantiateMsg as RgInstMsg,
  QueryMsg as RgQueryMsg,
} from "./interfaces/RgCw20.types";
import {
  WSubProofReqParams,
  LaunchType,
  MintOptions,
  ExecuteMsg as LaunchExecMsg,
} from "./interfaces/Launchpad.types";
import {
  QueryMsg as AdapterQueryMsg,
  Coin,
} from "./interfaces/Cw20Adapter.types";

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

  const { launchpad, adapter } = (await import(
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

  let query: AdapterQueryMsg = { new_denom_fee: {} };
  let factoryFee = (await qs.queryWasm(adapter, query)) as Coin[];
  console.log("tokenFactory fee: ", factoryFee);

  // ==========================
  //
  // User deploys a NEW token, not a transform from token => rg-token
  //
  // ==========================
  //
  // Defines the mint option,
  // i.e. for anyone who has the credential from all 3 (not always 3, depends on user pick)
  // will be able to mint, if they pay 1e17 inj
  // The cap  takes in decimal
  //let mint_option: MintOptions = {
  //  cap: "1000000000000",
  //  price: [{ denom: "inj", amount: "100000000000000000" }],
  //};

  //// This is new because it is a brand new token, not a transformed one, i.e. must be mint option
  //let launchtype_new: LaunchType = { new: mint_option };

  //// Then we can now define the actual rg_cw20 instant message
  //let rg20_instant_msg: RgInstMsg = {
  //  decimals: 6,
  //  initial_balances: [],
  //  marketing: null,
  //  mint: { cap: "1000000000000", minter: launchpad },
  //  name: "RG Token 1",
  //  req_params: parsed_params,
  //  trusted_issuers: ["infocert", "gayadeed", "identrust"],
  //  symbol: "rgHKT",
  //};

  //// Let launch it!
  //let launchMsg_new: LaunchExecMsg = {
  //  launch: {
  //    label: "RG Token 1",
  //    launch_type: launchtype_new,
  //    msg: rg20_instant_msg,
  //  },
  //};

  //let executeMsg_new = MsgExecuteContract.fromJSON({
  //  contractAddress: launchpad,
  //  sender: user.address,
  //  msg: launchMsg_new,
  //  funds: factoryFee,
  //});

  //let txResponse_new = await client.broadcast({
  //  msgs: executeMsg_new,
  //  injectiveAddress: user.address,
  //});

  //console.log("txResponse NEW:", JSON.stringify(txResponse_new));
  //const rg1_new_address = extractValueFromEvent(
  //  txResponse_new.rawLog,
  //  "cosmwasm.wasm.v1.EventContractInstantiated",
  //  "contract_address"
  //);

  //writeToFile(
  //  "./deploy/rg1_new_address.json",
  //  JSON.stringify(rg1_new_address, null, 2)
  //);

  //const minter: RgMinterData = await qs.queryWasm(rg1_new_address, {
  //  minter: {},
  //});
  //assert.equal(minter.minter, launchpad);
  //console.log("Minter: ", minter);

  //const rgContractsNew = await qs.queryWasm(launchpad, {
  //  registered_contracts: { contract_type: "new" },
  //});
  //console.log("NEW Rg20 on Launchpad: ", rgContractsNew);
  // ======================================
  //
  // NOW this is the transform
  //
  // =====================================
  let rg20_instant_msg_transform: RgInstMsg = {
    // to match inj
    decimals: 18,
    initial_balances: [],
    marketing: null,
    // this cap is ignored by the contract
    // as long as there are the right denom, it will be minted
    mint: { cap: "100", minter: launchpad },
    name: "RG-INJ",
    req_params: parsed_params,
    trusted_issuers: ["infocert", "gayadeed", "identrust"],
    symbol: "rgINJ",
  };

  let launchtype_transform: LaunchType = {
    transform: "inj",
  };

  let launchMsg_transform: LaunchExecMsg = {
    launch: {
      label: "RG-INJ",
      launch_type: launchtype_transform,
      msg: rg20_instant_msg_transform,
    },
  };

  let executeMsg_transform = MsgExecuteContract.fromJSON({
    contractAddress: launchpad,
    sender: user.address,
    msg: launchMsg_transform,
    funds: factoryFee,
  });

  let txResponse_transform = await client.broadcast({
    msgs: executeMsg_transform,
    injectiveAddress: user.address,
  });

  console.log("txResponse TRANSFORM:", JSON.stringify(txResponse_transform));

  const rg1_transform_address = extractValueFromEvent(
    txResponse_transform.rawLog,
    "cosmwasm.wasm.v1.EventContractInstantiated",
    "contract_address"
  );

  writeToFile(
    "./deploy/rg1_transform_address.json",
    JSON.stringify(rg1_transform_address, null, 2)
  );

  const rgContractsTransform = await qs.queryWasm(launchpad, {
    registered_contracts: { contract_type: "transform" },
  });
  console.log("TRANSFORM Rg20 on Launchpad: ", rgContractsTransform);

  let query_msg: RgQueryMsg = { trusted_issuers: {} };
  const trusted_issuers = await qs.queryWasm(rg1_transform_address, query_msg);
  console.log("trusted issuers: ", trusted_issuers);
})();
