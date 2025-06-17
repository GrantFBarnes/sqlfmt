use std::collections::HashMap;

use crate::configuration::{ConfigCase, ConfigTab, Configuration};
use crate::token::*;

struct FormatState {
    tokens: Vec<Token>,
    indent_stack: Vec<Token>,
    paren_stack: Vec<ParenCategory>,
}

impl FormatState {
    fn new() -> FormatState {
        FormatState {
            tokens: vec![],
            indent_stack: vec![],
            paren_stack: vec![],
        }
    }

    fn push(&mut self, token: Token) {
        self.tokens.push(token);
    }

    fn increase_paren_stack(&mut self, token: &Token) {
        if token.category != Some(TokenCategory::ParenOpen) {
            return;
        }

        if let Some(prev_token) = self.tokens.last() {
            match prev_token.category {
                Some(TokenCategory::DataType)
                | Some(TokenCategory::Method)
                | Some(TokenCategory::XmlMethod) => {
                    self.paren_stack.push(ParenCategory::Space0Newline0);
                    return;
                }
                Some(TokenCategory::Quote) => {
                    let mut quote_chars = prev_token.value.chars();
                    quote_chars.next();
                    quote_chars.next_back();
                    if get_token_category_from_value(quote_chars.as_str().to_uppercase().as_str())
                        == Some(TokenCategory::DataType)
                    {
                        self.paren_stack.push(ParenCategory::Space0Newline0);
                        return;
                    }
                }
                Some(TokenCategory::Function) | None => {
                    self.paren_stack.push(ParenCategory::Space0Newline1);
                    return;
                }
                _ => (),
            }
        }

        self.paren_stack.push(ParenCategory::Space1Newline1);
    }

    fn decrease_paren_stack(&mut self, token: &Token) {
        if token.category == Some(TokenCategory::ParenClose) {
            self.paren_stack.pop();
        }
    }

    fn add_pre_space(&mut self, token: &Token, config: &Configuration) {
        if self.tokens.is_empty() {
            return;
        }

        if token.behavior.contains(&TokenBehavior::NoWhiteSpaceBefore) {
            return;
        }

        if config.newlines {
            self.add_pre_newline(token);
            self.remove_extra_newline(token);
        }

        let prev_token: &Token = self
            .tokens
            .last()
            .expect("should always have a previous token");

        match token.category {
            Some(TokenCategory::Delimiter) => {
                if prev_token.value.to_uppercase() != "DELIMITER" {
                    return;
                }
            }
            Some(TokenCategory::ParenOpen) => {
                if let Some(paren) = self.paren_stack.last() {
                    if paren == &ParenCategory::Space0Newline0
                        || paren == &ParenCategory::Space0Newline1
                    {
                        return;
                    }
                }
            }
            _ => (),
        }

        if prev_token.behavior.contains(&TokenBehavior::NoSpaceAfter) {
            return;
        }

        if prev_token.category == Some(TokenCategory::NewLine) {
            self.push(Token::new_space(match config.tabs {
                ConfigTab::Tab => "\t".repeat(self.indent_stack.len()),
                ConfigTab::Space(c) => " ".repeat(c as usize * self.indent_stack.len()),
            }));
            return;
        }

        if token.behavior.contains(&TokenBehavior::NoSpaceBefore) {
            return;
        }

        self.push(Token::new_space(String::from(" ")));
    }

    fn add_pre_newline(&mut self, token: &Token) {
        if self.tokens.is_empty() {
            return;
        }

        if self.paren_stack.contains(&ParenCategory::Space0Newline0) {
            return;
        }

        let prev1_token: &Token = self
            .tokens
            .last()
            .expect("should always have a previous token");

        if prev1_token.behavior.contains(&TokenBehavior::NewLineAfter) {
            self.push(Token::newline());
            return;
        }

        match prev1_token.category {
            Some(TokenCategory::Delimiter) => {
                self.push(Token::newline());
                self.push(Token::newline());
                return;
            }
            Some(TokenCategory::ParenOpen) => {
                if token.category != Some(TokenCategory::ParenClose) {
                    self.push(Token::newline());
                }
                return;
            }
            _ => (),
        }

        match prev1_token.value.to_uppercase().as_str() {
            "BEGIN" => match token.value.to_uppercase().as_str() {
                "TRY" | "CATCH" => return,
                _ => {
                    self.push(Token::newline());
                    return;
                }
            },
            "INTO" => {
                if self.tokens.iter().nth_back(2).is_some_and(|t| {
                    t.value.to_uppercase() != "INSERT" && t.value.to_uppercase() != "IGNORE"
                }) {
                    self.push(Token::newline());
                    return;
                }
            }
            _ => (),
        }

        let prev3_token: Option<&Token> = self.tokens.iter().nth_back(2);

        if prev3_token.is_some_and(|t| t.value.to_uppercase() == "TOP") {
            self.push(Token::newline());
            return;
        }

        if token.behavior.contains(&TokenBehavior::NewLineBefore) {
            if prev1_token.value.to_uppercase() != "ON"
                && prev3_token.is_none_or(|t| t.value.to_uppercase() != "ON")
            {
                self.push(Token::newline());
                return;
            }
        }

        match &token.category {
            Some(TokenCategory::ParenClose) => {
                self.push(Token::newline());
                return;
            }
            _ => (),
        }

        match token.value.to_uppercase().as_str() {
            "IF" => {
                if prev3_token.is_none_or(|t| t.value.to_uppercase() != "CREATE") {
                    self.push(Token::newline());
                    return;
                }
            }
            "WHILE" => {
                if prev1_token.value.to_uppercase() != "END" {
                    self.push(Token::newline());
                    return;
                }
            }
            "INTO" => {
                if prev1_token.value.to_uppercase() != "INSERT"
                    && prev1_token.value.to_uppercase() != "IGNORE"
                {
                    self.push(Token::newline());
                    return;
                }
            }
            _ => (),
        }
    }

    fn remove_extra_newline(&mut self, token: &Token) {
        // remove double newline for end of section
        if token.value.to_uppercase() == "END" || token.value.to_uppercase() == "ELSE" {
            if self.tokens.len() < 2 {
                return;
            }

            if self.tokens[self.tokens.len() - 1].category != Some(TokenCategory::NewLine) {
                return;
            }

            if self.tokens[self.tokens.len() - 2].category != Some(TokenCategory::NewLine) {
                return;
            }

            self.tokens.pop();
            return;
        }

        // remove double newline for two consecutive single delimiter lines
        if token.category == Some(TokenCategory::Delimiter) {
            let mut prev_newline_positions: Vec<usize> = vec![];
            let mut prev_endline_tokens: Vec<&Token> = vec![];
            for i in (1..self.tokens.len()).rev() {
                if self.tokens[i].category == Some(TokenCategory::NewLine) {
                    prev_newline_positions.push(i);
                    prev_endline_tokens.push(&self.tokens[i - 1]);
                    if prev_newline_positions.len() >= 3 {
                        break;
                    }
                }
            }

            // need at least two newlines to remove extra
            if prev_newline_positions.len() < 2 {
                return;
            }

            // last two newlines need to be next to each other
            if prev_newline_positions[0] != prev_newline_positions[1] + 1 {
                return;
            }

            // need previous line to end in delimiter
            if prev_endline_tokens[1].category != Some(TokenCategory::Delimiter) {
                return;
            }

            if prev_endline_tokens.len() == 2
                || prev_endline_tokens[2].value.to_uppercase() == String::from("BEGIN")
                || prev_endline_tokens[2].value.to_uppercase() == String::from("DO")
                || prev_endline_tokens[2].category == Some(TokenCategory::Delimiter)
                || prev_endline_tokens[2].category == Some(TokenCategory::NewLine)
                || prev_endline_tokens[2].category == Some(TokenCategory::Comment)
            {
                self.tokens.remove(prev_newline_positions[0]);
                return;
            }
        }
    }

    fn increase_indent_stack(&mut self, token: &Token) {
        if token.behavior.contains(&TokenBehavior::IncreaseIndent) {
            if self
                .tokens
                .iter()
                .nth_back(2)
                .is_none_or(|t| t.value.to_uppercase() != "ON")
                && self
                    .tokens
                    .iter()
                    .nth_back(4)
                    .is_none_or(|t| t.value.to_uppercase() != "ON")
            {
                self.indent_stack.push(token.clone());
                return;
            }
        }

        match token.value.to_uppercase().as_str() {
            "INTO" => {
                if self.tokens.iter().nth_back(2).is_some_and(|t| {
                    t.value.to_uppercase() != "INSERT" && t.value.to_uppercase() != "IGNORE"
                }) {
                    self.indent_stack.push(token.clone());
                    return;
                }
            }
            "THEN" => {
                if self
                    .indent_stack
                    .last()
                    .is_none_or(|t| t.value.to_uppercase() != "CASE")
                {
                    self.indent_stack.push(token.clone());
                    return;
                }
            }
            _ => (),
        }
    }

    fn decrease_indent_stack(&mut self, tokens: &Vec<Token>, i: usize) {
        if self.indent_stack.is_empty() {
            return;
        }

        let token: &Token = &tokens[i];

        let token_value: String = token.value.to_uppercase();
        let top_of_stack: &Token = self
            .indent_stack
            .last()
            .expect("should always have item on stack");
        let top_of_stack_value: &String = &top_of_stack.value.to_uppercase();

        let required_to_decrease: HashMap<&str, &str> = HashMap::from([
            ("(", ")"),
            ("OPEN", "CLOSE"),
            ("BEGIN", "END"),
            ("DO", "END"),
            ("CASE", "END"),
            ("THEN", "END"),
        ]);

        if let Some(v) = required_to_decrease.get(top_of_stack_value.as_str()) {
            if &token_value == v {
                self.indent_stack.pop();
            }
            return;
        }

        match token.category {
            Some(TokenCategory::Comment) => {
                for n in i + 1..tokens.len() {
                    if tokens[n].category != Some(TokenCategory::Keyword) {
                        continue;
                    }

                    if tokens[n].behavior.contains(&TokenBehavior::IncreaseIndent)
                        && tokens[n].value.to_uppercase() != "FROM"
                    {
                        self.indent_stack.pop();
                        return;
                    }
                    break;
                }
            }
            Some(TokenCategory::Delimiter) => {
                if top_of_stack
                    .behavior
                    .contains(&TokenBehavior::DecreaseIndentOnSingleLine)
                {
                    self.indent_stack.pop();
                    return;
                }
            }
            _ => (),
        }

        if top_of_stack
            .behavior
            .contains(&TokenBehavior::DecreaseIndentIfFound)
        {
            if &token.value.to_uppercase() == top_of_stack_value
                || tokens
                    .get(i + 1)
                    .is_some_and(|t| &t.value.to_uppercase() == top_of_stack_value)
                || tokens
                    .get(i + 2)
                    .is_some_and(|t| &t.value.to_uppercase() == top_of_stack_value)
            {
                self.indent_stack.pop();
                return;
            }
        }

        let decrease_until_match: Vec<&str> = match token_value.as_str() {
            ")" => vec!["("],
            "CLOSE" => vec!["OPEN"],
            "END" => vec!["BEGIN", "CASE", "THEN", "ELSE"],
            "INTO" => vec!["SELECT", "INSERT"],
            "SET" => vec!["UPDATE"],
            "VALUES" => vec!["INTO"],
            "BEGIN" | "CALL" | "DECLARE" | "DELETE" | "DELIMITER" | "DROP" | "ELSE" | "EXEC"
            | "EXECUTE" | "FOR" | "IF" | "INSERT" | "OPEN" | "PIVOT" | "RETURN" | "SELECT"
            | "TRUNCATE" | "UNION" | "UPDATE" | "WITH" => {
                vec![
                    "BEGIN",
                    "CALL",
                    "DECLARE",
                    "DELETE",
                    "DELIMITER",
                    "DROP",
                    "EXEC",
                    "EXECUTE",
                    "ELSE",
                    "FOR",
                    "FROM",
                    "GROUP",
                    "HAVING",
                    "IF",
                    "INSERT",
                    "OPEN",
                    "PIVOT",
                    "RETURN",
                    "SELECT",
                    "SET",
                    "TRUNCATE",
                    "UNION",
                    "UPDATE",
                    "WHERE",
                    "WHILE",
                    "WITH",
                ]
            }
            "FROM" => vec!["SELECT", "DELETE", "UPDATE", "INTO"],
            "WHERE" | "ORDER" | "GROUP" | "HAVING" | "LIMIT" | "WHILE" => {
                vec!["FROM"]
            }
            _ => vec![],
        };
        if !decrease_until_match.is_empty() {
            loop {
                let top: Option<Token> = self.indent_stack.pop();
                if top.is_none() {
                    break;
                }
                let top: Token = top.unwrap();

                if let Some(v) = required_to_decrease.get(top.value.as_str()) {
                    if &token_value != v {
                        self.indent_stack.push(top);
                    }
                    return;
                }

                if decrease_until_match.contains(&top.value.as_str()) {
                    break;
                }
            }
        }
    }

