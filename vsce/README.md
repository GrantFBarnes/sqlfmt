# sqlfmt - VS Code Extension

This is a Visual Studio Code extension that allows you to utilize `sqlfmt` from within VS Code.

## Requirements

The extension assumes `sqlfmt` is already an executable program on your system.

## Extension Settings

This extension contributes the following settings:

- `sqlfmt.changeKeywordCase`: Change case on keywords
- `sqlfmt.replaceNewlines`: Replace newlines
- `sqlfmt.setSpaceCount`: Set amont of spaces for each level of indent
- `sqlfmt.useTabs`: Use tabs instead of spaces for indentation

## Installation

To install extension ensure you have the `vsce` tool installed.

```sh
npm install -g @vscode/vsce
```

Ensure you are in this directory and package the extension.

```sh
vsce package
```

You can then manully install the `sqlfmt-X.X.X.vsix` file to VS Code as an extension.
