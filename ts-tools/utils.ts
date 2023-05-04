import path from "path";
import fs from "fs";
import { toBase64, toUtf8 } from "@cosmjs/encoding";
import axios from "axios";

import {
  BigNumberBytes,
  WMap,
  WSubProofReq,
  WCredentialSchema,
  WNonCredentialSchema,
  WBTreeSetForString,
  WPrimaryEqualProof,
  WAggregatedProof,
  WProof,
  WSubProof,
} from "./interfaces/VcVerifier.types";
import {
  WCredentialPubKey,
  WCredentialPrimaryPubKey,
} from "./interfaces/IdentityPlugin.types";
import { WSubProofReqParams } from "./interfaces/RgCw20.types";
import { Network, NetworkEndpoints } from "@injectivelabs/networks";
import { ChainGrpcBankApi, ChainGrpcWasmApi } from "@injectivelabs/sdk-ts";

export class QueryService {
  network: Network;
  endpoints: NetworkEndpoints;
  wasmApi: ChainGrpcWasmApi;
  bankApi: ChainGrpcBankApi;
  constructor(network: Network, endpoints: NetworkEndpoints) {
    this.network = network;
    this.endpoints = endpoints;
    this.wasmApi = new ChainGrpcWasmApi(endpoints.grpc);
    this.bankApi = new ChainGrpcBankApi(endpoints.grpc);
  }

  async queryWasm<T>(contractAddr: string, msg: unknown): Promise<T> {
    const query = Buffer.from(JSON.stringify(msg)).toString("base64");
    const response = await this.wasmApi.fetchSmartContractState(
      contractAddr,
      query
    );
    return JSON.parse(Buffer.from(response.data).toString()) as T;
  }

  async queryBalances<T>(account: string): Promise<{}> {
    const balances = await this.bankApi.fetchBalances(account);
    return balances;
  }

  async queryBalance<T>(accountAddress: string, denom: string): Promise<{}> {
    const balances = await this.bankApi.fetchBalance({ accountAddress, denom });
    return balances;
  }
}

export function writeToFile(
  fullPath: string,
  content: string,
  encoding: BufferEncoding = "utf8"
): void {
  const dir = path.dirname(fullPath);
  if (!fs.existsSync(dir)) fs.mkdirSync(dir, { recursive: true });
  fs.writeFileSync(fullPath, content, { encoding });
}

export interface WalletPlugin {
  plugin: string;
  wallet: string;
}

export interface ContractsInterface {
  launchpad: string;
  vcverifier: string;
  adapter: string;
}

// Used to get subProofRequestParams from server
export async function getIssuerSubProofRequestParam(): Promise<[string]> {
  // This gets issuer data to instantiate rg-cw20,
  // you can pick 1 / 2 / 3 depending on what user picks on the frontend.
  // They are https://github.com/avida-zone/ursa-service-hackathon23/blob/main/ursa-demo/setup_data/issuers.json
  // The SAME issuers will be required when a user mints / transform / transfer / burn
  // i.e. `https://avida-api.vectis.space/generate-proof/controller_addr/wallet_addr/13/?issuer=gayadeed&issuer=infocert`
  const { data } = await axios.get(
    "https://avida-api.vectis.space/sub-proof-req-params/?issuer=gayadeed&issuer=identrust&issuer=infocert",
    { responseType: "json" }
  );

  return data;
}

export async function get_plugin_info(
  controller_addr: string,
  wallet_addr: string
): Promise<WCredentialPubKey> {
  const { data } = await axios.post(
    `https://avida-api.vectis.space/rg-holder-setup/${controller_addr}/${wallet_addr}`,
    //`http://0.0.0.0:8000/rg-holder-setup/${controller_addr}/${wallet_addr}`,
    { responseType: "json" }
  );

  let subPR = JSON.parse(data);
  let pkey = parseCredPubKey(JSON.stringify(subPR.credential_pub_key));
  return pkey;
}

export async function generateProof(
  controller_addr: string,
  wallet_addr: string,
  nonce: string
): Promise<WProof> {
  const { data } = await axios.post(
    `https://avida-api.vectis.space/generate-proof/${controller_addr}/${wallet_addr}/${nonce}/?issuer=gayadeed&issuer=identrust&issuer=infocert`,
    { responseType: "json" }
  );
  console.log(data);

  let proof = parseProof(data);
  return proof;
}

