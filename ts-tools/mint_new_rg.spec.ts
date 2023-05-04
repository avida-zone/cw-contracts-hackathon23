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
import { BalanceResponse } from "./interfaces/RgCw20.types";
import { ProxyT } from "@vectis/types";

describe("Mint: ", () => {
  let privateKey;
  let network;
  let endpoints;
  let client: MsgBroadcasterWithPk;
  let userAddr: string;
  let qs: QueryService;
  let launchpad: string;
  let wallet: string;
  let rg1_new_addr: string;

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
      "./deploy/injective-testnet-deployInfo.json"
    )) as ContractsInterface;
    launchpad = contracts.launchpad;

    let walletAddrs = (await import(
      "./deploy/plugin_account.json"
    )) as WalletPlugin;
    wallet = walletAddrs.wallet;

    let new_token = await import("./deploy/rg1_new_address.json");
    rg1_new_addr = new_token.default;
  });

  //const pricePerToken = "100000000000000000";
  it("should be able to mint new tokens with correct fee", async () => {
    const mintAmount = "3";
    const correctMintFee = "300000000000000000";

    const initNonce: string = await qs.queryWasm(rg1_new_addr, {
      proof_nonce: { address: wallet },
    });
    const initBalance: BalanceResponse = await qs.queryWasm(rg1_new_addr, {
      balance: { address: wallet },
    });
    let proof = await generateProof(userAddr, wallet, initNonce);

    let mint_msg: LaunchPadMsg = {
      mint: {
        amount: mintAmount,
        proof,
        rg_token_addr: rg1_new_addr,
      },
    };

    let proxy_msg: ProxyT.CosmosMsgForEmpty = {
      wasm: {
        execute: {
          contract_addr: launchpad,
          funds: [{ denom: "inj", amount: correctMintFee }],
          msg: toCosmosMsg(mint_msg),
        },
      },
    };

    let mint = MsgExecuteContract.fromJSON({
      contractAddress: wallet,
      sender: userAddr,
      msg: { execute: { msgs: [proxy_msg] } },
      funds: { denom: "inj", amount: correctMintFee },
    });

    await client.broadcast({
      msgs: mint,
      injectiveAddress: userAddr,
    });

    const afterBalance: BalanceResponse = await qs.queryWasm(rg1_new_addr, {
      balance: { address: wallet },
    });
    const afterNonce: string = await qs.queryWasm(rg1_new_addr, {
      proof_nonce: { address: wallet },
    });
    expect(afterNonce + 0).toEqual(initNonce + 1);
    expect(+afterBalance.balance).toEqual(+initBalance.balance + 3);
  });

  it("should not be able to mint new tokens with incorrect proof nonce", async () => {
    const mintAmount = "3";
    const correctMintFee = "300000000000000000";

    const initNonce: string = await qs.queryWasm(rg1_new_addr, {
      proof_nonce: { address: wallet },
    });

    const notNonce = (+initNonce + 10).toString();
    let proof = await generateProof(userAddr, wallet, notNonce);

    let mint_msg: LaunchPadMsg = {
      mint: {
        amount: mintAmount,
        proof,
        rg_token_addr: rg1_new_addr,
      },
    };

    let proxy_msg: ProxyT.CosmosMsgForEmpty = {
      wasm: {
        execute: {
          contract_addr: launchpad,
          funds: [{ denom: "inj", amount: correctMintFee }],
          msg: toCosmosMsg(mint_msg),
        },
      },
    };

    let mint = MsgExecuteContract.fromJSON({
      contractAddress: wallet,
      sender: userAddr,
      msg: { execute: { msgs: [proxy_msg] } },
      funds: { denom: "inj", amount: correctMintFee },
    });

    await expect(
      client.broadcast({
        msgs: mint,
        injectiveAddress: userAddr,
      })
    ).rejects.toThrowError();
  });

  it("should not be able to mint new tokens with incorrect fee", async () => {
    const mintAmount = "3";
    const incorrectMintFee = "1000000000000000";

    const initNonce: string = await qs.queryWasm(rg1_new_addr, {
      proof_nonce: { address: wallet },
    });
    let proof = await generateProof(userAddr, wallet, initNonce);

    let mint_msg: LaunchPadMsg = {
      mint: {
        amount: mintAmount,
        proof,
        rg_token_addr: rg1_new_addr,
      },
    };

    let proxy_msg: ProxyT.CosmosMsgForEmpty = {
      wasm: {
        execute: {
          contract_addr: launchpad,
          funds: [{ denom: "inj", amount: incorrectMintFee }],
          msg: toCosmosMsg(mint_msg),
        },
      },
    };

    let mint = MsgExecuteContract.fromJSON({
      contractAddress: wallet,
      sender: userAddr,
      msg: { execute: { msgs: [proxy_msg] } },
      funds: { denom: "inj", amount: incorrectMintFee },
    });

    await expect(
      client.broadcast({
        msgs: mint,
        injectiveAddress: userAddr,
      })
    ).rejects.toThrowError();
  });
});
