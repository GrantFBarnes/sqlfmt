use crate::configuration::Configuration;

const AMPERSAND: char = '&';
const ASTERISK: char = '*';
const BACKSLASH: char = '\\';
const BACKTICK: char = '`';
const BRACKET_CLOSE: char = ']';
const BRACKET_OPEN: char = '[';
const CARRIAGE_RETURN: char = '\r';
const CIRCUMFLEX: char = '^';
const COMMA: char = ',';
const CURLY_BRACKET_CLOSE: char = '}';
const CURLY_BRACKET_OPEN: char = '{';
const EQUAL: char = '=';
const EXCLAMATION: char = '!';
pub const FULL_STOP: char = '.';
const GREATER_THAN: char = '>';
const HYPHEN: char = '-';
const LESS_THAN: char = '<';
pub const NEW_LINE: char = '\n';
pub const PAREN_CLOSE: char = ')';
pub const PAREN_OPEN: char = '(';
const PERCENT: char = '%';
const PLUS: char = '+';
const QUOTE_DOUBLE: char = '"';
const QUOTE_SINGLE: char = '\'';
const SLASH_FORWARD: char = '/';
pub const TAB: char = '\t';
const VERTICAL_BAR: char = '|';

// invalid UTF-8 bytes (0x80..=0xFF) are carried through as private use chars
const RAW_BYTE_BASE: u32 = 0x10FF00;

fn raw_byte_char(byte: u8) -> char {
    char::from_u32(RAW_BYTE_BASE + byte as u32).expect("raw byte placeholder is a valid char")
}

fn is_raw_byte(c: char) -> bool {
    (RAW_BYTE_BASE + 0x80..=RAW_BYTE_BASE + 0xFF).contains(&(c as u32))
}

fn push_valid_utf8(result: &mut String, valid: &str) {
    // placeholder-range chars all start with the 0xF4 lead byte
    if !valid.bytes().any(|b: u8| b == 0xF4) {
        result.push_str(valid);
        return;
    }

    let mut buf: [u8; 4] = [0; 4];
    for character in valid.chars() {
        if is_raw_byte(character) {
            for byte in character.encode_utf8(&mut buf).bytes() {
                result.push(raw_byte_char(byte));
            }
        } else {
            result.push(character);
        }
    }
}

pub fn decode_input(bytes: &[u8]) -> String {
    let mut result: String = String::with_capacity(bytes.len());
    let mut rest: &[u8] = bytes;
    loop {
        match std::str::from_utf8(rest) {
            Ok(valid) => {
                push_valid_utf8(&mut result, valid);
                return result;
            }
            Err(err) => {
                let valid_len: usize = err.valid_up_to();
                let valid: &str =
                    std::str::from_utf8(&rest[..valid_len]).expect("prefix is valid UTF-8");
                push_valid_utf8(&mut result, valid);
                let invalid_len: usize = err.error_len().unwrap_or(rest.len() - valid_len);
                for byte in &rest[valid_len..valid_len + invalid_len] {
                    result.push(raw_byte_char(*byte));
                }
                rest = &rest[valid_len + invalid_len..];
            }
        }
    }
}

pub fn encode_output(sql: &str) -> Vec<u8> {
    let mut result: Vec<u8> = Vec::with_capacity(sql.len());
    let mut buf: [u8; 4] = [0; 4];
    for c in sql.chars() {
        if is_raw_byte(c) {
            result.push((c as u32 - RAW_BYTE_BASE) as u8);
        } else {
            result.extend_from_slice(c.encode_utf8(&mut buf).as_bytes());
        }
    }
    result
}

pub fn get_sql_tokens(config: &Configuration, input_sql: String) -> Vec<Token> {
    let mut tokens: Vec<Token> = vec![];

    let mut delimiter: Vec<char> = vec![';'];

    let mut curr_token: Token = Token::new();
    let mut in_special: Option<TokenSpecial> = None;
    let mut quote_scan: QuoteScan = QuoteScan::default();

    let sql_chars: Vec<char> = input_sql.chars().collect();

    // index of the first char after a delimiter that has already been consumed
    let mut skip_until: usize = 0;

    // push the current token (if any) and start a new one
    macro_rules! flush_token {
        () => {
            if !curr_token.is_empty() {
                curr_token.setup(config);
                tokens.push(curr_token);
                curr_token = Token::new();
            }
        };
    }

    for i in 0..sql_chars.len() {
        if i < skip_until {
            continue;
        }

        let prev2_ch: Option<&char> = if i >= 2 { sql_chars.get(i - 2) } else { None };
        let prev1_ch: Option<&char> = if i >= 1 { sql_chars.get(i - 1) } else { None };
        let curr_ch: char = sql_chars[i];
        let next1_ch: Option<&char> = sql_chars.get(i + 1);

        match in_special {
            None => {
                if (curr_ch.is_whitespace() && prev1_ch.is_some_and(|c| !c.is_whitespace()))
                    || (!curr_ch.is_whitespace() && prev1_ch.is_some_and(|c| c.is_whitespace()))
                {
                    flush_token!();
                }

                if get_in_delimiter_change(false, get_last_nonspace_token(&tokens), curr_ch) {
                    in_special = Some(TokenSpecial::DelimiterChange);
                    curr_token.category = TokenCategory::Delimiter;
                } else if let Some(ic) =
                    get_in_interpolation(None, prev2_ch, prev1_ch, curr_ch, next1_ch)
                {
                    flush_token!();
                    in_special = Some(TokenSpecial::Interpolation(ic));
                    curr_token.category = TokenCategory::Interpolation;
                } else if let Some(cc) = get_in_comment(
                    &None,
                    prev2_ch,
                    prev1_ch,
                    curr_ch,
                    next1_ch,
                    curr_token.value.chars().take(4).count(),
                ) {
                    flush_token!();
                    in_special = Some(TokenSpecial::Comment(cc));
                    curr_token.category = TokenCategory::Comment;
                } else if let Some(qc) = get_in_quote(
                    &None,
                    prev1_ch,
                    curr_ch,
                    next1_ch,
                    &curr_token,
                    config,
                    &mut quote_scan,
                ) {
                    quote_scan = QuoteScan::default();
                    flush_token!();
                    in_special = Some(TokenSpecial::Quote(qc));
                    curr_token.category = TokenCategory::Quote;
                }

                if in_special.is_some() {
                    curr_token.setup(config);
                    curr_token.value.push(curr_ch);
                    continue;
                }
            }
            Some(TokenSpecial::DelimiterChange) => {
                if get_in_delimiter_change(true, get_last_nonspace_token(&tokens), curr_ch) {
                    curr_token.value.push(curr_ch);
                    continue;
                } else {
                    in_special = None;
                    delimiter = curr_token.value.chars().collect();
                    curr_token.upper = curr_token.value.to_uppercase();
                    tokens.push(curr_token);
                    curr_token = Token::new();
                }
            }
            Some(TokenSpecial::Interpolation(ic)) => {
                if get_in_interpolation(Some(ic), prev2_ch, prev1_ch, curr_ch, next1_ch).is_some() {
                    if curr_ch != CARRIAGE_RETURN {
                        curr_token.value.push(curr_ch);
                    }
                    continue;
                } else {
                    in_special = None;
                    curr_token.upper = curr_token.value.to_uppercase();
                    tokens.push(curr_token);
                    curr_token = Token::new();
                }
            }
            Some(TokenSpecial::Comment(cc)) => {
                if get_in_comment(
                    &Some(cc),
                    prev2_ch,
                    prev1_ch,
                    curr_ch,
                    next1_ch,
                    curr_token.value.chars().take(4).count(),
                )
                .is_some()
                {
                    if curr_ch != CARRIAGE_RETURN {
                        curr_token.value.push(curr_ch);
                    }
                    continue;
                } else {
                    in_special = None;
                    curr_token.upper = curr_token.value.to_uppercase();
                    tokens.push(curr_token);
                    curr_token = Token::new();
                }
            }
            Some(TokenSpecial::Quote(qc)) => {
                if get_in_quote(
                    &Some(qc),
                    prev1_ch,
                    curr_ch,
                    next1_ch,
                    &curr_token,
                    config,
                    &mut quote_scan,
                )
                .is_some()
                {
                    curr_token.value.push(curr_ch);
                    continue;
                } else {
                    in_special = None;
                    flush_token!();
                }
            }
        }

        // checked before the char-specific branches below so a delimiter made
        // of operator or punctuation chars (e.g. "//") is still recognized
        if !delimiter.is_empty() && sql_chars[i..].starts_with(&delimiter) {
            flush_token!();
            curr_token.value = delimiter.iter().collect();
            curr_token.category = TokenCategory::Delimiter;
            flush_token!();
            skip_until = i + delimiter.len();
            continue;
        }

        match curr_ch {
            CARRIAGE_RETURN => continue,
            // always single character tokens
            NEW_LINE | COMMA | FULL_STOP | PAREN_OPEN | PAREN_CLOSE | AMPERSAND | VERTICAL_BAR
            | CIRCUMFLEX => {
                flush_token!();
                curr_token.value.push(curr_ch);
                flush_token!();
                continue;
            }
            // operators
            PLUS | HYPHEN | ASTERISK | SLASH_FORWARD | PERCENT => {
                flush_token!();
                curr_token.value.push(curr_ch);

                if curr_ch == PLUS || curr_ch == HYPHEN {
                    // check if a positive/negative sign
                    if get_last_nonspace_token(&tokens)
                        .is_some_and(|t| t.category != TokenCategory::Identifier)
                    {
                        continue;
                    }
                }

                curr_token.setup(config);

                if next1_ch != Some(&EQUAL) {
                    tokens.push(curr_token);
                    curr_token = Token::new();
                }

                continue;
            }
            // comparators
            EQUAL | LESS_THAN | GREATER_THAN | EXCLAMATION => {
                // operators
                if prev1_ch != Some(&PLUS)
                    && prev1_ch != Some(&HYPHEN)
                    && prev1_ch != Some(&ASTERISK)
                    && prev1_ch != Some(&SLASH_FORWARD)
                    && prev1_ch != Some(&PERCENT)
                    // comparators
                    && prev1_ch != Some(&EQUAL)
                    && prev1_ch != Some(&LESS_THAN)
                    && prev1_ch != Some(&GREATER_THAN)
                    && prev1_ch != Some(&EXCLAMATION)
                {
                    flush_token!();
                }
                curr_token.value.push(curr_ch);
                curr_token.category = match prev1_ch {
                    Some(&PLUS) => TokenCategory::Operator,
                    Some(&HYPHEN) => TokenCategory::Operator,
                    Some(&ASTERISK) => TokenCategory::Operator,
                    Some(&SLASH_FORWARD) => TokenCategory::Operator,
                    Some(&PERCENT) => TokenCategory::Operator,
                    _ => TokenCategory::Compare,
                };
                curr_token.setup(config);

                if next1_ch != Some(&EQUAL)
                    && next1_ch != Some(&LESS_THAN)
                    && next1_ch != Some(&GREATER_THAN)
                {
                    tokens.push(curr_token);
                    curr_token = Token::new();
                }

                continue;
            }
            _ => (),
        }

        curr_token.value.push(curr_ch);
    }

    if !curr_token.is_empty() {
        curr_token.setup(config);
        tokens.push(curr_token);
    }

    tokens
}

fn get_last_nonspace_token(tokens: &[Token]) -> Option<&Token> {
    for i in (0..tokens.len()).rev() {
        let token: &Token = &tokens[i];
        if token.category == TokenCategory::WhiteSpace {
            continue;
        }
        return Some(token);
    }
    None
}

fn get_in_delimiter_change(
    in_delimiter_change: bool,
    prev_token: Option<&Token>,
    curr_ch: char,
) -> bool {
    if curr_ch.is_whitespace() {
        return false;
    }

    if prev_token.is_some_and(|t| t.upper == "DELIMITER") {
        return true;
    }

    in_delimiter_change
}

fn get_in_interpolation(
    in_interpolation: Option<InterpolationCategory>,
    prev2_ch: Option<&char>,
    prev1_ch: Option<&char>,
    curr_ch: char,
    next1_ch: Option<&char>,
) -> Option<InterpolationCategory> {
    match in_interpolation {
        Some(InterpolationCategory::Bracket) => {
            if prev1_ch == Some(&CURLY_BRACKET_CLOSE) {
                return None;
            }
            Some(InterpolationCategory::Bracket)
        }
        Some(InterpolationCategory::Percent) => {
            if prev2_ch == Some(&PERCENT) {
                None
            } else {
                Some(InterpolationCategory::Percent)
            }
        }
        None => {
            match curr_ch {
                CURLY_BRACKET_OPEN => Some(InterpolationCategory::Bracket),
                PERCENT => match next1_ch {
                    // golang format verbs
                    Some('v') | Some('T') | Some('%') | Some('t') | Some('b') | Some('c')
                    | Some('d') | Some('o') | Some('O') | Some('q') | Some('x') | Some('X')
                    | Some('U') | Some('e') | Some('E') | Some('f') | Some('F') | Some('g')
                    | Some('G') | Some('s') | Some('p') => Some(InterpolationCategory::Percent),
                    _ => None,
                },
                _ => None,
            }
        }
    }
}

fn get_in_comment(
    in_comment: &Option<CommentCategory>,
    prev2_ch: Option<&char>,
    prev1_ch: Option<&char>,
    curr_ch: char,
    next1_ch: Option<&char>,
    curr_token_len: usize,
) -> Option<CommentCategory> {
    match in_comment {
        Some(cc) => {
            if curr_token_len <= 1 {
                return *in_comment;
            }

            match cc {
                CommentCategory::SingleLine => {
                    if curr_ch == NEW_LINE {
                        return None;
                    }
                    Some(CommentCategory::SingleLine)
                }
                CommentCategory::MultiLine => {
                    if curr_ch == SLASH_FORWARD && next1_ch == Some(&ASTERISK) {
                        return Some(CommentCategory::MultiLine);
                    }
                    // shortest closed comment is "/**/", so the opening "/*"
                    // cannot also be part of the closing "*/" (e.g. "/*/")
                    if curr_token_len >= 4
                        && prev2_ch == Some(&ASTERISK)
                        && prev1_ch == Some(&SLASH_FORWARD)
                    {
                        return None;
                    }
                    Some(CommentCategory::MultiLine)
                }
            }
        }
        None => {
            if curr_ch == HYPHEN && next1_ch == Some(&HYPHEN) {
                return Some(CommentCategory::SingleLine);
            }

            if curr_ch == SLASH_FORWARD && next1_ch == Some(&ASTERISK) {
                return Some(CommentCategory::MultiLine);
            }

            None
        }
    }
}