export const extractValueFromEvent = (
  rawLog: string,
  event: string,
  attribute: string
) => {
  const events = JSON.parse(rawLog)[0].events as {
    type: string;
    attributes: { key: string; value: string }[];
  }[];
  const e = events.find((e) => e.type === event);
  if (!e) throw new Error("It was not possible to find the event");
  const a = e.attributes.find((attr) => attr.key === attribute);
  if (!a) throw new Error("It was not possible to find the attribute");
  try {
    let value = JSON.parse(a.value);
    return value;
  } catch (e) {
    return a.value;
  }
};

export function parseProof(proofJSON: WProof): WProof {
  const aggregatedProof: WAggregatedProof = {
    c_hash: toBigNumberBytes(proofJSON.aggregated_proof.c_hash),
    c_list: proofJSON.aggregated_proof.c_list,
  };

  let parsedSubProofs: WSubProof[] = [];

  const subProofs = proofJSON.proofs;
  for (const s of subProofs) {
    const eqProof = s.primary_proof.eq_proof;
    let prim_eq_proof: WPrimaryEqualProof = {
      a_prime: toBigNumberBytes(eqProof.a_prime),
      e: toBigNumberBytes(eqProof.e),
      m: toWMap(eqProof.m),
      m2: toBigNumberBytes(eqProof.m2),
      revealed_attrs: toWMap(eqProof.revealed_attrs),
      v: toBigNumberBytes(eqProof.v),
    };
    parsedSubProofs.push({
      primary_proof: { eq_proof: prim_eq_proof, ne_proofs: [] },
    } as WSubProof);
  }

  return {
    aggregated_proof: aggregatedProof,
    proofs: parsedSubProofs,
  };
}

export function getSubProofReq(path: string): WSubProofReq {
  const subProofRequest = fs.readFileSync(path, { encoding: "utf8" });
  return parseSubProofReq(subProofRequest);
}

export function parseSubProofReq(input: string): WSubProofReq {
  const subPR = JSON.parse(input);
  const revealed_attrs: WBTreeSetForString = subPR.revealed_attrs;
  return {
    revealed_attrs,
    predicates: [],
  };
}

export function parseSubProofReqParam(input: string): WSubProofReqParams {
  const subPR: WSubProofReqParams = JSON.parse(input);

  let params: WSubProofReqParams = {
    credential_pub_key: parseCredPubKey(
      JSON.stringify(subPR.credential_pub_key)
    ),
    credential_schema: subPR.credential_schema,
    non_credential_schema: subPR.non_credential_schema,
    rev_key_pub: null,
    rev_reg: null,
    sub_proof_request: parseSubProofReq(
      JSON.stringify(subPR.sub_proof_request)
    ),
  };
  return params;
}

export function getCredentialSchema(path: string): WCredentialSchema {
  const cs_str = fs.readFileSync(path, { encoding: "utf8" });
  return JSON.parse(cs_str);
}

export function getNonCredentialSchema(path: string): WNonCredentialSchema {
  const non_credential_schema = fs.readFileSync(path, { encoding: "utf8" });
  return JSON.parse(non_credential_schema);
}

export function getNonce(path: string): BigNumberBytes {
  const nonce = fs.readFileSync(path, { encoding: "utf8" });
  return toBigNumberBytes(JSON.parse(nonce));
}

export function getCredentialPubKey(path: string): WCredentialPubKey {
  const credential_pub_key = fs.readFileSync(path, { encoding: "utf8" });
  return parseCredPubKey(credential_pub_key);
}

export function parseCredPubKey(input: string): WCredentialPubKey {
  const cpk = JSON.parse(input);
  const pkey = cpk.p_key;
  const p_key: WCredentialPrimaryPubKey = {
    n: toBigNumberBytes(pkey.n),
    r: toWMap(pkey.r),
    rctxt: toBigNumberBytes(pkey.rctxt),
    s: toBigNumberBytes(pkey.s),
    z: toBigNumberBytes(pkey.z),
  };
  return {
    p_key,
  };
}

export function toBigNumberBytes(s: string | any): BigNumberBytes {
  return s as string;
}

export function toWMap(e: {}): WMap {
  // export type WMap = [number[], BigNumberBytes][];
  let w_map: WMap = [];
  Object.entries(e).forEach(([key, value]) => {
    w_map.push([Array.from(toUtf8(key)), toBigNumberBytes(value)]);
  });
  return w_map;
}

export const toCosmosMsg = <T>(msg: T): string => {
  return toBase64(toUtf8(JSON.stringify(msg)));
};
