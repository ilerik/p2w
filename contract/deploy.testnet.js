const sh = require("shelljs");
//const { CONTRACT, MASTER_ACCOUNT } = process.env;
//const DELETE_BEFORE_DEPLOY = process.env.DELETE_BEFORE_DEPLOY === "true";
const CONTRACT = "p2w-v1.ilerik.testnet";
const MASTER_ACCOUNT = "ilerik.testnet";
const DELETE_BEFORE_DEPLOY = false;

// Recreate account
// if (DELETE_BEFORE_DEPLOY) {
//   sh.exec(`near delete ${CONTRACT} ${MASTER_ACCOUNT}`);
// }
// sh.exec(
//   `near create-account ${CONTRACT} --masterAccount=${MASTER_ACCOUNT} --initial-balance 30`
// );

// Deploy contract
sh.exec(
  `near deploy --wasmFile contract/target/wasm32-unknown-unknown/release/p2w.wasm --accountId ${CONTRACT}`
);

// Test run
//sh.exec(`near call ${CONTRACT} new --accountId ${CONTRACT}`);
//sh.exec(`near view ${CONTRACT} get_evidences_amount`);