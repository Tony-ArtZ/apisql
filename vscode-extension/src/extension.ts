import * as path from "path";
import { workspace, ExtensionContext } from "vscode";
import {
  LanguageClient,
  LanguageClientOptions,
  ServerOptions,
  TransportKind,
  Executable,
} from "vscode-languageclient/node";

let client: LanguageClient;

export function activate(context: ExtensionContext) {
  const platform = process.platform;
  let binaryName = "lsp-backend";
  if (platform === "win32") {
    binaryName = "lsp-backend.exe";
  }

  const arch = process.arch;
  const platformBinaryName =
    platform === "win32"
      ? `lsp-backend-win32-${arch}.exe`
      : `lsp-backend-${platform}-${arch}`;
  const platformServerPath = context.asAbsolutePath(
    path.join("bin", platformBinaryName)
  );

  const legacyPlatformBinaryName =
    platform === "win32" ? "lsp-backend-win32.exe" : `lsp-backend-${platform}`;
  const legacyPlatformServerPath = context.asAbsolutePath(
    path.join("bin", legacyPlatformBinaryName)
  );

  const defaultServerPath = context.asAbsolutePath(
    path.join("bin", binaryName)
  );

  const fs = require("fs");
  let serverPath = defaultServerPath;
  if (fs.existsSync(platformServerPath)) {
    serverPath = platformServerPath;
  } else if (fs.existsSync(legacyPlatformServerPath)) {
    serverPath = legacyPlatformServerPath;
  }

  if (fs.existsSync(serverPath) && platform !== "win32") {
    try {
      fs.chmodSync(serverPath, 0o755);
    } catch (err) {
      console.error("Failed to set execute permissions on LSP binary:", err);
    }
  }

  const serverExecutable: Executable = {
    command: serverPath,
    args: [],
    options: {
      env: process.env,
    },
  };

  // FALLBACK
  if (!fs.existsSync(serverPath)) {
    serverExecutable.command = "cargo";
    serverExecutable.args = ["run", "--bin", "lsp-backend", "--quiet"];
    serverExecutable.options = { cwd: path.join(context.extensionPath, "..") };
  }

  const serverOptions: ServerOptions = {
    run: serverExecutable,
    debug: serverExecutable,
  };

  const clientOptions: LanguageClientOptions = {
    documentSelector: [{ scheme: "file", language: "apisql" }],
    synchronize: {
      fileEvents: workspace.createFileSystemWatcher("**/.clientrc"),
    },
  };

  client = new LanguageClient(
    "apisql",
    "ApiSQL Language Server",
    serverOptions,
    clientOptions
  );

  client.start();
}

export function deactivate(): Thenable<void> | undefined {
  if (!client) {
    return undefined;
  }
  return client.stop();
}
