const ASTERISK: char = '*';
const BACKTICK: char = '`';
const BRACKET_CLOSE: char = ']';
const BRACKET_OPEN: char = '[';
const HYPHEN: char = '-';
const NEW_LINE: char = '\n';
const QUOTE_DOUBLE: char = '"';
const QUOTE_SINGLE: char = '\'';
const SLASH_FORWARD: char = '/';

#[derive(Debug, PartialEq)]
pub struct Token {
    pub value: String,
}

enum CommentType {
    SingleLine,
    MultiLine,
}

pub fn get_sql_tokens(sql: String) -> Vec<Token> {
    let mut tokens: Vec<Token> = vec![];

    let mut curr_token_value: String = String::new();
    let mut in_quote: Option<char> = None;
    let mut in_comment: Option<CommentType> = None;

    let sql_bytes: &[u8] = sql.as_bytes();
    for i in 0..sql_bytes.len() {
        let curr_ch: char = sql_bytes[i].into();

        let next_in_comment: Option<CommentType> =
            get_in_comment(&in_comment, sql_bytes, i, curr_token_value.len());
        if next_in_comment.is_some() {
            if in_comment.is_none() {
                // start of new comment, add any current token if any
                if !curr_token_value.is_empty() {
                    tokens.push(Token {
                        value: curr_token_value,
                    });
                    curr_token_value = String::new();
                }
            }

            curr_token_value.push(curr_ch);
            in_comment = next_in_comment;
            continue;
        } else {
            if in_comment.is_some() {
                // comment just ended, add comment token
                tokens.push(Token {
                    value: curr_token_value,
                });
                curr_token_value = String::new();
            }
            in_comment = next_in_comment;
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

fn get_in_comment(
    in_comment: &Option<CommentType>,
    sql_bytes: &[u8],
    curr_idx: usize,
    curr_token_len: usize,
) -> Option<CommentType> {
    let curr_ch: char = sql_bytes[curr_idx].into();
    match in_comment {
        Some(CommentType::SingleLine) => {
            if curr_ch == NEW_LINE {
                return None;
            }
            return Some(CommentType::SingleLine);
        }
        Some(CommentType::MultiLine) => {
            if curr_idx >= 2 && curr_token_len > 1 {
                let prev2_ch: char = sql_bytes[curr_idx - 2].into();
                let prev1_ch: char = sql_bytes[curr_idx - 1].into();
                if prev2_ch == ASTERISK && prev1_ch == SLASH_FORWARD {
                    return None;
                }
            }
            return Some(CommentType::MultiLine);
        }
        None => {
            if (curr_idx + 1) < sql_bytes.len() {
                let next_ch: char = sql_bytes[curr_idx + 1].into();

                if curr_ch == HYPHEN && next_ch == HYPHEN {
                    return Some(CommentType::SingleLine);
                }

                if curr_ch == SLASH_FORWARD && next_ch == ASTERISK {
                    return Some(CommentType::MultiLine);
                }
            }
            return None;
        }
    }
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
    fn test_get_sql_tokens_comment_single_inline() {
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
    fn test_get_sql_tokens_comment_single_newline() {
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

    #[test]
    fn test_get_sql_tokens_comment_multi_inline() {
        assert_eq!(
            vec![
                Token {
                    value: String::from("SELECT"),
                },
                Token {
                    value: String::from("*"),
                },
                Token {
                    value: String::from("/*multi inline*/"),
                },
                Token {
                    value: String::from("FROM"),
                },
                Token {
                    value: String::from("TBL1"),
                },
            ],
            get_sql_tokens(String::from("SELECT * /*multi inline*/ FROM TBL1"))
        );
    }

    #[test]
    fn test_get_sql_tokens_comment_multi_odd() {
        assert_eq!(
            vec![
                Token {
                    value: String::from("*"),
                },
                Token {
                    value: String::from("/*multi odd*/"),
                },
                Token {
                    value: String::from("*"),
                },
            ],
            get_sql_tokens(String::from("*/*multi odd*/*"))
        );
    }

    #[test]
    fn test_get_sql_tokens_comment_multi_newline() {
        assert_eq!(
            vec![
                Token {
                    value: String::from("SELECT"),
                },
                Token {
                    value: String::from("*"),
                },
                Token {
                    value: String::from(
                        r#"/*
                    multi line
                    comment
                */"#
                    ),
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
                /*
                    multi line
                    comment
                */
                FROM TBL1
                "#
            ))
        );
    }
}
