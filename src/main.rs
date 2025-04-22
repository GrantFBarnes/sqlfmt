mod format;
mod io;
mod token;

fn main() {
    let settings: format::Settings = io::get_settings_from_args();
    let sql_in: String = io::get_input_sql(&settings);
    let sql_out: String = format::get_formatted_sql(&settings, sql_in);
    io::output_result(&settings, &sql_out);
}