// resumable count_unescaped state for the current quote token, so a quote
// token is not rescanned from the start on every check, which is slow
#[derive(Default)]
struct QuoteScan {
    bytes_scanned: usize,
    count: usize,
    preceding_backslashes: usize,
}

// counts occurrences of target, skipping backslash-escaped ones (an odd run
// of preceding backslashes escapes the char) when backslash_escapes is set
fn count_unescaped(
    value: &str,
    target: char,
    backslash_escapes: bool,
    scan: &mut QuoteScan,
) -> usize {
    for c in value[scan.bytes_scanned..].chars() {
        if c == target && (!backslash_escapes || scan.preceding_backslashes.is_multiple_of(2)) {
            scan.count += 1;
        }
        if c == BACKSLASH {
            scan.preceding_backslashes += 1;
        } else {
            scan.preceding_backslashes = 0;
        }
    }
    scan.bytes_scanned = value.len();
    scan.count
}

fn get_in_quote(
    in_quote: &Option<QuoteCategory>,
    prev1_ch: Option<&char>,
    curr_ch: char,
    next1_ch: Option<&char>,
    curr_token: &Token,
    config: &Configuration,
    quote_scan: &mut QuoteScan,
) -> Option<QuoteCategory> {
    match in_quote {
        Some(qc) => {
            if curr_token.value.chars().nth(1).is_none() {
                return *in_quote;
            }

            // a doubled close char is an escaped literal, so only close once
            // the close char is not followed by another and the count of
            // close chars shows the quote is balanced
            match qc {
                QuoteCategory::Backtick => {
                    if prev1_ch == Some(&BACKTICK)
                        && curr_ch != BACKTICK
                        && count_unescaped(&curr_token.value, BACKTICK, false, quote_scan)
                            .is_multiple_of(2)
                    {
                        return None;
                    }
                    *in_quote
                }
                QuoteCategory::QuoteSingle => {
                    if prev1_ch == Some(&QUOTE_SINGLE)
                        && curr_ch != QUOTE_SINGLE
                        && count_unescaped(
                            &curr_token.value,
                            QUOTE_SINGLE,
                            // E'' strings always treat \' as an escaped quote
                            config.backslash_escapes || curr_token.value.starts_with(['E', 'e']),
                            quote_scan,
                        )
                        .is_multiple_of(2)
                    {
                        return None;
                    }
                    *in_quote
                }
                QuoteCategory::QuoteDouble => {
                    if prev1_ch == Some(&QUOTE_DOUBLE)
                        && curr_ch != QUOTE_DOUBLE
                        && count_unescaped(&curr_token.value, QUOTE_DOUBLE, false, quote_scan)
                            .is_multiple_of(2)
                    {
                        return None;
                    }
                    *in_quote
                }
                QuoteCategory::Bracket => {
                    if prev1_ch == Some(&BRACKET_CLOSE)
                        && curr_ch != BRACKET_CLOSE
                        && !count_unescaped(&curr_token.value, BRACKET_CLOSE, false, quote_scan)
                            .is_multiple_of(2)
                    {
                        return None;
                    }
                    *in_quote
                }
            }
        }
        None => match curr_ch {
            BACKTICK => Some(QuoteCategory::Backtick),
            QUOTE_SINGLE => Some(QuoteCategory::QuoteSingle),
            QUOTE_DOUBLE => Some(QuoteCategory::QuoteDouble),
            BRACKET_OPEN => Some(QuoteCategory::Bracket),
            // string prefixes: N'' national, X'' hex, B'' bit, E'' escape
            // only a prefix when it starts a new token, otherwise it is the
            // end of a word directly followed by a quote (e.g. "ELSE'a'")
            'N' | 'n' | 'X' | 'x' | 'B' | 'b' | 'E' | 'e'
                if next1_ch == Some(&QUOTE_SINGLE) && curr_token.is_empty() =>
            {
                Some(QuoteCategory::QuoteSingle)
            }
            _ => None,
        },
    }
}

#[derive(Clone)]
pub struct Token {
    pub value: String,
    pub upper: String,
    pub category: TokenCategory,
    pub behavior: Vec<TokenBehavior>,
}

impl std::fmt::Debug for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Token")
            .field("value", &self.value)
            .field("category", &self.category)
            .finish()
    }
}

impl PartialEq for Token {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value && self.category == other.category
    }
}

impl Token {
    fn new() -> Token {
        Token {
            value: String::new(),
            upper: String::new(),
            category: TokenCategory::Identifier,
            behavior: vec![],
        }
    }

    #[allow(dead_code)]
    fn new_test(value: &str, category: TokenCategory) -> Token {
        Token {
            value: String::from(value),
            upper: value.to_uppercase(),
            category,
            behavior: vec![],
        }
    }

    pub fn new_whitespace(value: String) -> Token {
        Token {
            upper: value.clone(),
            value,
            category: TokenCategory::WhiteSpace,
            behavior: vec![TokenBehavior::NoSpaceBefore, TokenBehavior::NoSpaceAfter],
        }
    }

    pub fn new_newline() -> Token {
        Token {
            value: "\n".to_string(),
            upper: "\n".to_string(),
            category: TokenCategory::NewLine,
            behavior: vec![
                TokenBehavior::NoWhiteSpaceBefore,
                TokenBehavior::NoNewLineAfterX2Skip,
            ],
        }
    }

    // display length in chars (not bytes) so non-ASCII text is not over-counted
    pub fn len(&self) -> usize {
        let is_whitespace: bool = self.category == TokenCategory::WhiteSpace;
        self.value
            .chars()
            .map(|c: char| if is_whitespace && c == TAB { 4 } else { 1 })
            .sum()
    }

    fn is_empty(&self) -> bool {
        self.value.is_empty()
    }

    fn setup(&mut self, config: &Configuration) {
        self.upper = self.value.to_uppercase();
        self.category = self.get_category();
        self.set_behavior(config);
    }

    fn get_category(&self) -> TokenCategory {
        if self.category == TokenCategory::Quote {
            let mut quote_chars = self.upper.chars();
            if let Some(open_quote) = quote_chars.next()
                && open_quote == BRACKET_OPEN
                && let Some(close_quote) = quote_chars.next_back()
                && close_quote == BRACKET_CLOSE
                && get_category_from_value(quote_chars.as_str()) == TokenCategory::DataType
            {
                return TokenCategory::DataType;
            }
        }

        if self.category != TokenCategory::Identifier {
            return self.category;
        }

        if self.value.len() == 1 {
            let value_char: char = self.value.chars().nth(0).unwrap();
            return match value_char {
                NEW_LINE => TokenCategory::NewLine,
                COMMA => TokenCategory::Comma,
                FULL_STOP => TokenCategory::FullStop,
                PAREN_OPEN => TokenCategory::ParenOpen,
                PAREN_CLOSE => TokenCategory::ParenClose,
                AMPERSAND => TokenCategory::Bitwise,
                VERTICAL_BAR => TokenCategory::Bitwise,
                CIRCUMFLEX => TokenCategory::Bitwise,
                LESS_THAN => TokenCategory::Compare,
                PLUS => TokenCategory::Operator,
                HYPHEN => TokenCategory::Operator,
                ASTERISK => TokenCategory::Operator,
                SLASH_FORWARD => TokenCategory::Operator,
                PERCENT => TokenCategory::Operator,
                _ => {
                    if value_char.is_whitespace() {
                        return TokenCategory::WhiteSpace;
                    }
                    TokenCategory::Identifier
                }
            };
        }

        let mut all_whitespace: bool = true;
        for c in self.value.chars() {
            if !c.is_whitespace() {
                all_whitespace = false;
                break;
            }
        }
        if all_whitespace {
            return TokenCategory::WhiteSpace;
        }

        get_category_from_value(self.upper.as_str())
    }