    fn get_result(&self, config: &Configuration) -> String {
        let mut result: String = String::new();
        for i in 0..self.tokens.len() {
            let token: &Token = &self.tokens[i];

            let mut token_value: String = token.value.clone();

            match token.category {
                Some(TokenCategory::Keyword)
                | Some(TokenCategory::DataType)
                | Some(TokenCategory::Method)
                | Some(TokenCategory::Function) => match config.case {
                    ConfigCase::Uppercase => token_value = token_value.to_uppercase(),
                    ConfigCase::Lowercase => token_value = token_value.to_lowercase(),
                    ConfigCase::Unchanged => (),
                },
                Some(TokenCategory::XmlMethod) => {
                    if result.ends_with(FULL_STOP)
                        && self
                            .tokens
                            .get(i + 1)
                            .is_some_and(|t| t.category == Some(TokenCategory::ParenOpen))
                    {
                        token_value = token_value.to_lowercase();
                    }
                }
                _ => (),
            }

            result.push_str(token_value.as_str());
        }
        return result.trim().to_string();
    }
}

#[derive(PartialEq, Eq, Debug)]
enum ParenCategory {
    Space0Newline0,
    Space0Newline1,
    Space1Newline1,
}

pub fn get_formatted_sql(config: &Configuration, sql: String) -> String {
    let mut state: FormatState = FormatState::new();

    let tokens: Vec<Token> = get_sql_tokens(sql);
    for i in 0..tokens.len() {
        let token: &Token = &tokens[i];

        if config.newlines && token.category == Some(TokenCategory::NewLine) {
            continue;
        }

        state.increase_paren_stack(token);
        state.decrease_indent_stack(&tokens, i);
        state.add_pre_space(token, config);
        state.push(token.clone());
        state.increase_indent_stack(token);
        state.decrease_paren_stack(token);
    }

    let mut result: String = state.get_result(config);

    if config.newlines {
        result = get_result_with_collapsed_paren(result, config);
    }

    return result;
}

fn get_result_with_collapsed_paren(sql: String, config: &Configuration) -> String {
    let mut current_line_char_count: usize = 0;
    let sql_bytes: &[u8] = sql.as_bytes();
    for i in 0..sql_bytes.len() {
        let curr_ch: char = sql_bytes[i].into();
        match curr_ch {
            NEW_LINE => {
                current_line_char_count = 0;
                continue;
            }
            PAREN_OPEN => {
                let mut paren_level: usize = 1;
                for j in i + 1..sql_bytes.len() {
                    let next_ch: char = sql_bytes[j].into();

                    match next_ch {
                        PAREN_OPEN => paren_level += 1,
                        PAREN_CLOSE => paren_level -= 1,
                        _ => continue,
                    }

                    if paren_level > 0 {
                        continue;
                    }

                    let paren: String = sql[i..=j].to_string();
                    let paren_collapsed: String = get_collapsed_paren(paren.clone());
                    if paren_collapsed == paren {
                        // no changes made
                        break;
                    }

                    // continue to count until next newline
                    for k in j + 1..sql_bytes.len() {
                        if char::from(sql_bytes[k]) == NEW_LINE {
                            break;
                        }
                        current_line_char_count += 1;
                    }

                    if current_line_char_count + paren_collapsed.len() > config.chars.into() {
                        // too long, keep with line breaks
                        break;
                    }

                    return get_result_with_collapsed_paren(
                        format!(
                            "{}{}{}",
                            sql[0..i].to_string(),
                            paren_collapsed,
                            sql[j + 1..].to_string(),
                        ),
                        config,
                    );
                }
            }
            _ => (),
        }
        current_line_char_count += 1;
    }

    return sql;
}

