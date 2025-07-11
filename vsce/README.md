# sqlfmt - VS Code Extension

This is a Visual Studio Code extension that allows you to utilize `sqlfmt` from within VS Code.

## Requirements

The extension assumes `sqlfmt` is already an executable program on your system.

## Extension Settings

This extension contributes the following settings:

- `sqlfmt.useConfigFile`: Controls whether to use config file or settings below
- `sqlfmt.replaceNewlines`: Replace newlines
- `sqlfmt.changeKeywordCase`: Change case on keywords
- `sqlfmt.useTabs`: Use tabs instead of spaces for indentation
- `sqlfmt.setSpaceCount`: Set amount of spaces for each level of indent
- `sqlfmt.setCharCount`: Set amount of chars to determine line breaks

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
