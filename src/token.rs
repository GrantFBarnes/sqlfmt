#[derive(Debug, PartialEq)]
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_sql_tokens_basic() {
        assert_eq!(
            vec![
                Token {
                    value: String::from("SELECT"),
                },
                Token {
                    value: String::from("*"),
                },
                Token {
                    value: String::from("FROM"),
                },
                Token {
                    value: String::from("TBL1"),
                },
            ],
            get_sql_tokens(String::from("SELECT * FROM TBL1"))
        );
    }
}
