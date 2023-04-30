/**
 * This file was automatically generated by @cosmwasm/ts-codegen@0.26.0.
 * DO NOT MODIFY IT BY HAND. Instead, modify the source JSONSchema file,
 * and run the @cosmwasm/ts-codegen generate command to regenerate this file.
 */

import {
  CosmWasmClient,
  SigningCosmWasmClient,
  ExecuteResult,
} from "@cosmjs/cosmwasm-stargate";
import { Coin, StdFee } from "@cosmjs/amino";
import {
  Uint128,
  Logo,
  EmbeddedLogo,
  Binary,
  Addr,
  BigNumberBytes,
  WMap,
  PointG1Bytes,
  PointG2Bytes,
  PointG1Bytes1,
  PointG1Bytes2,
  PointG1Bytes3,
  PointG1Bytes4,
  PointG2Bytes1,
  PointG1Bytes5,
  PointG1Bytes6,
  PointG2Bytes2,
  PointG2Bytes3,
  WBTreeSetForString,
  WPredicateType,
  WBTreeSetForWPredicate,
  InstantiateMsg,
  Cw20Coin,
  InstantiateMarketingInfo,
  RgMinterData,
  WSubProofReqParams,
  WCredentialPubKey,
  WCredentialPrimaryPubKey,
  WCredentialRevocationPubKey,
  WCredentialSchema,
  WNonCredentialSchema,
  WRevocationKeyPublic,
  WRevocationRegistry,
  WSubProofReq,
  WPredicate,
  ExecuteMsg,
  WProof,
  WAggregatedProof,
  WSubProof,
  WNonRevocProof,
  WGroupOrderElement,
  WPrimaryProof,
  WPrimaryEqualProof,
  WPrimaryPredicateInequalityProof,
  QueryMsg,
  AllAccountsResponse,
  BalanceResponse,
  DownloadLogoResponse,
  LogoInfo,
  MarketingInfoResponse,
  Uint64,
  TokenInfoResponse,
} from "./RgCw20.types";
export interface RgCw20ReadOnlyInterface {
  contractAddress: string;
  balance: ({ address }: { address: string }) => Promise<BalanceResponse>;
  proofNonce: ({ address }: { address: string }) => Promise<Uint64>;
  tokenInfo: () => Promise<TokenInfoResponse>;
  minter: () => Promise<RgMinterData>;
  allAccounts: ({
    limit,
    startAfter,
  }: {
    limit?: number;
    startAfter?: string;
  }) => Promise<AllAccountsResponse>;
  marketingInfo: () => Promise<MarketingInfoResponse>;
  downloadLogo: () => Promise<DownloadLogoResponse>;
}
export class RgCw20QueryClient implements RgCw20ReadOnlyInterface {
  client: CosmWasmClient;
  contractAddress: string;

  constructor(client: CosmWasmClient, contractAddress: string) {
    this.client = client;
    this.contractAddress = contractAddress;
    this.balance = this.balance.bind(this);
    this.proofNonce = this.proofNonce.bind(this);
    this.tokenInfo = this.tokenInfo.bind(this);
    this.minter = this.minter.bind(this);
    this.allAccounts = this.allAccounts.bind(this);
    this.marketingInfo = this.marketingInfo.bind(this);
    this.downloadLogo = this.downloadLogo.bind(this);
  }

  balance = async ({
    address,
  }: {
    address: string;
  }): Promise<BalanceResponse> => {
    return this.client.queryContractSmart(this.contractAddress, {
      balance: {
        address,
      },
    });
  };
  proofNonce = async ({ address }: { address: string }): Promise<Uint64> => {
    return this.client.queryContractSmart(this.contractAddress, {
      proof_nonce: {
        address,
      },
    });
  };
  tokenInfo = async (): Promise<TokenInfoResponse> => {
    return this.client.queryContractSmart(this.contractAddress, {
      token_info: {},
    });
  };
  minter = async (): Promise<RgMinterData> => {
    return this.client.queryContractSmart(this.contractAddress, {
      minter: {},
    });
  };
  allAccounts = async ({
    limit,
    startAfter,
  }: {
    limit?: number;
    startAfter?: string;
  }): Promise<AllAccountsResponse> => {
    return this.client.queryContractSmart(this.contractAddress, {
      all_accounts: {
        limit,
        start_after: startAfter,
      },
    });
  };
  marketingInfo = async (): Promise<MarketingInfoResponse> => {
    return this.client.queryContractSmart(this.contractAddress, {
      marketing_info: {},
    });
  };
  downloadLogo = async (): Promise<DownloadLogoResponse> => {
    return this.client.queryContractSmart(this.contractAddress, {
      download_logo: {},
    });
  };
}
export interface RgCw20Interface extends RgCw20ReadOnlyInterface {
  contractAddress: string;
  sender: string;
  transfer: (
    {
      amount,
      proof,
      recipient,
    }: {
      amount: Uint128;
      proof: WProof;
      recipient: string;
    },
    fee?: number | StdFee | "auto",
    memo?: string,
    funds?: Coin[]
  ) => Promise<ExecuteResult>;
  burn: (
    {
      amount,
      proof,
    }: {
      amount: Uint128;
      proof: WProof;
    },
    fee?: number | StdFee | "auto",
    memo?: string,
    funds?: Coin[]
  ) => Promise<ExecuteResult>;
  send: (
    {
      amount,
      contract,
      msg,
      proof,
    }: {
      amount: Uint128;
      contract: string;
      msg: Binary;
      proof: WProof;
    },
    fee?: number | StdFee | "auto",
    memo?: string,
    funds?: Coin[]
  ) => Promise<ExecuteResult>;
  mint: (
    {
      amount,
      proof,
      recipient,
    }: {
      amount: Uint128;
      proof: WProof;
      recipient: string;
    },
    fee?: number | StdFee | "auto",
    memo?: string,
    funds?: Coin[]
  ) => Promise<ExecuteResult>;
  updateMarketing: (
    {
      description,
      marketing,
      project,
    }: {
      description?: string;
      marketing?: string;
      project?: string;
    },
    fee?: number | StdFee | "auto",
    memo?: string,
    funds?: Coin[]
  ) => Promise<ExecuteResult>;
  uploadLogo: (
    fee?: number | StdFee | "auto",
    memo?: string,
    funds?: Coin[]
  ) => Promise<ExecuteResult>;
}
export class RgCw20Client extends RgCw20QueryClient implements RgCw20Interface {
  override client: SigningCosmWasmClient;
  sender: string;
  override contractAddress: string;

