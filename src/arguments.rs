use std::env;

pub struct Arguments {
    arg_type: Option<ArgType>,

    pub help: bool,
    pub version: bool,

    pub input: Option<String>,
    pub output: Option<String>,

    pub upper: bool,
    pub lower: bool,
    pub tabs: bool,
    pub spaces: u8,
}

impl Arguments {
    pub fn new() -> Arguments {
        Arguments {
            arg_type: None,

            help: false,
            version: false,

            input: None,
            output: None,

            upper: false,
            lower: false,
            tabs: false,
            spaces: 4,
        }
    }
}

enum ArgType {
    Input,
    Output,
    Spaces,
}

pub fn get_arguments() -> Result<Arguments, &'static str> {
    let mut arguments: Arguments = Arguments::new();

    for arg in env::args().skip(1) {
        match arg.as_str() {
            "-h" | "--help" => {
                arguments.help = true;
            }
            "-v" | "--version" => {
                arguments.version = true;
            }
            "-i" | "--input" => {
                if arguments.arg_type.is_some() {
                    return Err("Invalid arguments provided.");
                }
                arguments.arg_type = Some(ArgType::Input);
            }
            "-o" | "--output" => {
                if arguments.arg_type.is_some() {
                    return Err("Invalid arguments provided.");
                }
                arguments.arg_type = Some(ArgType::Output);
            }
            "-u" | "--upper" => {
                arguments.upper = true;
            }
            "-l" | "--lower" => {
                arguments.lower = true;
            }
            "-t" | "--tabs" => {
                arguments.tabs = true;
            }
            "-s" | "--spaces" => {
                if arguments.arg_type.is_some() {
                    return Err("Invalid arguments provided.");
                }
                arguments.arg_type = Some(ArgType::Spaces);
            }
            _ => match arguments.arg_type {
                Some(ArgType::Input) => {
                    arguments.input = Some(arg);
                    arguments.arg_type = None;
                }
                Some(ArgType::Output) => {
                    arguments.output = Some(arg);
                    arguments.arg_type = None;
                }
                Some(ArgType::Spaces) => {
                    let spaces: Result<u8, std::num::ParseIntError> = arg.parse::<u8>();
                    if spaces.is_err() {
                        return Err("Invalid space size provided (must be 0-255).");
                    }
                    arguments.spaces = spaces.unwrap();
                    arguments.arg_type = None;
                }
                None => {
                    return Err("Unknown argument provided");
                }
            },
        }
    }

    if arguments.arg_type.is_some() {
        return Err("Invalid arguments provided");
    }

    return Ok(arguments);
}
