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
import { StdFee } from "@cosmjs/amino";
import {
  InstantiateMsg,
  ExecuteMsg,
  LaunchType,
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
  PointG1Bytes7,
  PointG2Bytes4,
  MintOptions,
  Coin,
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
  WProof,
  WAggregatedProof,
  WSubProof,
  WNonRevocProof,
  WGroupOrderElement,
  WPrimaryProof,
  WPrimaryEqualProof,
  WPrimaryPredicateInequalityProof,
  QueryMsg,
  ContractType,
  ArrayOfContractResponse,
  ContractResponse,
  LaunchpadOptions,
} from "./Launchpad.types";
export interface LaunchpadReadOnlyInterface {
  contractAddress: string;
  registeredContracts: ({
    contractType,
    limit,
    startAfter,
  }: {
    contractType: ContractType;
    limit?: number;
    startAfter?: string;
  }) => Promise<ArrayOfContractResponse>;
  verifier: () => Promise<Addr>;
}
export class LaunchpadQueryClient implements LaunchpadReadOnlyInterface {
  client: CosmWasmClient;
  contractAddress: string;

  constructor(client: CosmWasmClient, contractAddress: string) {
    this.client = client;
    this.contractAddress = contractAddress;
    this.registeredContracts = this.registeredContracts.bind(this);
    this.verifier = this.verifier.bind(this);
  }

  registeredContracts = async ({
    contractType,
    limit,
    startAfter,
  }: {
    contractType: ContractType;
    limit?: number;
    startAfter?: string;
  }): Promise<ArrayOfContractResponse> => {
    return this.client.queryContractSmart(this.contractAddress, {
      registered_contracts: {
        contract_type: contractType,
        limit,
        start_after: startAfter,
      },
    });
  };
  verifier = async (): Promise<Addr> => {
    return this.client.queryContractSmart(this.contractAddress, {
      verifier: {},
    });
  };
}
export interface LaunchpadInterface extends LaunchpadReadOnlyInterface {
  contractAddress: string;
  sender: string;
  launch: (
    {
      label,
      launchType,
      msg,
    }: {
      label: string;
      launchType: LaunchType;
      msg: InstantiateMsg;
    },
    fee?: number | StdFee | "auto",
    memo?: string,
    funds?: Coin[]
  ) => Promise<ExecuteResult>;
  mint: (
    {
      amount,
      proof,
      rgTokenAddr,
    }: {
      amount: Uint128;
      proof: WProof;
      rgTokenAddr: string;
    },
    fee?: number | StdFee | "auto",
    memo?: string,
    funds?: Coin[]
  ) => Promise<ExecuteResult>;
  transform: (
    {
      proof,
      rgTokenAddr,
    }: {
      proof: WProof;
      rgTokenAddr: string;
    },
    fee?: number | StdFee | "auto",
    memo?: string,
    funds?: Coin[]
  ) => Promise<ExecuteResult>;
  revert: (
    {
      amount,
      recipient,
    }: {
      amount: Uint128;
      recipient: string;
    },
    fee?: number | StdFee | "auto",
    memo?: string,
    funds?: Coin[]
  ) => Promise<ExecuteResult>;
  updateVerifier: (
    {
      address,
    }: {
      address: string;
    },
    fee?: number | StdFee | "auto",
    memo?: string,
    funds?: Coin[]
  ) => Promise<ExecuteResult>;
}
export class LaunchpadClient
  extends LaunchpadQueryClient
  implements LaunchpadInterface
{
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
    this.launch = this.launch.bind(this);
    this.mint = this.mint.bind(this);
    this.transform = this.transform.bind(this);
    this.revert = this.revert.bind(this);
    this.updateVerifier = this.updateVerifier.bind(this);
  }

  launch = async (
    {
      label,
      launchType,
      msg,
    }: {
      label: string;
      launchType: LaunchType;
      msg: InstantiateMsg;
    },
    fee: number | StdFee | "auto" = "auto",
    memo?: string,
    funds?: Coin[]
  ): Promise<ExecuteResult> => {
    return await this.client.execute(
      this.sender,
      this.contractAddress,
      {
        launch: {
          label,
          launch_type: launchType,
          msg,
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
      rgTokenAddr,
    }: {
      amount: Uint128;
      proof: WProof;
      rgTokenAddr: string;
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
          rg_token_addr: rgTokenAddr,
        },
      },
      fee,
      memo,
      funds
    );
  };
  transform = async (
    {
      proof,
      rgTokenAddr,
    }: {
      proof: WProof;
      rgTokenAddr: string;
    },
    fee: number | StdFee | "auto" = "auto",
    memo?: string,
    funds?: Coin[]
  ): Promise<ExecuteResult> => {
    return await this.client.execute(
      this.sender,
      this.contractAddress,
      {
        transform: {
          proof,
          rg_token_addr: rgTokenAddr,
        },
      },
      fee,
      memo,
      funds
    );
  };
  revert = async (
    {
      amount,
      recipient,
    }: {
      amount: Uint128;
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
        revert: {
          amount,
          recipient,
        },
      },
      fee,
      memo,
      funds
    );
  };
  updateVerifier = async (
    {
      address,
    }: {
      address: string;
    },
    fee: number | StdFee | "auto" = "auto",
    memo?: string,
    funds?: Coin[]
  ): Promise<ExecuteResult> => {
    return await this.client.execute(
      this.sender,
      this.contractAddress,
      {
        update_verifier: {
          address,
        },
      },
      fee,
      memo,
      funds
    );
  };
}