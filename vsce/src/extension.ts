// The module 'vscode' contains the VS Code extensibility API
// Import the module and reference it with the alias vscode in your code below
import * as vscode from "vscode";
import cp from "child_process";

// This method is called when your extension is activated
// Your extension is activated the very first time the command is executed
export function activate(context: vscode.ExtensionContext) {
  // The command has been defined in the package.json file
  // Now provide the implementation of the command with registerCommand
  // The commandId parameter must match the command field in package.json
  const sqlfmt = vscode.commands.registerCommand("sqlfmt.sqlfmt", () => {
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

    const sqlIn = editor.document.getText(range);
    const args = getSqlFmtArguments();
    const process = cp.spawn("sqlfmt", args);
    process.stdin.write(sqlIn);
    process.stdin.end();

    process.stdout.on("data", (sqlOut: any) => {
      editor.edit((editBuilder) => {
        editBuilder.replace(range, sqlOut.toString());
      });
      vscode.window.showInformationMessage("SQL is formatted.");
    });

    process.stderr.on("data", (data: any) => {
      vscode.window.showErrorMessage(`stderr: ${data}`);
    });

    process.on("close", (code: any) => {
      console.log(`child process exited with code ${code}`);
    });
  });

  context.subscriptions.push(sqlfmt);
}

// This method is called when your extension is deactivated
export function deactivate() { }

function getRangeToFormat(): vscode.Range | null {
  let range: vscode.Range | null = null;

  const editor = vscode.window.activeTextEditor;
  const selection = editor?.selection;
  if (!selection) return range;

  if (selection.isEmpty) {
    const firstLine = editor.document.lineAt(0);
    const lastLine = editor.document.lineAt(editor.document.lineCount - 1);
    range = new vscode.Range(firstLine.range.start, lastLine.range.end);
  } else {
    range = new vscode.Range(
      selection.start.line,
      selection.start.character,
      selection.end.line,
      selection.end.character
    );
  }

  return range;
}

function getSqlFmtArguments(): string[] {
  let args: string[] = [];

  const config = vscode.workspace.getConfiguration();

  if (!config.get("sqlfmt.useConfigFile")) {
    if (config.get("sqlfmt.replaceNewlines")) {
      args.push("-n");
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
      let spaceCount = config.get("sqlfmt.setSpaceCount");
      if (typeof spaceCount == "number") {
        args.push("-s");
        args.push(spaceCount.toString());
      }
    }

    let charCount = config.get("sqlfmt.setCharCount");
    if (typeof charCount == "number") {
      args.push("-c");
      args.push(charCount.toString());
    }
  }

  return args;
}