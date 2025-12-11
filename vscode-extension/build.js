const { execSync } = require("child_process");
const fs = require("fs");
const path = require("path");
const os = require("os");

const rootDir = path.resolve(__dirname, "..");
const extensionDir = __dirname;
const binDir = path.join(extensionDir, "bin");

// Ensure bin directory
if (!fs.existsSync(binDir)) {
  fs.mkdirSync(binDir, { recursive: true });
}

const isWindows = os.platform() === "win32";
const binaryName = isWindows ? "lsp-backend.exe" : "lsp-backend";
const targetDir = path.join(rootDir, "target", "release");
const sourcePath = path.join(targetDir, binaryName);
const destPath = path.join(binDir, binaryName);

console.log("Building LSP backend...");
try {
  execSync("cargo build --release --bin lsp-backend", {
    cwd: rootDir,
    stdio: "inherit",
  });
} catch (error) {
  console.error("Failed to build LSP backend");
  process.exit(1);
}

console.log(`Copying binary from ${sourcePath} to ${destPath}...`);
if (fs.existsSync(sourcePath)) {
  fs.copyFileSync(sourcePath, destPath);
  console.log("Build complete.");
} else {
  console.error(`Binary not found at ${sourcePath}`);
  process.exit(1);
}
