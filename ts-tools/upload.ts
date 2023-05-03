import fs from "fs";
import {
  MsgBroadcasterWithPk,
  MsgStoreCode,
  PrivateKey,
} from "@injectivelabs/sdk-ts";
import { getNetworkEndpoints, Network } from "@injectivelabs/networks";
import { codePaths } from "./paths";
import { accounts } from "./accounts";
import { writeToFile } from "./utils";

(async function uploadCode() {
  const { admin } = accounts;
  const privateKey = PrivateKey.fromMnemonic(admin.mnemonic);
  const endpoints = getNetworkEndpoints(Network.TestnetK8s);

  console.log("uploading to: ", endpoints);

  const codesId = {} as Record<string, number>;

  for await (const [key, value] of Object.entries(codePaths)) {
    console.log(key);
    //if (key == "adapterCodePath") {
    try {
      const contract = fs.readFileSync(value);
      const msg = MsgStoreCode.fromJSON({
        sender: admin.address,
        wasmBytes: contract,
      });

      const txHash = await new MsgBroadcasterWithPk({
        privateKey,
        network: Network.Testnet,
        endpoints: endpoints,
        simulateTx: true,
      }).broadcast({
        msgs: msg,
        injectiveAddress: admin.address,
      });

      const [{ events }] = JSON.parse(txHash.rawLog);
      const { attributes } = events.find((e: any) => e.type === "store_code");
      const { value: codeId } = attributes.find(
        (a: any) => a.key === "code_id"
      );
      const name = key.replace("Path", "Id");
      codesId[name] = Number(codeId);
      console.log("Uploaded: ", name, codeId);
    } catch (err) {
      console.log("Upload failed: ", key, "errr: ", err);
    }
    //}
  }

  writeToFile(
    `./deploy/injective-testnet-uploadInfo.json`,
    JSON.stringify(codesId, null, 2)
  );
})();