  constructor(
    client: SigningCosmWasmClient,
    sender: string,
    contractAddress: string
  ) {
    super(client, contractAddress);
    this.client = client;
    this.sender = sender;
    this.contractAddress = contractAddress;
    this.transfer = this.transfer.bind(this);
    this.burn = this.burn.bind(this);
    this.send = this.send.bind(this);
    this.mint = this.mint.bind(this);
    this.updateMarketing = this.updateMarketing.bind(this);
    this.uploadLogo = this.uploadLogo.bind(this);
  }

  transfer = async (
    {
      amount,
      proof,
      recipient,
    }: {
      amount: Uint128;
      proof: WProof;
      recipient: string;
    },
    fee: number | StdFee | "auto" = "auto",
    memo?: string,
    funds?: Coin[]
  ): Promise<ExecuteResult> => {
    return await this.client.execute(
      this.sender,
      this.contractAddress,
      {
        transfer: {
          amount,
          proof,
          recipient,
        },
      },
      fee,
      memo,
      funds
    );
  };
  burn = async (
    {
      amount,
      proof,
    }: {
      amount: Uint128;
      proof: WProof;
    },
    fee: number | StdFee | "auto" = "auto",
    memo?: string,
    funds?: Coin[]
  ): Promise<ExecuteResult> => {
    return await this.client.execute(
      this.sender,
      this.contractAddress,
      {
        burn: {
          amount,
          proof,
        },
      },
      fee,
      memo,
      funds
    );
  };
  send = async (
    {
      amount,
      contract,
      msg,
      proof,
    }: {
      amount: Uint128;
      contract: string;
      msg: Binary;
      proof: WProof;
    },
    fee: number | StdFee | "auto" = "auto",
    memo?: string,
    funds?: Coin[]
  ): Promise<ExecuteResult> => {
    return await this.client.execute(
      this.sender,
      this.contractAddress,
      {
        send: {
          amount,
          contract,
          msg,
          proof,
        },
      },
      fee,
      memo,
      funds
    );
  };
  mint = async (
    {
      amount,
      proof,
      recipient,
    }: {
      amount: Uint128;
      proof: WProof;
      recipient: string;
    },
    fee: number | StdFee | "auto" = "auto",
    memo?: string,
    funds?: Coin[]
  ): Promise<ExecuteResult> => {
    return await this.client.execute(
      this.sender,
      this.contractAddress,
      {
        mint: {
          amount,
          proof,
          recipient,
        },
      },
      fee,
      memo,
      funds
    );
  };
  updateMarketing = async (
    {
      description,
      marketing,
      project,
    }: {
      description?: string;
      marketing?: string;
      project?: string;
    },
    fee: number | StdFee | "auto" = "auto",
    memo?: string,
    funds?: Coin[]
  ): Promise<ExecuteResult> => {
    return await this.client.execute(
      this.sender,
      this.contractAddress,
      {
        update_marketing: {
          description,
          marketing,
          project,
        },
      },
      fee,
      memo,
      funds
    );
  };
  uploadLogo = async (
    fee: number | StdFee | "auto" = "auto",
    memo?: string,
    funds?: Coin[]
  ): Promise<ExecuteResult> => {
    return await this.client.execute(
      this.sender,
      this.contractAddress,
      {
        upload_logo: {},
      },
      fee,
      memo,
      funds
    );
  };
}
