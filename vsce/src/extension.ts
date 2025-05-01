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
  const disposable = vscode.commands.registerCommand("sqlfmt.sqlfmt", () => {
    const editor = vscode.window.activeTextEditor;
    const selection = editor?.selection;
    if (!selection) return;

    let textRange: vscode.Range | null = null;
    if (selection.isEmpty) {
      let firstLine = editor.document.lineAt(0);
      let lastLine = editor.document.lineAt(editor.document.lineCount - 1);
      textRange = new vscode.Range(firstLine.range.start, lastLine.range.end);
    } else {
      textRange = new vscode.Range(
        selection.start.line,
        selection.start.character,
        selection.end.line,
        selection.end.character
      );
    }

    const sql_in = editor.document.getText(textRange);

    let args = [];

    let config = vscode.workspace.getConfiguration();

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
      args.push("-s");
      args.push(config.get("sqlfmt.setSpaceCount"));
    }

    let p = cp.spawn("sqlfmt", args);
    p.stdin.write(sql_in);
    p.stdin.end();

    p.stdout.on("data", (sql_out: any) => {
      editor.edit((editBuilder) => {
        editBuilder.replace(textRange, sql_out.toString());
      });
      vscode.window.showInformationMessage("Code is formatted");
    });

    p.stderr.on("data", (data: any) => {
      vscode.window.showErrorMessage(`stderr: ${data}`);
    });

    p.on("close", (code: any) => {
      console.log(`child process exited with code ${code}`);
    });
  });

  context.subscriptions.push(disposable);
}

// This method is called when your extension is deactivated
export function deactivate() {}
