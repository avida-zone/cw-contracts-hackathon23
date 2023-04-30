import path from "path";
import fs from "fs";
import { toUtf8 } from "@cosmjs/encoding";
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
export function writeToFile(
  fullPath: string,
  content: string,
  encoding: BufferEncoding = "utf8"
): void {
  const dir = path.dirname(fullPath);
  if (!fs.existsSync(dir)) fs.mkdirSync(dir, { recursive: true });
  fs.writeFileSync(fullPath, content, { encoding });
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
  return JSON.parse(a.value);
};

export function getProof(path: string): WProof {
  const proof = fs.readFileSync(path, { encoding: "utf8" });
  const proofJSON = JSON.parse(proof);

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
    // TODO ne_proofs
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
  const subPR = JSON.parse(subProofRequest);
  // TODO predicates
  const revealed_attrs: WBTreeSetForString = subPR.revealed_attrs;
  return {
    revealed_attrs,
    predicates: [],
  };
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
  const cpk = JSON.parse(credential_pub_key);
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
