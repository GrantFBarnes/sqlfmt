pub struct Token {
    pub value: String,
}

pub fn get_sql_tokens(sql: String) -> Vec<Token> {
    let mut tokens: Vec<Token> = vec![];
    for word in sql.split_whitespace() {
        tokens.push(Token {
            value: word.to_owned(),
        });
    }
    return tokens;
}
