/**
 * This file was automatically generated by @cosmwasm/ts-codegen@0.26.0.
 * DO NOT MODIFY IT BY HAND. Instead, modify the source JSONSchema file,
 * and run the @cosmwasm/ts-codegen generate command to regenerate this file.
 */

export type Uint128 = string;
export type Logo =
  | {
      url: string;
    }
  | {
      embedded: EmbeddedLogo;
    };
export type EmbeddedLogo =
  | {
      svg: Binary;
    }
  | {
      png: Binary;
    };
export type Binary = string;
export type Addr = string;
export type BigNumberBytes = string;
export type WMap = [number[], BigNumberBytes][];
export type PointG1Bytes = string;
export type PointG2Bytes = string;
export type PointG1Bytes1 = string;
export type PointG1Bytes2 = string;
export type PointG1Bytes3 = string;
export type PointG1Bytes4 = string;
export type PointG2Bytes1 = string;
export type PointG1Bytes5 = string;
export type PointG1Bytes6 = string;
export type PointG2Bytes2 = string;
export type PointG2Bytes3 = string;
export type WBTreeSetForString = string[];
export type WPredicateType = "GE" | "LE" | "GT" | "LT";
export type WBTreeSetForWPredicate = WPredicate[];
export interface InstantiateMsg {
  decimals: number;
  initial_balances: Cw20Coin[];
  marketing?: InstantiateMarketingInfo | null;
  mint?: RgMinterData | null;
  name: string;
  req_params: WSubProofReqParams[];
  symbol: string;
}
export interface Cw20Coin {
  address: string;
  amount: Uint128;
}
export interface InstantiateMarketingInfo {
  description?: string | null;
  logo?: Logo | null;
  marketing?: string | null;
  project?: string | null;
}
export interface RgMinterData {
  cap?: Uint128 | null;
  minter?: Addr | null;
}
export interface WSubProofReqParams {
  credential_pub_key: WCredentialPubKey;
  credential_schema: WCredentialSchema;
  non_credential_schema: WNonCredentialSchema;
  rev_key_pub?: WRevocationKeyPublic | null;
  rev_reg?: WRevocationRegistry | null;
  sub_proof_request: WSubProofReq;
  [k: string]: unknown;
}
export interface WCredentialPubKey {
  p_key: WCredentialPrimaryPubKey;
  r_key?: WCredentialRevocationPubKey | null;
  [k: string]: unknown;
}
export interface WCredentialPrimaryPubKey {
  n: BigNumberBytes;
  r: WMap;
  rctxt: BigNumberBytes;
  s: BigNumberBytes;
  z: BigNumberBytes;
  [k: string]: unknown;
}
export interface WCredentialRevocationPubKey {
  g: PointG1Bytes;
  g_dash: PointG2Bytes;
  h: PointG1Bytes1;
  h0: PointG1Bytes2;
  h1: PointG1Bytes3;
  h2: PointG1Bytes4;
  h_cap: PointG2Bytes1;
  htilde: PointG1Bytes5;
  pk: PointG1Bytes6;
  u: PointG2Bytes2;
  y: PointG2Bytes3;
  [k: string]: unknown;
}
export interface WCredentialSchema {
  attrs: WBTreeSetForString;
  [k: string]: unknown;
}
export interface WNonCredentialSchema {
  attrs: WBTreeSetForString;
  [k: string]: unknown;
}
export interface WRevocationKeyPublic {
  pair: string;
  [k: string]: unknown;
}
export interface WRevocationRegistry {
  accum: Binary;
  [k: string]: unknown;
}
export interface WSubProofReq {
  predicates: WBTreeSetForWPredicate;
  revealed_attrs: WBTreeSetForString;
  [k: string]: unknown;
}
export interface WPredicate {
  attr_name: string;
  p_type: WPredicateType;
  value: number;
  [k: string]: unknown;
}
export type ExecuteMsg =
  | {
      transfer: {
        amount: Uint128;
        proof: WProof;
        recipient: string;
      };
    }
  | {
      burn: {
        amount: Uint128;
        proof: WProof;
      };
    }
  | {
      send: {
        amount: Uint128;
        contract: string;
        msg: Binary;
        proof: WProof;
      };
    }
  | {
      mint: {
        amount: Uint128;
        proof: WProof;
        recipient: string;
      };
    }
  | {
      update_marketing: {
        description?: string | null;
        marketing?: string | null;
        project?: string | null;
      };
    }
  | {
      upload_logo: Logo;
    };
export interface WProof {
  aggregated_proof: WAggregatedProof;
  proofs: WSubProof[];
  [k: string]: unknown;
}
export interface WAggregatedProof {
  c_hash: BigNumberBytes;
  c_list: number[][];
  [k: string]: unknown;
}
export interface WSubProof {
  non_revoc_proof?: WNonRevocProof | null;
  primary_proof: WPrimaryProof;
  [k: string]: unknown;
}
export interface WNonRevocProof {
  c_list: [PointG1Bytes[], PointG2Bytes[]];
  x_list: WGroupOrderElement[];
  [k: string]: unknown;
}
export interface WGroupOrderElement {
  bn_hex: string;
  [k: string]: unknown;
}
export interface WPrimaryProof {
  eq_proof: WPrimaryEqualProof;
  ne_proofs: WPrimaryPredicateInequalityProof[];
  [k: string]: unknown;
}
export interface WPrimaryEqualProof {
  a_prime: BigNumberBytes;
  e: BigNumberBytes;
  m: WMap;
  m2: BigNumberBytes;
  revealed_attrs: WMap;
  v: BigNumberBytes;
  [k: string]: unknown;
}
export interface WPrimaryPredicateInequalityProof {
  alpha: BigNumberBytes;
  mj: BigNumberBytes;
  predicate: WPredicate;
  r: WMap;
  t: WMap;
  u: WMap;
  [k: string]: unknown;
}
export type QueryMsg =
  | {
      balance: {
        address: string;
      };
    }
  | {
      proof_nonce: {
        address: string;
      };
    }
  | {
      token_info: {};
    }
  | {
      minter: {};
    }
  | {
      all_accounts: {
        limit?: number | null;
        start_after?: string | null;
      };
    }
  | {
      marketing_info: {};
    }
  | {
      download_logo: {};
    };
export interface AllAccountsResponse {
  accounts: string[];
  [k: string]: unknown;
}
export interface BalanceResponse {
  balance: Uint128;
}
export interface DownloadLogoResponse {
  data: Binary;
  mime_type: string;
}
export type LogoInfo =
  | {
      url: string;
    }
  | "embedded";
export interface MarketingInfoResponse {
  description?: string | null;
  logo?: LogoInfo | null;
  marketing?: Addr | null;
  project?: string | null;
  [k: string]: unknown;
}
export type Uint64 = number;
export interface TokenInfoResponse {
  decimals: number;
  name: string;
  symbol: string;
  total_supply: Uint128;
}
