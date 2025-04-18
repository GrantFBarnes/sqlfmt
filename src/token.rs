const BACKTICK: char = '`';
const QUOTE_SINGLE: char = '\'';
const QUOTE_DOUBLE: char = '"';
const BRACKET_OPEN: char = '[';
const BRACKET_CLOSE: char = ']';
const HYPHEN: char = '-';
const NEW_LINE: char = '\n';

#[derive(Debug, PartialEq)]
pub struct Token {
    pub value: String,
}

pub fn get_sql_tokens(sql: String) -> Vec<Token> {
    let mut tokens: Vec<Token> = vec![];

    let mut curr_token_value: String = String::new();
    let mut in_quote: Option<char> = None;
    let mut in_comment: bool = false;

    let sql_bytes: &[u8] = sql.as_bytes();
    for i in 0..sql_bytes.len() {
        let curr_ch: char = sql_bytes[i].into();
        let next_ch: Option<char> = if (i + 1) < sql_bytes.len() {
            Some(sql_bytes[i + 1].into())
        } else {
            None
        };

        if get_in_comment(curr_ch, next_ch, in_comment) {
            if !in_comment {
                // start of new comment, add any current token if any
                if !curr_token_value.is_empty() {
                    tokens.push(Token {
                        value: curr_token_value,
                    });
                    curr_token_value = String::new();
                }
            }

            curr_token_value.push(curr_ch);
            in_comment = true;
            continue;
        } else {
            if in_comment {
                // comment just ended, add comment token
                if !curr_token_value.is_empty() {
                    tokens.push(Token {
                        value: curr_token_value,
                    });
                    curr_token_value = String::new();
                }
            }
            in_comment = false;
        }

        match curr_ch {
            BACKTICK => {
                if in_quote == Some(BACKTICK) {
                    in_quote = None;
                } else if in_quote.is_none() {
                    in_quote = Some(BACKTICK);
                }
            }
            QUOTE_SINGLE => {
                if in_quote == Some(QUOTE_SINGLE) {
                    in_quote = None;
                } else if in_quote.is_none() {
                    in_quote = Some(QUOTE_SINGLE);
                }
            }
            QUOTE_DOUBLE => {
                if in_quote == Some(QUOTE_DOUBLE) {
                    in_quote = None;
                } else if in_quote.is_none() {
                    in_quote = Some(QUOTE_DOUBLE);
                }
            }
            BRACKET_OPEN => {
                if in_quote.is_none() {
                    in_quote = Some(BRACKET_OPEN);
                }
            }
            BRACKET_CLOSE => {
                if in_quote == Some(BRACKET_OPEN) {
                    in_quote = None;
                }
            }
            NEW_LINE => {
                in_quote = None;
            }
            _ => (),
        }

        if in_quote.is_some() {
            curr_token_value.push(curr_ch);
            continue;
        }

        if curr_ch.is_whitespace() {
            if !curr_token_value.is_empty() {
                tokens.push(Token {
                    value: curr_token_value,
                });
                curr_token_value = String::new();
            }
            continue;
        }

        curr_token_value.push(curr_ch);
    }

    if !curr_token_value.is_empty() {
        tokens.push(Token {
            value: curr_token_value,
        });
    }

    return tokens;
}

fn get_in_comment(curr_ch: char, next_ch: Option<char>, in_comment: bool) -> bool {
    if in_comment {
        if curr_ch == NEW_LINE {
            return false;
        }
        return true;
    }

    if curr_ch == HYPHEN && next_ch == Some(HYPHEN) {
        return true;
    }
    return false;
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

    #[test]
    fn test_get_sql_tokens_quote_backtick() {
        assert_eq!(
            vec![
                Token {
                    value: String::from("SELECT"),
                },
                Token {
                    value: String::from("`Column 1`"),
                },
            ],
            get_sql_tokens(String::from("SELECT `Column 1`"))
        );
    }

    #[test]
    fn test_get_sql_tokens_quote_single() {
        assert_eq!(
            vec![
                Token {
                    value: String::from("SELECT"),
                },
                Token {
                    value: String::from("'Column 1'"),
                },
            ],
            get_sql_tokens(String::from("SELECT 'Column 1'"))
        );
    }

    #[test]
    fn test_get_sql_tokens_quote_double() {
        assert_eq!(
            vec![
                Token {
                    value: String::from("SELECT"),
                },
                Token {
                    value: String::from("\"Column 1\""),
                },
            ],
            get_sql_tokens(String::from("SELECT \"Column 1\""))
        );
    }

    #[test]
    fn test_get_sql_tokens_quote_bracket() {
        assert_eq!(
            vec![
                Token {
                    value: String::from("SELECT"),
                },
                Token {
                    value: String::from("[Column 1]"),
                },
            ],
            get_sql_tokens(String::from("SELECT [Column 1]"))
        );
    }

    #[test]
    fn test_get_sql_tokens_comment_inline() {
        assert_eq!(
            vec![
                Token {
                    value: String::from("SELECT"),
                },
                Token {
                    value: String::from("1"),
                },
                Token {
                    value: String::from("--comment inline"),
                },
            ],
            get_sql_tokens(String::from("SELECT 1 --comment inline"))
        );
    }

    #[test]
    fn test_get_sql_tokens_comment_newline() {
        assert_eq!(
            vec![
                Token {
                    value: String::from("SELECT"),
                },
                Token {
                    value: String::from("*"),
                },
                Token {
                    value: String::from("-- comment newline"),
                },
                Token {
                    value: String::from("FROM"),
                },
                Token {
                    value: String::from("TBL1"),
                },
            ],
            get_sql_tokens(String::from(
                r#"
                SELECT *
                -- comment newline
                FROM TBL1
                "#
            ))
        );
    }
}
