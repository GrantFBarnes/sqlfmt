# sqlfmt - SQL Format

Command line SQL formatter.

## Installation

The easiest way to get the executable is to download it from the [latest release](https://github.com/grantfbarnes/sqlfmt/releases/latest).

However, if preferred, the executable can be built manually:

```sh
cargo build --release
```

After you have the executable (either through download or manual build), simply place it somewhere in your PATH.

## Usage

```sh
  sqlfmt -i $INPUT_FILE_PATH
  $INPUT_STREAM | sqlfmt
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
    -n, --newlines     Replace newlines
    -u, --upper        Uppercase keywords
    -l, --lower        Lowercase keywords
    -t, --tabs         Use tabs for indents
    -s, --spaces <INT> Define amount of spaces per indent (default 4)
    -c, --chars  <INT> Define amount of max chars per line before break (default 80)
```

## Config File

`.sqlfmt`

This program will look for the config file in the current working directory and up (until root).
If found, the file content sets the default configuration values.
Any configuration arguments provided will override these defaults.

Format Configuration

```
newlines
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
vnoremap <Leader>snf :!sqlfmt -n<CR>
nnoremap <Leader>sf ggVG:!sqlfmt<CR>
nnoremap <Leader>snf ggVG:!sqlfmt -n<CR>
```

```lua
vim.keymap.set("v", "<leader>sf", ":!sqlfmt<CR>")
vim.keymap.set("v", "<leader>snf", ":!sqlfmt -n<CR>")
vim.keymap.set("n", "<leader>sf", "ggVG:!sqlfmt<CR>")
vim.keymap.set("n", "<leader>snf", "ggVG:!sqlfmt -n<CR>")
```
