# sqlfmt - SQL Format

This extension allows you to utilize [sqlfmt](https://github.com/GrantFBarnes/sqlfmt) from within VS Code.

## Requirements

This extension assumes `sqlfmt` is an already [installed](https://github.com/GrantFBarnes/sqlfmt?tab=readme-ov-file#installation) executable program on your system.

## Installation

The easiest way to get the extension is to download the `vsix` file from the [latest release](https://github.com/grantfbarnes/sqlfmt/releases/latest).
However, if preferred, the extension can be built manually following official [instructions](https://code.visualstudio.com/api/working-with-extensions/publishing-extension).

After you have the `vsix` file (either through download or manual build), you can then manully [install](https://code.visualstudio.com/api/working-with-extensions/publishing-extension#packaging-extensions) the `sqlfmt-X.X.X.vsix` file to VS Code as an extension.

## Operation

This extension is setup as a language formatter extension.
Meaning it can be [set](https://code.visualstudio.com/docs/configure/settings#_language-specific-editor-settings) as the default formatter for `sql` files.
This gives you all the standard formatting in VS Code, such as formatting entire files or just highlighted sections.

For situations where you want to format SQL inside another file type, such as a string in another programming language, you can use the `sqlfmt - Format SQL` [command](https://code.visualstudio.com/docs/getstarted/userinterface#_command-palette).
This will run the program against any highlighted text within any file and replace it with the formatted result.

## Settings

The list of settings can be seen in the `FEATURES` -> `Settings` tab of the [extension details page](https://code.visualstudio.com/docs/editor/extension-marketplace#_extension-details).
