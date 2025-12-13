# Contributing to ApiSQL

Thank you for your interest in contributing to ApiSQL! We welcome contributions from everyone.

## Development Setup

### Prerequisites

- **Rust**: Install the latest stable version via [rustup](https://rustup.rs/).
- **Node.js**: Required for the VS Code extension (v18+ recommended).
- **VS Code**: Recommended editor.

### Building the Project

This is a Rust workspace containing multiple crates.

1.  **Clone the repository**:

    ```bash
    git clone https://github.com/Tony-ArtZ/apisql.git
    cd apisql
    ```

2.  **Build all crates**:

    ```bash
    cargo build
    ```

3.  **Run Tests**:
    ```bash
    cargo test
    ```

### Working on the VS Code Extension

The extension source is in `vscode-extension/`.

1.  **Install Dependencies**:

    ```bash
    cd vscode-extension
    npm install
    ```

2.  **Build the Language Server**:
    The extension needs the `lsp-backend` binary.

    ```bash
    # From the root directory
    ./build-cross.sh
    ```

3.  **Run in Debug Mode**:
    - Open the project in VS Code.
    - Press `F5` to launch the "Extension" debug configuration.
    - This will open a new VS Code window with the extension loaded.

## Project Structure

- **`crates/core_lib`**: The core logic (AST, Parser, Lexer). Start here if you want to change the language syntax.
- **`crates/runtime`**: The execution engine. Handles HTTP requests and query processing.
- **`crates/lsp-backend`**: The Language Server Protocol implementation. Connects VS Code to the core logic.
- **`crates/cli`**: The command-line interface for running `.apisql` files.
- **`crates/wasm`**: WebAssembly bindings for running ApiSQL in the browser.

## Pull Request Guidelines

1.  Ensure `cargo test` passes.
2.  Format your code using `cargo fmt`.
3.  If adding a new feature, please include a test case in `crates/core_lib/tests` or `crates/runtime/tests`.
4.  Update documentation if necessary.

## License

By contributing, you agree that your contributions will be licensed under the MIT License.
