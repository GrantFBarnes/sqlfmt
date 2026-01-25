# sqlfmt - SQL Format

![sqlfmt logo](branding/icon.png)

Command line SQL formatter.

[![GitHub Release](https://img.shields.io/github/v/release/grantfbarnes/sqlfmt)](https://github.com/GrantFBarnes/sqlfmt/releases/latest)
[![GitHub Downloads](https://img.shields.io/github/downloads/grantfbarnes/sqlfmt/total)](https://github.com/GrantFBarnes/sqlfmt/releases)
[![Visual Studio Marketplace Installs](https://img.shields.io/visual-studio-marketplace/i/GrantFBarnes.sqlfmt?label=vs%20code%20installs)](https://marketplace.visualstudio.com/items?itemName=GrantFBarnes.sqlfmt)

## Installation

### Manual

The easiest way to get the executable is to download it from the [latest release](https://github.com/grantfbarnes/sqlfmt/releases/latest).

However, if preferred, the executable can be built manually:

```sh
cargo build --release
```

After you have the executable (either through download or manual build), simply place it somewhere in your `PATH`.

### Snap

For linux systems that support [snaps](https://snapcraft.io/).

[![Get it from the Snap Store](https://snapcraft.io/en/dark/install.svg)](https://snapcraft.io/sqlfmt)

### VS Code

For those who prefer to not use the command line, a [VS Code Extension](https://marketplace.visualstudio.com/items?itemName=GrantFBarnes.sqlfmt) is available.

## Usage

```sh
  <INPUT_STREAM> | sqlfmt
  sqlfmt -i <INPUT_FILE_PATH>
```

## Examples

```sh
$ cat input.sql
Select Column1    From Table1    Order By Column1;

$ cat input.sql | sqlfmt
Select Column1 From Table1 Order By Column1;

$ cat input.sql | sqlfmt --upper
SELECT Column1 FROM Table1 ORDER BY Column1;

$ cat input.sql | sqlfmt --newlines
Select
    Column1
From Table1
Order By Column1;

$ sqlfmt --newlines --upper --input input.sql --output output.sql
$ cat output.sql
SELECT
    Column1
FROM Table1
ORDER BY Column1;
```

## Arguments

```
  Basic
    -h, --help    Print help message
    -v, --version Print version

  IO
    -i, --input  <FILE_PATH> Define path to input SQL file
    -o, --output <FILE_PATH> Define path to output SQL file

  Format Configuration
    -n, --newlines      Replace newlines
    --comment-pre-space Replace comment pre-space with indent level
    --align-text-groups Align text by groups inside parentheses
    -u, --upper         Uppercase keywords
    -l, --lower         Lowercase keywords
    -t, --tabs          Use tabs for indents
    -s, --spaces <INT>  Define amount of spaces per indent (default 4)
    -c, --chars  <INT>  Define amount of max chars per line before break (default 80)
```

## Config File

`.sqlfmt`

This program will look for the config file in the current working directory and up (until root).
If found, the file content sets the default configuration values.
Any configuration arguments provided will override these defaults.

Format Configuration

```
newlines
comment_pre_space
align_text_groups
upper
lower
tabs
spaces=<INT>
chars=<INT>
```

## Extensions

A [VS Code extension](vsce/README.md) is offered to allow for easy use of this program as a VS Code language formatter.

For vim/neovim users, you can add keybindings to help run the program.

Here are some examples:

```vim
vnoremap <Leader>sf :!sqlfmt<CR>
vnoremap <Leader>sfn :!sqlfmt -n --comment-pre-space<CR>
vnoremap <Leader>sft :!sqlfmt -n -t --comment-pre-space<CR>
nnoremap <Leader>sf :%!sqlfmt<CR>
nnoremap <Leader>sfn :%!sqlfmt -n --comment-pre-space<CR>
nnoremap <Leader>sft :%!sqlfmt -n -t --comment-pre-space<CR>
```

```lua
vim.keymap.set("v", "<leader>sf", ":!sqlfmt<CR>")
vim.keymap.set("v", "<leader>sfn", ":!sqlfmt -n --comment-pre-space<CR>")
vim.keymap.set("v", "<leader>sft", ":!sqlfmt -n -t --comment-pre-space<CR>")
vim.keymap.set("n", "<leader>sf", ":%!sqlfmt<CR>")
vim.keymap.set("n", "<leader>sfn", ":%!sqlfmt -n --comment-pre-space<CR>")
vim.keymap.set("n", "<leader>sft", ":%!sqlfmt -n -t --comment-pre-space<CR>")
```