    fn set_behavior(&mut self, config: &Configuration) {
        let mut behavior: Vec<TokenBehavior> = vec![];

        match self.category {
            TokenCategory::WhiteSpace => (), // defined in self.new_whitespace() method
            TokenCategory::NewLine => (),    // defined in self.new_newline() method
            TokenCategory::Delimiter => {
                behavior.push(TokenBehavior::NewLineAfterX2);
                behavior.push(TokenBehavior::NoNewLineAfterX2Skip);
            }
            TokenCategory::ParenOpen => {
                behavior.push(TokenBehavior::NoSpaceAfter);
                behavior.push(TokenBehavior::IncreaseIndent);
            }
            TokenCategory::ParenClose => {
                behavior.push(TokenBehavior::NoSpaceBefore);
                behavior.push(TokenBehavior::NewLineBefore);
            }
            TokenCategory::Comma => {
                behavior.push(TokenBehavior::NoSpaceBefore);
                behavior.push(TokenBehavior::NewLineAfter);
            }
            TokenCategory::FullStop => {
                behavior.push(TokenBehavior::NoWhiteSpaceBefore);
                behavior.push(TokenBehavior::NoSpaceAfter);
            }
            TokenCategory::Interpolation => {
                behavior.push(TokenBehavior::NoSpaceAroundIfNotProvidedInput);
            }
            TokenCategory::Comment => {
                if config.comment_pre_space {
                    behavior.push(TokenBehavior::NewLineBefore);
                    behavior.push(TokenBehavior::NoNewLineAfterX2Skip);
                    behavior.push(TokenBehavior::NewLineAfter);
                } else {
                    behavior.push(TokenBehavior::PreSpaceKeepInput);
                    behavior.push(TokenBehavior::NoSpaceBeforeIfStartOfNewLine);
                    behavior.push(TokenBehavior::NoNewLineBeforeUnlessMatch);
                    behavior.push(TokenBehavior::NoNewLineAfterX2Skip);
                    behavior.push(TokenBehavior::NewLineAfter);
                }
            }
            _ => (),
        }

        match self.upper.as_str() {
            "AFTER" => {
                behavior.push(TokenBehavior::NewLineBefore);
            }
            "ALL" => {
                behavior.push(TokenBehavior::NoNewLineBefore);
            }
            "ALTER" => {
                behavior.push(TokenBehavior::NewLineBefore);
                behavior.push(TokenBehavior::DecreaseIndent);
            }
            "AND" => {
                behavior.push(TokenBehavior::NewLineBefore);
            }
            "BEFORE" => {
                behavior.push(TokenBehavior::NewLineBefore);
            }
            "BEGIN" => {
                behavior.push(TokenBehavior::NewLineBefore);
                behavior.push(TokenBehavior::NewLineAfter);
                behavior.push(TokenBehavior::NoNewLineAfterX2Skip);
                behavior.push(TokenBehavior::IncreaseIndent);
                behavior.push(TokenBehavior::DecreaseIndent);
            }
            "BREAK" => {
                behavior.push(TokenBehavior::DecreaseIndent);
            }
            "CALL" => {
                behavior.push(TokenBehavior::NewLineBefore);
                behavior.push(TokenBehavior::IncreaseIndent);
                behavior.push(TokenBehavior::DecreaseIndent);
            }
            "CASE" => {
                behavior.push(TokenBehavior::NewLineBefore);
                behavior.push(TokenBehavior::NewLineAfter);
                behavior.push(TokenBehavior::IncreaseIndentIfStandAlone);
            }
            "CATCH" => {
                behavior.push(TokenBehavior::NoNewLineBefore);
                behavior.push(TokenBehavior::NewLineAfterX2IfCombined);
                behavior.push(TokenBehavior::NewLineAfter);
                behavior.push(TokenBehavior::NoNewLineAfterX2Skip);
            }
            "CLOSE" => {
                behavior.push(TokenBehavior::NewLineBefore);
            }
            "COMMIT" => {
                behavior.push(TokenBehavior::NewLineBefore);
            }
            "CONTINUE" => {
                behavior.push(TokenBehavior::DecreaseIndent);
            }
            "CREATE" => {
                behavior.push(TokenBehavior::NewLineBefore);
                behavior.push(TokenBehavior::DecreaseIndent);
            }
            "CROSS" => {
                behavior.push(TokenBehavior::NewLineBefore);
            }
            "DEALLOCATE" => {
                behavior.push(TokenBehavior::NewLineBefore);
            }
            "DECLARE" => {
                behavior.push(TokenBehavior::NewLineBefore);
                behavior.push(TokenBehavior::IncreaseIndent);
                behavior.push(TokenBehavior::DecreaseIndent);
            }
            "DELETE" => {
                behavior.push(TokenBehavior::NewLineBeforeIfNotAfterEvent);
                behavior.push(TokenBehavior::DecreaseIndent);
            }
            "DELIMITER" => {
                behavior.push(TokenBehavior::NewLineBefore);
                behavior.push(TokenBehavior::DecreaseIndent);
            }
            "DISTINCT" => {
                behavior.push(TokenBehavior::NoNewLineBefore);
                behavior.push(TokenBehavior::NewLineAfter);
            }
            "DO" => {
                behavior.push(TokenBehavior::NewLineBefore);
                behavior.push(TokenBehavior::NewLineAfter);
                behavior.push(TokenBehavior::NoNewLineAfterX2Skip);
                behavior.push(TokenBehavior::IncreaseIndent);
                behavior.push(TokenBehavior::DecreaseIndent);
            }
            "DROP" => {
                behavior.push(TokenBehavior::NewLineBefore);
                behavior.push(TokenBehavior::DecreaseIndent);
            }
            "ELSE" => {
                behavior.push(TokenBehavior::NewLineBefore);
                behavior.push(TokenBehavior::NoNewLineBeforeX2);
                behavior.push(TokenBehavior::IncreaseIndentIfStandAlone);
                behavior.push(TokenBehavior::DecreaseIndent);
            }
            "END" => {
                behavior.push(TokenBehavior::NewLineBefore);
                behavior.push(TokenBehavior::NoNewLineBeforeX2);
                behavior.push(TokenBehavior::NewLineAfterX2IfCombined);
            }
            "EXEC" => {
                behavior.push(TokenBehavior::NewLineBefore);
                behavior.push(TokenBehavior::IncreaseIndent);
                behavior.push(TokenBehavior::DecreaseIndent);
            }
            "EXECUTE" => {
                behavior.push(TokenBehavior::NewLineBefore);
                behavior.push(TokenBehavior::IncreaseIndent);
                behavior.push(TokenBehavior::DecreaseIndent);
            }
            "FETCH" => {
                behavior.push(TokenBehavior::NewLineBefore);
            }
            "FOR" => {
                behavior.push(TokenBehavior::NewLineBefore);
                behavior.push(TokenBehavior::IncreaseIndent);
                behavior.push(TokenBehavior::DecreaseIndent);
            }
            "FROM" => {
                behavior.push(TokenBehavior::NewLineBefore);
                behavior.push(TokenBehavior::IncreaseIndent);
                behavior.push(TokenBehavior::DecreaseIndent);
            }
            "FULL" => {
                behavior.push(TokenBehavior::NewLineBeforeIfNotAfterKeywordOrMethod);
            }
            "GO" => {
                behavior.push(TokenBehavior::NewLineBefore);
                behavior.push(TokenBehavior::NewLineAfterX2);
                behavior.push(TokenBehavior::DecreaseIndent);
            }
            "GOTO" => {
                behavior.push(TokenBehavior::DecreaseIndent);
            }
            "GROUP" => {
                behavior.push(TokenBehavior::NewLineBeforeIfNotAfterWithin);
                behavior.push(TokenBehavior::IncreaseIndent);
                behavior.push(TokenBehavior::DecreaseIndent);
            }
            "HAVING" => {
                behavior.push(TokenBehavior::NewLineBefore);
                behavior.push(TokenBehavior::IncreaseIndent);
                behavior.push(TokenBehavior::DecreaseIndent);
            }
            "IF" => {
                behavior.push(TokenBehavior::NewLineBeforeIfNotAfterEvent);
                behavior.push(TokenBehavior::DecreaseIndent);
            }
            "INNER" => {
                behavior.push(TokenBehavior::NewLineBefore);
            }
            "INSERT" => {
                behavior.push(TokenBehavior::NewLineBeforeIfNotAfterEvent);
                behavior.push(TokenBehavior::IncreaseIndent);
                behavior.push(TokenBehavior::DecreaseIndent);
            }
            "INTO" => {
                behavior.push(TokenBehavior::NewLineBeforeIfNotAfterKeywordOrMethod);
                behavior.push(TokenBehavior::NewLineAfterIfNotAfterKeyword);
                behavior.push(TokenBehavior::IncreaseIndentIfNotAfterKeyword);
                behavior.push(TokenBehavior::DecreaseIndent);
            }
            "ITERATE" => {
                behavior.push(TokenBehavior::DecreaseIndent);
            }
            "LEAVE" => {
                behavior.push(TokenBehavior::NewLineBefore);
                behavior.push(TokenBehavior::DecreaseIndent);
            }
            "LEFT" => {
                behavior.push(TokenBehavior::NewLineBeforeIfNotMethodCall);
            }
            "LIMIT" => {
                behavior.push(TokenBehavior::NewLineBefore);
                behavior.push(TokenBehavior::DecreaseIndent);
            }
            "MERGE" => {
                behavior.push(TokenBehavior::NewLineBefore);
                behavior.push(TokenBehavior::IncreaseIndent);
                behavior.push(TokenBehavior::DecreaseIndent);
            }
            "OPEN" => {
                behavior.push(TokenBehavior::NewLineBefore);
                behavior.push(TokenBehavior::IncreaseIndent);
                behavior.push(TokenBehavior::DecreaseIndent);
            }
            "OPTION" => {
                behavior.push(TokenBehavior::NewLineBefore);
                behavior.push(TokenBehavior::IncreaseIndent);
                behavior.push(TokenBehavior::DecreaseIndent);
            }
            "OR" => {
                behavior.push(TokenBehavior::NewLineBefore);
            }
            "ORDER" => {
                behavior.push(TokenBehavior::NewLineBefore);
                behavior.push(TokenBehavior::IncreaseIndent);
                behavior.push(TokenBehavior::DecreaseIndent);
            }
            "OUTER" => {
                behavior.push(TokenBehavior::NewLineBeforeIfNotAfterKeywordOrMethod);
            }
            "PIVOT" => {
                behavior.push(TokenBehavior::NewLineBefore);
                behavior.push(TokenBehavior::IncreaseIndent);
                behavior.push(TokenBehavior::DecreaseIndent);
            }
            "PRINT" => {
                behavior.push(TokenBehavior::DecreaseIndent);
            }
            "RAISERROR" => {
                behavior.push(TokenBehavior::NewLineBefore);
                behavior.push(TokenBehavior::DecreaseIndent);
            }
            "RESIGNAL" => {
                behavior.push(TokenBehavior::DecreaseIndent);
            }
            "RETURN" => {
                behavior.push(TokenBehavior::NewLineBefore);
                behavior.push(TokenBehavior::DecreaseIndent);
            }
            "RETURNS" => {
                behavior.push(TokenBehavior::NewLineBefore);
                behavior.push(TokenBehavior::DecreaseIndent);
            }
            "RIGHT" => {
                behavior.push(TokenBehavior::NewLineBeforeIfNotMethodCall);
            }
            "ROLLBACK" => {
                behavior.push(TokenBehavior::NewLineBefore);
            }
            "SELECT" => {
                behavior.push(TokenBehavior::NewLineBefore);
                behavior.push(TokenBehavior::NewLineAfter);
                behavior.push(TokenBehavior::IncreaseIndent);
                behavior.push(TokenBehavior::DecreaseIndent);
            }
            "SET" => {
                behavior.push(TokenBehavior::NewLineBeforeIfNotAfterEvent);
                behavior.push(TokenBehavior::IncreaseIndentIfNotAfterKeyword);
                behavior.push(TokenBehavior::DecreaseIndent);
            }
            "SIGNAL" => {
                behavior.push(TokenBehavior::DecreaseIndent);
            }
            "THEN" => {
                behavior.push(TokenBehavior::IncreaseIndentIfStandAlone);
            }
            "THROW" => {
                behavior.push(TokenBehavior::NewLineBefore);
                behavior.push(TokenBehavior::DecreaseIndent);
            }
            "TOP" => {
                behavior.push(TokenBehavior::NoNewLineBefore);
                behavior.push(TokenBehavior::NewLineAfterSkip);
            }
            "TRAN" | "TRANSACTION" => {
                behavior.push(TokenBehavior::NoNewLineBefore);
            }
            "TRUNCATE" => {
                behavior.push(TokenBehavior::NewLineBefore);
                behavior.push(TokenBehavior::DecreaseIndent);
            }
            "TRY" => {
                behavior.push(TokenBehavior::NoNewLineBefore);
                behavior.push(TokenBehavior::NewLineAfter);
                behavior.push(TokenBehavior::NoNewLineAfterX2Skip);
            }
            "UNION" => {
                behavior.push(TokenBehavior::NewLineBefore);
                behavior.push(TokenBehavior::NewLineAfter);
                behavior.push(TokenBehavior::DecreaseIndent);
            }
            "UPDATE" => {
                behavior.push(TokenBehavior::NewLineBeforeIfNotAfterEvent);
                behavior.push(TokenBehavior::IncreaseIndent);
                behavior.push(TokenBehavior::DecreaseIndent);
            }
            "USING" => {
                behavior.push(TokenBehavior::NewLineBefore);
                behavior.push(TokenBehavior::IncreaseIndent);
                behavior.push(TokenBehavior::DecreaseIndent);
            }
            "VALUES" => {
                behavior.push(TokenBehavior::NewLineBefore);
                behavior.push(TokenBehavior::DecreaseIndent);
            }
            "WAITFOR" => {
                behavior.push(TokenBehavior::DecreaseIndent);
            }
            "WHEN" => {
                behavior.push(TokenBehavior::NewLineBefore);
                behavior.push(TokenBehavior::IncreaseIndentIfStandAlone);
                behavior.push(TokenBehavior::DecreaseIndent);
            }
            "WHERE" => {
                behavior.push(TokenBehavior::NewLineBefore);
                behavior.push(TokenBehavior::IncreaseIndent);
                behavior.push(TokenBehavior::DecreaseIndent);
            }
            "WHILE" => {
                behavior.push(TokenBehavior::NewLineBefore);
                behavior.push(TokenBehavior::IncreaseIndent);
                behavior.push(TokenBehavior::DecreaseIndent);
            }
            "WITH" => {
                behavior.push(TokenBehavior::IncreaseIndent);
                behavior.push(TokenBehavior::DecreaseIndent);
            }
            _ => (),
        };

        self.behavior = behavior;
    }
}

