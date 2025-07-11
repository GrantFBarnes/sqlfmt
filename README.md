# sqlfmt - SQL Format

Command line SQL formatter.

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

Program will look for file in current working directory and up (until root).
If found, file content sets the default configuration values.
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

## Installation

The easiest way to get the executable is to download it from the [latest release](https://github.com/grantfbarnes/sqlfmt/releases/latest).

However, if preferred, the executable can be built manually:

```sh
cargo build --release
```

After you have the executable (either through download or manual build), simply place it somewhere in your PATH.
