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
  // Path to the compiled binary
  const serverPath = context.asAbsolutePath(path.join("bin", "lsp-backend"));

  const serverExecutable: Executable = {
    command: serverPath,
    args: [],
    options: {
      env: process.env, // Pass environment variables
    },
  };

  // FALLBACK
  const fs = require("fs");
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
