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

#[derive(Clone)]
enum CommentType {
    SingleLine,
    MultiLine,
}

#[derive(Clone)]
enum QuoteType {
    Backtick,
    QuoteSingle,
    QuoteDouble,
    Bracket,
}

pub fn get_sql_tokens(sql: String) -> Vec<Token> {
    let mut tokens: Vec<Token> = vec![];

    let mut curr_token_value: String = String::new();
    let mut in_comment: Option<CommentType> = None;
    let mut in_quote: Option<QuoteType> = None;

    let sql_bytes: &[u8] = sql.as_bytes();
    for i in 0..sql_bytes.len() {
        let curr_ch: char = sql_bytes[i].into();
        let curr_token_len: usize = curr_token_value.len();

        let was_in_comment: Option<CommentType> = in_comment.clone();
        in_comment = get_in_comment(&in_comment, sql_bytes, i, curr_token_len);
        if in_comment.is_some() {
            if was_in_comment.is_none() {
                // start of new comment, add any current token if any
                if !curr_token_value.is_empty() {
                    tokens.push(Token {
                        value: curr_token_value,
                    });
                    curr_token_value = String::new();
                }
            }

            curr_token_value.push(curr_ch);
            continue;
        } else if was_in_comment.is_some() {
            // comment just ended, add comment token
            tokens.push(Token {
                value: curr_token_value,
            });
            curr_token_value = String::new();
        }

        let was_in_quote: Option<QuoteType> = in_quote.clone();
        in_quote = get_in_quote(&in_quote, sql_bytes, i, curr_token_len);
        if in_quote.is_some() {
            if was_in_quote.is_none() {
                // start of new quote, add any current token if any
                if !curr_token_value.is_empty() {
                    tokens.push(Token {
                        value: curr_token_value,
                    });
                    curr_token_value = String::new();
                }
            }

            curr_token_value.push(curr_ch);
            continue;
        } else if was_in_quote.is_some() {
            // quote just ended, add quote token
            tokens.push(Token {
                value: curr_token_value,
            });
            curr_token_value = String::new();
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

fn get_in_quote(
    in_quote: &Option<QuoteType>,
    sql_bytes: &[u8],
    curr_idx: usize,
    curr_token_len: usize,
) -> Option<QuoteType> {
    let curr_ch: char = sql_bytes[curr_idx].into();
    match in_quote {
        Some(qt) => {
            if curr_token_len <= 1 {
                return in_quote.clone();
            }
            // at least 2 characters in current token

            let prev1_ch: char = sql_bytes[curr_idx - 1].into();
            let prev2_ch: char = sql_bytes[curr_idx - 2].into();
            match qt {
                QuoteType::Backtick => {
                    if prev1_ch == BACKTICK {
                        return None;
                    }
                    return in_quote.clone();
                }
                QuoteType::QuoteSingle => {
                    if prev1_ch == QUOTE_SINGLE
                        && prev2_ch != QUOTE_SINGLE
                        && curr_ch != QUOTE_SINGLE
                    {
                        return None;
                    }
                    return in_quote.clone();
                }
                QuoteType::QuoteDouble => {
                    if prev1_ch == QUOTE_DOUBLE {
                        return None;
                    }
                    return in_quote.clone();
                }
                QuoteType::Bracket => {
                    if prev1_ch == BRACKET_CLOSE {
                        return None;
                    }
                    return in_quote.clone();
                }
            }
        }
        None => {
            return match curr_ch {
                BACKTICK => Some(QuoteType::Backtick),
                QUOTE_SINGLE => Some(QuoteType::QuoteSingle),
                QUOTE_DOUBLE => Some(QuoteType::QuoteDouble),
                BRACKET_OPEN => Some(QuoteType::Bracket),
                _ => None,
            };
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
    fn test_get_sql_tokens_quote_empty() {
        assert_eq!(
            vec![
                Token {
                    value: String::from("SELECT"),
                },
                Token {
                    value: String::from("''"),
                },
            ],
            get_sql_tokens(String::from("SELECT ''"))
        );
    }

    #[test]
    fn test_get_sql_tokens_quote_single_escape() {
        assert_eq!(
            vec![
                Token {
                    value: String::from("SELECT"),
                },
                Token {
                    value: String::from("'Column''s Name'"),
                },
            ],
            get_sql_tokens(String::from("SELECT 'Column''s Name'"))
        );
    }
}
