use crate::token::*;

pub fn get_formatted_sql(sql: String) -> String {
    let tokens: Vec<Token> = get_sql_tokens(sql);
    dbg!(tokens);
    return String::new();
}
