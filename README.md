# ApiSQL

![ApiSQL Logo](https://github.com/Tony-ArtZ/apisql/raw/main/public/banner.png)

> **Treat the Web like a Database.**

![ApiSQL Suggestions](https://github.com/Tony-ArtZ/apisql/raw/main/public/suggestions.gif)

ApiSQL is a revolutionary query language that brings the expressive power of SQL to REST APIs. It allows you to query any JSON API using familiar syntax (`SELECT`, `WHERE`, `ORDER BY`) without requiring any backend modifications.

Unlike gRPC or GraphQL, which require server-side integration, ApiSQL is **purely frontend-facing**. It acts as an intelligent adapter that runs in your editor (or CLI/Browser), fetching data from standard REST endpoints and transforming it into queryable datasets on the fly.

## ✨ Features

- **Live Schema Inference**: The Language Server executes your requests in the background, analyzes the JSON response, and generates instant auto-completion and type checking.
- **SQL-Like Syntax**: Use `SELECT`, `FROM`, `WHERE`, `LIMIT`, and `ORDER BY` to filter and shape your API data.
- **Zero Config**: Works with any existing JSON API. No `.d.ts` files, no Protobufs, no GraphQL schemas.
- **Cross-Platform**: Runs on Windows, Linux, macOS, and in the Browser (via WASM).

## 🚀 Getting Started

### VS Code Extension

The best way to experience ApiSQL is through the [VS Code Extension](https://marketplace.visualstudio.com/items?itemName=TonyArtZ.apisql-vscode).

1.  Install the extension.
2.  Create a file ending in `.apisql`.
3.  Start writing queries!

### Command Line Interface (CLI)

You can also run ApiSQL queries directly from your terminal.

```bash
# Clone and build
git clone https://github.com/Tony-ArtZ/apisql.git
cd apisql
cargo install --path crates/cli

# Run a query file
apisql run examples/pokemon.apisql
```

### JavaScript / TypeScript Library

ApiSQL is available as an [NPM package](https://www.npmjs.com/package/@tonyartz4/apisql) for use in Node.js or the Browser.

```bash
npm install @tonyartz4/apisql
```

```javascript
import { run_query } from "apisql";
// ...
```

## 📖 Usage Examples

### Basic Querying

Fetch data from an API and filter it just like a database table.

```sql
-- Define the Request
REQUEST GetPokemon
  GET https://pokeapi.co/api/v2/pokemon?limit=100

-- Query the Response
RESPONSE
  FROM body.results             -- 'results' is an array in the JSON response
  WHERE name =~ "^b"            -- Regex matching: starts with 'b'
  ORDER BY name ASC
  SELECT {
    name,
    url,
    id: url                     -- Rename fields or create computed ones
  }
  LIMIT 5
```

### Authenticated Requests

Use variables and headers to query private APIs.

```sql
USING
  token: "my-secret-token"
  baseUrl: "https://api.example.com"

REQUEST GetUsers
  GET {baseUrl}/users
  HEADERS
    Authorization: "Bearer {token}"
  CACHE 60                      -- Cache response for 60 seconds

RESPONSE
  FROM body.data
  WHERE active = true AND age > 21
  SELECT { id, email }
```

## 🏗️ Architecture & How It Works

ApiSQL is built as a modular Rust workspace, designed for performance and portability.

### The "Live Schema" Concept

Traditional tools require you to manually define types (TypeScript interfaces) or rely on the backend to provide a schema (GraphQL/Swagger).

ApiSQL takes a different approach: **"What you see is what you get."**

1.  **Execution**: When you define a `REQUEST`, the `lsp-backend` actually executes it (or uses a cached version).
2.  **Inference**: The `core_lib` analyzes the returned JSON structure.
3.  **Tooling**: This inferred structure is fed into the LSP, providing auto-completion for fields that _actually exist_ in the response.

### Crates Overview

| Crate                    | Description                                                                                                                                                                   |
| ------------------------ | ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| **`crates/core_lib`**    | The brain of the operation. Contains the **AST** (Abstract Syntax Tree), **Lexer**, and **Parser**. It defines the grammar of the ApiSQL language.                            |
| **`crates/runtime`**     | The engine. Handles **HTTP execution** (via `reqwest`), **Caching**, and the **Query Engine** that filters and transforms JSON data based on the AST.                         |
| **`crates/lsp-backend`** | The bridge. Implements the **Language Server Protocol**. It connects VS Code to the `core_lib` and `runtime`, enabling features like "Go to Definition" and live diagnostics. |
| **`crates/cli`**         | The runner. A lightweight command-line tool to execute `.apisql` files and output the results as JSON.                                                                        |
| **`crates/wasm`**        | The web adapter. Bindings that allow the ApiSQL runtime to compile to **WebAssembly**, enabling it to run entirely in the browser.                                            |

## 🛠️ Building from Source

### Prerequisites

- **Rust**: [rustup.rs](https://rustup.rs/)
- **Node.js**: (For the VS Code extension)

### Build Steps

1.  **Build the Rust Workspace**:

    ```bash
    cargo build --release
    ```

2.  **Build the VS Code Extension**:
    The extension requires the `lsp-backend` binary to be placed in `vscode-extension/bin`. We provide a script for this:

    ```bash
    # Builds binaries for Linux and Windows
    ./build-cross.sh

    # Package the extension
    cd vscode-extension
    npm install
    npm run package
    ```

## 🗺️ Roadmap

- [ ] **Persistent Caching**: Currently, caching is in-memory per instance. We plan to add swap file support for persistent caching across sessions.
- [ ] **TypeScript Generation**: Generate TypeScript interfaces (`.d.ts`) directly from ApiSQL queries to ensure end-to-end type safety in your frontend code.
- [ ] **API Joins**: Support for `JOIN` operations to combine data from multiple different APIs in a single query.
- [ ] **More SQL Features**: Support for `GROUP BY`, `HAVING`, and aggregate functions (`COUNT`, `SUM`, `AVG`).
- [ ] **Enhanced Tooling**: Combine multiple `.apisql` files, allowing for reusing components.
- [ ] **Advanced API Handling**: Automatic pagination support, rate limit handling, and OpenAPI/Swagger import capabilities.

## 🤝 Contributing

Contributions are welcome! Please see [CONTRIBUTING.md](CONTRIBUTING.md) for details on how to set up your development environment.

## 📄 License

This project is licensed under the [MIT License](LICENSE).
