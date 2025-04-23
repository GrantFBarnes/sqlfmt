use crate::format::{CaseSetting, Settings};

use std::{
    env, fs,
    io::{self, IsTerminal, Stdin},
    process,
};

const FONT_RED: &str = "\x1b[31m";
const FONT_RESET: &str = "\x1b[0m";

pub fn get_settings_from_args() -> Settings {
    let mut settings: Settings = Settings::new();

    enum ArgType {
        Input,
        Output,
        Spaces,
    }
    let mut arg_type: Option<ArgType> = None;

    for arg in env::args().skip(1) {
        match arg.as_str() {
            "-h" | "--help" => {
                print_help();
                process::exit(0);
            }
            "-v" | "--version" => {
                print_version();
                process::exit(0);
            }
            "-i" | "--input" => {
                if arg_type.is_some() {
                    print_error("Invalid arguments provided.");
                    process::exit(1);
                }
                arg_type = Some(ArgType::Input);
            }
            "-o" | "--output" => {
                if arg_type.is_some() {
                    print_error("Invalid arguments provided.");
                    process::exit(1);
                }
                arg_type = Some(ArgType::Output);
            }
            "-u" | "--upper" => {
                if settings.case.is_some() {
                    print_error("Case setting already defined.");
                    process::exit(1);
                }
                settings.case = Some(CaseSetting::Upper);
            }
            "-l" | "--lower" => {
                if settings.case.is_some() {
                    print_error("Case setting already defined.");
                    process::exit(1);
                }
                settings.case = Some(CaseSetting::Lower);
            }
            "-t" | "--tabs" => {
                if settings.tabs {
                    print_error("Tabs setting already defined.");
                    process::exit(1);
                }
                settings.tabs = true;
            }
            "-s" | "--spaces" => {
                if arg_type.is_some() {
                    print_error("Invalid arguments provided.");
                    process::exit(1);
                }
                if settings.tabs {
                    print_error("Tabs setting already defined.");
                    process::exit(1);
                }
                arg_type = Some(ArgType::Spaces);
            }
            _ => match arg_type {
                Some(ArgType::Input) => {
                    if settings.input.is_some() {
                        print_error("Input was already defined.");
                        process::exit(1);
                    }
                    settings.input = Some(arg);
                    arg_type = None;
                }
                Some(ArgType::Output) => {
                    if settings.output.is_some() {
                        print_error("Output was already defined.");
                        process::exit(1);
                    }
                    settings.output = Some(arg);
                    arg_type = None;
                }
                Some(ArgType::Spaces) => {
                    let spaces: Result<usize, std::num::ParseIntError> = arg.parse::<usize>();
                    if spaces.is_err() {
                        print_error("Invalid space count provided.");
                        process::exit(1);
                    }
                    settings.spaces = spaces.unwrap();
                    arg_type = None;
                }
                None => {
                    print_error("Invalid arguments provided.");
                    process::exit(1);
                }
            },
        }
    }

    if arg_type.is_some() {
        print_error("Invalid arguments provided.");
        process::exit(1);
    }

    return settings;
}

pub fn get_input_sql(settings: &Settings) -> String {
    let sql_input: Result<String, io::Error>;
    let stdin: Stdin = io::stdin();
    if stdin.is_terminal() {
        if settings.input.is_none() {
            print_error("Input file not provided.");
            process::exit(1);
        }

        sql_input = fs::read_to_string(settings.input.as_ref().unwrap());
    } else {
        sql_input = io::read_to_string(stdin);
    }

    if sql_input.is_err() {
        print_error("Failed to read input of SQL.");
        process::exit(1);
    }

    return sql_input.unwrap();
}

pub fn output_result(settings: &Settings, sql_out: &String) {
    if settings.output.is_some() {
        match fs::write(settings.output.as_ref().unwrap(), sql_out) {
            Ok(_) => (),
            Err(_) => {
                print_error("Error: Failed to write to output path.");
                process::exit(1);
            }
        }
    } else {
        println!("{}", sql_out);
    }
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

  Format Settings
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

fn print_error(msg: &str) {
    println!("{FONT_RED}Error:{FONT_RESET} {msg}");
    println!("Run with -h/--help to print help.");
}
