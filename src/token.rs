#[derive(Debug, PartialEq)]
pub struct Token {
    pub value: String,
}

pub fn get_sql_tokens(sql: String) -> Vec<Token> {
    let mut tokens: Vec<Token> = vec![];
    let mut curr_token_value: String = String::new();
    for ch in sql.chars() {
        if ch.is_whitespace() {
            if !curr_token_value.is_empty() {
                tokens.push(Token {
                    value: curr_token_value,
                });
                curr_token_value = String::new();
            }
            continue;
        }

        curr_token_value.push(ch);
    }

    if !curr_token_value.is_empty() {
        tokens.push(Token {
            value: curr_token_value,
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
