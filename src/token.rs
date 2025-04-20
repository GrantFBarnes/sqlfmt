const AMPERSAND: char = '&';
const ASTERISK: char = '*';
const BACKTICK: char = '`';
const BRACKET_CLOSE: char = ']';
const BRACKET_OPEN: char = '[';
const CIRCUMFLEX: char = '^';
const COMMA: char = ',';
const DELIMITER: char = ';';
const EQUAL: char = '=';
const FULL_STOP: char = '.';
const GREATER_THAN: char = '>';
const HYPHEN: char = '-';
const LESS_THAN: char = '<';
const NEW_LINE: char = '\n';
const PAREN_CLOSE: char = ')';
const PAREN_OPEN: char = '(';
const PERCENT: char = '%';
const PLUS: char = '+';
const QUOTE_DOUBLE: char = '"';
const QUOTE_SINGLE: char = '\'';
const SLASH_FORWARD: char = '/';
const VERTICAL_BAR: char = '|';

#[derive(Debug, PartialEq)]
pub struct Token {
    pub value: String,
    pub category: Option<TokenCategory>,
}

impl Token {
    fn new() -> Token {
        Token {
            value: String::new(),
            category: None,
        }
    }

    fn len(&self) -> usize {
        self.value.len()
    }

    fn is_empty(&self) -> bool {
        self.value.is_empty()
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum TokenCategory {
    Comment,
    Quote,
    NewLine,
    Delimiter,
    Comma,
    ParenOpen,
    ParenClose,
    Operator,
    Bitwise,
    Compare,
}

#[derive(Clone)]
enum CommentCategory {
    SingleLine,
    MultiLine,
}

#[derive(Clone)]
enum QuoteCategory {
    Backtick,
    QuoteSingle,
    QuoteDouble,
    Bracket,
}

pub fn get_sql_tokens(sql: String) -> Vec<Token> {
    let mut tokens: Vec<Token> = vec![];

    let mut curr_token: Token = Token::new();
    let mut in_comment: Option<CommentCategory> = None;
    let mut in_quote: Option<QuoteCategory> = None;

    let sql_bytes: &[u8] = sql.as_bytes();
    for i in 0..sql_bytes.len() {
        let curr_ch: char = sql_bytes[i].into();
        let curr_token_len: usize = curr_token.len();

        let was_in_comment: Option<CommentCategory> = in_comment.clone();
        in_comment = get_in_comment(&in_comment, sql_bytes, i, curr_token_len);
        if in_comment.is_some() {
            if was_in_comment.is_none() {
                // start of new comment, add any current token if any
                if !curr_token.is_empty() {
                    tokens.push(curr_token);
                    curr_token = Token::new();
                }
                curr_token.category = Some(TokenCategory::Comment);
            }

            curr_token.value.push(curr_ch);
            continue;
        } else if was_in_comment.is_some() {
            // comment just ended, add comment token
            tokens.push(curr_token);
            curr_token = Token::new();
        }

        let was_in_quote: Option<QuoteCategory> = in_quote.clone();
        in_quote = get_in_quote(&in_quote, sql_bytes, i, curr_token_len);
        if in_quote.is_some() {
            if was_in_quote.is_none() {
                // start of new quote, add any current token if any
                if !curr_token.is_empty() {
                    tokens.push(curr_token);
                    curr_token = Token::new();
                }
                curr_token.category = Some(TokenCategory::Quote);
            }

            curr_token.value.push(curr_ch);
            continue;
        } else if was_in_quote.is_some() {
            // quote just ended, add quote token
            tokens.push(curr_token);
            curr_token = Token::new();
        }

        match curr_ch {
            DELIMITER | NEW_LINE | COMMA | PAREN_OPEN | PAREN_CLOSE | AMPERSAND | VERTICAL_BAR
            | CIRCUMFLEX => {
                if !curr_token.is_empty() {
                    tokens.push(curr_token);
                    curr_token = Token::new();
                }
                curr_token.value.push(curr_ch);
                curr_token.category = match curr_ch {
                    DELIMITER => Some(TokenCategory::Delimiter),
                    NEW_LINE => Some(TokenCategory::NewLine),
                    COMMA => Some(TokenCategory::Comma),
                    PAREN_OPEN => Some(TokenCategory::ParenOpen),
                    PAREN_CLOSE => Some(TokenCategory::ParenClose),
                    AMPERSAND => Some(TokenCategory::Bitwise),
                    VERTICAL_BAR => Some(TokenCategory::Bitwise),
                    CIRCUMFLEX => Some(TokenCategory::Bitwise),
                    _ => None,
                };
                tokens.push(curr_token);
                curr_token = Token::new();
                continue;
            }
            LESS_THAN | PLUS | HYPHEN | ASTERISK | SLASH_FORWARD | PERCENT => {
                if !curr_token.is_empty() {
                    tokens.push(curr_token);
                    curr_token = Token::new();
                }
                curr_token.value.push(curr_ch);
                curr_token.category = match curr_ch {
                    LESS_THAN => Some(TokenCategory::Compare),
                    PLUS => Some(TokenCategory::Operator),
                    HYPHEN => Some(TokenCategory::Operator),
                    ASTERISK => Some(TokenCategory::Operator),
                    SLASH_FORWARD => Some(TokenCategory::Operator),
                    PERCENT => Some(TokenCategory::Operator),
                    _ => None,
                };

                let next_ch: Option<char> = if (i + 1) < sql_bytes.len() {
                    Some(sql_bytes[i + 1].into())
                } else {
                    None
                };

                if next_ch != Some(EQUAL) && next_ch != Some(GREATER_THAN) {
                    tokens.push(curr_token);
                    curr_token = Token::new();
                }

                continue;
            }
            EQUAL | GREATER_THAN => {
                let prev_ch: Option<char> = if i >= 1 {
                    Some(sql_bytes[i - 1].into())
                } else {
                    None
                };

                if !curr_token.is_empty()
                    && prev_ch != Some(LESS_THAN)
                    && prev_ch != Some(GREATER_THAN)
                    && prev_ch != Some(PLUS)
                    && prev_ch != Some(HYPHEN)
                    && prev_ch != Some(ASTERISK)
                    && prev_ch != Some(SLASH_FORWARD)
                    && prev_ch != Some(PERCENT)
                {
                    tokens.push(curr_token);
                    curr_token = Token::new();
                }
                curr_token.value.push(curr_ch);
                curr_token.category = match prev_ch {
                    Some(PLUS) => Some(TokenCategory::Operator),
                    Some(HYPHEN) => Some(TokenCategory::Operator),
                    Some(ASTERISK) => Some(TokenCategory::Operator),
                    Some(SLASH_FORWARD) => Some(TokenCategory::Operator),
                    Some(PERCENT) => Some(TokenCategory::Operator),
                    _ => Some(TokenCategory::Compare),
                };

                let next_ch: Option<char> = if (i + 1) < sql_bytes.len() {
                    Some(sql_bytes[i + 1].into())
                } else {
                    None
                };

                if next_ch != Some(EQUAL) {
                    tokens.push(curr_token);
                    curr_token = Token::new();
                }

                continue;
            }
            _ => (),
        }

        if curr_ch.is_whitespace() {
            if !curr_token.is_empty() {
                tokens.push(curr_token);
                curr_token = Token::new();
            }
            continue;
        }

        curr_token.value.push(curr_ch);
    }

    if !curr_token.is_empty() {
        tokens.push(curr_token);
    }

    return tokens;
}

fn get_in_comment(
    in_comment: &Option<CommentCategory>,
    sql_bytes: &[u8],
    curr_idx: usize,
    curr_token_len: usize,
) -> Option<CommentCategory> {
    let curr_ch: char = sql_bytes[curr_idx].into();
    match in_comment {
        Some(CommentCategory::SingleLine) => {
            if curr_ch == NEW_LINE {
                return None;
            }
            return Some(CommentCategory::SingleLine);
        }
        Some(CommentCategory::MultiLine) => {
            if curr_idx >= 2 && curr_token_len > 1 {
                let prev2_ch: char = sql_bytes[curr_idx - 2].into();
                let prev1_ch: char = sql_bytes[curr_idx - 1].into();
                if prev2_ch == ASTERISK && prev1_ch == SLASH_FORWARD {
                    return None;
                }
            }
            return Some(CommentCategory::MultiLine);
        }
        None => {
            if (curr_idx + 1) < sql_bytes.len() {
                let next_ch: char = sql_bytes[curr_idx + 1].into();

                if curr_ch == HYPHEN && next_ch == HYPHEN {
                    return Some(CommentCategory::SingleLine);
                }

                if curr_ch == SLASH_FORWARD && next_ch == ASTERISK {
                    return Some(CommentCategory::MultiLine);
                }
            }
            return None;
        }
    }
}

fn get_in_quote(
    in_quote: &Option<QuoteCategory>,
    sql_bytes: &[u8],
    curr_idx: usize,
    curr_token_len: usize,
) -> Option<QuoteCategory> {
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
                QuoteCategory::Backtick => {
                    if prev1_ch == BACKTICK {
                        return None;
                    }
                    return in_quote.clone();
                }
                QuoteCategory::QuoteSingle => {
                    if prev1_ch == QUOTE_SINGLE
                        && prev2_ch != QUOTE_SINGLE
                        && curr_ch != QUOTE_SINGLE
                    {
                        return None;
                    }
                    return in_quote.clone();
                }
                QuoteCategory::QuoteDouble => {
                    if prev1_ch == QUOTE_DOUBLE {
                        return None;
                    }
                    return in_quote.clone();
                }
                QuoteCategory::Bracket => {
                    if prev1_ch == BRACKET_CLOSE && curr_ch != FULL_STOP {
                        return None;
                    }
                    return in_quote.clone();
                }
            }
        }
        None => {
            return match curr_ch {
                BACKTICK => Some(QuoteCategory::Backtick),
                QUOTE_SINGLE => Some(QuoteCategory::QuoteSingle),
                QUOTE_DOUBLE => Some(QuoteCategory::QuoteDouble),
                BRACKET_OPEN => Some(QuoteCategory::Bracket),
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
                    category: None,
                },
                Token {
                    value: String::from("*"),
                    category: Some(TokenCategory::Operator),
                },
                Token {
                    value: String::from("FROM"),
                    category: None,
                },
                Token {
                    value: String::from("TBL1"),
                    category: None,
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
                    category: None,
                },
                Token {
                    value: String::from("1"),
                    category: None,
                },
                Token {
                    value: String::from("--comment inline"),
                    category: Some(TokenCategory::Comment),
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
                    category: None,
                },
                Token {
                    value: String::from("*"),
                    category: Some(TokenCategory::Operator),
                },
                Token {
                    value: String::from("\n"),
                    category: Some(TokenCategory::NewLine),
                },
                Token {
                    value: String::from("-- comment newline"),
                    category: Some(TokenCategory::Comment),
                },
                Token {
                    value: String::from("\n"),
                    category: Some(TokenCategory::NewLine),
                },
                Token {
                    value: String::from("FROM"),
                    category: None,
                },
                Token {
                    value: String::from("TBL1"),
                    category: None,
                },
            ],
            get_sql_tokens(String::from(
                r#"SELECT *
                -- comment newline
                FROM TBL1"#
            ))
        );
    }

    #[test]
    fn test_get_sql_tokens_comment_multi_inline() {
        assert_eq!(
            vec![
                Token {
                    value: String::from("SELECT"),
                    category: None,
                },
                Token {
                    value: String::from("*"),
                    category: Some(TokenCategory::Operator),
                },
                Token {
                    value: String::from("/*multi inline*/"),
                    category: Some(TokenCategory::Comment),
                },
                Token {
                    value: String::from("FROM"),
                    category: None,
                },
                Token {
                    value: String::from("TBL1"),
                    category: None,
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
                    category: Some(TokenCategory::Operator),
                },
                Token {
                    value: String::from("/*multi odd*/"),
                    category: Some(TokenCategory::Comment),
                },
                Token {
                    value: String::from("*"),
                    category: Some(TokenCategory::Operator),
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
                    category: None,
                },
                Token {
                    value: String::from("*"),
                    category: Some(TokenCategory::Operator),
                },
                Token {
                    value: String::from("\n"),
                    category: Some(TokenCategory::NewLine),
                },
                Token {
                    value: String::from(
                        r#"/*
                    multi line
                    comment
                */"#
                    ),
                    category: Some(TokenCategory::Comment),
                },
                Token {
                    value: String::from("\n"),
                    category: Some(TokenCategory::NewLine),
                },
                Token {
                    value: String::from("FROM"),
                    category: None,
                },
                Token {
                    value: String::from("TBL1"),
                    category: None,
                },
            ],
            get_sql_tokens(String::from(
                r#"SELECT *
                /*
                    multi line
                    comment
                */
                FROM TBL1"#
            ))
        );
    }

    #[test]
    fn test_get_sql_tokens_quote_backtick() {
        assert_eq!(
            vec![
                Token {
                    value: String::from("SELECT"),
                    category: None,
                },
                Token {
                    value: String::from("`Column 1`"),
                    category: Some(TokenCategory::Quote),
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
                    category: None,
                },
                Token {
                    value: String::from("'Column 1'"),
                    category: Some(TokenCategory::Quote),
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
                    category: None,
                },
                Token {
                    value: String::from("\"Column 1\""),
                    category: Some(TokenCategory::Quote),
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
                    category: None,
                },
                Token {
                    value: String::from("[Column 1]"),
                    category: Some(TokenCategory::Quote),
                },
            ],
            get_sql_tokens(String::from("SELECT [Column 1]"))
        );
    }

    #[test]
    fn test_get_sql_tokens_quote_bracket_schema() {
        assert_eq!(
            vec![
                Token {
                    value: String::from("SELECT"),
                    category: None,
                },
                Token {
                    value: String::from("*"),
                    category: Some(TokenCategory::Operator),
                },
                Token {
                    value: String::from("FROM"),
                    category: None,
                },
                Token {
                    value: String::from("[S].[TBL1]"),
                    category: Some(TokenCategory::Quote),
                },
            ],
            get_sql_tokens(String::from("SELECT * FROM [S].[TBL1]"))
        );
    }

    #[test]
    fn test_get_sql_tokens_quote_empty() {
        assert_eq!(
            vec![
                Token {
                    value: String::from("SELECT"),
                    category: None,
                },
                Token {
                    value: String::from("''"),
                    category: Some(TokenCategory::Quote),
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
                    category: None,
                },
                Token {
                    value: String::from("'Column''s Name'"),
                    category: Some(TokenCategory::Quote),
                },
            ],
            get_sql_tokens(String::from("SELECT 'Column''s Name'"))
        );
    }

    #[test]
    fn test_get_sql_tokens_quote_single_multiline() {
        assert_eq!(
            vec![
                Token {
                    value: String::from("SELECT"),
                    category: None,
                },
                Token {
                    value: String::from(
                        r#"'Column
Name'"#
                    ),
                    category: Some(TokenCategory::Quote),
                },
            ],
            get_sql_tokens(String::from(
                r#"SELECT 'Column
Name'"#
            ))
        );
    }

    #[test]
    fn test_get_sql_tokens_quote_single_abrupt_end() {
        assert_eq!(
            vec![
                Token {
                    value: String::from("SELECT"),
                    category: None,
                },
                Token {
                    value: String::from("'Column"),
                    category: Some(TokenCategory::Quote),
                },
            ],
            get_sql_tokens(String::from("SELECT 'Column"))
        );
    }

    #[test]
    fn test_get_sql_tokens_delimiter_basic() {
        assert_eq!(
            vec![
                Token {
                    value: String::from("SELECT"),
                    category: None,
                },
                Token {
                    value: String::from("1"),
                    category: None,
                },
                Token {
                    value: String::from(";"),
                    category: Some(TokenCategory::Delimiter),
                },
            ],
            get_sql_tokens(String::from("SELECT 1;"))
        );
    }

    #[test]
    fn test_get_sql_tokens_delimiter_two() {
        assert_eq!(
            vec![
                Token {
                    value: String::from("SELECT"),
                    category: None,
                },
                Token {
                    value: String::from("1"),
                    category: None,
                },
                Token {
                    value: String::from(";"),
                    category: Some(TokenCategory::Delimiter),
                },
                Token {
                    value: String::from("SELECT"),
                    category: None,
                },
                Token {
                    value: String::from("1"),
                    category: None,
                },
                Token {
                    value: String::from(";"),
                    category: Some(TokenCategory::Delimiter),
                },
            ],
            get_sql_tokens(String::from("SELECT 1; SELECT 1;"))
        );
    }

    #[test]
    fn test_get_sql_tokens_comma() {
        assert_eq!(
            vec![
                Token {
                    value: String::from("SELECT"),
                    category: None,
                },
                Token {
                    value: String::from("1"),
                    category: None,
                },
                Token {
                    value: String::from(","),
                    category: Some(TokenCategory::Comma),
                },
                Token {
                    value: String::from("2"),
                    category: None,
                },
                Token {
                    value: String::from(","),
                    category: Some(TokenCategory::Comma),
                },
                Token {
                    value: String::from("3"),
                    category: None,
                },
            ],
            get_sql_tokens(String::from("SELECT 1,2, 3"))
        );
    }

    #[test]
    fn test_get_sql_tokens_paren_empty() {
        assert_eq!(
            vec![
                Token {
                    value: String::from("SELECT"),
                    category: None,
                },
                Token {
                    value: String::from("MIN"),
                    category: None,
                },
                Token {
                    value: String::from("("),
                    category: Some(TokenCategory::ParenOpen),
                },
                Token {
                    value: String::from(")"),
                    category: Some(TokenCategory::ParenClose),
                },
            ],
            get_sql_tokens(String::from("SELECT MIN()"))
        );
    }

    #[test]
    fn test_get_sql_tokens_paren_content() {
        assert_eq!(
            vec![
                Token {
                    value: String::from("SELECT"),
                    category: None,
                },
                Token {
                    value: String::from("("),
                    category: Some(TokenCategory::ParenOpen),
                },
                Token {
                    value: String::from("SELECT"),
                    category: None,
                },
                Token {
                    value: String::from("1"),
                    category: None,
                },
                Token {
                    value: String::from(")"),
                    category: Some(TokenCategory::ParenClose),
                },
            ],
            get_sql_tokens(String::from("SELECT (SELECT 1)"))
        );
    }

    #[test]
    fn test_get_sql_tokens_operator_add() {
        assert_eq!(
            vec![
                Token {
                    value: String::from("1"),
                    category: None,
                },
                Token {
                    value: String::from("+"),
                    category: Some(TokenCategory::Operator),
                },
                Token {
                    value: String::from("2"),
                    category: None,
                },
                Token {
                    value: String::from("+"),
                    category: Some(TokenCategory::Operator),
                },
                Token {
                    value: String::from("3"),
                    category: None,
                },
            ],
            get_sql_tokens(String::from("1+2 + 3"))
        );
    }

    #[test]
    fn test_get_sql_tokens_operator_subtract() {
        assert_eq!(
            vec![
                Token {
                    value: String::from("1"),
                    category: None,
                },
                Token {
                    value: String::from("-"),
                    category: Some(TokenCategory::Operator),
                },
                Token {
                    value: String::from("2"),
                    category: None,
                },
                Token {
                    value: String::from("-"),
                    category: Some(TokenCategory::Operator),
                },
                Token {
                    value: String::from("3"),
                    category: None,
                },
            ],
            get_sql_tokens(String::from("1-2 - 3"))
        );
    }

    #[test]
    fn test_get_sql_tokens_operator_multiply() {
        assert_eq!(
            vec![
                Token {
                    value: String::from("1"),
                    category: None,
                },
                Token {
                    value: String::from("*"),
                    category: Some(TokenCategory::Operator),
                },
                Token {
                    value: String::from("2"),
                    category: None,
                },
                Token {
                    value: String::from("*"),
                    category: Some(TokenCategory::Operator),
                },
                Token {
                    value: String::from("3"),
                    category: None,
                },
            ],
            get_sql_tokens(String::from("1*2 * 3"))
        );
    }

    #[test]
    fn test_get_sql_tokens_operator_divide() {
        assert_eq!(
            vec![
                Token {
                    value: String::from("1"),
                    category: None,
                },
                Token {
                    value: String::from("/"),
                    category: Some(TokenCategory::Operator),
                },
                Token {
                    value: String::from("2"),
                    category: None,
                },
                Token {
                    value: String::from("/"),
                    category: Some(TokenCategory::Operator),
                },
                Token {
                    value: String::from("3"),
                    category: None,
                },
            ],
            get_sql_tokens(String::from("1/2 / 3"))
        );
    }

    #[test]
    fn test_get_sql_tokens_operator_modulo() {
        assert_eq!(
            vec![
                Token {
                    value: String::from("1"),
                    category: None,
                },
                Token {
                    value: String::from("%"),
                    category: Some(TokenCategory::Operator),
                },
                Token {
                    value: String::from("2"),
                    category: None,
                },
                Token {
                    value: String::from("%"),
                    category: Some(TokenCategory::Operator),
                },
                Token {
                    value: String::from("3"),
                    category: None,
                },
            ],
            get_sql_tokens(String::from("1%2 % 3"))
        );
    }

    #[test]
    fn test_get_sql_tokens_operator_add_equal() {
        assert_eq!(
            vec![
                Token {
                    value: String::from("V"),
                    category: None,
                },
                Token {
                    value: String::from("+="),
                    category: Some(TokenCategory::Operator),
                },
                Token {
                    value: String::from("1"),
                    category: None,
                },
            ],
            get_sql_tokens(String::from("V+=1"))
        );
    }

    #[test]
    fn test_get_sql_tokens_operator_minus_equal() {
        assert_eq!(
            vec![
                Token {
                    value: String::from("V"),
                    category: None,
                },
                Token {
                    value: String::from("-="),
                    category: Some(TokenCategory::Operator),
                },
                Token {
                    value: String::from("1"),
                    category: None,
                },
            ],
            get_sql_tokens(String::from("V-=1"))
        );
    }

    #[test]
    fn test_get_sql_tokens_operator_multiply_equal() {
        assert_eq!(
            vec![
                Token {
                    value: String::from("V"),
                    category: None,
                },
                Token {
                    value: String::from("*="),
                    category: Some(TokenCategory::Operator),
                },
                Token {
                    value: String::from("1"),
                    category: None,
                },
            ],
            get_sql_tokens(String::from("V*=1"))
        );
    }

    #[test]
    fn test_get_sql_tokens_operator_divide_equal() {
        assert_eq!(
            vec![
                Token {
                    value: String::from("V"),
                    category: None,
                },
                Token {
                    value: String::from("/="),
                    category: Some(TokenCategory::Operator),
                },
                Token {
                    value: String::from("1"),
                    category: None,
                },
            ],
            get_sql_tokens(String::from("V/=1"))
        );
    }

    #[test]
    fn test_get_sql_tokens_operator_modulo_equal() {
        assert_eq!(
            vec![
                Token {
                    value: String::from("V"),
                    category: None,
                },
                Token {
                    value: String::from("%="),
                    category: Some(TokenCategory::Operator),
                },
                Token {
                    value: String::from("1"),
                    category: None,
                },
            ],
            get_sql_tokens(String::from("V%=1"))
        );
    }

    #[test]
    fn test_get_sql_tokens_bitwise_and() {
        assert_eq!(
            vec![
                Token {
                    value: String::from("V1"),
                    category: None,
                },
                Token {
                    value: String::from("&"),
                    category: Some(TokenCategory::Bitwise),
                },
                Token {
                    value: String::from("V2"),
                    category: None,
                },
            ],
            get_sql_tokens(String::from("V1&V2"))
        );
    }

    #[test]
    fn test_get_sql_tokens_bitwise_or() {
        assert_eq!(
            vec![
                Token {
                    value: String::from("V1"),
                    category: None,
                },
                Token {
                    value: String::from("|"),
                    category: Some(TokenCategory::Bitwise),
                },
                Token {
                    value: String::from("V2"),
                    category: None,
                },
            ],
            get_sql_tokens(String::from("V1|V2"))
        );
    }

    #[test]
    fn test_get_sql_tokens_bitwise_exclusive_or() {
        assert_eq!(
            vec![
                Token {
                    value: String::from("V1"),
                    category: None,
                },
                Token {
                    value: String::from("^"),
                    category: Some(TokenCategory::Bitwise),
                },
                Token {
                    value: String::from("V2"),
                    category: None,
                },
            ],
            get_sql_tokens(String::from("V1^V2"))
        );
    }

    #[test]
    fn test_get_sql_tokens_paren_compare_lt() {
        assert_eq!(
            vec![
                Token {
                    value: String::from("WHERE"),
                    category: None,
                },
                Token {
                    value: String::from("C1"),
                    category: None,
                },
                Token {
                    value: String::from("<"),
                    category: Some(TokenCategory::Compare),
                },
                Token {
                    value: String::from("C2"),
                    category: None,
                },
                Token {
                    value: String::from("AND"),
                    category: None,
                },
                Token {
                    value: String::from("C1"),
                    category: None,
                },
                Token {
                    value: String::from("<"),
                    category: Some(TokenCategory::Compare),
                },
                Token {
                    value: String::from("C2"),
                    category: None,
                },
            ],
            get_sql_tokens(String::from("WHERE C1<C2 AND C1 < C2"))
        );
    }

    #[test]
    fn test_get_sql_tokens_paren_compare_gt() {
        assert_eq!(
            vec![
                Token {
                    value: String::from("WHERE"),
                    category: None,
                },
                Token {
                    value: String::from("C1"),
                    category: None,
                },
                Token {
                    value: String::from(">"),
                    category: Some(TokenCategory::Compare),
                },
                Token {
                    value: String::from("C2"),
                    category: None,
                },
                Token {
                    value: String::from("AND"),
                    category: None,
                },
                Token {
                    value: String::from("C1"),
                    category: None,
                },
                Token {
                    value: String::from(">"),
                    category: Some(TokenCategory::Compare),
                },
                Token {
                    value: String::from("C2"),
                    category: None,
                },
            ],
            get_sql_tokens(String::from("WHERE C1>C2 AND C1 > C2"))
        );
    }

    #[test]
    fn test_get_sql_tokens_paren_compare_eq() {
        assert_eq!(
            vec![
                Token {
                    value: String::from("WHERE"),
                    category: None,
                },
                Token {
                    value: String::from("C1"),
                    category: None,
                },
                Token {
                    value: String::from("="),
                    category: Some(TokenCategory::Compare),
                },
                Token {
                    value: String::from("C2"),
                    category: None,
                },
                Token {
                    value: String::from("AND"),
                    category: None,
                },
                Token {
                    value: String::from("C1"),
                    category: None,
                },
                Token {
                    value: String::from("="),
                    category: Some(TokenCategory::Compare),
                },
                Token {
                    value: String::from("C2"),
                    category: None,
                },
            ],
            get_sql_tokens(String::from("WHERE C1=C2 AND C1 = C2"))
        );
    }

    #[test]
    fn test_get_sql_tokens_paren_compare_neq() {
        assert_eq!(
            vec![
                Token {
                    value: String::from("WHERE"),
                    category: None,
                },
                Token {
                    value: String::from("C1"),
                    category: None,
                },
                Token {
                    value: String::from("<>"),
                    category: Some(TokenCategory::Compare),
                },
                Token {
                    value: String::from("C2"),
                    category: None,
                },
                Token {
                    value: String::from("AND"),
                    category: None,
                },
                Token {
                    value: String::from("C1"),
                    category: None,
                },
                Token {
                    value: String::from("<>"),
                    category: Some(TokenCategory::Compare),
                },
                Token {
                    value: String::from("C2"),
                    category: None,
                },
            ],
            get_sql_tokens(String::from("WHERE C1<>C2 AND C1 <> C2"))
        );
    }

    #[test]
    fn test_get_sql_tokens_paren_compare_gteq() {
        assert_eq!(
            vec![
                Token {
                    value: String::from("WHERE"),
                    category: None,
                },
                Token {
                    value: String::from("C1"),
                    category: None,
                },
                Token {
                    value: String::from(">="),
                    category: Some(TokenCategory::Compare),
                },
                Token {
                    value: String::from("C2"),
                    category: None,
                },
                Token {
                    value: String::from("AND"),
                    category: None,
                },
                Token {
                    value: String::from("C1"),
                    category: None,
                },
                Token {
                    value: String::from(">="),
                    category: Some(TokenCategory::Compare),
                },
                Token {
                    value: String::from("C2"),
                    category: None,
                },
            ],
            get_sql_tokens(String::from("WHERE C1>=C2 AND C1 >= C2"))
        );
    }

    #[test]
    fn test_get_sql_tokens_paren_compare_lteq() {
        assert_eq!(
            vec![
                Token {
                    value: String::from("WHERE"),
                    category: None,
                },
                Token {
                    value: String::from("C1"),
                    category: None,
                },
                Token {
                    value: String::from("<="),
                    category: Some(TokenCategory::Compare),
                },
                Token {
                    value: String::from("C2"),
                    category: None,
                },
                Token {
                    value: String::from("AND"),
                    category: None,
                },
                Token {
                    value: String::from("C1"),
                    category: None,
                },
                Token {
                    value: String::from("<="),
                    category: Some(TokenCategory::Compare),
                },
                Token {
                    value: String::from("C2"),
                    category: None,
                },
            ],
            get_sql_tokens(String::from("WHERE C1<=C2 AND C1 <= C2"))
        );
    }
}
