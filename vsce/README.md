# sqlfmt - VS Code Extension

This is a Visual Studio Code extension that allows you to utilize `sqlfmt` from within VS Code.

## Requirements

This extension assumes `sqlfmt` is an already [installed](../README.md##Installation) executable program on your system.

## Installation

The easiest way to get the extension is to download the `vsix` file from the [latest release](https://github.com/grantfbarnes/sqlfmt/releases/latest).

However, if preferred, the extension can be built manually with the following steps:

Ensure you have the `vsce` tool installed.

```sh
npm install -g @vscode/vsce
```

Ensure you are in this directory and package the extension.

```sh
vsce package
```

After you have the `vsix` file (either through download or manual build), you can then manully install the `sqlfmt-X.X.X.vsix` file to VS Code as an extension.

## Operation

This extension is setup as a language formatter extension.
To set the extension as the default formatter for `sql` files, add the following to your VS Code settings:

```json
"[sql]": {
  "editor.defaultFormatter": "GrantFBarnes.sqlfmt"
}
```

This gives you all the standard formatting in VS Code, such as formatting entire files or just highlighted sections.

For situations where you want to format SQL inside another file type, such as a string in another programming language, you can use the `sqlfmt - Format SQL` command.
This will run the program against any highlighted text within any file and replace it with the formated result.

## Extension Settings

This extension contributes the following settings:

- `sqlfmt.useConfigFile`: Controls whether to use config file or settings below
- `sqlfmt.replaceNewlines`: Replace newlines
- `sqlfmt.changeKeywordCase`: Change case on keywords
- `sqlfmt.useTabs`: Use tabs instead of spaces for indentation
- `sqlfmt.setSpaceCount`: Set amount of spaces for each level of indent
- `sqlfmt.setCharCount`: Set amount of chars to determine line breaks
