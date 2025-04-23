mod format;
mod io;
mod token;

struct Settings {
    input: Option<String>,
    output: Option<String>,

    upper: bool,
    lower: bool,
    tabs: bool,
    spaces: usize,
}

impl Settings {
    fn new() -> Settings {
        Settings {
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
    let settings: Settings = io::get_settings_from_args();
    let sql_in: String = io::get_input_sql(&settings);
    let sql_out: String = format::get_formatted_sql(&settings, sql_in);
    io::output_result(&settings, &sql_out);
}
