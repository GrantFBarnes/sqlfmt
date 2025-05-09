use std::env;
use std::fs;
use std::io::{self, IsTerminal};
use std::process;

mod arguments;
mod configuration;
mod format;
mod token;

fn main() {
    let args: Result<arguments::Arguments, &str> = arguments::Arguments::from(env::args().skip(1));
    if args.is_err() {
        print_error(args.err().unwrap());
        process::exit(1);
    }
    let args: arguments::Arguments = args.unwrap();

    if args.help {
        print_help();
        process::exit(0);
    }

    if args.version {
        print_version();
        process::exit(0);
    }

    let config: configuration::Configuration = configuration::Configuration::from(&args);

    let sql_in: Result<String, io::Error> = get_input_sql(&args.input);
    if sql_in.is_err() {
        print_error(sql_in.err().unwrap().to_string().as_str());
        process::exit(1);
    }
    let sql_in: String = sql_in.unwrap();

    let sql_out: String = format::get_formatted_sql(&config, sql_in);

    if args.output.is_some() {
        match fs::write(args.output.unwrap(), &sql_out) {
            Ok(_) => (),
            Err(result) => {
                print_error(result.to_string().as_str());
                process::exit(1);
            }
        }
    } else {
        println!("{sql_out}");
    }

    process::exit(0);
}

const FONT_RED: &str = "\x1b[31m";
const FONT_RESET: &str = "\x1b[0m";

fn print_error(msg: &str) {
    println!("{FONT_RED}Error:{FONT_RESET} {msg}");
    println!("Run with -h/--help to print help.");
}

fn print_help() {
    println!(
        "sqlfmt - SQL Format

Usage:
  sqlfmt -i <INPUT_FILE_PATH>
  <INPUT_STREAM> | sqlfmt

Arguments:
  Basic
    -h, --help    Print this message
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

Config File:
  .sqlfmt

  Program will look for file in current working directory and up (until root).
  If found, file content sets the default configuration values.
  Any configuration arguments provided will override these defaults.

  Format Configuration
    newlines
    upper
    lower
    tabs
    spaces=<INT>"
    );
}

fn print_version() {
    let version: &str = env!("CARGO_PKG_VERSION");
    println!("{version}");
}

fn get_input_sql(input: &Option<String>) -> Result<String, io::Error> {
    let stdin: io::Stdin = io::stdin();
    if stdin.is_terminal() {
        if input.is_none() {
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "Input file not defined.",
            ));
        }
        return fs::read_to_string(input.as_ref().unwrap());
    } else {
        return io::read_to_string(stdin);
    }
}