fn get_collapsed_paren(mut sql: String) -> String {
    let find_replace_with_space: [&'static str; 3] = ["\n", "\t", "  "];
    for f in find_replace_with_space {
        while sql.contains(f) {
            sql = sql.replace(f, " ")
        }
    }

    let find_trim_space: [&'static str; 2] = ["( ", " )"];
    for f in find_trim_space {
        while sql.contains(f) {
            sql = sql.replace(f, f.trim())
        }
    }

    return sql;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_formatted_sql_select_simple() {
        assert_eq!(
            get_formatted_sql(&Configuration::new(), String::from("SELECT * FROM TBL1")),
            r#"SELECT * FROM TBL1"#
        );
    }

    #[test]
    fn test_get_formatted_sql_newlines() {
        let mut config: Configuration = Configuration::new();
        config.newlines = true;
        assert_eq!(
            get_formatted_sql(&config, String::from("SELECT * FROM TBL1")),
            r#"SELECT
    *
FROM TBL1"#
        );
    }

    #[test]
    fn test_get_formatted_sql_upper() {
        let mut config: Configuration = Configuration::new();
        config.case = ConfigCase::Uppercase;
        assert_eq!(
            get_formatted_sql(&config, String::from("select * from tbl1")),
            r#"SELECT * FROM tbl1"#
        );
    }

    #[test]
    fn test_get_formatted_sql_lower() {
        let mut config: Configuration = Configuration::new();
        config.case = ConfigCase::Lowercase;
        assert_eq!(
            get_formatted_sql(&config, String::from("SELECT * FROM TBL1")),
            r#"select * from TBL1"#
        );
    }

    #[test]
    fn test_get_formatted_sql_select_simple_newlines() {
        assert_eq!(
            get_formatted_sql(
                &Configuration::new(),
                String::from(
                    r#"
                    SELECT  *
                    FROM  TBL1
                    "#
                )
            ),
            r#"SELECT *
FROM TBL1"#
        );
    }

    #[test]
    fn test_get_formatted_sql_config_tabs() {
        let mut config: Configuration = Configuration::new();
        config.tabs = ConfigTab::Tab;
        assert_eq!(
            get_formatted_sql(
                &config,
                String::from(
                    r#"
                    SELECT
                        C1,
                        C2
                    FROM TBL1
                    "#
                )
            ),
            r#"SELECT
	C1,
	C2
FROM TBL1"#
        );
    }

    #[test]
    fn test_get_formatted_sql_config_spaces() {
        let mut config: Configuration = Configuration::new();
        config.tabs = ConfigTab::Space(2);
        assert_eq!(
            get_formatted_sql(
                &config,
                String::from(
                    r#"
                    SELECT
                        C1,
                        C2
                    FROM TBL1
                    "#
                )
            ),
            r#"SELECT
  C1,
  C2
FROM TBL1"#
        );
    }

    #[test]
    fn test_get_formatted_sql_config_chars_short() {
        let mut config: Configuration = Configuration::new();
        config.newlines = true;
        config.chars = 10;
        assert_eq!(
            get_formatted_sql(&config, String::from(r#"(SELECT C1, C2 FROM TBL1)"#)),
            r#"(
    SELECT
        C1,
        C2
    FROM TBL1
)"#
        );
    }

    #[test]
    fn test_get_formatted_sql_config_chars_long() {
        let mut config: Configuration = Configuration::new();
        config.newlines = true;
        config.chars = 120;
        assert_eq!(
            get_formatted_sql(&config, String::from(r#"(SELECT C1, C2 FROM TBL1)"#)),
            r#"(SELECT C1, C2 FROM TBL1)"#
        );
    }

    #[test]
    fn test_get_formatted_sql_select_multiple_columns_inline() {
        assert_eq!(
            get_formatted_sql(
                &Configuration::new(),
                String::from("SELECT C1,C2, C3 FROM TBL1")
            ),
            r#"SELECT C1, C2, C3 FROM TBL1"#
        );
    }

    #[test]
    fn test_get_formatted_sql_select_multiple_columns_newlines() {
        assert_eq!(
            get_formatted_sql(
                &Configuration::new(),
                String::from(
                    r#"
                    SELECT
                    C1 AS 'Column 1',
                      C2 AS 'Column 2',
                    C3
                    FROM TBL1 AS T
                    "#
                )
            ),
            r#"SELECT
    C1 AS 'Column 1',
    C2 AS 'Column 2',
    C3
FROM TBL1 AS T"#
        );
    }

    #[test]
    fn test_get_formatted_sql_alias() {
        assert_eq!(
            get_formatted_sql(
                &Configuration::new(),
                String::from(r#"SELECT C1 AS 'Column 1' FROM TBL1"#)
            ),
            r#"SELECT C1 AS 'Column 1' FROM TBL1"#
        );
    }

    #[test]
    fn test_get_formatted_sql_alias_config_newline() {
        let mut config: Configuration = Configuration::new();
        config.newlines = true;
        assert_eq!(
            get_formatted_sql(
                &config,
                String::from(r#"SELECT C1 AS 'Column 1' FROM TBL1"#)
            ),
            r#"SELECT
    C1 AS 'Column 1'
FROM TBL1"#
        );
    }

    #[test]
    fn test_get_formatted_sql_datatype_quote() {
        assert_eq!(
            get_formatted_sql(&Configuration::new(), String::from("[NVARCHAR](36)")),
            r#"[NVARCHAR](36)"#
        );
    }

    #[test]
    fn test_get_formatted_sql_datatype_quote_config_newline() {
        let mut config: Configuration = Configuration::new();
        config.newlines = true;
        assert_eq!(
            get_formatted_sql(&config, String::from("[NVARCHAR](36)")),
            r#"[NVARCHAR](36)"#
        );
    }

    #[test]
    fn test_get_formatted_sql_convert() {
        assert_eq!(
            get_formatted_sql(
                &Configuration::new(),
                String::from("CONVERT(NVARCHAR(36), ID)")
            ),
            r#"CONVERT(NVARCHAR(36), ID)"#
        );
    }

    #[test]
    fn test_get_formatted_sql_convert_config_newline() {
        let mut config: Configuration = Configuration::new();
        config.newlines = true;
        assert_eq!(
            get_formatted_sql(&config, String::from("CONVERT(NVARCHAR(36), ID)")),
            r#"CONVERT(NVARCHAR(36), ID)"#
        );
    }

    #[test]
    fn test_get_formatted_sql_select_curly_string() {
        assert_eq!(
            get_formatted_sql(
                &Configuration::new(),
                String::from(
                    r#"
                    SELECT *
                    FROM {tableNames[i]}
                    WHERE C1 = 1
                    "#,
                )
            ),
            r#"SELECT *
FROM {tableNames[i]}
WHERE C1 = 1"#
        );
    }

    #[test]
    fn test_get_formatted_sql_select_curly_string_config_newline() {
        let mut config: Configuration = Configuration::new();
        config.newlines = true;
        assert_eq!(
            get_formatted_sql(
                &config,
                String::from(
                    r#"
                    SELECT *
                    FROM {tableNames[i]}
                    WHERE C1 = 1
                    "#,
                )
            ),
            r#"SELECT
    *
FROM {tableNames[i]}
WHERE C1 = 1"#
        );
    }

    #[test]
    fn test_get_formatted_sql_call_curly_string() {
        assert_eq!(
            get_formatted_sql(
                &Configuration::new(),
                String::from(r#"CALL SCH.{procedureName}();"#)
            ),
            r#"CALL SCH.{procedureName}();"#
        );
    }

    #[test]
    fn test_get_formatted_sql_call_curly_string_config_newline() {
        let mut config: Configuration = Configuration::new();
        config.newlines = true;
        assert_eq!(
            get_formatted_sql(&config, String::from(r#"CALL SCH.{procedureName}();"#)),
            r#"CALL SCH.{procedureName}();"#
        );
    }

    #[test]
    fn test_get_formatted_sql_embedded_conditions() {
        assert_eq!(
            get_formatted_sql(
                &Configuration::new(),
                String::from(
                    r#"
                    SELECT * FROM TBL1
                    WHERE ((C1=0 AND C2=0)OR(C1=1 AND C2=1))
                    "#
                )
            ),
            r#"SELECT * FROM TBL1
WHERE ((C1 = 0 AND C2 = 0) OR (C1 = 1 AND C2 = 1))"#
        );
    }

    #[test]
    fn test_get_formatted_sql_embedded_conditions_config_newline() {
        let mut config: Configuration = Configuration::new();
        config.newlines = true;
        assert_eq!(
            get_formatted_sql(
                &config,
                String::from(
                    r#"
                    SELECT * FROM TBL1
                    WHERE ((C1=0 AND C2=0)OR(C1=1 AND C2=1))
                    "#
                )
            ),
            r#"SELECT
    *
FROM TBL1
WHERE ((C1 = 0 AND C2 = 0) OR (C1 = 1 AND C2 = 1))"#
        );
    }

    #[test]
    fn test_get_formatted_sql_sub_query_inline() {
        assert_eq!(
            get_formatted_sql(
                &Configuration::new(),
                String::from(r#" SELECT ( SELECT TOP 1 ID FROM TBL1 ) AS ID "#)
            ),
            r#"SELECT (SELECT TOP 1 ID FROM TBL1) AS ID"#
        );
    }

    #[test]
    fn test_get_formatted_sql_sub_query_inline_config_newline() {
        let mut config: Configuration = Configuration::new();
        config.newlines = true;
        assert_eq!(
            get_formatted_sql(
                &config,
                String::from(r#" SELECT ( SELECT TOP 1 ID FROM TBL1 ) AS ID "#)
            ),
            r#"SELECT
    (SELECT TOP 1 ID FROM TBL1) AS ID"#
        );
    }

    #[test]
    fn test_get_formatted_sql_union() {
        assert_eq!(
            get_formatted_sql(
                &Configuration::new(),
                String::from("SELECT C1 UNION SELECT C2")
            ),
            r#"SELECT C1 UNION SELECT C2"#
        );
    }

    #[test]
    fn test_get_formatted_sql_union_config_newline() {
        let mut config: Configuration = Configuration::new();
        config.newlines = true;
        assert_eq!(
            get_formatted_sql(&config, String::from("SELECT C1 UNION SELECT C2")),
            r#"SELECT
    C1
UNION
SELECT
    C2"#
        );
    }

    #[test]
    fn test_get_formatted_sql_union_complex() {
        assert_eq!(
            get_formatted_sql(
                &Configuration::new(),
                String::from(
                    r#"
                    SELECT C1 FROM TBL1
                    UNION SELECT C2 FROM TBL2
                    UNION SELECT C3 FROM TBL3
                    "#
                )
            ),
            r#"SELECT C1 FROM TBL1
UNION SELECT C2 FROM TBL2
UNION SELECT C3 FROM TBL3"#
        );
    }

    #[test]
    fn test_get_formatted_sql_union_complex_config_newline() {
        let mut config: Configuration = Configuration::new();
        config.newlines = true;
        assert_eq!(
            get_formatted_sql(
                &config,
                String::from(
                    r#"
                    SELECT C1 FROM TBL1
                    UNION SELECT C2 FROM TBL2
                    UNION SELECT C3 FROM TBL3
                    "#
                )
            ),
            r#"SELECT
    C1
FROM TBL1
UNION
SELECT
    C2
FROM TBL2
UNION
SELECT
    C3
FROM TBL3"#
        );
    }

    #[test]
    fn test_get_formatted_sql_comma_start_multiline() {
        assert_eq!(
            get_formatted_sql(
                &Configuration::new(),
                String::from(
                    r#"
                    SELECT
                    C1
                    ,C2
                    ,C3
                    FROM TBL1
                    "#
                )
            ),
            r#"SELECT
    C1
    , C2
    , C3
FROM TBL1"#
        );
    }

    #[test]
    fn test_get_formatted_sql_comma_start_multiline_config_newline() {
        let mut config: Configuration = Configuration::new();
        config.newlines = true;
        assert_eq!(
            get_formatted_sql(
                &config,
                String::from(
                    r#"
                    SELECT
                    C1
                    ,C2
                    ,C3
                    FROM TBL1
                    "#
                )
            ),
            r#"SELECT
    C1,
    C2,
    C3
FROM TBL1"#
        );
    }

    #[test]
    fn test_get_formatted_sql_sub_query_multiline() {
        assert_eq!(
            get_formatted_sql(
                &Configuration::new(),
                String::from(
                    r#"
                    SELECT (
                    SELECT TOP 1 ID FROM TBL1
                    ) AS ID,
                    C1
                    FROM TBL1
                    "#
                )
            ),
            r#"SELECT (
        SELECT TOP 1 ID FROM TBL1
    ) AS ID,
    C1
FROM TBL1"#
        );
    }

    #[test]
    fn test_get_formatted_sql_sub_query_multiline_config_newline() {
        let mut config: Configuration = Configuration::new();
        config.newlines = true;
        assert_eq!(
            get_formatted_sql(
                &config,
                String::from(
                    r#"
                    SELECT (
                    SELECT TOP 1 ID FROM TBL1
                    ) AS ID,
                    C1
                    FROM TBL1
                    "#
                )
            ),
            r#"SELECT
    (SELECT TOP 1 ID FROM TBL1) AS ID,
    C1
FROM TBL1"#
        );
    }

    #[test]
    fn test_get_formatted_sql_delimiter() {
        assert_eq!(
            get_formatted_sql(
                &Configuration::new(),
                String::from(
                    r#"
                    SELECT * FROM TBL1;DECLARE C1=1;DECLARE C2= 2;  DECLARE C3 = 3;SELECT * FROM TBL1  DECLARE C4=4;DECLARE C5=5;
                    "#
                )
            ),
            r#"SELECT * FROM TBL1; DECLARE C1 = 1; DECLARE C2 = 2; DECLARE C3 = 3; SELECT * FROM TBL1 DECLARE C4 = 4; DECLARE C5 = 5;"#
        );
    }

    #[test]
    fn test_get_formatted_sql_delimiter_config_newline() {
        let mut config: Configuration = Configuration::new();
        config.newlines = true;
        assert_eq!(
            get_formatted_sql(
                &config,
                String::from(
                    r#"
                    SELECT * FROM TBL1;DECLARE C1=1;DECLARE C2= 2;  DECLARE C3 = 3;SELECT * FROM TBL1  DECLARE C4=4;DECLARE C5=5;
                    "#
                )
            ),
            r#"SELECT
    *
FROM TBL1;

DECLARE C1 = 1;
DECLARE C2 = 2;
DECLARE C3 = 3;

SELECT
    *
FROM TBL1
DECLARE C4 = 4;

DECLARE C5 = 5;"#
        );
    }

    #[test]
    fn test_get_formatted_sql_delimiter_comment() {
        assert_eq!(
            get_formatted_sql(
                &Configuration::new(),
                String::from(
                    r#"
                    -- COMMENT
                    DECLARE C1=1;DECLARE C2=2;
                    "#
                )
            ),
            r#"-- COMMENT
DECLARE C1 = 1; DECLARE C2 = 2;"#
        );
    }

    #[test]
    fn test_get_formatted_sql_delimiter_comment_config_newline() {
        let mut config: Configuration = Configuration::new();
        config.newlines = true;
        assert_eq!(
            get_formatted_sql(
                &config,
                String::from(
                    r#"
                    -- COMMENT
                    DECLARE C1=1;DECLARE C2=2;
                    "#
                )
            ),
            r#"-- COMMENT
DECLARE C1 = 1;
DECLARE C2 = 2;"#
        );
    }

    #[test]
    fn test_get_formatted_sql_delimiter_change() {
        assert_eq!(
            get_formatted_sql(
                &Configuration::new(),
                String::from(
                    r#"
                    SELECT 1;DELIMITER $$ SELECT 1; DELIMITER ;
                    "#
                )
            ),
            r#"SELECT 1; DELIMITER $$ SELECT 1; DELIMITER ;"#
        );
    }

    #[test]
    fn test_get_formatted_sql_delimiter_change_config_newline() {
        let mut config: Configuration = Configuration::new();
        config.newlines = true;
        assert_eq!(
            get_formatted_sql(
                &config,
                String::from(
                    r#"
                    SELECT 1;DELIMITER $$ SELECT 1; DELIMITER ;
                    "#
                )
            ),
            r#"SELECT
    1;

DELIMITER $$

SELECT
    1;
DELIMITER ;"#
        );
    }

    #[test]
    fn test_get_formatted_sql_declare_no_delimiter() {
        assert_eq!(
            get_formatted_sql(
                &Configuration::new(),
                String::from("DECLARE C1 = 1 DECLARE C2 = 2   DECLARE C3 = 3 ")
            ),
            r#"DECLARE C1 = 1 DECLARE C2 = 2 DECLARE C3 = 3"#
        );
    }

    #[test]
    fn test_get_formatted_sql_declare_no_delimiter_config_newline() {
        let mut config: Configuration = Configuration::new();
        config.newlines = true;
        assert_eq!(
            get_formatted_sql(
                &config,
                String::from("DECLARE C1 = 1 DECLARE C2 = 2   DECLARE C3 = 3 ")
            ),
            r#"DECLARE C1 = 1
DECLARE C2 = 2
DECLARE C3 = 3"#
        );
    }

    #[test]
    fn test_get_formatted_sql_multiple_declare() {
        assert_eq!(
            get_formatted_sql(
                &Configuration::new(),
                String::from("DECLARE C1 = 1, C2 = 2, C3 = 3;")
            ),
            r#"DECLARE C1 = 1, C2 = 2, C3 = 3;"#
        );
    }

    #[test]
    fn test_get_formatted_sql_multiple_declare_config_newline() {
        let mut config: Configuration = Configuration::new();
        config.newlines = true;
        assert_eq!(
            get_formatted_sql(&config, String::from("DECLARE C1 = 1, C2 = 2, C3 = 3;")),
            r#"DECLARE C1 = 1,
    C2 = 2,
    C3 = 3;"#
        );
    }

    #[test]
    fn test_get_formatted_sql_set_no_delimiter() {
        assert_eq!(
            get_formatted_sql(
                &Configuration::new(),
                String::from("SET C1 = 1 SET C2 = 2   SET C3 = 3 ")
            ),
            r#"SET C1 = 1 SET C2 = 2 SET C3 = 3"#
        );
    }

    #[test]
    fn test_get_formatted_sql_set_no_delimiter_config_newline() {
        let mut config: Configuration = Configuration::new();
        config.newlines = true;
        assert_eq!(
            get_formatted_sql(&config, String::from("SET C1 = 1 SET C2 = 2   SET C3 = 3 ")),
            r#"SET C1 = 1
SET C2 = 2
SET C3 = 3"#
        );
    }

    #[test]
    fn test_get_formatted_sql_set() {
        assert_eq!(
            get_formatted_sql(
                &Configuration::new(),
                String::from(
                    r#"
                    SET C1 = 1
                    SET C2 = 2
                    "#,
                )
            ),
            r#"SET C1 = 1
SET C2 = 2"#
        );
    }

    #[test]
    fn test_get_formatted_sql_set_config_newline() {
        let mut config: Configuration = Configuration::new();
        config.newlines = true;
        assert_eq!(
            get_formatted_sql(
                &config,
                String::from(
                    r#"
                    SET C1 = 1
                    SET C2 = 2
                    "#,
                )
            ),
            r#"SET C1 = 1
SET C2 = 2"#
        );
    }

    #[test]
    fn test_get_formatted_sql_update() {
        assert_eq!(
            get_formatted_sql(
                &Configuration::new(),
                String::from(
                    r#"
                    UPDATE TBL1
                    SET
                    C1 = 1,
                    C2 = 2
                    WHERE C3 = 3
                    "#,
                )
            ),
            r#"UPDATE TBL1
SET
    C1 = 1,
    C2 = 2
WHERE C3 = 3"#
        );
    }

    #[test]
    fn test_get_formatted_sql_update_config_newline() {
        let mut config: Configuration = Configuration::new();
        config.newlines = true;
        assert_eq!(
            get_formatted_sql(
                &config,
                String::from(
                    r#"
                    UPDATE TBL1
                    SET
                    C1 = 1,
                    C2 = 2
                    WHERE C3 = 3
                    "#,
                )
            ),
            r#"UPDATE TBL1
SET C1 = 1,
    C2 = 2
WHERE C3 = 3"#
        );
    }

    #[test]
    fn test_get_formatted_sql_select_where_quote() {
        assert_eq!(
            get_formatted_sql(
                &Configuration::new(),
                String::from(
                    r#"
                    SELECT *
                    FROM TBL1
                    WHERE C1 = 'some value'
                    "#,
                )
            ),
            r#"SELECT *
FROM TBL1
WHERE C1 = 'some value'"#
        );
    }

    #[test]
    fn test_get_formatted_sql_select_where_quote_config_newline() {
        let mut config: Configuration = Configuration::new();
        config.newlines = true;
        assert_eq!(
            get_formatted_sql(
                &config,
                String::from(
                    r#"
                    SELECT *
                    FROM TBL1
                    WHERE C1 = 'some value'
                    "#,
                )
            ),
            r#"SELECT
    *
FROM TBL1
WHERE C1 = 'some value'"#
        );
    }

    #[test]
    fn test_get_formatted_sql_select_where_in() {
        assert_eq!(
            get_formatted_sql(
                &Configuration::new(),
                String::from(
                    r#"
                    SELECT *
                    FROM TBL1
                    WHERE C1 IN (1,2,3)
                    "#,
                )
            ),
            r#"SELECT *
FROM TBL1
WHERE C1 IN (1, 2, 3)"#
        );
    }

    #[test]
    fn test_get_formatted_sql_select_where_in_config_newline() {
        let mut config: Configuration = Configuration::new();
        config.newlines = true;
        assert_eq!(
            get_formatted_sql(
                &config,
                String::from(
                    r#"
                    SELECT *
                    FROM TBL1
                    WHERE C1 IN (1,2,3)
                    "#,
                )
            ),
            r#"SELECT
    *
FROM TBL1
WHERE C1 IN (1, 2, 3)"#
        );
    }

    #[test]
    fn test_get_formatted_sql_count_distinct() {
        assert_eq!(
            get_formatted_sql(
                &Configuration::new(),
                String::from(
                    r#"
                    SELECT COUNT(DISTINCT YEAR(D1))
                    FROM TBL1
                    "#,
                )
            ),
            r#"SELECT COUNT(DISTINCT YEAR(D1))
FROM TBL1"#
        );
    }

    #[test]
    fn test_get_formatted_sql_count_distinct_config_newline() {
        let mut config: Configuration = Configuration::new();
        config.newlines = true;
        assert_eq!(
            get_formatted_sql(
                &config,
                String::from(
                    r#"
                    SELECT COUNT(DISTINCT YEAR(D1))
                    FROM TBL1
                    "#,
                )
            ),
            r#"SELECT
    COUNT(DISTINCT YEAR(D1))
FROM TBL1"#
        );
    }

    #[test]
    fn test_get_formatted_sql_select_group_by() {
        assert_eq!(
            get_formatted_sql(
                &Configuration::new(),
                String::from(
                    r#"
                    SELECT C1,
                    COUNT(*) AS CNT
                    FROM TBL1
                    GROUP BY C1
                    HAVING COUNT(*) > 1
                    "#,
                )
            ),
            r#"SELECT C1,
    COUNT(*) AS CNT
FROM TBL1
GROUP BY C1
HAVING COUNT(*) > 1"#
        );
    }

    #[test]
    fn test_get_formatted_sql_select_group_by_config_newline() {
        let mut config: Configuration = Configuration::new();
        config.newlines = true;
        assert_eq!(
            get_formatted_sql(
                &config,
                String::from(
                    r#"
                    SELECT C1,
                    COUNT(*) AS CNT
                    FROM TBL1
                    GROUP BY C1
                    HAVING COUNT(*) > 1
                    "#,
                )
            ),
            r#"SELECT
    C1,
    COUNT(*) AS CNT
FROM TBL1
GROUP BY C1
HAVING COUNT(*) > 1"#
        );
    }

    #[test]
    fn test_get_formatted_sql_join() {
        assert_eq!(
            get_formatted_sql(
                &Configuration::new(),
                String::from(
                    r#"
                    SELECT T1.C1, T1.C2,
                    T2.C2
                    FROM TBL1 AS T1
                    INNER JOIN TBL2 AS T2 ON T2.C1 = T1.C1
                    "#
                )
            ),
            r#"SELECT T1.C1, T1.C2,
    T2.C2
FROM TBL1 AS T1
    INNER JOIN TBL2 AS T2 ON T2.C1 = T1.C1"#
        );
    }

    #[test]
    fn test_get_formatted_sql_join_config_newline() {
        let mut config: Configuration = Configuration::new();
        config.newlines = true;
        assert_eq!(
            get_formatted_sql(
                &config,
                String::from(
                    r#"
                    SELECT T1.C1, T1.C2,
                    T2.C2
                    FROM TBL1 AS T1
                    INNER JOIN TBL2 AS T2 ON T2.C1 = T1.C1
                    "#
                )
            ),
            r#"SELECT
    T1.C1,
    T1.C2,
    T2.C2
FROM TBL1 AS T1
    INNER JOIN TBL2 AS T2 ON T2.C1 = T1.C1"#
        );
    }

    #[test]
    fn test_get_formatted_sql_select_where() {
        assert_eq!(
            get_formatted_sql(
                &Configuration::new(),
                String::from(
                    r#"
                    SELECT
                    C1,
                    C2,
                    C3
                    FROM TBL1
                    WHERE C1>1
                    AND C2 IS NOT NULL
                    "#
                )
            ),
            r#"SELECT
    C1,
    C2,
    C3
FROM TBL1
WHERE C1 > 1
    AND C2 IS NOT NULL"#
        );
    }

    #[test]
    fn test_get_formatted_sql_select_where_config_newline() {
        let mut config: Configuration = Configuration::new();
        config.newlines = true;
        assert_eq!(
            get_formatted_sql(
                &config,
                String::from(
                    r#"
                    SELECT
                    C1,
                    C2,
                    C3
                    FROM TBL1
                    WHERE C1>1
                    AND C2 IS NOT NULL
                    "#
                )
            ),
            r#"SELECT
    C1,
    C2,
    C3
FROM TBL1
WHERE C1 > 1
    AND C2 IS NOT NULL"#
        );
    }

    #[test]
    fn test_get_formatted_sql_multi_join() {
        assert_eq!(
            get_formatted_sql(
                &Configuration::new(),
                String::from(
                    r#"
                    SELECT DISTINCT
                    T1.C1 AS C1,
                    T2.C2 AS C2,
                    T3.C3 AS C3
                    FROM TBL1 AS T1
                    INNER JOIN TBL2 AS T2
                    ON T2.C1 = T1.C1
                    AND T2.C2 = T1.C2
                    INNER JOIN TBL3 AS T3 ON T3.C2 = T2.C2
                    WHERE (T1.C2<>T2.C2 OR T1.C2<>T3.C2)
                    ORDER BY T1.C1
                    LIMIT 1
                    "#
                )
            ),
            r#"SELECT DISTINCT
    T1.C1 AS C1,
    T2.C2 AS C2,
    T3.C3 AS C3
FROM TBL1 AS T1
    INNER JOIN TBL2 AS T2
        ON T2.C1 = T1.C1
        AND T2.C2 = T1.C2
    INNER JOIN TBL3 AS T3 ON T3.C2 = T2.C2
WHERE (T1.C2 <> T2.C2 OR T1.C2 <> T3.C2)
ORDER BY T1.C1
LIMIT 1"#
        );
    }

    #[test]
    fn test_get_formatted_sql_multi_join_config_newline() {
        let mut config: Configuration = Configuration::new();
        config.newlines = true;
        assert_eq!(
            get_formatted_sql(
                &config,
                String::from(
                    r#"
                    SELECT DISTINCT
                    T1.C1 AS C1,
                    T2.C2 AS C2,
                    T3.C3 AS C3
                    FROM TBL1 AS T1
                    INNER JOIN TBL2 AS T2
                    ON T2.C1 = T1.C1
                    AND T2.C2 = T1.C2
                    INNER JOIN TBL3 AS T3 ON T3.C2 = T2.C2
                    WHERE (T1.C2<>T2.C2 OR T1.C2<>T3.C2)
                    ORDER BY T1.C1
                    LIMIT 1
                    "#
                )
            ),
            r#"SELECT
    DISTINCT
    T1.C1 AS C1,
    T2.C2 AS C2,
    T3.C3 AS C3
FROM TBL1 AS T1
    INNER JOIN TBL2 AS T2 ON T2.C1 = T1.C1
        AND T2.C2 = T1.C2
    INNER JOIN TBL3 AS T3 ON T3.C2 = T2.C2
WHERE (T1.C2 <> T2.C2 OR T1.C2 <> T3.C2)
ORDER BY T1.C1
LIMIT 1"#
        );
    }

    #[test]
    fn test_get_formatted_sql_two_statements() {
        assert_eq!(
            get_formatted_sql(
                &Configuration::new(),
                String::from("SELECT * FROM TBL1;SELECT * FROM TBL1;")
            ),
            String::from("SELECT * FROM TBL1; SELECT * FROM TBL1;")
        );
    }

    #[test]
    fn test_get_formatted_sql_two_statements_config_newline() {
        let mut config: Configuration = Configuration::new();
        config.newlines = true;
        assert_eq!(
            get_formatted_sql(
                &config,
                String::from("SELECT * FROM TBL1;SELECT * FROM TBL1;")
            ),
            String::from(
                r#"SELECT
    *
FROM TBL1;

SELECT
    *
FROM TBL1;"#
            )
        );
    }

    #[test]
    fn test_get_formatted_sql_single_comments() {
        assert_eq!(
            get_formatted_sql(
                &Configuration::new(),
                String::from(
                    r#"
                    -- top comment
                    SELECT C1,--inline comment
                    -- after comment 1
                    -- after comment 2
                    C2
                    -- after comment 3
                    FROM TBL1
                    "#,
                )
            ),
            r#"-- top comment
SELECT C1, --inline comment
    -- after comment 1
    -- after comment 2
    C2
    -- after comment 3
FROM TBL1"#
        );
    }

    #[test]
    fn test_get_formatted_sql_single_comments_config_newline() {
        let mut config: Configuration = Configuration::new();
        config.newlines = true;
        assert_eq!(
            get_formatted_sql(
                &config,
                String::from(
                    r#"
                    -- top comment
                    SELECT C1,--inline comment
                    -- after comment 1
                    -- after comment 2
                    C2
                    -- after comment 3
                    FROM TBL1
                    "#,
                )
            ),
            r#"-- top comment
SELECT
    C1,
    --inline comment
    -- after comment 1
    -- after comment 2
    C2
    -- after comment 3
FROM TBL1"#
        );
    }

    #[test]
    fn test_get_formatted_sql_single_comment_new_statement() {
        assert_eq!(
            get_formatted_sql(
                &Configuration::new(),
                String::from(
                    r#"
                    -- comment
                    SELECT
                        C1,
                        C2
                    FROM TBL1;

                    -- comment
                    SELECT
                        C1,
                        C2
                    FROM TBL1;
                    "#,
                )
            ),
            r#"-- comment
SELECT
    C1,
    C2
FROM TBL1;

-- comment
SELECT
    C1,
    C2
FROM TBL1;"#
        );
    }

    #[test]
    fn test_get_formatted_sql_single_comment_new_statement_config_newline() {
        let mut config: Configuration = Configuration::new();
        config.newlines = true;
        assert_eq!(
            get_formatted_sql(
                &config,
                String::from(
                    r#"
                    -- comment
                    SELECT
                        C1,
                        C2
                    FROM TBL1;

                    -- comment
                    SELECT
                        C1,
                        C2
                    FROM TBL1;
                    "#,
                )
            ),
            r#"-- comment
SELECT
    C1,
    C2
FROM TBL1;

-- comment
SELECT
    C1,
    C2
FROM TBL1;"#
        );
    }

    #[test]
    fn test_get_formatted_sql_single_comment_before_set() {
        assert_eq!(
            get_formatted_sql(
                &Configuration::new(),
                String::from(
                    r#"
                    SELECT C1
                    FROM TBL1
                    ORDER BY C1

                    -- COMMENT
                    SET V1 = 1
                    "#,
                )
            ),
            r#"SELECT C1
FROM TBL1
ORDER BY C1

-- COMMENT
SET V1 = 1"#
        );
    }

    #[test]
    fn test_get_formatted_sql_single_comment_before_set_config_newline() {
        let mut config: Configuration = Configuration::new();
        config.newlines = true;
        assert_eq!(
            get_formatted_sql(
                &config,
                String::from(
                    r#"
                    SELECT C1
                    FROM TBL1
                    ORDER BY C1

                    -- COMMENT
                    SET V1 = 1
                    "#,
                )
            ),
            r#"SELECT
    C1
FROM TBL1
ORDER BY C1
-- COMMENT
SET V1 = 1"#
        );
    }

    #[test]
    fn test_get_formatted_sql_multiline_comments() {
        assert_eq!(
            get_formatted_sql(
                &Configuration::new(),
                String::from(
                    r#"
                    /* top comment */
                    SELECT C1/* inline comment */
                    /*

                    after

                    comment
                      indent

                    */FROM TBL1
                    "#,
                )
            ),
            r#"/* top comment */
SELECT C1 /* inline comment */
    /*

                    after

                    comment
                      indent

                    */ FROM TBL1"#
        );
    }

    #[test]
    fn test_get_formatted_sql_multiline_comments_config_newline() {
        let mut config: Configuration = Configuration::new();
        config.newlines = true;
        assert_eq!(
            get_formatted_sql(
                &config,
                String::from(
                    r#"
                    /* top comment */
                    SELECT C1/* inline comment */
                    /*

                    after

                    comment
                      indent

                    */FROM TBL1
                    "#,
                )
            ),
            r#"/* top comment */
SELECT
    C1
    /* inline comment */
    /*

                    after

                    comment
                      indent

                    */
FROM TBL1"#
        );
    }

    #[test]
    fn test_get_formatted_sql_select_into() {
        assert_eq!(
            get_formatted_sql(
                &Configuration::new(),
                String::from(
                    r#"
                    SELECT
                    C1,
                    C2,
                    C3
                    INTO TBL2
                    FROM TBL1
                    "#,
                )
            ),
            r#"SELECT
    C1,
    C2,
    C3
INTO TBL2
FROM TBL1"#
        );
    }

    #[test]
    fn test_get_formatted_sql_select_into_config_newline() {
        let mut config: Configuration = Configuration::new();
        config.newlines = true;
        assert_eq!(
            get_formatted_sql(
                &config,
                String::from(
                    r#"
                    SELECT
                    C1,
                    C2,
                    C3
                    INTO TBL2
                    FROM TBL1
                    "#,
                )
            ),
            r#"SELECT
    C1,
    C2,
    C3
INTO
    TBL2
FROM TBL1"#
        );
    }

    #[test]
    fn test_get_formatted_sql_cte_after_select() {
        assert_eq!(
            get_formatted_sql(
                &Configuration::new(),
                String::from(
                    r#"
                    SELECT C1 FROM TBL1
                    WITH CTE2 AS
                    (SELECT C2 FROM TBL2)
                    SELECT * FROM CTE2
                    "#,
                )
            ),
            r#"SELECT C1 FROM TBL1
WITH CTE2 AS
    (SELECT C2 FROM TBL2)
SELECT * FROM CTE2"#
        );
    }

    #[test]
    fn test_get_formatted_sql_cte_after_select_config_newline() {
        let mut config: Configuration = Configuration::new();
        config.newlines = true;
        assert_eq!(
            get_formatted_sql(
                &config,
                String::from(
                    r#"
                    SELECT C1 FROM TBL1
                    WITH CTE2 AS
                    (SELECT C2 FROM TBL2)
                    SELECT * FROM CTE2
                    "#,
                )
            ),
            r#"SELECT
    C1
FROM TBL1
WITH CTE2 AS (SELECT C2 FROM TBL2)
SELECT
    *
FROM CTE2"#
        );
    }

    #[test]
    fn test_get_formatted_sql_insert_after_cte() {
        assert_eq!(
            get_formatted_sql(
                &Configuration::new(),
                String::from(
                    r#"
                    WITH CTE1 AS
                    (SELECT C1 FROM TBL1)
                    INSERT INTO TBL2 (C1)
                    SELECT C1 FROM CTE1
                    "#,
                )
            ),
            r#"WITH CTE1 AS
    (SELECT C1 FROM TBL1)
INSERT INTO TBL2(C1)
SELECT C1 FROM CTE1"#
        );
    }

    #[test]
    fn test_get_formatted_sql_insert_after_cte_config_newline() {
        let mut config: Configuration = Configuration::new();
        config.newlines = true;
        assert_eq!(
            get_formatted_sql(
                &config,
                String::from(
                    r#"
                    WITH CTE1 AS
                    (SELECT C1 FROM TBL1)
                    INSERT INTO TBL2 (C1)
                    SELECT C1 FROM CTE1
                    "#,
                )
            ),
            r#"WITH CTE1 AS (SELECT C1 FROM TBL1)
INSERT INTO TBL2(C1)
SELECT
    C1
FROM CTE1"#
        );
    }

    #[test]
    fn test_get_formatted_sql_insert_after_cte_config_newline_long() {
        let mut config: Configuration = Configuration::new();
        config.newlines = true;
        assert_eq!(
            get_formatted_sql(
                &config,
                String::from(
                    r#"
                    WITH CTE1 AS
                    (SELECT C00000000000000000000000000000,C00000000000000000000000000001,C00000000000000000000000000002 FROM TBL1)
                    INSERT INTO TBL2 (C00000000000000000000000000000,C00000000000000000000000000001,C00000000000000000000000000002)
                    SELECT C00000000000000000000000000000,C00000000000000000000000000001,C00000000000000000000000000002 FROM CTE1
                    "#,
                )
            ),
            r#"WITH CTE1 AS (
        SELECT
            C00000000000000000000000000000,
            C00000000000000000000000000001,
            C00000000000000000000000000002
        FROM TBL1
    )
INSERT INTO TBL2(
    C00000000000000000000000000000,
    C00000000000000000000000000001,
    C00000000000000000000000000002
)
SELECT
    C00000000000000000000000000000,
    C00000000000000000000000000001,
    C00000000000000000000000000002
FROM CTE1"#
        );
    }

    #[test]
    fn test_get_formatted_sql_select_multiple_cte() {
        assert_eq!(
            get_formatted_sql(
                &Configuration::new(),
                String::from(
                    r#"
                    WITH CTE1 AS
                    (SELECT C1 FROM TBL1),
                    CTE2 AS
                    (SELECT C2 FROM TBL2)
                    SELECT * FROM CTE1
                    INNER JOIN CTE2 ON CTE2.C2 = CTE1.C1
                    "#,
                )
            ),
            r#"WITH CTE1 AS
    (SELECT C1 FROM TBL1),
    CTE2 AS
    (SELECT C2 FROM TBL2)
SELECT * FROM CTE1
    INNER JOIN CTE2 ON CTE2.C2 = CTE1.C1"#
        );
    }

    #[test]
    fn test_get_formatted_sql_select_multiple_cte_config_newline() {
        let mut config: Configuration = Configuration::new();
        config.newlines = true;
        assert_eq!(
            get_formatted_sql(
                &config,
                String::from(
                    r#"
                    WITH CTE1 AS
                    (SELECT C1 FROM TBL1),
                    CTE2 AS
                    (SELECT C2 FROM TBL2)
                    SELECT * FROM CTE1
                    INNER JOIN CTE2 ON CTE2.C2 = CTE1.C1
                    "#,
                )
            ),
            r#"WITH CTE1 AS (SELECT C1 FROM TBL1),
    CTE2 AS (SELECT C2 FROM TBL2)
SELECT
    *
FROM CTE1
    INNER JOIN CTE2 ON CTE2.C2 = CTE1.C1"#
        );
    }

    #[test]
    fn test_get_formatted_sql_join_subquery() {
        assert_eq!(
            get_formatted_sql(
                &Configuration::new(),
                String::from(
                    r#"
                    SELECT * FROM T1
                    LEFT JOIN (SELECT C2 FROM T2) AS ST1 ON ST1.C2 = T1.C1
                    "#,
                )
            ),
            r#"SELECT * FROM T1
    LEFT JOIN (SELECT C2 FROM T2) AS ST1 ON ST1.C2 = T1.C1"#
        );
    }

    #[test]
    fn test_get_formatted_sql_join_subquery_config_newline() {
        let mut config: Configuration = Configuration::new();
        config.newlines = true;
        assert_eq!(
            get_formatted_sql(
                &config,
                String::from(
                    r#"
                    SELECT * FROM T1
                    LEFT JOIN (SELECT C2 FROM T2) AS ST1 ON ST1.C2 = T1.C1
                    "#,
                )
            ),
            r#"SELECT
    *
FROM T1
    LEFT JOIN (SELECT C2 FROM T2) AS ST1 ON ST1.C2 = T1.C1"#
        );
    }

    #[test]
    fn test_get_formatted_sql_select_if() {
        assert_eq!(
            get_formatted_sql(
                &Configuration::new(),
                String::from("SELECT IIF(C1>5,1,0) AS C1 FROM TBL1")
            ),
            r#"SELECT IIF(C1 > 5, 1, 0) AS C1 FROM TBL1"#
        );
    }

    #[test]
    fn test_get_formatted_sql_select_if_config_newline() {
        let mut config: Configuration = Configuration::new();
        config.newlines = true;
        assert_eq!(
            get_formatted_sql(
                &config,
                String::from("SELECT IIF(C1>5,1,0) AS C1 FROM TBL1")
            ),
            r#"SELECT
    IIF(C1 > 5, 1, 0) AS C1
FROM TBL1"#
        );
    }

    #[test]
    fn test_get_formatted_sql_case() {
        assert_eq!(
            get_formatted_sql(
                &Configuration::new(),
                String::from(
                    r#"
                    SELECT
                    C1,
                    CASE WHEN C1<=1 THEN 'small'
                    WHEN C1<=3 THEN 'medium'
                    ELSE 'large' END AS C2,
                    C3
                    FROM TBL1
                    "#
                )
            ),
            r#"SELECT
    C1,
    CASE WHEN C1 <= 1 THEN 'small'
        WHEN C1 <= 3 THEN 'medium'
        ELSE 'large' END AS C2,
    C3
FROM TBL1"#
        );
    }

    #[test]
    fn test_get_formatted_sql_case_config_newline() {
        let mut config: Configuration = Configuration::new();
        config.newlines = true;
        assert_eq!(
            get_formatted_sql(
                &config,
                String::from(
                    r#"
                    SELECT
                    C1,
                    CASE WHEN C1<=1 THEN 'small'
                    WHEN C1<=3 THEN 'medium'
                    ELSE 'large' END AS C2,
                    C3
                    FROM TBL1
                    "#
                )
            ),
            r#"SELECT
    C1,
    CASE
        WHEN C1 <= 1 THEN 'small'
        WHEN C1 <= 3 THEN 'medium'
        ELSE 'large'
    END AS C2,
    C3
FROM TBL1"#
        );
    }

    #[test]
    fn test_get_formatted_sql_insert_simple() {
        assert_eq!(
            get_formatted_sql(
                &Configuration::new(),
                String::from("INSERT INTO TBL1(ID)VALUES(1)")
            ),
            r#"INSERT INTO TBL1(ID) VALUES (1)"#
        );
    }

    #[test]
    fn test_get_formatted_sql_insert_simple_config_newline() {
        let mut config: Configuration = Configuration::new();
        config.newlines = true;
        assert_eq!(
            get_formatted_sql(&config, String::from("INSERT INTO TBL1(ID)VALUES(1)")),
            r#"INSERT INTO TBL1(ID)
VALUES (1)"#
        );
    }

    #[test]
    fn test_get_formatted_sql_insert_multiple_columns() {
        assert_eq!(
            get_formatted_sql(
                &Configuration::new(),
                String::from(
                    "INSERT INTO TBL1 (C1,C2,C3,C4,C5,C6,C7,C8,C9,C10,C11,C12,C13,C14,C15,C16,C17,C18,C19,C20,C21) VALUES (1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19,20,21)"
                )
            ),
            r#"INSERT INTO TBL1(C1, C2, C3, C4, C5, C6, C7, C8, C9, C10, C11, C12, C13, C14, C15, C16, C17, C18, C19, C20, C21) VALUES (1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21)"#
        );
    }

    #[test]
    fn test_get_formatted_sql_insert_multiple_columns_config_newline() {
        let mut config: Configuration = Configuration::new();
        config.newlines = true;
        assert_eq!(
            get_formatted_sql(
                &config,
                String::from(
                    "INSERT INTO TBL1 (C1,C2,C3,C4,C5,C6,C7,C8,C9,C10,C11,C12,C13,C14,C15,C16,C17,C18,C19,C20,C21) VALUES (1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19,20,21)"
                )
            ),
            r#"INSERT INTO TBL1(
    C1,
    C2,
    C3,
    C4,
    C5,
    C6,
    C7,
    C8,
    C9,
    C10,
    C11,
    C12,
    C13,
    C14,
    C15,
    C16,
    C17,
    C18,
    C19,
    C20,
    C21
)
VALUES (
    1,
    2,
    3,
    4,
    5,
    6,
    7,
    8,
    9,
    10,
    11,
    12,
    13,
    14,
    15,
    16,
    17,
    18,
    19,
    20,
    21
)"#
        );
    }

    #[test]
    fn test_get_formatted_sql_insert_select() {
        assert_eq!(
            get_formatted_sql(
                &Configuration::new(),
                String::from(
                    r#"
                    INSERT INTO TBL1 (C1,C2,C3)
                    SELECT C1,C2,C3
                    FROM TBL1
                    "#,
                )
            ),
            r#"INSERT INTO TBL1(C1, C2, C3)
SELECT C1, C2, C3
FROM TBL1"#
        );
    }

    #[test]
    fn test_get_formatted_sql_insert_select_config_newline() {
        let mut config: Configuration = Configuration::new();
        config.newlines = true;
        assert_eq!(
            get_formatted_sql(
                &config,
                String::from(
                    r#"
                    INSERT INTO TBL1 (C1,C2,C3)
                    SELECT C1,C2,C3
                    FROM TBL1
                    "#,
                )
            ),
            r#"INSERT INTO TBL1(C1, C2, C3)
SELECT
    C1,
    C2,
    C3
FROM TBL1"#
        );
    }

    #[test]
    fn test_get_formatted_sql_delete_simple() {
        assert_eq!(
            get_formatted_sql(
                &Configuration::new(),
                String::from("DELETE FROM TBL1 WHERE C<=1")
            ),
            r#"DELETE FROM TBL1 WHERE C <= 1"#
        );
    }

    #[test]
    fn test_get_formatted_sql_delete_simple_config_newline() {
        let mut config: Configuration = Configuration::new();
        config.newlines = true;
        assert_eq!(
            get_formatted_sql(&config, String::from("DELETE FROM TBL1 WHERE C<=1")),
            r#"DELETE
FROM TBL1
WHERE C <= 1"#
        );
    }

    #[test]
    fn test_get_formatted_sql_delete_newline() {
        assert_eq!(
            get_formatted_sql(
                &Configuration::new(),
                String::from(
                    r#"
                    DELETE
                    FROM TBL1
                    WHERE C<=1
                    "#
                )
            ),
            r#"DELETE
FROM TBL1
WHERE C <= 1"#
        );
    }

    #[test]
    fn test_get_formatted_sql_delete_newline_config_newline() {
        let mut config: Configuration = Configuration::new();
        config.newlines = true;
        assert_eq!(
            get_formatted_sql(
                &config,
                String::from(
                    r#"
                    DELETE
                    FROM TBL1
                    WHERE C<=1
                    "#
                )
            ),
            r#"DELETE
FROM TBL1
WHERE C <= 1"#
        );
    }

    #[test]
    fn test_get_formatted_sql_truncate_table() {
        assert_eq!(
            get_formatted_sql(
                &Configuration::new(),
                String::from(
                    r#"
                    TRUNCATE TABLE TBL1 TRUNCATE TABLE TBL2
                    TRUNCATE TABLE TBL3
                    "#
                )
            ),
            r#"TRUNCATE TABLE TBL1 TRUNCATE TABLE TBL2
TRUNCATE TABLE TBL3"#
        );
    }

    #[test]
    fn test_get_formatted_sql_truncate_table_config_newline() {
        let mut config: Configuration = Configuration::new();
        config.newlines = true;
        assert_eq!(
            get_formatted_sql(
                &config,
                String::from(
                    r#"
                    TRUNCATE TABLE TBL1 TRUNCATE TABLE TBL2
                    TRUNCATE TABLE TBL3
                    "#
                )
            ),
            r#"TRUNCATE TABLE TBL1
TRUNCATE TABLE TBL2
TRUNCATE TABLE TBL3"#
        );
    }

    #[test]
    fn test_get_formatted_sql_drop_table() {
        assert_eq!(
            get_formatted_sql(
                &Configuration::new(),
                String::from(
                    r#"
                    DROP TABLE TBL1 DROP TABLE TBL2
                    DROP TABLE TBL3
                    "#
                )
            ),
            r#"DROP TABLE TBL1 DROP TABLE TBL2
DROP TABLE TBL3"#
        );
    }

    #[test]
    fn test_get_formatted_sql_drop_table_config_newline() {
        let mut config: Configuration = Configuration::new();
        config.newlines = true;
        assert_eq!(
            get_formatted_sql(
                &config,
                String::from(
                    r#"
                    DROP TABLE TBL1 DROP TABLE TBL2
                    DROP TABLE TBL3
                    "#
                )
            ),
            r#"DROP TABLE TBL1
DROP TABLE TBL2
DROP TABLE TBL3"#
        );
    }

    #[test]
    fn test_get_formatted_sql_execute() {
        assert_eq!(
            get_formatted_sql(
                &Configuration::new(),
                String::from(
                    r#"
                    EXEC SP1();EXEC SP1();
                    EXEC SP1();
                    "#
                )
            ),
            r#"EXEC SP1(); EXEC SP1();
EXEC SP1();"#
        );
    }

    #[test]
    fn test_get_formatted_sql_execute_config_newline() {
        let mut config: Configuration = Configuration::new();
        config.newlines = true;
        assert_eq!(
            get_formatted_sql(
                &config,
                String::from(
                    r#"
                    EXEC SP1();EXEC SP1();
                    EXEC SP1();
                    "#
                )
            ),
            r#"EXEC SP1();
EXEC SP1();
EXEC SP1();"#
        );
    }

    #[test]
    fn test_get_formatted_sql_execute_no_delimiter() {
        assert_eq!(
            get_formatted_sql(
                &Configuration::new(),
                String::from("EXEC SP1() EXEC SP1() EXEC SP1()")
            ),
            r#"EXEC SP1() EXEC SP1() EXEC SP1()"#
        );
    }

    #[test]
    fn test_get_formatted_sql_execute_no_delimiter_config_newline() {
        let mut config: Configuration = Configuration::new();
        config.newlines = true;
        assert_eq!(
            get_formatted_sql(&config, String::from("EXEC SP1() EXEC SP1() EXEC SP1()")),
            r#"EXEC SP1()
EXEC SP1()
EXEC SP1()"#
        );
    }

    #[test]
    fn test_get_formatted_sql_execute_parameters() {
        assert_eq!(
            get_formatted_sql(
                &Configuration::new(),
                String::from("EXEC SP1 P1, P2, P3 EXEC SP1 P1, P2, P3")
            ),
            r#"EXEC SP1 P1, P2, P3 EXEC SP1 P1, P2, P3"#
        );
    }

    #[test]
    fn test_get_formatted_sql_execute_parameters_config_newline() {
        let mut config: Configuration = Configuration::new();
        config.newlines = true;
        assert_eq!(
            get_formatted_sql(
                &config,
                String::from("EXEC SP1 P1, P2, P3 EXEC SP1 P1, P2, P3")
            ),
            r#"EXEC SP1 P1,
    P2,
    P3
EXEC SP1 P1,
    P2,
    P3"#
        );
    }

    #[test]
    fn test_get_formatted_sql_call_no_delimiter() {
        assert_eq!(
            get_formatted_sql(
                &Configuration::new(),
                String::from("CALL SP1() CALL SP1() CALL SP1()")
            ),
            r#"CALL SP1() CALL SP1() CALL SP1()"#
        );
    }

    #[test]
    fn test_get_formatted_sql_call_no_delimiter_config_newline() {
        let mut config: Configuration = Configuration::new();
        config.newlines = true;
        assert_eq!(
            get_formatted_sql(&config, String::from("CALL SP1() CALL SP1() CALL SP1()")),
            r#"CALL SP1()
CALL SP1()
CALL SP1()"#
        );
    }

    #[test]
    fn test_get_formatted_sql_if() {
        assert_eq!(
            get_formatted_sql(
                &Configuration::new(),
                String::from(
                    r#"
                    IF V1 IS NULL AND V2 IS NULL BEGIN SET V1 = 0; SET V2 = 0; END
                    IF V1 IS NULL THEN SET V1 = 0 END IF
                    IF V2 IS NULL SET V2 = 0
                    "#
                )
            ),
            r#"IF V1 IS NULL AND V2 IS NULL BEGIN SET V1 = 0; SET V2 = 0; END
IF V1 IS NULL THEN SET V1 = 0 END IF
IF V2 IS NULL SET V2 = 0"#
        );
    }

    #[test]
    fn test_get_formatted_sql_if_config_newline() {
        let mut config: Configuration = Configuration::new();
        config.newlines = true;
        assert_eq!(
            get_formatted_sql(
                &config,
                String::from(
                    r#"
                    IF V1 IS NULL AND V2 IS NULL BEGIN SET V1 = 0; SET V2 = 0; END
                    IF V1 IS NULL THEN SET V1 = 0 END IF
                    IF V2 IS NULL SET V2 = 0
                    "#
                )
            ),
            r#"IF V1 IS NULL
AND V2 IS NULL
BEGIN
    SET V1 = 0;
    SET V2 = 0;
END
IF V1 IS NULL THEN
    SET V1 = 0
END
IF
IF V2 IS NULL
SET V2 = 0"#
        );
    }

    #[test]
    fn test_get_formatted_sql_if_else() {
        assert_eq!(
            get_formatted_sql(
                &Configuration::new(),
                String::from(
                    r#"
                    IF V1 IS NULL THEN SET V1 = 0
                    ELSE SET V2 = NULL
                    "#
                )
            ),
            r#"IF V1 IS NULL THEN SET V1 = 0
    ELSE SET V2 = NULL"#
        );
    }

    #[test]
    fn test_get_formatted_sql_if_else_config_newline() {
        let mut config: Configuration = Configuration::new();
        config.newlines = true;
        assert_eq!(
            get_formatted_sql(
                &config,
                String::from(
                    r#"
                    IF V1 IS NULL THEN SET V1 = 0
                    ELSE SET V2 = NULL
                    "#
                )
            ),
            r#"IF V1 IS NULL THEN
    SET V1 = 0
    ELSE
    SET V2 = NULL"#
        );
    }

    #[test]
    fn test_get_formatted_sql_if_else_begin_end() {
        assert_eq!(
            get_formatted_sql(
                &Configuration::new(),
                String::from(
                    r#"
                    IF V1 IS NULL BEGIN
                    SET V1 = 0;
                    END
                    ELSE BEGIN
                    SET V2 = NULL;
                    END
                    "#
                )
            ),
            r#"IF V1 IS NULL BEGIN
    SET V1 = 0;
END
ELSE BEGIN
    SET V2 = NULL;
END"#
        );
    }

    #[test]
    fn test_get_formatted_sql_if_else_begin_end_config_newline() {
        let mut config: Configuration = Configuration::new();
        config.newlines = true;
        assert_eq!(
            get_formatted_sql(
                &config,
                String::from(
                    r#"
                    IF V1 IS NULL BEGIN
                    SET V1 = 0;
                    SET V1 = 0;
                    SET V1 = 0;
                    END
                    ELSE BEGIN
                    SET V2 = NULL;
                    END
                    "#
                )
            ),
            r#"IF V1 IS NULL
BEGIN
    SET V1 = 0;
    SET V1 = 0;
    SET V1 = 0;
END
ELSE
BEGIN
    SET V2 = NULL;
END"#
        );
    }

    #[test]
    fn test_get_formatted_sql_try_catch() {
        assert_eq!(
            get_formatted_sql(
                &Configuration::new(),
                String::from(
                    r#"
                    SET V1 = 0;
                    BEGIN TRY
                        CALL SP1;
                    END TRY
                    BEGIN CATCH
                        RETURN 1
                    END CATCH
                    RETURN 0
                    "#
                )
            ),
            r#"SET V1 = 0;
BEGIN TRY
    CALL SP1;
END TRY
BEGIN CATCH
    RETURN 1
END CATCH
RETURN 0"#
        );
    }

    #[test]
    fn test_get_formatted_sql_try_catch_config_newline() {
        let mut config: Configuration = Configuration::new();
        config.newlines = true;
        assert_eq!(
            get_formatted_sql(
                &config,
                String::from(
                    r#"
                    SET V1 = 0;
                    BEGIN TRY
                        CALL SP1;
                    END TRY
                    BEGIN CATCH
                        RETURN 1
                    END CATCH
                    RETURN 0
                    "#
                )
            ),
            r#"SET V1 = 0;

BEGIN TRY
    CALL SP1;
END TRY
BEGIN CATCH
    RETURN 1
END CATCH
RETURN 0"#
        );
    }

    #[test]
    fn test_get_formatted_sql_try_catch_insert() {
        assert_eq!(
            get_formatted_sql(
                &Configuration::new(),
                String::from(
                    r#"
                    SET V1 = 0;
                    BEGIN TRY
                        -- COMMENT
                        INSERT INTO TBL1 (C1) VALUES (1)
                    END TRY
                    BEGIN CATCH
                        RETURN 1
                    END CATCH
                    RETURN 0
                    "#
                )
            ),
            r#"SET V1 = 0;
BEGIN TRY
    -- COMMENT
    INSERT INTO TBL1(C1) VALUES (1)
END TRY
BEGIN CATCH
    RETURN 1
END CATCH
RETURN 0"#
        );
    }

    #[test]
    fn test_get_formatted_sql_try_catch_insert_config_newline() {
        let mut config: Configuration = Configuration::new();
        config.newlines = true;
        assert_eq!(
            get_formatted_sql(
                &config,
                String::from(
                    r#"
                    SET V1 = 0;
                    BEGIN TRY
                        -- COMMENT
                        INSERT INTO TBL1 (C1) VALUES (1)
                    END TRY
                    BEGIN CATCH
                        RETURN 1
                    END CATCH
                    RETURN 0
                    "#
                )
            ),
            r#"SET V1 = 0;

BEGIN TRY
    -- COMMENT
    INSERT INTO TBL1(C1)
    VALUES (1)
END TRY
BEGIN CATCH
    RETURN 1
END CATCH
RETURN 0"#
        );
    }

    #[test]
    fn test_get_formatted_sql_catch_update() {
        assert_eq!(
            get_formatted_sql(
                &Configuration::new(),
                String::from(
                    r#"
                    BEGIN CATCH END CATCH UPDATE TBL1 SET C1 = 1
                    "#
                )
            ),
            r#"BEGIN CATCH END CATCH UPDATE TBL1 SET C1 = 1"#
        );
    }

    #[test]
    fn test_get_formatted_sql_catch_update_config_newline() {
        let mut config: Configuration = Configuration::new();
        config.newlines = true;
        assert_eq!(
            get_formatted_sql(
                &config,
                String::from(
                    r#"
                    BEGIN CATCH END CATCH UPDATE TBL1 SET C1 = 1
                    "#
                )
            ),
            r#"BEGIN CATCH
END CATCH
UPDATE TBL1
SET C1 = 1"#
        );
    }

    #[test]
    fn test_get_formatted_sql_return() {
        assert_eq!(
            get_formatted_sql(
                &Configuration::new(),
                String::from(
                    r#"
                    SELECT *
                    FROM TBL
                    RETURN 0
                    "#
                )
            ),
            r#"SELECT *
FROM TBL
RETURN 0"#
        );
    }

    #[test]
    fn test_get_formatted_sql_return_config_newline() {
        let mut config: Configuration = Configuration::new();
        config.newlines = true;
        assert_eq!(
            get_formatted_sql(
                &config,
                String::from(
                    r#"
                    SELECT *
                    FROM TBL
                    RETURN 0
                    "#
                )
            ),
            r#"SELECT
    *
FROM TBL
RETURN 0"#
        );
    }

    #[test]
    fn test_get_formatted_sql_declare_select() {
        assert_eq!(
            get_formatted_sql(
                &Configuration::new(),
                String::from(
                    r#"
                    DECLARE V1 INT = (
                    SELECT C1
                    FROM TBL
                    );
                    "#
                )
            ),
            r#"DECLARE V1 INT = (
        SELECT C1
        FROM TBL
    );"#
        );
    }

    #[test]
    fn test_get_formatted_sql_declare_select_config_newline() {
        let mut config: Configuration = Configuration::new();
        config.newlines = true;
        assert_eq!(
            get_formatted_sql(
                &config,
                String::from(
                    r#"
                    DECLARE V1 INT = (
                    SELECT C1
                    FROM TBL
                    );
                    "#
                )
            ),
            r#"DECLARE V1 INT = (SELECT C1 FROM TBL);"#
        );
    }

    #[test]
    fn test_get_formatted_sql_xml() {
        assert_eq!(
            get_formatted_sql(
                &Configuration::new(),
                String::from(
                    r#"
                    SELECT C1 AS ID
                    FROM TBL1
                    FOR XML RAW('ITEM'), TYPE, ELEMENTS, ROOT('VALUES'), BINARY BASE64
                    "#
                )
            ),
            r#"SELECT C1 AS ID
FROM TBL1
FOR XML RAW('ITEM'), TYPE, ELEMENTS, ROOT('VALUES'), BINARY BASE64"#
        );
    }

    #[test]
    fn test_get_formatted_sql_xml_config_newline() {
        let mut config: Configuration = Configuration::new();
        config.newlines = true;
        assert_eq!(
            get_formatted_sql(
                &config,
                String::from(
                    r#"
                    SELECT C1 AS ID
                    FROM TBL1
                    FOR XML RAW('ITEM'), TYPE, ELEMENTS, ROOT('VALUES'), BINARY BASE64
                    "#
                )
            ),
            r#"SELECT
    C1 AS ID
FROM TBL1
FOR XML RAW('ITEM'),
    TYPE,
    ELEMENTS,
    ROOT('VALUES'),
    BINARY BASE64"#
        );
    }

    #[test]
    fn test_get_formatted_sql_xml_method() {
        assert_eq!(
            get_formatted_sql(
                &Configuration::new(),
                String::from(
                    r#"
                    SELECT T2.Loc.QUERY('.')
                    FROM T
                    CROSS APPLY Instructions.VALUE('/root/Location') AS T2(Loc)
                    "#
                )
            ),
            r#"SELECT T2.Loc.query('.')
FROM T
    CROSS APPLY Instructions.value('/root/Location') AS T2(Loc)"#
        );
    }

    #[test]
    fn test_get_formatted_sql_xml_method_config_newline() {
        let mut config: Configuration = Configuration::new();
        config.newlines = true;
        config.case = ConfigCase::Uppercase;
        assert_eq!(
            get_formatted_sql(
                &config,
                String::from(
                    r#"
                    select T2.Loc.QUERY('.')
                    from T
                    cross apply Instructions.VALUE('/root/Location') as T2(Loc)
                    "#
                )
            ),
            r#"SELECT
    T2.Loc.query('.')
FROM T
    CROSS APPLY Instructions.value('/root/Location') AS T2(Loc)"#
        );
    }

    #[test]
    fn test_get_formatted_sql_keyword_column_name() {
        assert_eq!(
            get_formatted_sql(
                &Configuration::new(),
                String::from(
                    r#"
                    SELECT
                    T.VALUE AS VALUE, T.[VALUE] AS [VALUE], 'VALUE' AS 'VALUE',
                    t.days as days, t.[days] as [days], 'days' as 'days'
                    FROM TBL1 AS T
                    "#
                )
            ),
            r#"SELECT
    T.VALUE AS VALUE, T.[VALUE] AS [VALUE], 'VALUE' AS 'VALUE',
    t.days as days, t.[days] as [days], 'days' as 'days'
FROM TBL1 AS T"#
        );
    }

    #[test]
    fn test_get_formatted_sql_keyword_column_name_config_newline() {
        let mut config: Configuration = Configuration::new();
        config.newlines = true;
        config.case = ConfigCase::Uppercase;
        assert_eq!(
            get_formatted_sql(
                &config,
                String::from(
                    r#"
                    SELECT
                    T.VALUE AS VALUE, T.[VALUE] AS [VALUE], 'VALUE' AS 'VALUE',
                    t.days as days, t.[days] as [days], 'days' as 'days'
                    FROM TBL1 AS T
                    "#
                )
            ),
            r#"SELECT
    T.VALUE AS VALUE,
    T.[VALUE] AS [VALUE],
    'VALUE' AS 'VALUE',
    t.DAYS AS DAYS,
    t.[days] AS [days],
    'days' AS 'days'
FROM TBL1 AS T"#
        );
    }

    #[test]
    fn test_get_formatted_sql_stuff_comma_list() {
        assert_eq!(
            get_formatted_sql(
                &Configuration::new(),
                String::from(
                    r#"
                    STUFF((SELECT ', ' + C1 FROM TBL1 FOR XML PATH('')), 1, 2, '')
                    "#
                )
            ),
            r#"STUFF((SELECT ', ' + C1 FROM TBL1 FOR XML PATH('')), 1, 2, '')"#
        );
    }

    #[test]
    fn test_get_formatted_sql_stuff_comma_list_config_newline() {
        let mut config: Configuration = Configuration::new();
        config.newlines = true;
        assert_eq!(
            get_formatted_sql(
                &config,
                String::from(
                    r#"
                    STUFF((SELECT ', ' + C1 FROM TBL1 FOR XML PATH('')), 1, 2, '')
                    "#
                )
            ),
            r#"STUFF((SELECT ', ' + C1 FROM TBL1 FOR XML PATH('')), 1, 2, '')"#
        );
    }

    #[test]
    fn test_get_formatted_sql_create_table_simple() {
        assert_eq!(
            get_formatted_sql(
                &Configuration::new(),
                String::from("CREATE TABLE TBL1 (C1 INT)")
            ),
            r#"CREATE TABLE TBL1(C1 INT)"#
        );
    }

    #[test]
    fn test_get_formatted_sql_create_table_simple_config_newline() {
        let mut config: Configuration = Configuration::new();
        config.newlines = true;
        assert_eq!(
            get_formatted_sql(&config, String::from("CREATE TABLE TBL1 (C1 INT)")),
            r#"CREATE TABLE TBL1(C1 INT)"#
        );
    }

    #[test]
    fn test_get_formatted_sql_create_table_varchar() {
        assert_eq!(
            get_formatted_sql(
                &Configuration::new(),
                String::from("CREATE TABLE TBL1 (C1 VARCHAR(10))")
            ),
            r#"CREATE TABLE TBL1(C1 VARCHAR(10))"#
        );
    }

    #[test]
    fn test_get_formatted_sql_create_table_varchar_config_newline() {
        let mut config: Configuration = Configuration::new();
        config.newlines = true;
        assert_eq!(
            get_formatted_sql(&config, String::from("CREATE TABLE TBL1 (C1 VARCHAR(10))")),
            r#"CREATE TABLE TBL1(C1 VARCHAR(10))"#
        );
    }

    #[test]
    fn test_get_formatted_sql_create_table_default() {
        assert_eq!(
            get_formatted_sql(
                &Configuration::new(),
                String::from(
                    r#"
                    CREATE TABLE TBL1 (
                      ID UUID NOT NULL DEFAULT UUID()
                    )
                    "#
                )
            ),
            r#"CREATE TABLE TBL1(
    ID UUID NOT NULL DEFAULT UUID()
)"#
        );
    }

    #[test]
    fn test_get_formatted_sql_create_table_default_config_newline() {
        let mut config: Configuration = Configuration::new();
        config.newlines = true;
        assert_eq!(
            get_formatted_sql(
                &config,
                String::from(
                    r#"
                    CREATE TABLE TBL1 (
                      ID UUID NOT NULL DEFAULT UUID()
                    )
                    "#
                )
            ),
            r#"CREATE TABLE TBL1(ID UUID NOT NULL DEFAULT UUID())"#
        );
    }

    #[test]
    fn test_get_formatted_sql_create_table_complex() {
        assert_eq!(
            get_formatted_sql(
                &Configuration::new(),
                String::from(
                    r#"
                    CREATE TABLE IF NOT EXISTS TBL1 (
                        ID UUID NOT NULL DEFAULT UUID(),
                        C1 VARCHAR(10) NOT NULL,
                        D1 DATETIME NULL,
                        I1 INT,
                        I2 INT, PRIMARY KEY (ID), FOREIGN KEY (I1) REFERENCES TBL2 (ID) ON DELETE CASCADE,
                        FOREIGN KEY (I2) REFERENCES TBL3 (ID) ON DELETE SET NULL
                    )
                    "#
                )
            ),
            r#"CREATE TABLE IF NOT EXISTS TBL1(
    ID UUID NOT NULL DEFAULT UUID(),
    C1 VARCHAR(10) NOT NULL,
    D1 DATETIME NULL,
    I1 INT,
    I2 INT, PRIMARY KEY(ID), FOREIGN KEY(I1) REFERENCES TBL2(ID) ON DELETE CASCADE,
    FOREIGN KEY(I2) REFERENCES TBL3(ID) ON DELETE SET NULL
)"#
        );
    }

    #[test]
    fn test_get_formatted_sql_create_table_complex_config_newline() {
        let mut config: Configuration = Configuration::new();
        config.newlines = true;
        assert_eq!(
            get_formatted_sql(
                &config,
                String::from(
                    r#"
                    CREATE TABLE IF NOT EXISTS TBL1 (
                        ID UUID NOT NULL DEFAULT UUID(),
                        C1 VARCHAR(10) NOT NULL,
                        D1 DATETIME NULL,
                        I1 INT,
                        I2 INT, PRIMARY KEY (ID), FOREIGN KEY (I1) REFERENCES TBL2 (ID) ON DELETE CASCADE,
                        FOREIGN KEY (I2) REFERENCES TBL3 (ID) ON DELETE SET NULL
                    )
                    "#
                )
            ),
            r#"CREATE TABLE IF NOT EXISTS TBL1(
    ID UUID NOT NULL DEFAULT UUID(),
    C1 VARCHAR(10) NOT NULL,
    D1 DATETIME NULL,
    I1 INT,
    I2 INT,
    PRIMARY KEY(ID),
    FOREIGN KEY(I1) REFERENCES TBL2(ID) ON DELETE CASCADE,
    FOREIGN KEY(I2) REFERENCES TBL3(ID) ON DELETE SET NULL
)"#
        );
    }

    #[test]
    fn test_get_formatted_sql_trigger() {
        assert_eq!(
            get_formatted_sql(
                &Configuration::new(),
                String::from(
                    r#"
                    CREATE TRIGGER IF NOT EXISTS TR1
                    AFTER INSERT
                    ON TBL1
                    FOR EACH ROW
                    BEGIN
                    CALL SP1(NEW.ID);
                    END;
                    "#
                )
            ),
            r#"CREATE TRIGGER IF NOT EXISTS TR1
AFTER INSERT
    ON TBL1
FOR EACH ROW
BEGIN
    CALL SP1(NEW.ID);
END;"#
        );
    }

    #[test]
    fn test_get_formatted_sql_trigger_config_newline() {
        let mut config: Configuration = Configuration::new();
        config.newlines = true;
        assert_eq!(
            get_formatted_sql(
                &config,
                String::from(
                    r#"
                    CREATE TRIGGER IF NOT EXISTS TR1
                    AFTER INSERT
                    ON TBL1
                    FOR EACH ROW
                    BEGIN
                    CALL SP1(NEW.ID);
                    END;
                    "#
                )
            ),
            r#"CREATE TRIGGER IF NOT EXISTS TR1
AFTER
INSERT ON TBL1 FOR EACH ROW
BEGIN
    CALL SP1(NEW.ID);
END;"#
        );
    }

    #[test]
    fn test_get_formatted_sql_while_loop() {
        assert_eq!(
            get_formatted_sql(
                &Configuration::new(),
                String::from(
                    r#"
                    DECLARE VAR_COUNT INT;

                    SELECT COUNT(ID)
                    INTO VAR_COUNT
                    FROM TBL1;

                    WHILE VAR_COUNT > 0 DO
                    DELETE FROM TBL1
                    WHERE ID = VAR_COUNT;

                    SELECT COUNT(ID)
                    INTO VAR_COUNT
                    FROM TBL1;
                    END WHILE;
                    "#
                )
            ),
            r#"DECLARE VAR_COUNT INT;

SELECT COUNT(ID)
INTO VAR_COUNT
FROM TBL1;

WHILE VAR_COUNT > 0 DO
        DELETE FROM TBL1
        WHERE ID = VAR_COUNT;

        SELECT COUNT(ID)
        INTO VAR_COUNT
        FROM TBL1;
    END WHILE;"#
        );
    }

    #[test]
    fn test_get_formatted_sql_while_loop_config_newline() {
        let mut config: Configuration = Configuration::new();
        config.newlines = true;
        assert_eq!(
            get_formatted_sql(
                &config,
                String::from(
                    r#"
                    DECLARE VAR_COUNT INT;

                    SELECT COUNT(ID)
                    INTO VAR_COUNT
                    FROM TBL1;

                    WHILE VAR_COUNT > 0 DO
                    DELETE FROM TBL1
                    WHERE ID = VAR_COUNT;

                    SELECT COUNT(ID)
                    INTO VAR_COUNT
                    FROM TBL1;
                    END WHILE;
                    "#
                )
            ),
            r#"DECLARE VAR_COUNT INT;

SELECT
    COUNT(ID)
INTO
    VAR_COUNT
FROM TBL1;

WHILE VAR_COUNT > 0
    DO
        DELETE
        FROM TBL1
        WHERE ID = VAR_COUNT;

        SELECT
            COUNT(ID)
        INTO
            VAR_COUNT
        FROM TBL1;
    END WHILE;"#
        );
    }

    #[test]
    fn test_get_formatted_sql_pivot() {
        assert_eq!(
            get_formatted_sql(
                &Configuration::new(),
                String::from(
                    r#"
                    SELECT 'AverageCost' AS CostSortedByProductionDays,
                    [0],[1],[2],[3],[4]
                    FROM (
                    SELECT DaysToManufacture, StandardCost
                    FROM Production.Product
                    ) AS SourceTable
                    PIVOT (
                    AVG(StandardCost) FOR DaysToManufacture IN
                    ([0],[1],[2],[3],[4])
                    ) AS PivotTable;
                    "#
                )
            ),
            r#"SELECT 'AverageCost' AS CostSortedByProductionDays,
    [0], [1], [2], [3], [4]
FROM (
        SELECT DaysToManufacture, StandardCost
        FROM Production.Product
    ) AS SourceTable
PIVOT (
        AVG(StandardCost) FOR DaysToManufacture IN
            ([0], [1], [2], [3], [4])
    ) AS PivotTable;"#
        );
    }

    #[test]
    fn test_get_formatted_sql_pivot_config_newline() {
        let mut config: Configuration = Configuration::new();
        config.newlines = true;
        assert_eq!(
            get_formatted_sql(
                &config,
                String::from(
                    r#"
                    SELECT 'AverageCost' AS CostSortedByProductionDays,
                    [0],[1],[2],[3],[4]
                    FROM (
                    SELECT DaysToManufacture, StandardCost
                    FROM Production.Product
                    ) AS SourceTable
                    PIVOT (
                    AVG(StandardCost) FOR DaysToManufacture IN
                    ([0],[1],[2],[3],[4])
                    ) AS PivotTable;
                    "#
                )
            ),
            r#"SELECT
    'AverageCost' AS CostSortedByProductionDays,
    [0],
    [1],
    [2],
    [3],
    [4]
FROM (
        SELECT
            DaysToManufacture,
            StandardCost
        FROM Production.Product
    ) AS SourceTable
PIVOT (
        AVG(StandardCost)
        FOR DaysToManufacture IN ([0], [1], [2], [3], [4])
    ) AS PivotTable;"#
        );
    }

    #[test]
    fn test_get_formatted_sql_cursor() {
        assert_eq!(
            get_formatted_sql(
                &Configuration::new(),
                String::from(
                    r#"
                    DECLARE @ID INT, @NAME NVARCHAR(50);

                    DECLARE SAMPLE_CURSOR CURSOR FOR
                    SELECT ID, NAME
                    FROM TBL1;

                    OPEN SAMPLE_CURSOR

                    FETCH NEXT FROM SAMPLE_CURSOR
                    INTO @ID, @NAME

                    WHILE @@FETCH_STATUS = 0
                    BEGIN
                    FETCH NEXT FROM SAMPLE_CURSOR
                    INTO @VENDOR_ID, @VENDOR_NAME
                    END
                    CLOSE SAMPLE_CURSOR;
                    DEALLOCATE SAMPLE_CURSOR;
                    "#,
                ),
            ),
            r#"DECLARE @ID INT, @NAME NVARCHAR(50);

DECLARE SAMPLE_CURSOR CURSOR FOR
SELECT ID, NAME
FROM TBL1;

OPEN SAMPLE_CURSOR

    FETCH NEXT FROM SAMPLE_CURSOR
    INTO @ID, @NAME

    WHILE @@FETCH_STATUS = 0
    BEGIN
        FETCH NEXT FROM SAMPLE_CURSOR
        INTO @VENDOR_ID, @VENDOR_NAME
    END
CLOSE SAMPLE_CURSOR;
DEALLOCATE SAMPLE_CURSOR;"#
        );
    }

    #[test]
    fn test_get_formatted_sql_cursor_config_newline() {
        let mut config: Configuration = Configuration::new();
        config.newlines = true;
        assert_eq!(
            get_formatted_sql(
                &config,
                String::from(
                    r#"
                    DECLARE @ID INT, @NAME NVARCHAR(50);

                    DECLARE SAMPLE_CURSOR CURSOR FOR
                    SELECT ID, NAME
                    FROM TBL1;

                    OPEN SAMPLE_CURSOR

                    FETCH NEXT FROM SAMPLE_CURSOR
                    INTO @ID, @NAME

                    WHILE @@FETCH_STATUS = 0
                    BEGIN
                    FETCH NEXT FROM SAMPLE_CURSOR
                    INTO @VENDOR_ID, @VENDOR_NAME
                    END
                    CLOSE SAMPLE_CURSOR;
                    DEALLOCATE SAMPLE_CURSOR;
                    "#,
                ),
            ),
            r#"DECLARE @ID INT,
    @NAME NVARCHAR(50);

DECLARE SAMPLE_CURSOR CURSOR
FOR
SELECT
    ID,
    NAME
FROM TBL1;

OPEN SAMPLE_CURSOR
    FETCH NEXT
    FROM SAMPLE_CURSOR
    INTO
        @ID,
        @NAME
    WHILE @@FETCH_STATUS = 0
    BEGIN
        FETCH NEXT
        FROM SAMPLE_CURSOR
        INTO
            @VENDOR_ID,
            @VENDOR_NAME
    END
CLOSE SAMPLE_CURSOR;

DEALLOCATE SAMPLE_CURSOR;"#
        );
    }
}
