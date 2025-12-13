# ApiSQL WASM

The WebAssembly bindings for [ApiSQL](https://github.com/Tony-ArtZ/apisql), allowing you to run ApiSQL queries directly in the browser or Node.js.

## Installation

```bash
npm install apisql
```

## Usage

### In the Browser

```javascript
import init, { run_query } from "apisql";

async function main() {
  await init();

  const query = `
        REQUEST GetPokemon
          GET https://pokeapi.co/api/v2/pokemon?limit=5

        RESPONSE
          FROM body.results
          SELECT { name, url }
    `;

  try {
    const result = await run_query(query);
    console.log(JSON.parse(result));
  } catch (e) {
    console.error("Query failed:", e);
  }
}

main();
```

### In Node.js

```javascript
const { run_query } = require("apisql");

const query = `...`;
// ... usage
```

## Building from Source

```bash
wasm-pack build --target web
```

## License

MIT
