![ApiSQL Logo](https://github.com/Tony-ArtZ/apisql/raw/main/public/banner,png)

# ApiSQL WASM

![ApiSQL Suggestions](https://github.com/Tony-ArtZ/apisql/raw/main/public/suggestions.gif)

The WebAssembly bindings for [ApiSQL](https://github.com/Tony-ArtZ/apisql), allowing you to run ApiSQL queries directly in Node.js or the browser (via bundlers).

## Installation

```bash
npm install @tonyartz4/apisql
```

## Usage

### Node.js (CommonJS)

```javascript
const { run } = require("@tonyartz4/apisql");

async function main() {
  const query = `
    REQUEST GetPokemon
      GET https://pokeapi.co/api/v2/pokemon?limit=5

    RESPONSE
      FROM body.results
      SELECT { name, url }
  `;

  try {
    // The run function returns the result directly as a JavaScript object
    const result = await run(query);
    console.log(result);
  } catch (e) {
    console.error("Query failed:", e);
  }
}

main();
```

### ES Modules / Bundlers (Webpack, Vite, etc.)

```javascript
import { run } from "@tonyartz4/apisql";

const query = `
  REQUEST GetPokemon
    GET https://pokeapi.co/api/v2/pokemon?limit=5

  RESPONSE
    FROM body.results
    SELECT { name, url }
`;

run(query)
  .then((result) => console.log(result))
  .catch((err) => console.error(err));
```

## API Reference

### `run(source: string): Promise<any>`

Executes an ApiSQL query string and returns the result.

- `source`: The ApiSQL query string.
- Returns: A `Promise` that resolves to the query result (as a JavaScript object) or rejects with an error.

## Building from Source

To build the WASM package locally:

1. Install `wasm-pack`: https://rustwasm.github.io/wasm-pack/installer/
2. Run the build script:

```bash
cd crates/wasm
npm run build
```

## License

MIT
