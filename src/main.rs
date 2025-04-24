use std::fs;
use std::io;
use std::process;

mod format;
mod input;
mod token;

fn main() {
    let args: Result<input::Arguments, &str> = input::get_arguments();
    if args.is_err() {
        print_error(args.err().unwrap());
        process::exit(1);
    }
    let args: input::Arguments = args.unwrap();

    if args.help {
        print_help();
        process::exit(0);
    }

    if args.version {
        print_version();
        process::exit(0);
    }

    let sql_in: Result<String, io::Error> = input::get_input_sql(&args.input);
    if sql_in.is_err() {
        print_error(sql_in.err().unwrap().to_string().as_str());
        process::exit(1);
    }
    let sql_in: String = sql_in.unwrap();
    let sql_out: String = format::get_formatted_sql(&args, sql_in);

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

Options:
  Basic
    -h, --help    Print this message
    -v, --version Print version

  IO
    -i, --input  <FILE_PATH> Define path to input SQL file
    -o, --output <FILE_PATH> Define path to output SQL file

  Format Configuration
    -u, --upper        Uppercase keywords
    -l, --lower        Lowercase keywords
    -t, --tabs         Use tabs for indents
    -s, --spaces <INT> Define amount of spaces per indent"
    );
}

fn print_version() {
    let version: &str = env!("CARGO_PKG_VERSION");
    println!("{version}");
}
