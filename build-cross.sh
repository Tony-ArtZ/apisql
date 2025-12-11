#!/bin/bash
set -e

# Define targets
# Linux x64
TARGET_LINUX="x86_64-unknown-linux-gnu"
# Windows x64
TARGET_WINDOWS="x86_64-pc-windows-gnu"
# macOS x64 (Intel)
TARGET_MACOS_X64="x86_64-apple-darwin"
# macOS arm64 (Apple Silicon)
TARGET_MACOS_ARM64="aarch64-apple-darwin"

# Output directory
BIN_DIR="vscode-extension/bin"
mkdir -p $BIN_DIR

echo "Cleaning old binaries..."
rm -f $BIN_DIR/lsp-backend-*

echo "Building for Linux ($TARGET_LINUX)..."
cargo build --release --bin lsp-backend --target $TARGET_LINUX
cp target/$TARGET_LINUX/release/lsp-backend $BIN_DIR/lsp-backend-linux-x64
echo "✓ Linux x64"

echo "Building for Windows ($TARGET_WINDOWS)..."
cargo build --release --bin lsp-backend --target $TARGET_WINDOWS
cp target/$TARGET_WINDOWS/release/lsp-backend.exe $BIN_DIR/lsp-backend-win32-x64.exe
echo "✓ Windows x64"

# MACOS builds doesnt work when using Zig linker with cargo-zigbuild. Sad... :(

# echo "Building for macOS Intel ($TARGET_MACOS_X64)..."
# MACOSX_DEPLOYMENT_TARGET=12.0 cargo zigbuild --release --bin lsp-backend --target $TARGET_MACOS_X64
# cp target/$TARGET_MACOS_X64/release/lsp-backend $BIN_DIR/lsp-backend-darwin-x64
# echo "✓ macOS x64"

# echo "Building for macOS Apple Silicon ($TARGET_MACOS_ARM64)..."
# MACOSX_DEPLOYMENT_TARGET=12.0 cargo zigbuild --release --bin lsp-backend --target $TARGET_MACOS_ARM64
# cp target/$TARGET_MACOS_ARM64/release/lsp-backend $BIN_DIR/lsp-backend-darwin-arm64
# echo "✓ macOS arm64"

echo "Build complete! Binaries are in $BIN_DIR"
ls -lh $BIN_DIR
