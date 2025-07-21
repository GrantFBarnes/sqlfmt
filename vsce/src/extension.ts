import * as vscode from "vscode";
import cp from "child_process";

export function activate(context: vscode.ExtensionContext) {
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

    getFormattedSql(editor.document, range).then((formattedSql: string) => {
      vscode.window.showInformationMessage("SQL is formatted.");
      editor.edit((editBuilder) => {
        editBuilder.replace(range, formattedSql);
      });
    }).catch(() => { });
  });

  context.subscriptions.push(sqlfmt);

  vscode.languages.registerDocumentFormattingEditProvider('sql', {
    async provideDocumentFormattingEdits(document: vscode.TextDocument): Promise<vscode.TextEdit[]> {
      const firstLine = document.lineAt(0);
      const lastLine = document.lineAt(document.lineCount - 1);
      const range = new vscode.Range(firstLine.range.start, lastLine.range.end);

      const formattedSql: string = await getFormattedSql(document, range);
      return formattedSql ? [vscode.TextEdit.replace(range, formattedSql)] : [];
    }
  });

  vscode.languages.registerDocumentRangeFormattingEditProvider('sql', {
    async provideDocumentRangeFormattingEdits(document: vscode.TextDocument, range: vscode.Range): Promise<vscode.TextEdit[]> {
      const formattedSql: string = await getFormattedSql(document, range);
      return formattedSql ? [vscode.TextEdit.replace(range, formattedSql)] : [];
    }
  });
}

export function deactivate() { }

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

function getFormattedSql(document: vscode.TextDocument, range: vscode.Range): Promise<string> {
  return new Promise((resolve, reject) => {
    let formattedSql = "";

    const unformattedSql = document.getText(range);
    const processArguments = getProcessArguments();
    const process = cp.spawn("sqlfmt", processArguments);
    process.stdin.write(unformattedSql);
    process.stdin.end();

    process.stdout.on("data", (data: any) => {
      formattedSql += data.toString();
    });

    process.stderr.on("data", (data: any) => {
      vscode.window.showErrorMessage(`stderr: ${data}`);
    });

    process.on("close", (code: any) => {
      if (code === 0) {
        // remove single extra newline if found
        if (formattedSql.endsWith("\n")) {
          formattedSql = formattedSql.replace(/\n$/, "");
        }
        resolve(formattedSql);
      } else {
        reject(`Process exited with code ${code}`);
      }
    });

    process.on("error", (error: any) => {
      vscode.window.showErrorMessage(`error: ${error}`);
      reject(`Process error: ${error}`);
    });
  });
}

function getProcessArguments(): string[] {
  const config = vscode.workspace.getConfiguration();

  if (config.get("sqlfmt.useConfigFile")) {
    return [];
  }

  let args: string[] = [];

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