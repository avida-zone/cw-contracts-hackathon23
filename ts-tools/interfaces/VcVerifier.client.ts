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
  Addr,
  WBTreeSetForString,
  WPredicateType,
  WBTreeSetForWPredicate,
  InstantiateMsg,
  WCredentialSchema,
  WNonCredentialSchema,
  WSubProofReq,
  WPredicate,
  ExecuteMsg,
  BigNumberBytes,
  PointG1Bytes,
  PointG2Bytes,
  WMap,
  WProof,
  WAggregatedProof,
  WSubProof,
  WNonRevocProof,
  WGroupOrderElement,
  WPrimaryProof,
  WPrimaryEqualProof,
  WPrimaryPredicateInequalityProof,
  QueryMsg,
} from "./VcVerifier.types";
export interface VcVerifierReadOnlyInterface {
  contractAddress: string;
}
export class VcVerifierQueryClient implements VcVerifierReadOnlyInterface {
  client: CosmWasmClient;
  contractAddress: string;

  constructor(client: CosmWasmClient, contractAddress: string) {
    this.client = client;
    this.contractAddress = contractAddress;
  }
}
export interface VcVerifierInterface extends VcVerifierReadOnlyInterface {
  contractAddress: string;
  sender: string;
  verify: (
    {
      proof,
      proofReqNonce,
      walletAddr,
    }: {
      proof: WProof;
      proofReqNonce: BigNumberBytes;
      walletAddr: Addr;
    },
    fee?: number | StdFee | "auto",
    memo?: string,
    funds?: Coin[]
  ) => Promise<ExecuteResult>;
}
export class VcVerifierClient
  extends VcVerifierQueryClient
  implements VcVerifierInterface
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
    this.verify = this.verify.bind(this);
  }

  verify = async (
    {
      proof,
      proofReqNonce,
      walletAddr,
    }: {
      proof: WProof;
      proofReqNonce: BigNumberBytes;
      walletAddr: Addr;
    },
    fee: number | StdFee | "auto" = "auto",
    memo?: string,
    funds?: Coin[]
  ): Promise<ExecuteResult> => {
    return await this.client.execute(
      this.sender,
      this.contractAddress,
      {
        verify: {
          proof,
          proof_req_nonce: proofReqNonce,
          wallet_addr: walletAddr,
        },
      },
      fee,
      memo,
      funds
    );
  };
}