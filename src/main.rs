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

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.contains(&String::from("-h")) || args.contains(&String::from("--help")) {
        print_help();
        process::exit(0);
    }

    let sql: Option<String>;
    let mut output_path: Option<String> = None;

    let stdin: Stdin = io::stdin();
    if stdin.is_terminal() {
        match args.len() {
            2 | 3 => {
                let input_file: Result<String, io::Error> = fs::read_to_string(&args[1]);
                if input_file.is_err() {
                    print_error("Failed to read input file.");
                    process::exit(1);
                }
                sql = Some(input_file.unwrap());

                if args.len() == 3 {
                    output_path = Some(args[2].clone());
                }
            }
            _ => {
                print_error("Invalid arguments provided.");
                print_help();
                process::exit(1);
            }
        }
    } else {
        let input_text: Result<String, io::Error> = io::read_to_string(stdin);
        if input_text.is_err() {
            print_error("Failed to read input to string.");
            process::exit(1);
        }
        sql = Some(input_text.unwrap());
    }

    if sql.is_none() {
        print_error("Failed to get SQL to format.");
        process::exit(1);
    }

    let sql: String = sql.unwrap();

    let formatted_sql: String = get_formatted_sql(sql);

    if output_path.is_some() {
        match fs::write(output_path.unwrap(), formatted_sql) {
            Ok(_) => (),
            Err(_) => {
                print_error("Failed to write to output path.");
                process::exit(1);
            }
        }
    } else {
        println!("{}", formatted_sql);
    }

    process::exit(0);
}

fn print_help() {
    println!(
        "Format SQL

usage: sqlfmt {}<input file path>{} {}[output file path]{}
       {}<input stream>{} | sqlfmt

notes: {}required{}, {}optional{}
       output is printed to stdout if no output file provided

options:
    {}-h{}, {}--help{} Show this message",
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
        FONT_YELLOW,
        FONT_RESET,
        FONT_YELLOW,
        FONT_RESET,
    );
}

fn print_error(msg: &str) {
    println!("{}Error:{} {}", FONT_RED, FONT_RESET, msg);
}
