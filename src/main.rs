use std::io::{self, Stdin};

use format::*;

mod format;
mod token;

fn main() {
    let stdin: Stdin = io::stdin();
    let sql: Result<String, io::Error> = io::read_to_string(stdin);
    if sql.is_err() {
        panic!("Failed to read input to string.");
    }
    let sql: String = sql.unwrap();

    let formatted_sql: String = get_formatted_sql(sql);
    println!("{}", formatted_sql);
}
