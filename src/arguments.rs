pub struct Arguments {
    arg_type: Option<ArgType>,

    pub help: bool,
    pub version: bool,

    pub input: Option<String>,
    pub output: Option<String>,

    pub newlines: bool,
    pub upper: bool,
    pub lower: bool,
    pub tabs: bool,
    pub spaces: Option<u8>,
    pub chars: Option<u8>,
}

impl Arguments {
    fn new() -> Arguments {
        Arguments {
            arg_type: None,

            help: false,
            version: false,

            input: None,
            output: None,

            newlines: false,
            upper: false,
            lower: false,
            tabs: false,
            spaces: None,
            chars: None,
        }
    }

    pub fn from<T: Iterator<Item = String>>(args: T) -> Result<Arguments, &'static str> {
        let mut arguments: Arguments = Arguments::new();

        for arg in args {
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
                "-n" | "--newlines" => {
                    arguments.newlines = true;
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
                "-c" | "--chars" => {
                    if arguments.arg_type.is_some() {
                        return Err("Invalid arguments provided.");
                    }
                    arguments.arg_type = Some(ArgType::Chars);
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
                        arguments.spaces = Some(spaces.unwrap());
                        arguments.arg_type = None;
                    }
                    Some(ArgType::Chars) => {
                        let chars: Result<u8, std::num::ParseIntError> = arg.parse::<u8>();
                        if chars.is_err() {
                            return Err("Invalid char size provided (must be 0-255).");
                        }
                        arguments.chars = Some(chars.unwrap());
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
}

enum ArgType {
    Input,
    Output,
    Spaces,
    Chars,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_arguments_empty() {
        let args: Vec<String> = vec![];
        let arguments: Result<Arguments, &str> = Arguments::from(args.into_iter());
        assert_eq!(arguments.is_ok(), true);
        let arguments: Arguments = arguments.unwrap();
        assert_eq!(arguments.help, false);
        assert_eq!(arguments.version, false);
        assert_eq!(arguments.input, None);
        assert_eq!(arguments.output, None);
        assert_eq!(arguments.newlines, false);
        assert_eq!(arguments.upper, false);
        assert_eq!(arguments.lower, false);
        assert_eq!(arguments.tabs, false);
        assert_eq!(arguments.spaces, None);
        assert_eq!(arguments.chars, None);
    }

    #[test]
    fn test_get_arguments_everything() {
        let args: Vec<String> = vec![
            String::from("-h"),
            String::from("-v"),
            String::from("-i"),
            String::from("in.sql"),
            String::from("-o"),
            String::from("out.sql"),
            String::from("-n"),
            String::from("-u"),
            String::from("-l"),
            String::from("-t"),
            String::from("-s"),
            String::from("2"),
            String::from("-c"),
            String::from("120"),
        ];
        let arguments: Result<Arguments, &str> = Arguments::from(args.into_iter());
        assert_eq!(arguments.is_ok(), true);
        let arguments: Arguments = arguments.unwrap();
        assert_eq!(arguments.help, true);
        assert_eq!(arguments.version, true);
        assert_eq!(arguments.input, Some(String::from("in.sql")));
        assert_eq!(arguments.output, Some(String::from("out.sql")));
        assert_eq!(arguments.newlines, true);
        assert_eq!(arguments.upper, true);
        assert_eq!(arguments.lower, true);
        assert_eq!(arguments.tabs, true);
        assert_eq!(arguments.spaces, Some(2));
        assert_eq!(arguments.chars, Some(120));
    }

    #[test]
    fn test_get_arguments_help_short() {
        let args: Vec<String> = vec![String::from("-h")];
        let arguments: Result<Arguments, &str> = Arguments::from(args.into_iter());
        assert_eq!(arguments.is_ok(), true);
        let arguments: Arguments = arguments.unwrap();
        assert_eq!(arguments.help, true);
        assert_eq!(arguments.version, false);
        assert_eq!(arguments.input, None);
        assert_eq!(arguments.output, None);
        assert_eq!(arguments.newlines, false);
        assert_eq!(arguments.upper, false);
        assert_eq!(arguments.lower, false);
        assert_eq!(arguments.tabs, false);
        assert_eq!(arguments.spaces, None);
        assert_eq!(arguments.chars, None);
    }

    #[test]
    fn test_get_arguments_help_long() {
        let args: Vec<String> = vec![String::from("--help")];
        let arguments: Result<Arguments, &str> = Arguments::from(args.into_iter());
        assert_eq!(arguments.is_ok(), true);
        let arguments: Arguments = arguments.unwrap();
        assert_eq!(arguments.help, true);
        assert_eq!(arguments.version, false);
        assert_eq!(arguments.input, None);
        assert_eq!(arguments.output, None);
        assert_eq!(arguments.newlines, false);
        assert_eq!(arguments.upper, false);
        assert_eq!(arguments.lower, false);
        assert_eq!(arguments.tabs, false);
        assert_eq!(arguments.spaces, None);
        assert_eq!(arguments.chars, None);
    }

    #[test]
    fn test_get_arguments_version_short() {
        let args: Vec<String> = vec![String::from("-v")];
        let arguments: Result<Arguments, &str> = Arguments::from(args.into_iter());
        assert_eq!(arguments.is_ok(), true);
        let arguments: Arguments = arguments.unwrap();
        assert_eq!(arguments.help, false);
        assert_eq!(arguments.version, true);
        assert_eq!(arguments.input, None);
        assert_eq!(arguments.output, None);
        assert_eq!(arguments.newlines, false);
        assert_eq!(arguments.upper, false);
        assert_eq!(arguments.lower, false);
        assert_eq!(arguments.tabs, false);
        assert_eq!(arguments.spaces, None);
        assert_eq!(arguments.chars, None);
    }

    #[test]
    fn test_get_arguments_version_long() {
        let args: Vec<String> = vec![String::from("--version")];
        let arguments: Result<Arguments, &str> = Arguments::from(args.into_iter());
        assert_eq!(arguments.is_ok(), true);
        let arguments: Arguments = arguments.unwrap();
        assert_eq!(arguments.help, false);
        assert_eq!(arguments.version, true);
        assert_eq!(arguments.input, None);
        assert_eq!(arguments.output, None);
        assert_eq!(arguments.newlines, false);
        assert_eq!(arguments.upper, false);
        assert_eq!(arguments.lower, false);
        assert_eq!(arguments.tabs, false);
        assert_eq!(arguments.spaces, None);
        assert_eq!(arguments.chars, None);
    }

    #[test]
    fn test_get_arguments_input_short() {
        let args: Vec<String> = vec![String::from("-i"), String::from("file.sql")];
        let arguments: Result<Arguments, &str> = Arguments::from(args.into_iter());
        assert_eq!(arguments.is_ok(), true);
        let arguments: Arguments = arguments.unwrap();
        assert_eq!(arguments.help, false);
        assert_eq!(arguments.version, false);
        assert_eq!(arguments.input, Some(String::from("file.sql")));
        assert_eq!(arguments.output, None);
        assert_eq!(arguments.newlines, false);
        assert_eq!(arguments.upper, false);
        assert_eq!(arguments.lower, false);
        assert_eq!(arguments.tabs, false);
        assert_eq!(arguments.spaces, None);
        assert_eq!(arguments.chars, None);
    }

    #[test]
    fn test_get_arguments_input_long() {
        let args: Vec<String> = vec![String::from("--input"), String::from("file.sql")];
        let arguments: Result<Arguments, &str> = Arguments::from(args.into_iter());
        assert_eq!(arguments.is_ok(), true);
        let arguments: Arguments = arguments.unwrap();
        assert_eq!(arguments.help, false);
        assert_eq!(arguments.version, false);
        assert_eq!(arguments.input, Some(String::from("file.sql")));
        assert_eq!(arguments.output, None);
        assert_eq!(arguments.newlines, false);
        assert_eq!(arguments.upper, false);
        assert_eq!(arguments.lower, false);
        assert_eq!(arguments.tabs, false);
        assert_eq!(arguments.spaces, None);
        assert_eq!(arguments.chars, None);
    }

    #[test]
    fn test_get_arguments_output_short() {
        let args: Vec<String> = vec![String::from("-o"), String::from("file.sql")];
        let arguments: Result<Arguments, &str> = Arguments::from(args.into_iter());
        assert_eq!(arguments.is_ok(), true);
        let arguments: Arguments = arguments.unwrap();
        assert_eq!(arguments.help, false);
        assert_eq!(arguments.version, false);
        assert_eq!(arguments.input, None);
        assert_eq!(arguments.output, Some(String::from("file.sql")));
        assert_eq!(arguments.newlines, false);
        assert_eq!(arguments.upper, false);
        assert_eq!(arguments.lower, false);
        assert_eq!(arguments.tabs, false);
        assert_eq!(arguments.spaces, None);
        assert_eq!(arguments.chars, None);
    }

    #[test]
    fn test_get_arguments_output_long() {
        let args: Vec<String> = vec![String::from("--output"), String::from("file.sql")];
        let arguments: Result<Arguments, &str> = Arguments::from(args.into_iter());
        assert_eq!(arguments.is_ok(), true);
        let arguments: Arguments = arguments.unwrap();
        assert_eq!(arguments.help, false);
        assert_eq!(arguments.version, false);
        assert_eq!(arguments.input, None);
        assert_eq!(arguments.output, Some(String::from("file.sql")));
        assert_eq!(arguments.newlines, false);
        assert_eq!(arguments.upper, false);
        assert_eq!(arguments.lower, false);
        assert_eq!(arguments.tabs, false);
        assert_eq!(arguments.spaces, None);
        assert_eq!(arguments.chars, None);
    }

    #[test]
    fn test_get_arguments_newlines_short() {
        let args: Vec<String> = vec![String::from("-n")];
        let arguments: Result<Arguments, &str> = Arguments::from(args.into_iter());
        assert_eq!(arguments.is_ok(), true);
        let arguments: Arguments = arguments.unwrap();
        assert_eq!(arguments.help, false);
        assert_eq!(arguments.version, false);
        assert_eq!(arguments.input, None);
        assert_eq!(arguments.output, None);
        assert_eq!(arguments.newlines, true);
        assert_eq!(arguments.upper, false);
        assert_eq!(arguments.lower, false);
        assert_eq!(arguments.tabs, false);
        assert_eq!(arguments.spaces, None);
        assert_eq!(arguments.chars, None);
    }

    #[test]
    fn test_get_arguments_newlines_long() {
        let args: Vec<String> = vec![String::from("--newlines")];
        let arguments: Result<Arguments, &str> = Arguments::from(args.into_iter());
        assert_eq!(arguments.is_ok(), true);
        let arguments: Arguments = arguments.unwrap();
        assert_eq!(arguments.help, false);
        assert_eq!(arguments.version, false);
        assert_eq!(arguments.input, None);
        assert_eq!(arguments.output, None);
        assert_eq!(arguments.newlines, true);
        assert_eq!(arguments.upper, false);
        assert_eq!(arguments.lower, false);
        assert_eq!(arguments.tabs, false);
        assert_eq!(arguments.spaces, None);
        assert_eq!(arguments.chars, None);
    }

    #[test]
    fn test_get_arguments_upper_short() {
        let args: Vec<String> = vec![String::from("-u")];
        let arguments: Result<Arguments, &str> = Arguments::from(args.into_iter());
        assert_eq!(arguments.is_ok(), true);
        let arguments: Arguments = arguments.unwrap();
        assert_eq!(arguments.help, false);
        assert_eq!(arguments.version, false);
        assert_eq!(arguments.input, None);
        assert_eq!(arguments.output, None);
        assert_eq!(arguments.newlines, false);
        assert_eq!(arguments.upper, true);
        assert_eq!(arguments.lower, false);
        assert_eq!(arguments.tabs, false);
        assert_eq!(arguments.spaces, None);
        assert_eq!(arguments.chars, None);
    }

    #[test]
    fn test_get_arguments_upper_long() {
        let args: Vec<String> = vec![String::from("--upper")];
        let arguments: Result<Arguments, &str> = Arguments::from(args.into_iter());
        assert_eq!(arguments.is_ok(), true);
        let arguments: Arguments = arguments.unwrap();
        assert_eq!(arguments.help, false);
        assert_eq!(arguments.version, false);
        assert_eq!(arguments.input, None);
        assert_eq!(arguments.output, None);
        assert_eq!(arguments.newlines, false);
        assert_eq!(arguments.upper, true);
        assert_eq!(arguments.lower, false);
        assert_eq!(arguments.tabs, false);
        assert_eq!(arguments.spaces, None);
        assert_eq!(arguments.chars, None);
    }

    #[test]
    fn test_get_arguments_lower_short() {
        let args: Vec<String> = vec![String::from("-l")];
        let arguments: Result<Arguments, &str> = Arguments::from(args.into_iter());
        assert_eq!(arguments.is_ok(), true);
        let arguments: Arguments = arguments.unwrap();
        assert_eq!(arguments.help, false);
        assert_eq!(arguments.version, false);
        assert_eq!(arguments.input, None);
        assert_eq!(arguments.output, None);
        assert_eq!(arguments.newlines, false);
        assert_eq!(arguments.upper, false);
        assert_eq!(arguments.lower, true);
        assert_eq!(arguments.tabs, false);
        assert_eq!(arguments.spaces, None);
        assert_eq!(arguments.chars, None);
    }

    #[test]
    fn test_get_arguments_lower_long() {
        let args: Vec<String> = vec![String::from("--lower")];
        let arguments: Result<Arguments, &str> = Arguments::from(args.into_iter());
        assert_eq!(arguments.is_ok(), true);
        let arguments: Arguments = arguments.unwrap();
        assert_eq!(arguments.help, false);
        assert_eq!(arguments.version, false);
        assert_eq!(arguments.input, None);
        assert_eq!(arguments.output, None);
        assert_eq!(arguments.newlines, false);
        assert_eq!(arguments.upper, false);
        assert_eq!(arguments.lower, true);
        assert_eq!(arguments.tabs, false);
        assert_eq!(arguments.spaces, None);
        assert_eq!(arguments.chars, None);
    }

    #[test]
    fn test_get_arguments_tabs_short() {
        let args: Vec<String> = vec![String::from("-t")];
        let arguments: Result<Arguments, &str> = Arguments::from(args.into_iter());
        assert_eq!(arguments.is_ok(), true);
        let arguments: Arguments = arguments.unwrap();
        assert_eq!(arguments.help, false);
        assert_eq!(arguments.version, false);
        assert_eq!(arguments.input, None);
        assert_eq!(arguments.output, None);
        assert_eq!(arguments.newlines, false);
        assert_eq!(arguments.upper, false);
        assert_eq!(arguments.lower, false);
        assert_eq!(arguments.tabs, true);
        assert_eq!(arguments.spaces, None);
        assert_eq!(arguments.chars, None);
    }

    #[test]
    fn test_get_arguments_tabs_long() {
        let args: Vec<String> = vec![String::from("--tabs")];
        let arguments: Result<Arguments, &str> = Arguments::from(args.into_iter());
        assert_eq!(arguments.is_ok(), true);
        let arguments: Arguments = arguments.unwrap();
        assert_eq!(arguments.help, false);
        assert_eq!(arguments.version, false);
        assert_eq!(arguments.input, None);
        assert_eq!(arguments.output, None);
        assert_eq!(arguments.newlines, false);
        assert_eq!(arguments.upper, false);
        assert_eq!(arguments.lower, false);
        assert_eq!(arguments.tabs, true);
        assert_eq!(arguments.spaces, None);
        assert_eq!(arguments.chars, None);
    }

    #[test]
    fn test_get_arguments_spaces_short() {
        let args: Vec<String> = vec![String::from("-s"), String::from("2")];
        let arguments: Result<Arguments, &str> = Arguments::from(args.into_iter());
        assert_eq!(arguments.is_ok(), true);
        let arguments: Arguments = arguments.unwrap();
        assert_eq!(arguments.help, false);
        assert_eq!(arguments.version, false);
        assert_eq!(arguments.input, None);
        assert_eq!(arguments.output, None);
        assert_eq!(arguments.newlines, false);
        assert_eq!(arguments.upper, false);
        assert_eq!(arguments.lower, false);
        assert_eq!(arguments.tabs, false);
        assert_eq!(arguments.spaces, Some(2));
        assert_eq!(arguments.chars, None);
    }

    #[test]
    fn test_get_arguments_spaces_long() {
        let args: Vec<String> = vec![String::from("--spaces"), String::from("2")];
        let arguments: Result<Arguments, &str> = Arguments::from(args.into_iter());
        assert_eq!(arguments.is_ok(), true);
        let arguments: Arguments = arguments.unwrap();
        assert_eq!(arguments.help, false);
        assert_eq!(arguments.version, false);
        assert_eq!(arguments.input, None);
        assert_eq!(arguments.output, None);
        assert_eq!(arguments.newlines, false);
        assert_eq!(arguments.upper, false);
        assert_eq!(arguments.lower, false);
        assert_eq!(arguments.tabs, false);
        assert_eq!(arguments.spaces, Some(2));
        assert_eq!(arguments.chars, None);
    }

    #[test]
    fn test_get_arguments_chars_short() {
        let args: Vec<String> = vec![String::from("-c"), String::from("120")];
        let arguments: Result<Arguments, &str> = Arguments::from(args.into_iter());
        assert_eq!(arguments.is_ok(), true);
        let arguments: Arguments = arguments.unwrap();
        assert_eq!(arguments.help, false);
        assert_eq!(arguments.version, false);
        assert_eq!(arguments.input, None);
        assert_eq!(arguments.output, None);
        assert_eq!(arguments.newlines, false);
        assert_eq!(arguments.upper, false);
        assert_eq!(arguments.lower, false);
        assert_eq!(arguments.tabs, false);
        assert_eq!(arguments.spaces, None);
        assert_eq!(arguments.chars, Some(120));
    }

    #[test]
    fn test_get_arguments_chars_long() {
        let args: Vec<String> = vec![String::from("--chars"), String::from("120")];
        let arguments: Result<Arguments, &str> = Arguments::from(args.into_iter());
        assert_eq!(arguments.is_ok(), true);
        let arguments: Arguments = arguments.unwrap();
        assert_eq!(arguments.help, false);
        assert_eq!(arguments.version, false);
        assert_eq!(arguments.input, None);
        assert_eq!(arguments.output, None);
        assert_eq!(arguments.newlines, false);
        assert_eq!(arguments.upper, false);
        assert_eq!(arguments.lower, false);
        assert_eq!(arguments.tabs, false);
        assert_eq!(arguments.spaces, None);
        assert_eq!(arguments.chars, Some(120));
    }

    #[test]
    fn test_get_arguments_input_short_no_file() {
        let args: Vec<String> = vec![String::from("-i")];
        let arguments: Result<Arguments, &str> = Arguments::from(args.into_iter());
        assert_eq!(arguments.is_err(), true);
    }

    #[test]
    fn test_get_arguments_input_long_no_file() {
        let args: Vec<String> = vec![String::from("--input")];
        let arguments: Result<Arguments, &str> = Arguments::from(args.into_iter());
        assert_eq!(arguments.is_err(), true);
    }

    #[test]
    fn test_get_arguments_ouput_short_no_file() {
        let args: Vec<String> = vec![String::from("-o")];
        let arguments: Result<Arguments, &str> = Arguments::from(args.into_iter());
        assert_eq!(arguments.is_err(), true);
    }

    #[test]
    fn test_get_arguments_ouput_long_no_file() {
        let args: Vec<String> = vec![String::from("--output")];
        let arguments: Result<Arguments, &str> = Arguments::from(args.into_iter());
        assert_eq!(arguments.is_err(), true);
    }

    #[test]
    fn test_get_arguments_spaces_no_number() {
        let args: Vec<String> = vec![String::from("--spaces")];
        let arguments: Result<Arguments, &str> = Arguments::from(args.into_iter());
        assert_eq!(arguments.is_err(), true);
    }

    #[test]
    fn test_get_arguments_spaces_not_number() {
        let args: Vec<String> = vec![String::from("--spaces"), String::from("true")];
        let arguments: Result<Arguments, &str> = Arguments::from(args.into_iter());
        assert_eq!(arguments.is_err(), true);
    }

    #[test]
    fn test_get_arguments_chars_no_number() {
        let args: Vec<String> = vec![String::from("--chars")];
        let arguments: Result<Arguments, &str> = Arguments::from(args.into_iter());
        assert_eq!(arguments.is_err(), true);
    }

    #[test]
    fn test_get_arguments_chars_not_number() {
        let args: Vec<String> = vec![String::from("--chars"), String::from("true")];
        let arguments: Result<Arguments, &str> = Arguments::from(args.into_iter());
        assert_eq!(arguments.is_err(), true);
    }
}