fn get_category_from_value(value: &str) -> TokenCategory {
    match value {
        // Keywords
        "ABORT" => TokenCategory::Keyword,
        "ABORTSESSION" => TokenCategory::Keyword,
        "ABSENT" => TokenCategory::Keyword,
        "ABSOLUTE" => TokenCategory::Keyword,
        "ACCESS" => TokenCategory::Keyword,
        "ACCESSIBLE" => TokenCategory::Keyword,
        "ACCESS_LOCK" => TokenCategory::Keyword,
        "ACCOUNT" => TokenCategory::Keyword,
        "ACTION" => TokenCategory::Keyword,
        "ADA" => TokenCategory::Keyword,
        "ADD" => TokenCategory::Keyword,
        "ADMIN" => TokenCategory::Keyword,
        "AGGREGATE" => TokenCategory::Keyword,
        "ALIAS" => TokenCategory::Keyword,
        "ALL" => TokenCategory::Keyword,
        "ALLOCATE" => TokenCategory::Keyword,
        "ALLOW" => TokenCategory::Keyword,
        "ALTER" => TokenCategory::Keyword,
        "ALTERAND" => TokenCategory::Keyword,
        "ALWAYS" => TokenCategory::Keyword,
        "AMP" => TokenCategory::Keyword,
        "ANALYSE" => TokenCategory::Keyword,
        "ANALYZE" => TokenCategory::Keyword,
        "AND" => TokenCategory::Keyword,
        "ANSIDATE" => TokenCategory::Keyword,
        "ANSI_NULLS" => TokenCategory::Keyword,
        "ANTI" => TokenCategory::Keyword,
        "ANY" => TokenCategory::Keyword,
        "APPLY" => TokenCategory::Keyword,
        "ARE" => TokenCategory::Keyword,
        "ARRAY" => TokenCategory::Keyword,
        "ARRAY_EXISTS" => TokenCategory::Keyword,
        "ARRAY_MAX_CARDINALITY" => TokenCategory::Keyword,
        "AS" => TokenCategory::Keyword,
        "ASC" => TokenCategory::Keyword,
        "ASENSITIVE" => TokenCategory::Keyword,
        "ASOF" => TokenCategory::Keyword,
        "ASSERTION" => TokenCategory::Keyword,
        "ASSOCIATE" => TokenCategory::Keyword,
        "ASUTIME" => TokenCategory::Keyword,
        "ASYMMETRIC" => TokenCategory::Keyword,
        "AT" => TokenCategory::Keyword,
        "ATOMIC" => TokenCategory::Keyword,
        "AUDIT" => TokenCategory::Keyword,
        "AUTHORIZATION" => TokenCategory::Keyword,
        "AUTO" => TokenCategory::Keyword,
        "AUTO_INCREMENT" => TokenCategory::Keyword,
        "AUX" => TokenCategory::Keyword,
        "AUXILIARY" => TokenCategory::Keyword,
        "BACKUP" => TokenCategory::Keyword,
        "BEGIN" => TokenCategory::Keyword,
        "BEGIN_FRAME" => TokenCategory::Keyword,
        "BEGIN_PARTITION" => TokenCategory::Keyword,
        "BETWEEN" => TokenCategory::Keyword,
        "BOTH" => TokenCategory::Keyword,
        "BREADTH" => TokenCategory::Keyword,
        "BREAK" => TokenCategory::Keyword,
        "BROWSE" => TokenCategory::Keyword,
        "BT" => TokenCategory::Keyword,
        "BUFFERPOOL" => TokenCategory::Keyword,
        "BULK" => TokenCategory::Keyword,
        "BUT" => TokenCategory::Keyword,
        "BY" => TokenCategory::Keyword,
        "BYTE" => TokenCategory::Keyword,
        "BYTEINT" => TokenCategory::Keyword,
        "BYTES" => TokenCategory::Keyword,
        "CACHE" => TokenCategory::Keyword,
        "CALL" => TokenCategory::Keyword,
        "CALLED" => TokenCategory::Keyword,
        "CALLER" => TokenCategory::Keyword,
        "CAPTURE" => TokenCategory::Keyword,
        "CASCADE" => TokenCategory::Keyword,
        "CASCADED" => TokenCategory::Keyword,
        "CASE" => TokenCategory::Keyword,
        "CASESPECIFIC" => TokenCategory::Keyword,
        "CATALOG" => TokenCategory::Keyword,
        "CATCH" => TokenCategory::Keyword,
        "CCSID" => TokenCategory::Keyword,
        "CD" => TokenCategory::Keyword,
        "CHANGE" => TokenCategory::Keyword,
        "CHARSET" => TokenCategory::Keyword,
        "CHECK" => TokenCategory::Keyword,
        "CHECKPOINT" => TokenCategory::Keyword,
        "CLASS" => TokenCategory::Keyword,
        "CLOB" => TokenCategory::Keyword,
        "CLONE" => TokenCategory::Keyword,
        "CLOSE" => TokenCategory::Keyword,
        "CLUSTER" => TokenCategory::Keyword,
        "CLUSTERED" => TokenCategory::Keyword,
        "CM" => TokenCategory::Keyword,
        "COLLATE" => TokenCategory::Keyword,
        "COLLATION" => TokenCategory::Keyword,
        "COLLECT" => TokenCategory::Keyword,
        "COLLECTION" => TokenCategory::Keyword,
        "COLLID" => TokenCategory::Keyword,
        "COLUMNSTORE" => TokenCategory::Keyword,
        "COLUMN_VALUE" => TokenCategory::Keyword,
        "COMMENT" => TokenCategory::Keyword,
        "COMMIT" => TokenCategory::Keyword,
        "COMMITTED" => TokenCategory::Keyword,
        "COMPLETION" => TokenCategory::Keyword,
        "COMPRESS" => TokenCategory::Keyword,
        "COMPUTE" => TokenCategory::Keyword,
        "CONCURRENTLY" => TokenCategory::Keyword,
        "CONDITION" => TokenCategory::Keyword,
        "CONFLICT" => TokenCategory::Keyword,
        "CONNECT" => TokenCategory::Keyword,
        "CONNECTION" => TokenCategory::Keyword,
        "CONSTRAINT" => TokenCategory::Keyword,
        "CONSTRAINTS" => TokenCategory::Keyword,
        "CONSTRUCTOR" => TokenCategory::Keyword,
        "CONTAINED" => TokenCategory::Keyword,
        "CONTENT" => TokenCategory::Keyword,
        "CONTINUE" => TokenCategory::Keyword,
        "CONVERT_TABLE_HEADER" => TokenCategory::Keyword,
        "COPY" => TokenCategory::Keyword,
        "CORRESPONDING" => TokenCategory::Keyword,
        "CREATE" => TokenCategory::Keyword,
        "CROSS" => TokenCategory::Keyword,
        "CS" => TokenCategory::Keyword,
        "CT" => TokenCategory::Keyword,
        "CURRENT" => TokenCategory::Keyword,
        "CURRENT_CATALOG" => TokenCategory::Keyword,
        "CURRENT_DEFAULT_TRANSFORM_GROUP" => TokenCategory::Keyword,
        "CURRENT_LC_CTYPE" => TokenCategory::Keyword,
        "CURRENT_PATH" => TokenCategory::Keyword,
        "CURRENT_ROLE" => TokenCategory::Keyword,
        "CURRENT_ROW" => TokenCategory::Keyword,
        "CURRENT_SCHEMA" => TokenCategory::Keyword,
        "CURRENT_SERVER" => TokenCategory::Keyword,
        "CURRENT_TIMEZONE" => TokenCategory::Keyword,
        "CURRENT_TRANSFORM_GROUP_FOR_TYPE" => TokenCategory::Keyword,
        "CURSOR" => TokenCategory::Keyword,
        "CV" => TokenCategory::Keyword,
        "CYCLE" => TokenCategory::Keyword,
        "DATA" => TokenCategory::Keyword,
        "DATABASES" => TokenCategory::Keyword,
        "DATABLOCKSIZE" => TokenCategory::Keyword,
        "DATEFORM" => TokenCategory::Keyword,
        "DAYS" => TokenCategory::Keyword,
        "DAY_HOUR" => TokenCategory::Keyword,
        "DAY_MICROSECOND" => TokenCategory::Keyword,
        "DAY_MINUTE" => TokenCategory::Keyword,
        "DAY_SECOND" => TokenCategory::Keyword,
        "DBCC" => TokenCategory::Keyword,
        "DBINFO" => TokenCategory::Keyword,
        "DEALLOCATE" => TokenCategory::Keyword,
        "DECFLOAT" => TokenCategory::Keyword,
        "DECLARE" => TokenCategory::Keyword,
        "DEFAULT" => TokenCategory::Keyword,
        "DEFERRABLE" => TokenCategory::Keyword,
        "DEFERRED" => TokenCategory::Keyword,
        "DEFINE" => TokenCategory::Keyword,
        "DEFINER" => TokenCategory::Keyword,
        "DEL" => TokenCategory::Keyword,
        "DELAY" => TokenCategory::Keyword,
        "DELAYED" => TokenCategory::Keyword,
        "DELETE" => TokenCategory::Keyword,
        "DELETED" => TokenCategory::Keyword,
        "DELIMITER" => TokenCategory::Keyword,
        "DENY" => TokenCategory::Keyword,
        "DEPTH" => TokenCategory::Keyword,
        "DESC" => TokenCategory::Keyword,
        "DESCRIBE" => TokenCategory::Keyword,
        "DESCRIPTOR" => TokenCategory::Keyword,
        "DESTROY" => TokenCategory::Keyword,
        "DESTRUCTOR" => TokenCategory::Keyword,
        "DETERMINISTIC" => TokenCategory::Keyword,
        "DIAGNOSTIC" => TokenCategory::Keyword,
        "DIAGNOSTICS" => TokenCategory::Keyword,
        "DICTIONARY" => TokenCategory::Keyword,
        "DISABLE" => TokenCategory::Keyword,
        "DISABLED" => TokenCategory::Keyword,
        "DISALLOW" => TokenCategory::Keyword,
        "DISCONNECT" => TokenCategory::Keyword,
        "DISK" => TokenCategory::Keyword,
        "DISTINCT" => TokenCategory::Keyword,
        "DISTINCTROW" => TokenCategory::Keyword,
        "DISTRIBUTED" => TokenCategory::Keyword,
        "DO" => TokenCategory::Keyword,
        "DOCUMENT" => TokenCategory::Keyword,
        "DOMAIN" => TokenCategory::Keyword,
        "DROP" => TokenCategory::Keyword,
        "DSSIZE" => TokenCategory::Keyword,
        "DUAL" => TokenCategory::Keyword,
        "DUMP" => TokenCategory::Keyword,
        "DUPLICATE" => TokenCategory::Keyword,
        "DYNAMIC" => TokenCategory::Keyword,
        "EACH" => TokenCategory::Keyword,
        "ECHO" => TokenCategory::Keyword,
        "EDITPROC" => TokenCategory::Keyword,
        "ELEMENTS" => TokenCategory::Keyword,
        "ELSE" => TokenCategory::Keyword,
        "ELSEIF" => TokenCategory::Keyword,
        "ELSIF" => TokenCategory::Keyword,
        "EMPTY" => TokenCategory::Keyword,
        "ENABLED" => TokenCategory::Keyword,
        "ENCLOSED" => TokenCategory::Keyword,
        "ENCODING" => TokenCategory::Keyword,
        "ENCRYPTED" => TokenCategory::Keyword,
        "ENCRYPTION" => TokenCategory::Keyword,
        "END" => TokenCategory::Keyword,
        "END-EXEC" => TokenCategory::Keyword,
        "ENDING" => TokenCategory::Keyword,
        "END_FRAME" => TokenCategory::Keyword,
        "END_PARTITION" => TokenCategory::Keyword,
        "ENGINE" => TokenCategory::Keyword,
        "EQ" => TokenCategory::Keyword,
        "EQUALS" => TokenCategory::Keyword,
        "ERASE" => TokenCategory::Keyword,
        "ERRLVL" => TokenCategory::Keyword,
        "ERROR" => TokenCategory::Keyword,
        "ERRORFILES" => TokenCategory::Keyword,
        "ERRORTABLES" => TokenCategory::Keyword,
        "ESCAPE" => TokenCategory::Keyword,
        "ESCAPED" => TokenCategory::Keyword,
        "ET" => TokenCategory::Keyword,
        "EXCEPT" => TokenCategory::Keyword,
        "EXCEPTION" => TokenCategory::Keyword,
        "EXCLUDE" => TokenCategory::Keyword,
        "EXCLUSIVE" => TokenCategory::Keyword,
        "EXEC" => TokenCategory::Keyword,
        "EXECUTE" => TokenCategory::Keyword,
        "EXIT" => TokenCategory::Keyword,
        "EXPAND" => TokenCategory::Keyword,
        "EXPLAIN" => TokenCategory::Keyword,
        "EXPLICIT" => TokenCategory::Keyword,
        "EXTENSION" => TokenCategory::Keyword,
        "EXTERNAL" => TokenCategory::Keyword,
        "FALLBACK" => TokenCategory::Keyword,
        "FALSE" => TokenCategory::Keyword,
        "FASTEXPORT" => TokenCategory::Keyword,
        "FAST_FORWARD" => TokenCategory::Keyword,
        "FENCED" => TokenCategory::Keyword,
        "FETCH" => TokenCategory::Keyword,
        "FIELD" => TokenCategory::Keyword,
        "FIELDPROC" => TokenCategory::Keyword,
        "FIELDTERMINATOR" => TokenCategory::Keyword,
        "FILE" => TokenCategory::Keyword,
        "FILLFACTOR" => TokenCategory::Keyword,
        "FILTER" => TokenCategory::Keyword,
        "FINAL" => TokenCategory::Keyword,
        "FLOAT4" => TokenCategory::Keyword,
        "FLOAT8" => TokenCategory::Keyword,
        "FOLLOWING" => TokenCategory::Keyword,
        "FOR" => TokenCategory::Keyword,
        "FORCE" => TokenCategory::Keyword,
        "FOREIGN" => TokenCategory::Keyword,
        "FORTRAN" => TokenCategory::Keyword,
        "FOUND" => TokenCategory::Keyword,
        "FRAME_ROW" => TokenCategory::Keyword,
        "FREE" => TokenCategory::Keyword,
        "FREESPACE" => TokenCategory::Keyword,
        "FREEZE" => TokenCategory::Keyword,
        "FROM" => TokenCategory::Keyword,
        "FULL" => TokenCategory::Keyword,
        "FULLTEXT" => TokenCategory::Keyword,
        "GE" => TokenCategory::Keyword,
        "GENERAL" => TokenCategory::Keyword,
        "GENERATED" => TokenCategory::Keyword,
        "GET" => TokenCategory::Keyword,
        "GIVE" => TokenCategory::Keyword,
        "GLOBAL" => TokenCategory::Keyword,
        "GO" => TokenCategory::Keyword,
        "GOTO" => TokenCategory::Keyword,
        "GRANT" => TokenCategory::Keyword,
        "GRAPHIC" => TokenCategory::Keyword,
        "GROUP" => TokenCategory::Keyword,
        "GROUPS" => TokenCategory::Keyword,
        "GT" => TokenCategory::Keyword,
        "HANDLER" => TokenCategory::Keyword,
        "HASH" => TokenCategory::Keyword,
        "HAVING" => TokenCategory::Keyword,
        "HELP" => TokenCategory::Keyword,
        "HIGH_PRIORITY" => TokenCategory::Keyword,
        "HOLD" => TokenCategory::Keyword,
        "HOLDLOCK" => TokenCategory::Keyword,
        "HOST" => TokenCategory::Keyword,
        "HOURS" => TokenCategory::Keyword,
        "HOUR_MICROSECOND" => TokenCategory::Keyword,
        "HOUR_MINUTE" => TokenCategory::Keyword,
        "HOUR_SECOND" => TokenCategory::Keyword,
        "IDENTIFIED" => TokenCategory::Keyword,
        "IDENTITYCOL" => TokenCategory::Keyword,
        "IDENTITY_INSERT" => TokenCategory::Keyword,
        "IGNORE" => TokenCategory::Keyword,
        "ILIKE" => TokenCategory::Keyword,
        "IMMEDIATE" => TokenCategory::Keyword,
        "IMMUTABLE" => TokenCategory::Keyword,
        "IN" => TokenCategory::Keyword,
        "INCLUDE" => TokenCategory::Keyword,
        "INCLUSIVE" => TokenCategory::Keyword,
        "INCONSISTENT" => TokenCategory::Keyword,
        "INCREMENT" => TokenCategory::Keyword,
        "INDICATOR" => TokenCategory::Keyword,
        "INFILE" => TokenCategory::Keyword,
        "INHERIT" => TokenCategory::Keyword,
        "INITIAL" => TokenCategory::Keyword,
        "INITIALIZE" => TokenCategory::Keyword,
        "INITIALLY" => TokenCategory::Keyword,
        "INITIATE" => TokenCategory::Keyword,
        "INNER" => TokenCategory::Keyword,
        "INOUT" => TokenCategory::Keyword,
        "INPUT" => TokenCategory::Keyword,
        "INS" => TokenCategory::Keyword,
        "INSENSITIVE" => TokenCategory::Keyword,
        "INSERT" => TokenCategory::Keyword,
        "INSERTED" => TokenCategory::Keyword,
        "INSTEAD" => TokenCategory::Keyword,
        "INT1" => TokenCategory::Keyword,
        "INT2" => TokenCategory::Keyword,
        "INT3" => TokenCategory::Keyword,
        "INT4" => TokenCategory::Keyword,
        "INT8" => TokenCategory::Keyword,
        "INTEGERDATE" => TokenCategory::Keyword,
        "INTERSECT" => TokenCategory::Keyword,
        "INTERSECTION" => TokenCategory::Keyword,
        "INTERVAL" => TokenCategory::Keyword,
        "INTO" => TokenCategory::Keyword,
        "INVISIBLE" => TokenCategory::Keyword,
        "INVOKER" => TokenCategory::Keyword,
        "IO_AFTER_GTIDS" => TokenCategory::Keyword,
        "IO_BEFORE_GTIDS" => TokenCategory::Keyword,
        "IS" => TokenCategory::Keyword,
        "ISOBID" => TokenCategory::Keyword,
        "ISOLATION" => TokenCategory::Keyword,
        "ITERATE" => TokenCategory::Keyword,
        "JAR" => TokenCategory::Keyword,
        "JOIN" => TokenCategory::Keyword,
        "JOURNAL" => TokenCategory::Keyword,
        "JSON" => TokenCategory::Keyword,
        "JSON_TABLE_PRIMITIVE" => TokenCategory::Keyword,
        "KEEP" => TokenCategory::Keyword,
        "KEYS" => TokenCategory::Keyword,
        "KILL" => TokenCategory::Keyword,
        "LABEL" => TokenCategory::Keyword,
        "LANGUAGE" => TokenCategory::Keyword,
        "LARGE" => TokenCategory::Keyword,
        "LATERAL" => TokenCategory::Keyword,
        "LC_CTYPE" => TokenCategory::Keyword,
        "LE" => TokenCategory::Keyword,
        "LEADING" => TokenCategory::Keyword,
        "LEAVE" => TokenCategory::Keyword,
        "LESS" => TokenCategory::Keyword,
        "LEVEL" => TokenCategory::Keyword,
        "LIKE" => TokenCategory::Keyword,
        "LIKE_REGEX" => TokenCategory::Keyword,
        "LIMIT" => TokenCategory::Keyword,
        "LINEAR" => TokenCategory::Keyword,
        "LINENO" => TokenCategory::Keyword,
        "LINES" => TokenCategory::Keyword,
        "LIST" => TokenCategory::Keyword,
        "LOAD" => TokenCategory::Keyword,
        "LOADING" => TokenCategory::Keyword,
        "LOCAL" => TokenCategory::Keyword,
        "LOCALE" => TokenCategory::Keyword,
        "LOCATOR" => TokenCategory::Keyword,
        "LOCATORS" => TokenCategory::Keyword,
        "LOCK" => TokenCategory::Keyword,
        "LOCKED" => TokenCategory::Keyword,
        "LOCKING" => TokenCategory::Keyword,
        "LOCKMAX" => TokenCategory::Keyword,
        "LOCKSIZE" => TokenCategory::Keyword,
        "LOGGING" => TokenCategory::Keyword,
        "LOGIN" => TokenCategory::Keyword,
        "LOGON" => TokenCategory::Keyword,
        "LONG" => TokenCategory::Keyword,
        "LOOP" => TokenCategory::Keyword,
        "LOW_PRIORITY" => TokenCategory::Keyword,
        "LT" => TokenCategory::Keyword,
        "MACRO" => TokenCategory::Keyword,
        "MAINTAINED" => TokenCategory::Keyword,
        "MAP" => TokenCategory::Keyword,
        "MASKED" => TokenCategory::Keyword,
        "MASTER_BIND" => TokenCategory::Keyword,
        "MASTER_SSL_VERIFY_SERVER_CERT" => TokenCategory::Keyword,
        "MATCHED" => TokenCategory::Keyword,
        "MATCHES" => TokenCategory::Keyword,
        "MATCH_RECOGNIZE" => TokenCategory::Keyword,
        "MATERIALIZED" => TokenCategory::Keyword,
        "MAXDOP" => TokenCategory::Keyword,
        "MAXEXTENTS" => TokenCategory::Keyword,
        "MAXIMUM" => TokenCategory::Keyword,
        "MAXVALUE" => TokenCategory::Keyword,
        "MCHARACTERS" => TokenCategory::Keyword,
        "MEMBER" => TokenCategory::Keyword,
        "MERGE" => TokenCategory::Keyword,
        "METHOD" => TokenCategory::Keyword,
        "MICROSECONDS" => TokenCategory::Keyword,
        "MIDDLEINT" => TokenCategory::Keyword,
        "MINDEX" => TokenCategory::Keyword,
        "MINIMUM" => TokenCategory::Keyword,
        "MINUS" => TokenCategory::Keyword,
        "MINUTES" => TokenCategory::Keyword,
        "MINUTE_MICROSECOND" => TokenCategory::Keyword,
        "MINUTE_SECOND" => TokenCategory::Keyword,
        "MINVALUE" => TokenCategory::Keyword,
        "MLOAD" => TokenCategory::Keyword,
        "MLSLABEL" => TokenCategory::Keyword,
        "MODE" => TokenCategory::Keyword,
        "MODIFIES" => TokenCategory::Keyword,
        "MODULE" => TokenCategory::Keyword,
        "MONRESOURCE" => TokenCategory::Keyword,
        "MONSESSION" => TokenCategory::Keyword,
        "MONTHS" => TokenCategory::Keyword,
        "MSUBSTR" => TokenCategory::Keyword,
        "MULTISET" => TokenCategory::Keyword,
        "NAMED" => TokenCategory::Keyword,
        "NAMES" => TokenCategory::Keyword,
        "NATIONAL" => TokenCategory::Keyword,
        "NATURAL" => TokenCategory::Keyword,
        "NCLOB" => TokenCategory::Keyword,
        "NE" => TokenCategory::Keyword,
        "NESTED_TABLE_ID" => TokenCategory::Keyword,
        "NEW" => TokenCategory::Keyword,
        "NEW_TABLE" => TokenCategory::Keyword,
        "NEXT" => TokenCategory::Keyword,
        "NO" => TokenCategory::Keyword,
        "NOAUDIT" => TokenCategory::Keyword,
        "NOCHECK" => TokenCategory::Keyword,
        "NOCOMPRESS" => TokenCategory::Keyword,
        "NOCOUNT" => TokenCategory::Keyword,
        "NOEXEC" => TokenCategory::Keyword,
        "NOEXPAND" => TokenCategory::Keyword,
        "NOLOCK" => TokenCategory::Keyword,
        "NONCLUSTERED" => TokenCategory::Keyword,
        "NONE" => TokenCategory::Keyword,
        "NOT" => TokenCategory::Keyword,
        "NOTHING" => TokenCategory::Keyword,
        "NOWAIT" => TokenCategory::Keyword,
        "NO_WRITE_TO_BINLOG" => TokenCategory::Keyword,
        "NULL" => TokenCategory::Keyword,
        "NULLS" => TokenCategory::Keyword,
        "NUMBER" => TokenCategory::Keyword,
        "NUMPARTS" => TokenCategory::Keyword,
        "OBID" => TokenCategory::Keyword,
        "OBJECT" => TokenCategory::Keyword,
        "OBJECTS" => TokenCategory::Keyword,
        "OF" => TokenCategory::Keyword,
        "OFF" => TokenCategory::Keyword,
        "OFFLINE" => TokenCategory::Keyword,
        "OFFSET" => TokenCategory::Keyword,
        "OFFSETS" => TokenCategory::Keyword,
        "OLD" => TokenCategory::Keyword,
        "OLD_TABLE" => TokenCategory::Keyword,
        "OMIT" => TokenCategory::Keyword,
        "ONE" => TokenCategory::Keyword,
        "ONLINE" => TokenCategory::Keyword,
        "ONLY" => TokenCategory::Keyword,
        "OPEN" => TokenCategory::Keyword,
        "OPERATION" => TokenCategory::Keyword,
        "OPTIMIZATION" => TokenCategory::Keyword,
        "OPTIMIZE" => TokenCategory::Keyword,
        "OPTIMIZER_COSTS" => TokenCategory::Keyword,
        "OPTION" => TokenCategory::Keyword,
        "OPTIONALLY" => TokenCategory::Keyword,
        "OR" => TokenCategory::Keyword,
        "ORDER" => TokenCategory::Keyword,
        "ORDINALITY" => TokenCategory::Keyword,
        "ORGANIZATION" => TokenCategory::Keyword,
        "OUT" => TokenCategory::Keyword,
        "OUTER" => TokenCategory::Keyword,
        "OUTFILE" => TokenCategory::Keyword,
        "OUTPUT" => TokenCategory::Keyword,
        "OVER" => TokenCategory::Keyword,
        "OVERLAPS" => TokenCategory::Keyword,
        "OVERRIDE" => TokenCategory::Keyword,
        "OWNED" => TokenCategory::Keyword,
        "OWNER" => TokenCategory::Keyword,
        "PACKAGE" => TokenCategory::Keyword,
        "PAD" => TokenCategory::Keyword,
        "PADDED" => TokenCategory::Keyword,
        "PARAMETER" => TokenCategory::Keyword,
        "PARAMETERS" => TokenCategory::Keyword,
        "PART" => TokenCategory::Keyword,
        "PARTIAL" => TokenCategory::Keyword,
        "PARTITION" => TokenCategory::Keyword,
        "PARTITIONED" => TokenCategory::Keyword,
        "PARTITIONING" => TokenCategory::Keyword,
        "PASCAL" => TokenCategory::Keyword,
        "PASSWORD" => TokenCategory::Keyword,
        "PATTERN" => TokenCategory::Keyword,
        "PCTFREE" => TokenCategory::Keyword,
        "PER" => TokenCategory::Keyword,
        "PERCENT" => TokenCategory::Keyword,
        "PERIOD" => TokenCategory::Keyword,
        "PERM" => TokenCategory::Keyword,
        "PERMANENT" => TokenCategory::Keyword,
        "PERSISTED" => TokenCategory::Keyword,
        "PIECESIZE" => TokenCategory::Keyword,
        "PIVOT" => TokenCategory::Keyword,
        "PLACING" => TokenCategory::Keyword,
        "PLAN" => TokenCategory::Keyword,
        "PLPGSQL" => TokenCategory::Keyword,
        "PORTION" => TokenCategory::Keyword,
        "POSTFIX" => TokenCategory::Keyword,
        "PRECEDES" => TokenCategory::Keyword,
        "PRECEDING" => TokenCategory::Keyword,
        "PRECISION" => TokenCategory::Keyword,
        "PREFIX" => TokenCategory::Keyword,
        "PREORDER" => TokenCategory::Keyword,
        "PREPARE" => TokenCategory::Keyword,
        "PRESERVE" => TokenCategory::Keyword,
        "PREVVAL" => TokenCategory::Keyword,
        "PRIMARY" => TokenCategory::Keyword,
        "PRINT" => TokenCategory::Keyword,
        "PRIOR" => TokenCategory::Keyword,
        "PRIQTY" => TokenCategory::Keyword,
        "PRIVATE" => TokenCategory::Keyword,
        "PRIVILEGES" => TokenCategory::Keyword,
        "PROC" => TokenCategory::Keyword,
        "PROFILE" => TokenCategory::Keyword,
        "PROGRAM" => TokenCategory::Keyword,
        "PROPORTIONAL" => TokenCategory::Keyword,
        "PROTECTION" => TokenCategory::Keyword,
        "PSID" => TokenCategory::Keyword,
        "PTF" => TokenCategory::Keyword,
        "PUBLIC" => TokenCategory::Keyword,
        "PURGE" => TokenCategory::Keyword,
        "QUALIFIED" => TokenCategory::Keyword,
        "QUALIFY" => TokenCategory::Keyword,
        "QUERYNO" => TokenCategory::Keyword,
        "QUOTED_IDENTIFIER" => TokenCategory::Keyword,
        "RANGE" => TokenCategory::Keyword,
        "READ" => TokenCategory::Keyword,
        "READPAST" => TokenCategory::Keyword,
        "READS" => TokenCategory::Keyword,
        "READTEXT" => TokenCategory::Keyword,
        "READ_WRITE" => TokenCategory::Keyword,
        "RECOMPILE" => TokenCategory::Keyword,
        "RECONFIGURE" => TokenCategory::Keyword,
        "RECOVERY" => TokenCategory::Keyword,
        "RECURSIVE" => TokenCategory::Keyword,
        "REF" => TokenCategory::Keyword,
        "REFERENCING" => TokenCategory::Keyword,
        "REFRESH" => TokenCategory::Keyword,
        "REGEXP" => TokenCategory::Keyword,
        "RELATIVE" => TokenCategory::Keyword,
        "RELEASE" => TokenCategory::Keyword,
        "RENAME" => TokenCategory::Keyword,
        "REPEATABLE" => TokenCategory::Keyword,
        "REPLICATION" => TokenCategory::Keyword,
        "REPOVERRIDE" => TokenCategory::Keyword,
        "REQUEST" => TokenCategory::Keyword,
        "REQUIRE" => TokenCategory::Keyword,
        "RESET" => TokenCategory::Keyword,
        "RESIGNAL" => TokenCategory::Keyword,
        "RESOURCE" => TokenCategory::Keyword,
        "RESTART" => TokenCategory::Keyword,
        "RESTORE" => TokenCategory::Keyword,
        "RESTRICT" => TokenCategory::Keyword,
        "RESULT" => TokenCategory::Keyword,
        "RESULT_SET_LOCATOR" => TokenCategory::Keyword,
        "RESUMABLE" => TokenCategory::Keyword,
        "RESUME" => TokenCategory::Keyword,
        "RET" => TokenCategory::Keyword,
        "RETRIEVE" => TokenCategory::Keyword,
        "RETURN" => TokenCategory::Keyword,
        "RETURNING" => TokenCategory::Keyword,
        "RETURNS" => TokenCategory::Keyword,
        "REVALIDATE" => TokenCategory::Keyword,
        "REVERT" => TokenCategory::Keyword,
        "REVOKE" => TokenCategory::Keyword,
        "RIGHTS" => TokenCategory::Keyword,
        "RLIKE" => TokenCategory::Keyword,
        "ROLE" => TokenCategory::Keyword,
        "ROLLBACK" => TokenCategory::Keyword,
        "ROLLFORWARD" => TokenCategory::Keyword,
        "ROUTINE" => TokenCategory::Keyword,
        "ROWCOUNT" => TokenCategory::Keyword,
        "ROWGUIDCOL" => TokenCategory::Keyword,
        "ROWID" => TokenCategory::Keyword,
        "ROWLOCK" => TokenCategory::Keyword,
        "ROWNUM" => TokenCategory::Keyword,
        "ROWS" => TokenCategory::Keyword,
        "ROWSET" => TokenCategory::Keyword,
        "ROWTERMINATOR" => TokenCategory::Keyword,
        "RULE" => TokenCategory::Keyword,
        "RUN" => TokenCategory::Keyword,
        "RUNNING" => TokenCategory::Keyword,
        "SAMPLE" => TokenCategory::Keyword,
        "SAMPLEID" => TokenCategory::Keyword,
        "SAVE" => TokenCategory::Keyword,
        "SAVEPOINT" => TokenCategory::Keyword,
        "SCHEMA" => TokenCategory::Keyword,
        "SCHEMAS" => TokenCategory::Keyword,
        "SCOPE" => TokenCategory::Keyword,
        "SCRATCHPAD" => TokenCategory::Keyword,
        "SCROLL" => TokenCategory::Keyword,
        "SEARCH" => TokenCategory::Keyword,
        "SECONDS" => TokenCategory::Keyword,
        "SECOND_MICROSECOND" => TokenCategory::Keyword,
        "SECQTY" => TokenCategory::Keyword,
        "SECTION" => TokenCategory::Keyword,
        "SECURITY" => TokenCategory::Keyword,
        "SECURITYAUDIT" => TokenCategory::Keyword,
        "SEEK" => TokenCategory::Keyword,
        "SEL" => TokenCategory::Keyword,
        "SELECT" => TokenCategory::Keyword,
        "SELF" => TokenCategory::Keyword,
        "SEMI" => TokenCategory::Keyword,
        "SENSITIVE" => TokenCategory::Keyword,
        "SEPARATOR" => TokenCategory::Keyword,
        "SEQUENCE" => TokenCategory::Keyword,
        "SERIALIZABLE" => TokenCategory::Keyword,
        "SESSION" => TokenCategory::Keyword,
        "SET" => TokenCategory::Keyword,
        "SETRESRATE" => TokenCategory::Keyword,
        "SETS" => TokenCategory::Keyword,
        "SETSESSRATE" => TokenCategory::Keyword,
        "SETUSER" => TokenCategory::Keyword,
        "SHARE" => TokenCategory::Keyword,
        "SHOW" => TokenCategory::Keyword,
        "SHUTDOWN" => TokenCategory::Keyword,
        "SIGNAL" => TokenCategory::Keyword,
        "SIMILAR" => TokenCategory::Keyword,
        "SIMPLE" => TokenCategory::Keyword,
        "SIZE" => TokenCategory::Keyword,
        "SKIP" => TokenCategory::Keyword,
        "SNAPSHOT" => TokenCategory::Keyword,
        "SOME" => TokenCategory::Keyword,
        "SOURCE" => TokenCategory::Keyword,
        "SPARSE" => TokenCategory::Keyword,
        "SPATIAL" => TokenCategory::Keyword,
        "SPECIFIC" => TokenCategory::Keyword,
        "SPECIFICTYPE" => TokenCategory::Keyword,
        "SPOOL" => TokenCategory::Keyword,
        "SQL" => TokenCategory::Keyword,
        "SQLCA" => TokenCategory::Keyword,
        "SQLCODE" => TokenCategory::Keyword,
        "SQLERROR" => TokenCategory::Keyword,
        "SQLEXCEPTION" => TokenCategory::Keyword,
        "SQLSTATE" => TokenCategory::Keyword,
        "SQLTEXT" => TokenCategory::Keyword,
        "SQLWARNING" => TokenCategory::Keyword,
        "SQL_BIG_RESULT" => TokenCategory::Keyword,
        "SQL_CALC_FOUND_ROWS" => TokenCategory::Keyword,
        "SQL_SMALL_RESULT" => TokenCategory::Keyword,
        "SS" => TokenCategory::Keyword,
        "SSL" => TokenCategory::Keyword,
        "STABLE" => TokenCategory::Keyword,
        "STANDARD" => TokenCategory::Keyword,
        "START" => TokenCategory::Keyword,
        "STARTING" => TokenCategory::Keyword,
        "STARTUP" => TokenCategory::Keyword,
        "STATE" => TokenCategory::Keyword,
        "STATEMENT" => TokenCategory::Keyword,
        "STATIC" => TokenCategory::Keyword,
        "STATISTICS" => TokenCategory::Keyword,
        "STATS" => TokenCategory::Keyword,
        "STAY" => TokenCategory::Keyword,
        "STEPINFO" => TokenCategory::Keyword,
        "STOGROUP" => TokenCategory::Keyword,
        "STORED" => TokenCategory::Keyword,
        "STORES" => TokenCategory::Keyword,
        "STRAIGHT_JOIN" => TokenCategory::Keyword,
        "STRICT" => TokenCategory::Keyword,
        "STRING" => TokenCategory::Keyword,
        "STRING_CS" => TokenCategory::Keyword,
        "STRUCTURE" => TokenCategory::Keyword,
        "STYLE" => TokenCategory::Keyword,
        "SUBMULTISET" => TokenCategory::Keyword,
        "SUBPARTITION" => TokenCategory::Keyword,
        "SUBSCRIBER" => TokenCategory::Keyword,
        "SUBSET" => TokenCategory::Keyword,
        "SUCCEEDS" => TokenCategory::Keyword,
        "SUCCESSFUL" => TokenCategory::Keyword,
        "SUMMARY" => TokenCategory::Keyword,
        "SUSPEND" => TokenCategory::Keyword,
        "SYMMETRIC" => TokenCategory::Keyword,
        "SYNONYM" => TokenCategory::Keyword,
        "SYSTEM" => TokenCategory::Keyword,
        "SYSTEM_TIME" => TokenCategory::Keyword,
        "SYSTIMESTAMP" => TokenCategory::Keyword,
        "TABLESAMPLE" => TokenCategory::Keyword,
        "TABLESPACE" => TokenCategory::Keyword,
        "TABLOCK" => TokenCategory::Keyword,
        "TABLOCKX" => TokenCategory::Keyword,
        "TARGET" => TokenCategory::Keyword,
        "TBL_CS" => TokenCategory::Keyword,
        "TEMP" => TokenCategory::Keyword,
        "TEMPORARY" => TokenCategory::Keyword,
        "TERMINATE" => TokenCategory::Keyword,
        "TERMINATED" => TokenCategory::Keyword,
        "TEXTSIZE" => TokenCategory::Keyword,
        "THAN" => TokenCategory::Keyword,
        "THEN" => TokenCategory::Keyword,
        "THRESHOLD" => TokenCategory::Keyword,
        "THROW" => TokenCategory::Keyword,
        "TIES" => TokenCategory::Keyword,
        "TIMEZONE_HOUR" => TokenCategory::Keyword,
        "TIMEZONE_MINUTE" => TokenCategory::Keyword,
        "TITLE" => TokenCategory::Keyword,
        "TO" => TokenCategory::Keyword,
        "TOP" => TokenCategory::Keyword,
        "TRACE" => TokenCategory::Keyword,
        "TRAILING" => TokenCategory::Keyword,
        "TRAN" => TokenCategory::Keyword,
        "TRANSACTION" => TokenCategory::Keyword,
        "TRANSLATION" => TokenCategory::Keyword,
        "TRUE" => TokenCategory::Keyword,
        "TRY" => TokenCategory::Keyword,
        "TYPE" => TokenCategory::Keyword,
        "UC" => TokenCategory::Keyword,
        "UESCAPE" => TokenCategory::Keyword,
        "UID" => TokenCategory::Keyword,
        "UNBOUNDED" => TokenCategory::Keyword,
        "UNCOMMITTED" => TokenCategory::Keyword,
        "UNDEFINED" => TokenCategory::Keyword,
        "UNDER" => TokenCategory::Keyword,
        "UNDO" => TokenCategory::Keyword,
        "UNION" => TokenCategory::Keyword,
        "UNKNOWN" => TokenCategory::Keyword,
        "UNLOCK" => TokenCategory::Keyword,
        "UNLOGGED" => TokenCategory::Keyword,
        "UNPIVOT" => TokenCategory::Keyword,
        "UNSIGNED" => TokenCategory::Keyword,
        "UNTIL" => TokenCategory::Keyword,
        "UPD" => TokenCategory::Keyword,
        "UPDATE" => TokenCategory::Keyword,
        "UPDATETEXT" => TokenCategory::Keyword,
        "UPDLOCK" => TokenCategory::Keyword,
        "UPPERCASE" => TokenCategory::Keyword,
        "USAGE" => TokenCategory::Keyword,
        "USE" => TokenCategory::Keyword,
        "USER" => TokenCategory::Keyword,
        "USING" => TokenCategory::Keyword,
        "UTC_DATE" => TokenCategory::Keyword,
        "UTC_TIME" => TokenCategory::Keyword,
        "UTC_TIMESTAMP" => TokenCategory::Keyword,
        "VACUUM" => TokenCategory::Keyword,
        "VALIDATE" => TokenCategory::Keyword,
        "VALIDPROC" => TokenCategory::Keyword,
        "VALUES" => TokenCategory::Keyword,
        "VALUE_OF" => TokenCategory::Keyword,
        "VARGRAPHIC" => TokenCategory::Keyword,
        "VARIABLE" => TokenCategory::Keyword,
        "VARIADIC" => TokenCategory::Keyword,
        "VARIANT" => TokenCategory::Keyword,
        "VARYING" => TokenCategory::Keyword,
        "VCAT" => TokenCategory::Keyword,
        "VERBOSE" => TokenCategory::Keyword,
        "VERSIONING" => TokenCategory::Keyword,
        "VIRTUAL" => TokenCategory::Keyword,
        "VISIBLE" => TokenCategory::Keyword,
        "VOLATILE" => TokenCategory::Keyword,
        "VOLUMES" => TokenCategory::Keyword,
        "WAIT" => TokenCategory::Keyword,
        "WAITFOR" => TokenCategory::Keyword,
        "WHEN" => TokenCategory::Keyword,
        "WHENEVER" => TokenCategory::Keyword,
        "WHERE" => TokenCategory::Keyword,
        "WHILE" => TokenCategory::Keyword,
        "WINDOW" => TokenCategory::Keyword,
        "WITH" => TokenCategory::Keyword,
        "WITHIN" => TokenCategory::Keyword,
        "WITHIN_GROUP" => TokenCategory::Keyword,
        "WITHOUT" => TokenCategory::Keyword,
        "WLM" => TokenCategory::Keyword,
        "WORK" => TokenCategory::Keyword,
        "WRITE" => TokenCategory::Keyword,
        "WRITETEXT" => TokenCategory::Keyword,
        "XACT_ABORT" => TokenCategory::Keyword,
        "XOR" => TokenCategory::Keyword,
        "YEARS" => TokenCategory::Keyword,
        "YEAR_MONTH" => TokenCategory::Keyword,
        "ZEROFILL" => TokenCategory::Keyword,
        "ZONE" => TokenCategory::Keyword,

        // DataTypes
        "BIGINT" => TokenCategory::DataType,
        "BIGSERIAL" => TokenCategory::DataType,
        "BINARY" => TokenCategory::DataType,
        "BINARY_DOUBLE" => TokenCategory::DataType,
        "BINARY_FLOAT" => TokenCategory::DataType,
        "BIT" => TokenCategory::DataType,
        "BLOB" => TokenCategory::DataType,
        "BOOL" => TokenCategory::DataType,
        "BOOLEAN" => TokenCategory::DataType,
        "BYTEA" => TokenCategory::DataType,
        "CHAR" => TokenCategory::DataType,
        "CHARACTER" => TokenCategory::DataType,
        "CIDR" => TokenCategory::DataType,
        "CITEXT" => TokenCategory::DataType,
        "COLUMN" => TokenCategory::DataType,
        "DATABASE" => TokenCategory::DataType,
        "DATE" => TokenCategory::DataType,
        "DATETIME" => TokenCategory::DataType,
        "DATETIME2" => TokenCategory::DataType,
        "DATETIMEOFFSET" => TokenCategory::DataType,
        "DEC" => TokenCategory::DataType,
        "DECIMAL" => TokenCategory::DataType,
        "DOUBLE" => TokenCategory::DataType,
        "ENUM" => TokenCategory::DataType,
        "EVENT" => TokenCategory::DataType,
        "FLOAT" => TokenCategory::DataType,
        "FUNCTION" => TokenCategory::DataType,
        "GEOGRAPHY" => TokenCategory::DataType,
        "GEOMETRY" => TokenCategory::DataType,
        "HIERARCHYID" => TokenCategory::DataType,
        "IMAGE" => TokenCategory::DataType,
        "INDEX" => TokenCategory::DataType,
        "INET" => TokenCategory::DataType,
        "INT" => TokenCategory::DataType,
        "INTEGER" => TokenCategory::DataType,
        "JSONB" => TokenCategory::DataType,
        "KEY" => TokenCategory::DataType,
        "LONGBLOB" => TokenCategory::DataType,
        "LONGTEXT" => TokenCategory::DataType,
        "MACADDR" => TokenCategory::DataType,
        "MEDIUMBLOB" => TokenCategory::DataType,
        "MEDIUMINT" => TokenCategory::DataType,
        "MEDIUMTEXT" => TokenCategory::DataType,
        "MONEY" => TokenCategory::DataType,
        "NCHAR" => TokenCategory::DataType,
        "NTEXT" => TokenCategory::DataType,
        "NUMERIC" => TokenCategory::DataType,
        "NVARCHAR" => TokenCategory::DataType,
        "NVARCHAR2" => TokenCategory::DataType,
        "PROCEDURE" => TokenCategory::DataType,
        "REAL" => TokenCategory::DataType,
        "ROWVERSION" => TokenCategory::DataType,
        "SERIAL" => TokenCategory::DataType,
        "SMALLDATETIME" => TokenCategory::DataType,
        "SMALLINT" => TokenCategory::DataType,
        "SMALLMONEY" => TokenCategory::DataType,
        "SMALLSERIAL" => TokenCategory::DataType,
        "SQL_VARIANT" => TokenCategory::DataType,
        "SYSNAME" => TokenCategory::DataType,
        "TABLE" => TokenCategory::DataType,
        "TEXT" => TokenCategory::DataType,
        "TIME" => TokenCategory::DataType,
        "TIMESTAMP" => TokenCategory::DataType,
        "TIMESTAMPTZ" => TokenCategory::DataType,
        "TIMETZ" => TokenCategory::DataType,
        "TINYBLOB" => TokenCategory::DataType,
        "TINYINT" => TokenCategory::DataType,
        "TINYTEXT" => TokenCategory::DataType,
        "TRIGGER" => TokenCategory::DataType,
        "TSVECTOR" => TokenCategory::DataType,
        "UNIQUEIDENTIFIER" => TokenCategory::DataType,
        "UUID" => TokenCategory::DataType,
        "VARBINARY" => TokenCategory::DataType,
        "VARBIT" => TokenCategory::DataType,
        "VARBYTE" => TokenCategory::DataType,
        "VARCHAR" => TokenCategory::DataType,
        "VARCHAR2" => TokenCategory::DataType,
        "VARCHARACTER" => TokenCategory::DataType,
        "VECTOR" => TokenCategory::DataType,
        "VIEW" => TokenCategory::DataType,
        "XML" => TokenCategory::DataType,
        "YEAR" => TokenCategory::DataType,

        // Events
        "AFTER" => TokenCategory::Event,
        "BEFORE" => TokenCategory::Event,
        "ON" => TokenCategory::Event,

        // Methods
        "ABS" => TokenCategory::Method,
        "ACOS" => TokenCategory::Method,
        "ACOSH" => TokenCategory::Method,
        "ADDDATE" => TokenCategory::Method,
        "ADDTIME" => TokenCategory::Method,
        "ADD_MONTHS" => TokenCategory::Method,
        "AGE" => TokenCategory::Method,
        "ANY_VALUE" => TokenCategory::Method,
        "APPROX_COUNT_DISTINCT" => TokenCategory::Method,
        "APPROX_PERCENTILE_CONT" => TokenCategory::Method,
        "APP_NAME" => TokenCategory::Method,
        "ARRAY_AGG" => TokenCategory::Method,
        "ARRAY_APPEND" => TokenCategory::Method,
        "ARRAY_LENGTH" => TokenCategory::Method,
        "ARRAY_TO_STRING" => TokenCategory::Method,
        "ASCII" => TokenCategory::Method,
        "ASIN" => TokenCategory::Method,
        "ASINH" => TokenCategory::Method,
        "ATAN" => TokenCategory::Method,
        "ATAN2" => TokenCategory::Method,
        "ATANH" => TokenCategory::Method,
        "ATN2" => TokenCategory::Method,
        "AVE" => TokenCategory::Method,
        "AVERAGE" => TokenCategory::Method,
        "AVG" => TokenCategory::Method,
        "BENCHMARK" => TokenCategory::Method,
        "BIN" => TokenCategory::Method,
        "BINARY_CHECKSUM" => TokenCategory::Method,
        "BIT_AND" => TokenCategory::Method,
        "BIT_LENGTH" => TokenCategory::Method,
        "BIT_OR" => TokenCategory::Method,
        "BIT_XOR" => TokenCategory::Method,
        "BOOL_AND" => TokenCategory::Method,
        "BOOL_OR" => TokenCategory::Method,
        "BTRIM" => TokenCategory::Method,
        "CARDINALITY" => TokenCategory::Method,
        "CASE_N" => TokenCategory::Method,
        "CAST" => TokenCategory::Method,
        "CBRT" => TokenCategory::Method,
        "CEIL" => TokenCategory::Method,
        "CEILING" => TokenCategory::Method,
        "CHAR2HEXINT" => TokenCategory::Method,
        "CHARACTERS" => TokenCategory::Method,
        "CHARACTER_LENGTH" => TokenCategory::Method,
        "CHARINDEX" => TokenCategory::Method,
        "CHARS" => TokenCategory::Method,
        "CHAR_LENGTH" => TokenCategory::Method,
        "CHECKSUM" => TokenCategory::Method,
        "CHECKSUM_AGG" => TokenCategory::Method,
        "CHOOSE" => TokenCategory::Method,
        "CHR" => TokenCategory::Method,
        "CLASSIFIER" => TokenCategory::Method,
        "CLOCK_TIMESTAMP" => TokenCategory::Method,
        "COALESCE" => TokenCategory::Method,
        "COL_LENGTH" => TokenCategory::Method,
        "COL_NAME" => TokenCategory::Method,
        "CONCAT" => TokenCategory::Method,
        "CONCAT_WS" => TokenCategory::Method,
        "CONNECTION_ID" => TokenCategory::Method,
        "CONNECT_BY_ROOT" => TokenCategory::Method,
        "CONTAINS" => TokenCategory::Method,
        "CONTAINSTABLE" => TokenCategory::Method,
        "CONV" => TokenCategory::Method,
        "CONVERT" => TokenCategory::Method,
        "CONVERT_TZ" => TokenCategory::Method,
        "CORR" => TokenCategory::Method,
        "COS" => TokenCategory::Method,
        "COSH" => TokenCategory::Method,
        "COT" => TokenCategory::Method,
        "COUNT" => TokenCategory::Method,
        "COUNTIF" => TokenCategory::Method,
        "COUNT_BIG" => TokenCategory::Method,
        "COVAR_POP" => TokenCategory::Method,
        "COVAR_SAMP" => TokenCategory::Method,
        "CRC32" => TokenCategory::Method,
        "CSUM" => TokenCategory::Method,
        "CUBE" => TokenCategory::Method,
        "CUME_DIST" => TokenCategory::Method,
        "CURDATE" => TokenCategory::Method,
        "CURRENT_DATE" => TokenCategory::Method,
        "CURRENT_TIME" => TokenCategory::Method,
        "CURRENT_TIMESTAMP" => TokenCategory::Method,
        "CURRENT_USER" => TokenCategory::Method,
        "CURRVAL" => TokenCategory::Method,
        "CURTIME" => TokenCategory::Method,
        "DATABASEPROPERTYEX" => TokenCategory::Method,
        "DATALENGTH" => TokenCategory::Method,
        "DATEADD" => TokenCategory::Method,
        "DATEDIFF" => TokenCategory::Method,
        "DATEDIFF_BIG" => TokenCategory::Method,
        "DATEFROMPARTS" => TokenCategory::Method,
        "DATENAME" => TokenCategory::Method,
        "DATEPART" => TokenCategory::Method,
        "DATETIME2FROMPARTS" => TokenCategory::Method,
        "DATETIMEFROMPARTS" => TokenCategory::Method,
        "DATETIMEOFFSETFROMPARTS" => TokenCategory::Method,
        "DATETRUNC" => TokenCategory::Method,
        "DATE_ADD" => TokenCategory::Method,
        "DATE_BUCKET" => TokenCategory::Method,
        "DATE_FORMAT" => TokenCategory::Method,
        "DATE_PART" => TokenCategory::Method,
        "DATE_SUB" => TokenCategory::Method,
        "DATE_TRUNC" => TokenCategory::Method,
        "DAY" => TokenCategory::Method,
        "DAYNAME" => TokenCategory::Method,
        "DAYOFMONTH" => TokenCategory::Method,
        "DAYOFWEEK" => TokenCategory::Method,
        "DAYOFYEAR" => TokenCategory::Method,
        "DB_ID" => TokenCategory::Method,
        "DB_NAME" => TokenCategory::Method,
        "DECODE" => TokenCategory::Method,
        "DECOMPRESS" => TokenCategory::Method,
        "DEGREES" => TokenCategory::Method,
        "DENSE_RANK" => TokenCategory::Method,
        "DEREF" => TokenCategory::Method,
        "DIFFERENCE" => TokenCategory::Method,
        "DIV" => TokenCategory::Method,
        "ELEMENT" => TokenCategory::Method,
        "ELT" => TokenCategory::Method,
        "EOMONTH" => TokenCategory::Method,
        "ERROR_LINE" => TokenCategory::Method,
        "ERROR_MESSAGE" => TokenCategory::Method,
        "ERROR_NUMBER" => TokenCategory::Method,
        "ERROR_PROCEDURE" => TokenCategory::Method,
        "ERROR_SEVERITY" => TokenCategory::Method,
        "ERROR_STATE" => TokenCategory::Method,
        "EVERY" => TokenCategory::Method,
        "EXISTS" => TokenCategory::Method,
        "EXP" => TokenCategory::Method,
        "EXPORT_SET" => TokenCategory::Method,
        "EXTRACT" => TokenCategory::Method,
        "FIND_IN_SET" => TokenCategory::Method,
        "FIRST" => TokenCategory::Method,
        "FIRST_VALUE" => TokenCategory::Method,
        "FLOOR" => TokenCategory::Method,
        "FORMAT" => TokenCategory::Method,
        "FORMATMESSAGE" => TokenCategory::Method,
        "FOUND_ROWS" => TokenCategory::Method,
        "FREETEXT" => TokenCategory::Method,
        "FREETEXTTABLE" => TokenCategory::Method,
        "FROM_BASE64" => TokenCategory::Method,
        "FROM_BYTES" => TokenCategory::Method,
        "FROM_DAYS" => TokenCategory::Method,
        "FROM_UNIXTIME" => TokenCategory::Method,
        "FUSION" => TokenCategory::Method,
        "GENERATE_SERIES" => TokenCategory::Method,
        "GEN_RANDOM_UUID" => TokenCategory::Method,
        "GETDATE" => TokenCategory::Method,
        "GETUTCDATE" => TokenCategory::Method,
        "GREATEST" => TokenCategory::Method,
        "GROUPING" => TokenCategory::Method,
        "GROUPING_ID" => TokenCategory::Method,
        "GROUP_CONCAT" => TokenCategory::Method,
        "HASHAMP" => TokenCategory::Method,
        "HASHBAKAMP" => TokenCategory::Method,
        "HASHBUCKET" => TokenCategory::Method,
        "HASHBYTES" => TokenCategory::Method,
        "HASHROW" => TokenCategory::Method,
        "HEX" => TokenCategory::Method,
        "HOST_NAME" => TokenCategory::Method,
        "HOUR" => TokenCategory::Method,
        "IDENTITY" => TokenCategory::Method,
        "IDENT_CURRENT" => TokenCategory::Method,
        "IF" => TokenCategory::Method,
        "IFNULL" => TokenCategory::Method,
        "IIF" => TokenCategory::Method,
        "INET_ATON" => TokenCategory::Method,
        "INET_NTOA" => TokenCategory::Method,
        "INITCAP" => TokenCategory::Method,
        "INSTR" => TokenCategory::Method,
        "ISDATE" => TokenCategory::Method,
        "ISJSON" => TokenCategory::Method,
        "ISNULL" => TokenCategory::Method,
        "ISNUMERIC" => TokenCategory::Method,
        "IS_IPV4" => TokenCategory::Method,
        "JSONB_AGG" => TokenCategory::Method,
        "JSON_AGG" => TokenCategory::Method,
        "JSON_ARRAY" => TokenCategory::Method,
        "JSON_ARRAYAGG" => TokenCategory::Method,
        "JSON_BUILD_OBJECT" => TokenCategory::Method,
        "JSON_CONTAINS" => TokenCategory::Method,
        "JSON_EXISTS" => TokenCategory::Method,
        "JSON_EXTRACT" => TokenCategory::Method,
        "JSON_INSERT" => TokenCategory::Method,
        "JSON_KEYS" => TokenCategory::Method,
        "JSON_LENGTH" => TokenCategory::Method,
        "JSON_MERGE_PATCH" => TokenCategory::Method,
        "JSON_MODIFY" => TokenCategory::Method,
        "JSON_OBJECT" => TokenCategory::Method,
        "JSON_OBJECTAGG" => TokenCategory::Method,
        "JSON_PRETTY" => TokenCategory::Method,
        "JSON_QUERY" => TokenCategory::Method,
        "JSON_REMOVE" => TokenCategory::Method,
        "JSON_REPLACE" => TokenCategory::Method,
        "JSON_SCALAR" => TokenCategory::Method,
        "JSON_SEARCH" => TokenCategory::Method,
        "JSON_SERIALIZE" => TokenCategory::Method,
        "JSON_SET" => TokenCategory::Method,
        "JSON_TABLE" => TokenCategory::Method,
        "JSON_TYPE" => TokenCategory::Method,
        "JSON_UNQUOTE" => TokenCategory::Method,
        "JSON_VALID" => TokenCategory::Method,
        "JSON_VALUE" => TokenCategory::Method,
        "JUSTIFY_DAYS" => TokenCategory::Method,
        "KURTOSIS" => TokenCategory::Method,
        "LAG" => TokenCategory::Method,
        "LAST" => TokenCategory::Method,
        "LAST_DAY" => TokenCategory::Method,
        "LAST_INSERT_ID" => TokenCategory::Method,
        "LAST_VALUE" => TokenCategory::Method,
        "LCASE" => TokenCategory::Method,
        "LEAD" => TokenCategory::Method,
        "LEAST" => TokenCategory::Method,
        "LEFT" => TokenCategory::Method,
        "LEN" => TokenCategory::Method,
        "LENGTH" => TokenCategory::Method,
        "LISTAGG" => TokenCategory::Method,
        "LN" => TokenCategory::Method,
        "LOAD_FILE" => TokenCategory::Method,
        "LOCALTIME" => TokenCategory::Method,
        "LOCALTIMESTAMP" => TokenCategory::Method,
        "LOCATE" => TokenCategory::Method,
        "LOG" => TokenCategory::Method,
        "LOG10" => TokenCategory::Method,
        "LOG2" => TokenCategory::Method,
        "LOWER" => TokenCategory::Method,
        "LPAD" => TokenCategory::Method,
        "LTRIM" => TokenCategory::Method,
        "MAKEDATE" => TokenCategory::Method,
        "MAKETIME" => TokenCategory::Method,
        "MAKE_DATE" => TokenCategory::Method,
        "MAKE_SET" => TokenCategory::Method,
        "MAKE_TIMESTAMP" => TokenCategory::Method,
        "MATCH" => TokenCategory::Method,
        "MATCH_NUMBER" => TokenCategory::Method,
        "MAVG" => TokenCategory::Method,
        "MAX" => TokenCategory::Method,
        "MD5" => TokenCategory::Method,
        "MDIFF" => TokenCategory::Method,
        "MEDIAN" => TokenCategory::Method,
        "MICROSECOND" => TokenCategory::Method,
        "MID" => TokenCategory::Method,
        "MIN" => TokenCategory::Method,
        "MINUTE" => TokenCategory::Method,
        "MLINREG" => TokenCategory::Method,
        "MOD" => TokenCategory::Method,
        "MONTH" => TokenCategory::Method,
        "MONTHNAME" => TokenCategory::Method,
        "MONTHS_BETWEEN" => TokenCategory::Method,
        "MSUM" => TokenCategory::Method,
        "NEWID" => TokenCategory::Method,
        "NEWSEQUENTIALID" => TokenCategory::Method,
        "NEXTVAL" => TokenCategory::Method,
        "NEXT_DAY" => TokenCategory::Method,
        "NORMALIZE" => TokenCategory::Method,
        "NOTNULL" => TokenCategory::Method,
        "NOW" => TokenCategory::Method,
        "NTH_VALUE" => TokenCategory::Method,
        "NTILE" => TokenCategory::Method,
        "NULLIF" => TokenCategory::Method,
        "NULLIFZERO" => TokenCategory::Method,
        "NUMTODSINTERVAL" => TokenCategory::Method,
        "NVL" => TokenCategory::Method,
        "NVL2" => TokenCategory::Method,
        "OBJECTPROPERTY" => TokenCategory::Method,
        "OBJECT_ID" => TokenCategory::Method,
        "OBJECT_NAME" => TokenCategory::Method,
        "OCCURRENCES_REGEX" => TokenCategory::Method,
        "OCT" => TokenCategory::Method,
        "OCTET_LENGTH" => TokenCategory::Method,
        "OPENDATASOURCE" => TokenCategory::Method,
        "OPENJSON" => TokenCategory::Method,
        "OPENQUERY" => TokenCategory::Method,
        "OPENROWSET" => TokenCategory::Method,
        "OPENXML" => TokenCategory::Method,
        "ORD" => TokenCategory::Method,
        "OREPLACE" => TokenCategory::Method,
        "OTRANSLATE" => TokenCategory::Method,
        "OVERLAY" => TokenCategory::Method,
        "PARSE" => TokenCategory::Method,
        "PATH" => TokenCategory::Method,
        "PATINDEX" => TokenCategory::Method,
        "PERCENTILE" => TokenCategory::Method,
        "PERCENTILE_CONT" => TokenCategory::Method,
        "PERCENTILE_DISC" => TokenCategory::Method,
        "PERCENT_RANK" => TokenCategory::Method,
        "PERIOD_ADD" => TokenCategory::Method,
        "PERIOD_DIFF" => TokenCategory::Method,
        "PG_SLEEP" => TokenCategory::Method,
        "PI" => TokenCategory::Method,
        "POSITION" => TokenCategory::Method,
        "POSITION_REGEX" => TokenCategory::Method,
        "POW" => TokenCategory::Method,
        "POWER" => TokenCategory::Method,
        "QUANTILE" => TokenCategory::Method,
        "QUARTER" => TokenCategory::Method,
        "QUOTE" => TokenCategory::Method,
        "QUOTENAME" => TokenCategory::Method,
        "RADIANS" => TokenCategory::Method,
        "RAISERROR" => TokenCategory::Method,
        "RAND" => TokenCategory::Method,
        "RANDOM" => TokenCategory::Method,
        "RANGE_N" => TokenCategory::Method,
        "RANK" => TokenCategory::Method,
        "RATIO_TO_REPORT" => TokenCategory::Method,
        "RAW" => TokenCategory::Method,
        "REFERENCES" => TokenCategory::Method,
        "REGEXP_COUNT" => TokenCategory::Method,
        "REGEXP_INSTR" => TokenCategory::Method,
        "REGEXP_LIKE" => TokenCategory::Method,
        "REGEXP_MATCHES" => TokenCategory::Method,
        "REGEXP_REPLACE" => TokenCategory::Method,
        "REGEXP_SPLIT_TO_TABLE" => TokenCategory::Method,
        "REGEXP_SUBSTR" => TokenCategory::Method,
        "REGR_AVGX" => TokenCategory::Method,
        "REGR_AVGY" => TokenCategory::Method,
        "REGR_COUNT" => TokenCategory::Method,
        "REGR_INTERCEPT" => TokenCategory::Method,
        "REGR_R2" => TokenCategory::Method,
        "REGR_SLOPE" => TokenCategory::Method,
        "REGR_SXX" => TokenCategory::Method,
        "REGR_SXY" => TokenCategory::Method,
        "REGR_SYY" => TokenCategory::Method,
        "REPEAT" => TokenCategory::Method,
        "REPLACE" => TokenCategory::Method,
        "REPLICATE" => TokenCategory::Method,
        "REVERSE" => TokenCategory::Method,
        "RIGHT" => TokenCategory::Method,
        "ROLLUP" => TokenCategory::Method,
        "ROOT" => TokenCategory::Method,
        "ROUND" => TokenCategory::Method,
        "ROUND_CEILING" => TokenCategory::Method,
        "ROUND_DOWN" => TokenCategory::Method,
        "ROUND_FLOOR" => TokenCategory::Method,
        "ROUND_HALF_DOWN" => TokenCategory::Method,
        "ROUND_HALF_EVEN" => TokenCategory::Method,
        "ROUND_HALF_UP" => TokenCategory::Method,
        "ROUND_UP" => TokenCategory::Method,
        "ROW" => TokenCategory::Method,
        "ROW_COUNT" => TokenCategory::Method,
        "ROW_NUMBER" => TokenCategory::Method,
        "RPAD" => TokenCategory::Method,
        "RTRIM" => TokenCategory::Method,
        "SCHEMA_ID" => TokenCategory::Method,
        "SCHEMA_NAME" => TokenCategory::Method,
        "SCOPE_IDENTITY" => TokenCategory::Method,
        "SECOND" => TokenCategory::Method,
        "SEC_TO_TIME" => TokenCategory::Method,
        "SEMANTICKEYPHRASETABLE" => TokenCategory::Method,
        "SEMANTICSIMILARITYDETAILSTABLE" => TokenCategory::Method,
        "SEMANTICSIMILARITYTABLE" => TokenCategory::Method,
        "SERVERPROPERTY" => TokenCategory::Method,
        "SESSIONPROPERTY" => TokenCategory::Method,
        "SESSION_USER" => TokenCategory::Method,
        "SHA1" => TokenCategory::Method,
        "SHA2" => TokenCategory::Method,
        "SIGN" => TokenCategory::Method,
        "SIN" => TokenCategory::Method,
        "SINH" => TokenCategory::Method,
        "SKEW" => TokenCategory::Method,
        "SLEEP" => TokenCategory::Method,
        "SMALLDATETIMEFROMPARTS" => TokenCategory::Method,
        "SOUNDEX" => TokenCategory::Method,
        "SPACE" => TokenCategory::Method,
        "SPLIT_PART" => TokenCategory::Method,
        "SQRT" => TokenCategory::Method,
        "SQUARE" => TokenCategory::Method,
        "STATEMENT_TIMESTAMP" => TokenCategory::Method,
        "STDDEV" => TokenCategory::Method,
        "STDDEV_POP" => TokenCategory::Method,
        "STDDEV_SAMP" => TokenCategory::Method,
        "STDEV" => TokenCategory::Method,
        "STDEVP" => TokenCategory::Method,
        "STR" => TokenCategory::Method,
        "STRCMP" => TokenCategory::Method,
        "STRING_AGG" => TokenCategory::Method,
        "STRING_ESCAPE" => TokenCategory::Method,
        "STRING_SPLIT" => TokenCategory::Method,
        "STRING_TO_ARRAY" => TokenCategory::Method,
        "STRPOS" => TokenCategory::Method,
        "STR_TO_DATE" => TokenCategory::Method,
        "STUFF" => TokenCategory::Method,
        "SUBDATE" => TokenCategory::Method,
        "SUBSTR" => TokenCategory::Method,
        "SUBSTRING" => TokenCategory::Method,
        "SUBSTRING_INDEX" => TokenCategory::Method,
        "SUBSTRING_REGEX" => TokenCategory::Method,
        "SUBTIME" => TokenCategory::Method,
        "SUM" => TokenCategory::Method,
        "SUSER_NAME" => TokenCategory::Method,
        "SUSER_SNAME" => TokenCategory::Method,
        "SWITCHOFFSET" => TokenCategory::Method,
        "SYSDATE" => TokenCategory::Method,
        "SYSDATETIME" => TokenCategory::Method,
        "SYSDATETIMEOFFSET" => TokenCategory::Method,
        "SYSTEM_USER" => TokenCategory::Method,
        "SYSUTCDATETIME" => TokenCategory::Method,
        "SYS_CONNECT_BY_PATH" => TokenCategory::Method,
        "SYS_GUID" => TokenCategory::Method,
        "TAN" => TokenCategory::Method,
        "TANH" => TokenCategory::Method,
        "TIMEDIFF" => TokenCategory::Method,
        "TIMEFROMPARTS" => TokenCategory::Method,
        "TIMESTAMPADD" => TokenCategory::Method,
        "TIMESTAMPDIFF" => TokenCategory::Method,
        "TIME_FORMAT" => TokenCategory::Method,
        "TIME_TO_SEC" => TokenCategory::Method,
        "TODATETIMEOFFSET" => TokenCategory::Method,
        "TO_BASE64" => TokenCategory::Method,
        "TO_BYTES" => TokenCategory::Method,
        "TO_CHAR" => TokenCategory::Method,
        "TO_DATE" => TokenCategory::Method,
        "TO_DAYS" => TokenCategory::Method,
        "TO_NUMBER" => TokenCategory::Method,
        "TO_TIMESTAMP" => TokenCategory::Method,
        "TRANSACTION_TIMESTAMP" => TokenCategory::Method,
        "TRANSLATE" => TokenCategory::Method,
        "TRANSLATE_CHK" => TokenCategory::Method,
        "TRANSLATE_REGEX" => TokenCategory::Method,
        "TREAT" => TokenCategory::Method,
        "TRIM" => TokenCategory::Method,
        "TRIM_ARRAY" => TokenCategory::Method,
        "TRUNC" => TokenCategory::Method,
        "TRUNCATE" => TokenCategory::Method,
        "TRY_CAST" => TokenCategory::Method,
        "TRY_CONVERT" => TokenCategory::Method,
        "TRY_PARSE" => TokenCategory::Method,
        "TSEQUAL" => TokenCategory::Method,
        "TYPE_NAME" => TokenCategory::Method,
        "UCASE" => TokenCategory::Method,
        "UNHEX" => TokenCategory::Method,
        "UNICODE" => TokenCategory::Method,
        "UNIQUE" => TokenCategory::Method,
        "UNIX_TIMESTAMP" => TokenCategory::Method,
        "UNNEST" => TokenCategory::Method,
        "UPPER" => TokenCategory::Method,
        "USER_NAME" => TokenCategory::Method,
        "UUID_SHORT" => TokenCategory::Method,
        "VAR" => TokenCategory::Method,
        "VARIANCE" => TokenCategory::Method,
        "VARP" => TokenCategory::Method,
        "VAR_POP" => TokenCategory::Method,
        "VAR_SAMP" => TokenCategory::Method,
        "VERSION" => TokenCategory::Method,
        "WEEK" => TokenCategory::Method,
        "WEEKDAY" => TokenCategory::Method,
        "WEEKOFYEAR" => TokenCategory::Method,
        "WEIGHT_STRING" => TokenCategory::Method,
        "WIDTH_BUCKET" => TokenCategory::Method,
        "XACT_STATE" => TokenCategory::Method,
        "XMLCAST" => TokenCategory::Method,
        "XMLEXISTS" => TokenCategory::Method,
        "XMLNAMESPACES" => TokenCategory::Method,
        "YEARWEEK" => TokenCategory::Method,
        "ZEROIFNULL" => TokenCategory::Method,

        // Xml Methods
        "EXIST" => TokenCategory::XmlMethod,
        "MODIFY" => TokenCategory::XmlMethod,
        "NODES" => TokenCategory::XmlMethod,
        "QUERY" => TokenCategory::XmlMethod,
        "VALUE" => TokenCategory::XmlMethod,

        _ => TokenCategory::Identifier,
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum TokenCategory {
    // names, numbers, and any other text not otherwise categorized
    Identifier,
    NewLine,
    WhiteSpace,
    Delimiter,
    Interpolation,
    Comment,
    Quote,
    Comma,
    FullStop,
    ParenOpen,
    ParenClose,
    Operator,
    Bitwise,
    Compare,
    Keyword,
    DataType,
    Event,
    Method,
    XmlMethod,
}

#[derive(Debug, PartialEq, Clone)]
pub enum TokenBehavior {
    DecreaseIndent,
    IncreaseIndent,
    IncreaseIndentIfNotAfterKeyword,
    IncreaseIndentIfStandAlone,
    NewLineAfter,
    NewLineAfterIfNotAfterKeyword,
    NewLineAfterSkip,
    NewLineAfterX2,
    NewLineAfterX2IfCombined,
    NewLineBefore,
    NewLineBeforeIfNotAfterEvent,
    NewLineBeforeIfNotAfterKeywordOrMethod,
    NewLineBeforeIfNotAfterWithin,
    NewLineBeforeIfNotMethodCall,
    NoNewLineAfterX2Skip,
    NoNewLineBefore,
    NoNewLineBeforeUnlessMatch,
    NoNewLineBeforeX2,
    NoSpaceAfter,
    NoSpaceAroundIfNotProvidedInput,
    NoSpaceBefore,
    NoSpaceBeforeIfStartOfNewLine,
    NoWhiteSpaceBefore,
    PreSpaceKeepInput,
}

enum TokenSpecial {
    DelimiterChange,
    Interpolation(InterpolationCategory),
    Comment(CommentCategory),
    Quote(QuoteCategory),
}

#[derive(Clone, Copy)]
enum InterpolationCategory {
    Bracket,
    Percent,
}

#[derive(Clone, Copy)]
enum CommentCategory {
    SingleLine,
    MultiLine,
}

#[derive(Clone, Copy)]
enum QuoteCategory {
    Backtick,
    QuoteSingle,
    QuoteDouble,
    Bracket,
}

#[cfg(test)]
mod tests;
