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
