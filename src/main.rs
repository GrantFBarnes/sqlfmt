use token::*;

mod token;

fn main() {
    let tokens: Vec<token::Token> = get_sql_tokens(String::from("SELECT * FROM TBL1"));
    dbg!(tokens);
}
