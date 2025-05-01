# sqlfmt - SQL Format

Command line SQL formatter.

Usage:
```sh
  sqlfmt -i $INPUT_FILE_PATH
  $INPUT_STREAM | sqlfmt
```

Options:
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
    -s, --spaces <INT> Define amount of spaces per indent
```
