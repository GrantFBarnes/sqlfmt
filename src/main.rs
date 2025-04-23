mod format;
mod io;
mod token;

struct Configuration {
    input: Option<String>,
    output: Option<String>,

    upper: bool,
    lower: bool,
    tabs: bool,
    spaces: usize,
}

impl Configuration {
    fn new() -> Configuration {
        Configuration {
            input: None,
            output: None,

            upper: false,
            lower: false,
            tabs: false,
            spaces: 4,
        }
    }
}

fn main() {
    let config: Configuration = io::get_config_from_args();
    let sql_in: String = io::get_input_sql(&config);
    let sql_out: String = format::get_formatted_sql(&config, sql_in);
    io::output_result(&config, &sql_out);
}
