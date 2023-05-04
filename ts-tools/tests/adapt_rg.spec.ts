import {
  MsgBroadcasterWithPk,
  MsgExecuteContract,
  PrivateKey,
} from "@injectivelabs/sdk-ts";
import { getNetworkEndpoints, Network } from "@injectivelabs/networks";
import { accounts } from "../accounts";
import {
  ContractsInterface,
  QueryService,
  WalletPlugin,
  toCosmosMsg,
  generateProof,
} from "../utils";
import {
  BalanceResponse,
  ExecuteMsg as RgCw20ExecMsg,
} from "../interfaces/RgCw20.types";
import { ProxyT } from "@vectis/types";
import { ExecuteMsg as AdapterExecMsg } from "../interfaces/Cw20Adapter.types";

describe("Adapt rgToken => Native rgToken and back: ", () => {
  let privateKey;
  let network;
  let endpoints;
  let client: MsgBroadcasterWithPk;
  let userAddr: string;
  let qs: QueryService;
  let adapter: string;
  let wallet: string;
  let rg1_addr: string;
  let tfDenom: string;
  let adaptAmount: string;

  beforeAll(async () => {
    userAddr = accounts.user.address;
    privateKey = PrivateKey.fromMnemonic(accounts.user.mnemonic);
    network = Network.Testnet;
    endpoints = getNetworkEndpoints(network);
    client = new MsgBroadcasterWithPk({
      privateKey,
      network,
      simulateTx: true,
    });
    qs = new QueryService(network, endpoints);

    let contracts = (await import(
      "../deploy/injective-testnet-deployInfo.json"
    )) as ContractsInterface;
    adapter = contracts.adapter;

    let walletAddrs = (await import(
      "../deploy/plugin_account.json"
    )) as WalletPlugin;
    wallet = walletAddrs.wallet;

    let new_token = await import("../deploy/rg1_transform_address.json");
    rg1_addr = new_token.default;

    tfDenom = "factory/" + adapter + "/" + rg1_addr;
    adaptAmount = "1";
  });

  it("should not able to adapt token with wrong proof", async () => {
    const initNonce: string = await qs.queryWasm(rg1_addr, {
      proof_nonce: { address: wallet },
    });

    let proof = await generateProof(
      userAddr,
      wallet,
      (+initNonce + 4).toString()
    );

    let rg_cw20_send_msg: RgCw20ExecMsg = {
      send: {
        amount: adaptAmount,
        contract: adapter,
        // msg is ignored by the cw20-adapter
        msg: "",
        proof,
      },
    };

    let proxy_msg: ProxyT.CosmosMsgForEmpty = {
      wasm: {
        execute: {
          contract_addr: rg1_addr,
          funds: [],
          msg: toCosmosMsg(rg_cw20_send_msg),
        },
      },
    };

    let adapt = MsgExecuteContract.fromJSON({
      contractAddress: wallet,
      sender: userAddr,
      msg: { execute: { msgs: [proxy_msg] } },
    });

    await expect(
      client.broadcast({
        msgs: adapt,
        injectiveAddress: userAddr,
      })
    ).rejects.toThrowError();
  });

  it("should be able to adapt token", async () => {
    const initNonce: string = await qs.queryWasm(rg1_addr, {
      proof_nonce: { address: wallet },
    });
    const initRgBalance: BalanceResponse = await qs.queryWasm(rg1_addr, {
      balance: { address: wallet },
    });
    const initTfBalance = await qs.queryBalance(wallet, tfDenom);

    let proof = await generateProof(userAddr, wallet, initNonce);

    let rg_cw20_send_msg: RgCw20ExecMsg = {
      send: {
        amount: adaptAmount,
        contract: adapter,
        // msg is ignored by the cw20-adapter
        msg: "",
        proof,
      },
    };

    let proxy_msg: ProxyT.CosmosMsgForEmpty = {
      wasm: {
        execute: {
          contract_addr: rg1_addr,
          funds: [],
          msg: toCosmosMsg(rg_cw20_send_msg),
        },
      },
    };

    let adapt = MsgExecuteContract.fromJSON({
      contractAddress: wallet,
      sender: userAddr,
      msg: { execute: { msgs: [proxy_msg] } },
    });

    await client.broadcast({
      msgs: adapt,
      injectiveAddress: userAddr,
    });

    const afterNonce: string = await qs.queryWasm(rg1_addr, {
      proof_nonce: { address: wallet },
    });
    const afterRgBalance: BalanceResponse = await qs.queryWasm(rg1_addr, {
      balance: { address: wallet },
    });
    const afterTfBalance = await qs.queryBalance(wallet, tfDenom);

    expect(+afterNonce).toEqual(+initNonce + 1);
    expect(+afterRgBalance.balance).toEqual(
      +initRgBalance.balance - +adaptAmount
    );
    expect(+afterTfBalance).toEqual(+initTfBalance + +adaptAmount);
  });

  it("should be able to unadapt token", async () => {
    const initRgBalance: BalanceResponse = await qs.queryWasm(rg1_addr, {
      balance: { address: wallet },
    });
    const initTfBalance = await qs.queryBalance(wallet, tfDenom);

    // create the redeem message
    const user_to_adapter: AdapterExecMsg = {
      // default send to itself
      redeem_and_transfer: {},
    };

    let proxy_msg: ProxyT.CosmosMsgForEmpty = {
      wasm: {
        execute: {
          contract_addr: adapter,
          funds: [{ denom: tfDenom, amount: adaptAmount }],
          msg: toCosmosMsg(user_to_adapter),
        },
      },
    };

    // We do not need to send funds here because the wallet has the funds
    let unadapt = MsgExecuteContract.fromJSON({
      contractAddress: wallet,
      sender: userAddr,
      msg: { execute: { msgs: [proxy_msg] } },
    });

    await client.broadcast({
      msgs: unadapt,
      injectiveAddress: userAddr,
    });

    const afterRgBalance: BalanceResponse = await qs.queryWasm(rg1_addr, {
      balance: { address: wallet },
    });
    const afterTfBalance = await qs.queryBalance(wallet, tfDenom);

    console.log(initRgBalance);
    console.log(afterRgBalance);
    expect(+afterRgBalance.balance).toEqual(
      +initRgBalance.balance + +adaptAmount
    );
    expect(+afterTfBalance).toEqual(+initTfBalance - +adaptAmount);
  });
});
