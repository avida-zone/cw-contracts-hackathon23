import tsGenerator from "@cosmwasm/ts-codegen";
import { join } from "path";

const contractsPath = (path: string) => join(__dirname, "../contracts/", path);
const outPath = join(__dirname, "./interfaces");

(async function downloadSchemas() {
  tsGenerator({
    contracts: [
      {
        name: "identity_plugin",
        dir: contractsPath("identity-plugin/schema"),
      },
      {
        name: "launchpad",
        dir: contractsPath("launchpad/schema"),
      },
      {
        name: "rg_cw20",
        dir: contractsPath("rg-cw20/schema"),
      },
      {
        name: "vc_verifier",
        dir: contractsPath("vc-verifier/schema"),
      },
    ],
    outPath,
    options: {
      bundle: {
        enabled: false,
      },
      client: {
        noImplicitOverride: true,
      },
    },
  }).then(() => console.log("Generated typescript interfaces for contracts"));
})();
