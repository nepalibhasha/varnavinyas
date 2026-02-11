import * as fs from "fs";
import * as path from "path";
import * as vscode from "vscode";
import {
  LanguageClient,
  LanguageClientOptions,
  ServerOptions,
} from "vscode-languageclient/node";

let client: LanguageClient | undefined;

const OUTPUT = vscode.window.createOutputChannel("Varnavinyas");

const BINARY_NAME =
  process.platform === "win32" ? "varnavinyas-lsp.exe" : "varnavinyas-lsp";

function findServerBinary(extensionPath: string): string {
  const config = vscode.workspace.getConfiguration("varnavinyas");
  const explicit = config.get<string>("serverPath", "");
  if (explicit) {
    if (fs.existsSync(explicit)) {
      OUTPUT.appendLine(`Using explicit serverPath: ${explicit}`);
      return explicit;
    }
    OUTPUT.appendLine(`WARNING: explicit serverPath not found: ${explicit}, falling back to auto-detect`);
    vscode.window.showWarningMessage(
      `Varnavinyas: serverPath not found: ${explicit} — falling back to auto-detect`,
    );
  }

  // Collect candidate roots: workspace folders + walk up from extension dir
  const roots: string[] = [];

  const folders = vscode.workspace.workspaceFolders;
  if (folders) {
    for (const folder of folders) {
      roots.push(folder.uri.fsPath);
    }
  }

  // Walk up from extension directory to find project root with target/
  let dir = extensionPath;
  for (let i = 0; i < 5; i++) {
    roots.push(dir);
    const parent = path.dirname(dir);
    if (parent === dir) break;
    dir = parent;
  }

  for (const root of roots) {
    for (const profile of ["release", "debug"]) {
      const candidate = path.join(root, "target", profile, BINARY_NAME);
      if (fs.existsSync(candidate)) {
        OUTPUT.appendLine(`Auto-detected server at: ${candidate}`);
        return candidate;
      }
    }
  }

  OUTPUT.appendLine(
    "Could not auto-detect varnavinyas-lsp, falling back to PATH",
  );
  return BINARY_NAME;
}

export function activate(context: vscode.ExtensionContext): void {
  const serverPath = findServerBinary(context.extensionPath);
  OUTPUT.appendLine(`Activating Varnavinyas LSP with server: ${serverPath}`);

  const serverOptions: ServerOptions = {
    command: serverPath,
    args: [],
  };

  const config = vscode.workspace.getConfiguration("varnavinyas");
  const selector: { language: string; scheme: string }[] = [
    { language: "nepali", scheme: "file" },
  ];
  if (config.get<boolean>("enablePlaintext", false)) {
    selector.push({ language: "plaintext", scheme: "file" });
  }

  const clientOptions: LanguageClientOptions = {
    documentSelector: selector,
    synchronize: {
      configurationSection: "varnavinyas",
    },
  };

  client = new LanguageClient(
    "varnavinyas",
    "Varnavinyas",
    serverOptions,
    clientOptions,
  );

  client.start();
  context.subscriptions.push({
    dispose: () => {
      if (client) {
        client.stop();
      }
    },
  });
}

export function deactivate(): Thenable<void> | undefined {
  if (client) {
    return client.stop();
  }
  return undefined;
}
