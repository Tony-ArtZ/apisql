# ApiSQL

## -- WORK IN PROGRESS --

ApiSQL is a language for querying APIs using SQL-like syntax.

## VS Code Extension

The VS Code extension provides syntax highlighting, diagnostics, and autocompletion.

## Installation & Building from Source

### Prerequisites

- **Rust**: Install via [rustup](https://rustup.rs/).
- **Node.js**: Install via [nodejs.org](https://nodejs.org/) (v16+ recommended).
- **VSCE**: Install globally via npm: `npm install -g @vscode/vsce`.

### Building Locally (Linux & Windows)

We provide a script to build the language server for Linux and Windows (using MinGW).

1.  **Install Dependencies (Linux)**:
    You may need `mingw-w64` to cross-compile for Windows.
    ```bash
    sudo apt-get install mingw-w64
    ```

2.  **Run the Build Script**:
    ```bash
    ./build-cross.sh
    ```
    This will compile the Rust backend and place the binaries in `vscode-extension/bin`.

3.  **Package the Extension**:
    ```bash
    cd vscode-extension
    npm install
    npm run compile
    vsce package
    ```
    This generates a `.vsix` file that you can install in VS Code.

### Building for macOS

Cross-compiling for macOS from Linux is complex due to linker requirements. We recommend using the provided GitHub Actions workflow or building manually on a macOS machine:

```bash
# On macOS
cargo build --release --bin lsp-backend
mkdir -p vscode-extension/bin
cp target/release/lsp-backend vscode-extension/bin/lsp-backend-darwin-$(uname -m)
```
