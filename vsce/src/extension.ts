import * as vscode from "vscode";
import cp from "child_process";
import * as path from "path";
import { getBinaryPath } from "./binary";

export function activate(context: vscode.ExtensionContext) {
  getBinaryPath(context.globalStorageUri.fsPath)
    .then((binaryPath) => {
      context.subscriptions.push(getSqlfmtCommand(binaryPath));
      setupLanguageFormatting(binaryPath);
    })
    .catch(() => {
      context.subscriptions.push(getSqlfmtCommand(null));
      vscode.window.showErrorMessage("Failed to find sqlfmt binary.");
    });
}

export function deactivate() { }

function getSqlfmtCommand(binaryPath: string | null): vscode.Disposable {
  return vscode.commands.registerCommand("sqlfmt.sqlfmt", () => {
    if (binaryPath == null) {
      vscode.window.showErrorMessage("Could not find sqlfmt binary.");
      return;
    }

    const editor = vscode.window.activeTextEditor;
    if (editor == null) {
      vscode.window.showErrorMessage("Could not find active editor.");
      return;
    }

    const range = getRangeToFormat();
    if (range == null) {
      vscode.window.showErrorMessage("Could not gather text to format.");
      return;
    }

    getFormattedSql(binaryPath, editor.document, editor.document.getText(range))
      .then((formattedSql: string) => {
        editor.edit((tee: vscode.TextEditorEdit) => tee.replace(range, formattedSql));
        vscode.window.showInformationMessage("SQL is formatted.");
      }).catch(() => {
        vscode.window.showErrorMessage("Failed to format SQL.");
      });
  });
}

function setupLanguageFormatting(binaryPath: string) {
  vscode.languages.registerDocumentFormattingEditProvider("sql", {
    async provideDocumentFormattingEdits(document: vscode.TextDocument): Promise<vscode.TextEdit[]> {
      const firstLine = document.lineAt(0);
      const lastLine = document.lineAt(document.lineCount - 1);
      const range = new vscode.Range(firstLine.range.start, lastLine.range.end);

      try {
        const formattedSql: string = await getFormattedSql(binaryPath, document, document.getText(range));
        return [vscode.TextEdit.replace(range, formattedSql)];
      }
      catch {
        vscode.window.showErrorMessage("Failed to format SQL.");
        return [];
      }
    }
  });

  vscode.languages.registerDocumentRangeFormattingEditProvider("sql", {
    async provideDocumentRangeFormattingEdits(document: vscode.TextDocument, range: vscode.Range): Promise<vscode.TextEdit[]> {
      try {
        const formattedSql: string = await getFormattedSql(binaryPath, document, document.getText(range));
        return [vscode.TextEdit.replace(range, formattedSql)];
      }
      catch {
        vscode.window.showErrorMessage("Failed to format SQL.");
        return [];
      }
    }
  });
}

function getRangeToFormat(): vscode.Range | null {
  const editor = vscode.window.activeTextEditor;
  const selection = editor?.selection;
  if (!selection) return null;

  if (selection.isEmpty) {
    const firstLine = editor.document.lineAt(0);
    const lastLine = editor.document.lineAt(editor.document.lineCount - 1);
    return new vscode.Range(firstLine.range.start, lastLine.range.end);
  } else {
    return new vscode.Range(
      selection.start.line,
      selection.start.character,
      selection.end.line,
      selection.end.character
    );
  }
}

function getFormattedSql(binaryPath: string, document: vscode.TextDocument, inputSql: string): Promise<string> {
  return new Promise((resolve, reject) => {
    try {
      const processArguments: string[] = getProcessArguments();
      const process = cp.spawn(binaryPath, processArguments, { cwd: getProcessCwd(document) });

      process.stdin.write(inputSql);
      process.stdin.end();

      let outputSql: string = "";

      process.stdout.on("data", (data: any) => {
        outputSql += data.toString();
      });

      process.on("close", (code: any) => {
        if (code === 0) {
          // sqlfmt ends output with a newline, only adding one when the
          // formatted sql lacks it, so remove it unless the input had one
          if (outputSql.endsWith("\n") && !inputSql.endsWith("\n")) {
            outputSql = outputSql.replace(/\n$/, "");
          }
          resolve(outputSql);
        } else {
          reject(`Process exited with code ${code}`);
        }
      });

      process.on("error", (error: any) => {
        reject(`Process error: ${error}`);
      });
    }
    catch (error: any) {
      reject(`Error: ${error}`);
    }
  });
}

// sqlfmt looks for a .sqlfmt config file from its working directory up, so
// run it from the document's folder (or the workspace for unsaved documents)
function getProcessCwd(document: vscode.TextDocument): string | undefined {
  if (document.uri.scheme === "file") {
    return path.dirname(document.uri.fsPath);
  }

  const workspaceFolder = vscode.workspace.getWorkspaceFolder(document.uri)
    ?? vscode.workspace.workspaceFolders?.[0];
  if (workspaceFolder?.uri.scheme === "file") {
    return workspaceFolder.uri.fsPath;
  }

  return undefined;
}

function getProcessArguments(): string[] {
  const config = vscode.workspace.getConfiguration();

  if (config.get("sqlfmt.useConfigFile")) {
    return [];
  }

  // extension settings are used instead, so ignore any .sqlfmt config file
  let args: string[] = ["--no-config"];

  if (config.get("sqlfmt.replaceNewlines")) {
    args.push("-n");
  }

  if (config.get("sqlfmt.replaceCommentPreSpace")) {
    args.push("--comment-pre-space");
  }

  if (config.get("sqlfmt.alignTextGroups")) {
    args.push("--align-text-groups");
  }

  if (config.get("sqlfmt.backslashEscapes")) {
    args.push("--backslash-escapes");
  }

  switch (config.get("sqlfmt.changeKeywordCase")) {
    case "uppercase":
      args.push("-u");
      break;

    case "lowercase":
      args.push("-l");
      break;

    default:
      break;
  }

  if (config.get("sqlfmt.useTabs")) {
    args.push("-t");
  } else {
    const spaceCount = config.get("sqlfmt.setSpaceCount");
    if (typeof spaceCount == "number") {
      args.push("-s");
      args.push(spaceCount.toString());
    }
  }

  const charCount = config.get("sqlfmt.setCharCount");
  if (typeof charCount == "number") {
    args.push("-c");
    args.push(charCount.toString());
  }

  return args;
}