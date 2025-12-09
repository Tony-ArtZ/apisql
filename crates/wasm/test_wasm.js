const { run } = require("./pkg/apisql_wasm.js");
const fs = require("fs");
const path = require("path");

async function main() {
  const source = fs.readFileSync("../../examples/pokemon.apisql", "utf8");
  try {
    const result = await run(source);
    console.log(JSON.stringify(result, null, 2));
  } catch (e) {
    console.error("Error:", e);
  }
}

main();
