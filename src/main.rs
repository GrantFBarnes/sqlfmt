use std::{
    env, fs,
    io::{self, IsTerminal, Stdin},
    process,
};

use format::*;

mod format;
mod token;

const FONT_RED: &str = "\x1b[31m";
const FONT_CYAN: &str = "\x1b[36m";
const FONT_MAGENTA: &str = "\x1b[35m";
const FONT_YELLOW: &str = "\x1b[33m";
const FONT_RESET: &str = "\x1b[0m";

struct Settings {
    input: Option<String>,
    output: Option<String>,
    case: Option<CaseSetting>,
}

impl Settings {
    fn new() -> Settings {
        Settings {
            input: None,
            output: None,
            case: None,
        }
    }
}

#[derive(PartialEq, Eq)]
enum CaseSetting {
    Upper,
    Lower,
}

fn main() {
    let settings: Settings = get_settings_from_args();
    let sql_in: String = get_input_sql(&settings);
    let sql_out: String = get_formatted_sql(&settings, sql_in);
    output_result(&settings, &sql_out);
}

fn get_settings_from_args() -> Settings {
    let mut settings: Settings = Settings::new();

    enum ArgType {
        Input,
        Output,
    }
    let mut arg_type: Option<ArgType> = None;

    for arg in env::args().skip(1) {
        match arg.as_str() {
            "-h" | "--help" => {
                print_help();
                process::exit(0);
            }
            "-u" | "--upper" => {
                if settings.case.is_some() {
                    print_error("Case setting already defined.");
                    print_help();
                    process::exit(1);
                }
                settings.case = Some(CaseSetting::Upper);
            }
            "-l" | "--lower" => {
                if settings.case.is_some() {
                    print_error("Case setting already defined.");
                    print_help();
                    process::exit(1);
                }
                settings.case = Some(CaseSetting::Lower);
            }
            "-i" | "--input" => {
                if arg_type.is_some() {
                    print_error("Invalid arguments provided.");
                    print_help();
                    process::exit(1);
                }
                arg_type = Some(ArgType::Input);
            }
            "-o" | "--output" => {
                if arg_type.is_some() {
                    print_error("Invalid arguments provided.");
                    print_help();
                    process::exit(1);
                }
                arg_type = Some(ArgType::Output);
            }
            _ => match arg_type {
                Some(ArgType::Input) => {
                    if settings.input.is_some() {
                        print_error("Input was already defined.");
                        print_help();
                        process::exit(1);
                    }
                    settings.input = Some(arg);
                    arg_type = None;
                }
                Some(ArgType::Output) => {
                    if settings.output.is_some() {
                        print_error("Output was already defined.");
                        print_help();
                        process::exit(1);
                    }
                    settings.output = Some(arg);
                    arg_type = None;
                }
                None => {
                    print_error("Invalid arguments provided.");
                    print_help();
                    process::exit(1);
                }
            },
        }
    }

    return settings;
}

fn get_input_sql(settings: &Settings) -> String {
    let sql_input: Result<String, io::Error>;
    let stdin: Stdin = io::stdin();
    if stdin.is_terminal() {
        if settings.input.is_none() {
            print_error("Input file not provided.");
            print_help();
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

fn output_result(settings: &Settings, sql_out: &String) {
    if settings.output.is_some() {
        match fs::write(settings.output.as_ref().unwrap(), sql_out) {
            Ok(_) => (),
            Err(_) => {
                print_error("Failed to write to output path.");
                process::exit(1);
            }
        }
    } else {
        println!("{}", sql_out);
    }
}

fn print_help() {
    println!(
        "Format SQL

usage: sqlfmt {}-i <input file path>{} {}-o [output file path]{}
       {}<input stream>{} | sqlfmt

notes: {}required{}, {}optional{}
       output is printed to stdout if no output file provided

options:
    {}
    {}
    {}
    {}
    {}",
        FONT_CYAN,
        FONT_RESET,
        FONT_MAGENTA,
        FONT_RESET,
        FONT_CYAN,
        FONT_RESET,
        FONT_CYAN,
        FONT_RESET,
        FONT_MAGENTA,
        FONT_RESET,
        get_help_parameter('h', "help", "Show this message"),
        get_help_parameter('i', "input", "Input file path"),
        get_help_parameter('o', "output", "Output file path"),
        get_help_parameter('u', "upper", "Uppercase keywords"),
        get_help_parameter('l', "lower", "Lowercase keywords"),
    );
}

fn get_help_parameter(letter: char, word: &str, description: &str) -> String {
    format!(
        "    {}-{}{}, {}--{}{}\t{}",
        FONT_YELLOW, letter, FONT_RESET, FONT_YELLOW, word, FONT_RESET, description
    )
}

fn print_error(msg: &str) {
    println!("{}Error:{} {}", FONT_RED, FONT_RESET, msg);
}
