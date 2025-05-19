use crate::configuration::{ConfigCase, ConfigTab, Configuration};
use crate::token::*;

pub fn get_formatted_sql(config: &Configuration, sql: String) -> String {
    let mut state: FormatState = FormatState::new();

    let tokens: Vec<Token> = get_sql_tokens(sql);
    for i in 0..tokens.len() {
        let token: &Token = &tokens[i];

        if config.newlines {
            if token.category == Some(TokenCategory::NewLine) {
                continue;
            }
        }

        if token.category == Some(TokenCategory::ParenOpen) {
            for p in 0..3 {
                if let Some(t) = state.tokens.iter().nth_back(p) {
                    if t.category == Some(TokenCategory::Method)
                        || t.category == Some(TokenCategory::DataType)
                    {
                        state.in_method = true;
                        break;
                    }
                }
            }
        }

        let mut next_keyword_token: Option<&Token> = None;
        if token.category == Some(TokenCategory::Comment) {
            for n in i + 1..tokens.len() {
                if tokens[n].category == Some(TokenCategory::Keyword) {
                    next_keyword_token = Some(&tokens[n]);
                    break;
                }
            }
        }

        state.decrease_indent_stack(
            token,
            tokens.get(i + 1),
            tokens.get(i + 2),
            next_keyword_token,
        );
        state.add_pre_space(token, config);
        state.push(token.clone());
        state.increase_indent_stack(token);

        if token.category == Some(TokenCategory::ParenClose) {
            state.in_method = false;
        }
    }

    return state.get_result(config);
}

struct FormatState {
    tokens: Vec<Token>,
    indent_stack: Vec<String>,
    in_method: bool,
}

impl FormatState {
    fn new() -> FormatState {
        FormatState {
            tokens: vec![],
            indent_stack: vec![],
            in_method: false,
        }
    }

    fn push(&mut self, token: Token) {
        self.tokens.push(token);
    }

    fn get_last_line_indent(&self, config: &Configuration) -> usize {
        for i in (1..self.tokens.len()).rev() {
            if self.tokens[i].category == Some(TokenCategory::Space) {
                if self.tokens[i - 1].category == Some(TokenCategory::NewLine) {
                    return match config.tabs {
                        ConfigTab::Tab => self.tokens[i].value.len(),
                        ConfigTab::Space(c) => self.tokens[i].value.len() / c as usize,
                    };
                }
            }
        }
        return 0;
    }

    fn add_pre_space(&mut self, token: &Token, config: &Configuration) {
        if self.tokens.is_empty() {
            return;
        }

        if config.newlines {
            self.remove_extra_newline(token);
            self.add_pre_newline(token);
        }

        if token.category == Some(TokenCategory::NewLine)
            || token.category == Some(TokenCategory::Delimiter)
        {
            return;
        }

        let prev_token: &Token = self
            .tokens
            .last()
            .expect("should always have a previous token");

        match prev_token.category {
            Some(TokenCategory::NewLine) => {
                // if last two tokens are new lines
                if self
                    .tokens
                    .iter()
                    .nth_back(1)
                    .is_some_and(|t| t.category == Some(TokenCategory::NewLine))
                {
                    // if current line indent is less than previous line indent
                    if self.indent_stack.len() < self.get_last_line_indent(config) {
                        // remove one new line
                        self.tokens.pop();
                    }
                }
                self.push(Token::new_space(match config.tabs {
                    ConfigTab::Tab => "\t".repeat(self.indent_stack.len()),
                    ConfigTab::Space(c) => " ".repeat(c as usize * self.indent_stack.len()),
                }));
                return;
            }
            Some(TokenCategory::ParenOpen) => return,
            _ => (),
        }

        match token.category {
            Some(TokenCategory::Comma) => return,
            Some(TokenCategory::ParenClose) => return,
            Some(TokenCategory::ParenOpen) => {
                if self.in_method {
                    return;
                }
            }
            _ => (),
        }

        self.push(Token::new_space(String::from(" ")));
    }

    fn add_pre_newline(&mut self, token: &Token) {
        if self.tokens.is_empty() {
            return;
        }

        let prev_token: &Token = self
            .tokens
            .last()
            .expect("should always have a previous token");

        match prev_token.category {
            Some(TokenCategory::Delimiter) => {
                self.push(Token::newline());
                self.push(Token::newline());
                return;
            }
            Some(TokenCategory::Comment) => {
                self.push(Token::newline());
                return;
            }
            Some(TokenCategory::ParenOpen) | Some(TokenCategory::Comma) => {
                if !self.in_method {
                    self.push(Token::newline());
                }
                return;
            }
            _ => (),
        }

        match prev_token.value.to_uppercase().as_str() {
            "CASE" | "DISTINCT" | "UNION" | "DO" => {
                self.push(Token::newline());
                return;
            }
            "BEGIN" => match token.value.to_uppercase().as_str() {
                "TRY" | "CATCH" => return,
                _ => {
                    self.push(Token::newline());
                    return;
                }
            },
            "SELECT" => match token.value.to_uppercase().as_str() {
                "DISTINCT" | "TOP" => return,
                _ => {
                    self.push(Token::newline());
                    return;
                }
            },
            "INTO" => {
                if let Some(prev2_token) = self.tokens.iter().nth_back(1) {
                    match prev2_token.value.to_uppercase().as_str() {
                        "INSERT" => (),
                        _ => {
                            self.push(Token::newline());
                            return;
                        }
                    }
                }
            }
            _ => (),
        }

        if let Some(prev2_token) = self.tokens.iter().nth_back(2) {
            match prev2_token.value.to_uppercase().as_str() {
                "TOP" => {
                    self.push(Token::newline());
                    return;
                }
                _ => (),
            }
        }

        match &token.category {
            Some(TokenCategory::Comment) => {
                self.push(Token::newline());
                return;
            }
            Some(TokenCategory::ParenClose) => {
                if !self.in_method {
                    self.push(Token::newline());
                }
                return;
            }
            _ => (),
        }

        match token.value.to_uppercase().as_str() {
            "AFTER" | "AND" | "BEFORE" | "BEGIN" | "CALL" | "CASE" | "CLOSE" | "CROSS"
            | "DECLARE" | "DO" | "ELSE" | "END" | "EXEC" | "EXECUTE" | "FETCH" | "FOR" | "FROM"
            | "GROUP" | "INNER" | "LEFT" | "LIMIT" | "UNION" | "OPEN" | "OR" | "ORDER"
            | "OUTER" | "PRIMARY" | "RETURN" | "RIGHT" | "SELECT" | "SET" | "WHEN" | "WHERE" => {
                self.push(Token::newline());
                return;
            }
            "WHILE" => {
                if prev_token.value.to_uppercase().as_str() != "END" {
                    self.push(Token::newline());
                    return;
                }
            }
            "INTO" => {
                if prev_token.value.to_uppercase().as_str() != "INSERT" {
                    self.push(Token::newline());
                    return;
                }
            }
            _ => (),
        }
    }

    fn remove_extra_newline(&mut self, token: &Token) {
        if token.category == Some(TokenCategory::Delimiter) {
            for i in (1..self.tokens.len()).rev() {
                if self.tokens[i].category == Some(TokenCategory::NewLine) {
                    if self.tokens[i - 1].category == Some(TokenCategory::NewLine) {
                        self.tokens.remove(i);
                    }
                    break;
                }
            }
        }
    }

    fn increase_indent_stack(&mut self, token: &Token) {
        let token_value: String = token.value.to_uppercase();
        if match token_value.as_str() {
            "SELECT" | "INSERT" | "DELETE" | "UPDATE" | "FROM" | "JOIN" | "WHERE" | "ORDER"
            | "GROUP" | "HAVING" | "CASE" | "BEGIN" | "OPEN" | "INTO" | "DECLARE" | "SET"
            | "VALUE" | "VALUES" | "WHILE" | "WITH" | "ELSE" | "DO" | "(" => true,
            "THEN" => self.indent_stack.last() != Some(&String::from("CASE")),
            _ => false,
        } {
            self.indent_stack.push(token_value);
        }
    }

    fn decrease_indent_stack(
        &mut self,
        token: &Token,
        next1_token: Option<&Token>,
        next2_token: Option<&Token>,
        next_keyword_token: Option<&Token>,
    ) {
        if self.indent_stack.is_empty() {
            return;
        }

        let top_of_stack: &String = self.indent_stack.last().unwrap();
        let decrease_if_found: Vec<&str> = vec!["JOIN"];
        if decrease_if_found.contains(&top_of_stack.as_str()) {
            if &token.value.to_uppercase() == top_of_stack
                || next1_token.is_some_and(|t| &t.value.to_uppercase() == top_of_stack)
                || next2_token.is_some_and(|t| &t.value.to_uppercase() == top_of_stack)
            {
                self.indent_stack.pop();
                return;
            }
        }

        if token.category == Some(TokenCategory::Comment) {
            let decrease_comment_keywords: Vec<&str> = vec![
                "SELECT", "INSERT", "UPDATE", "DELETE", "UNION", "WITH", "WHILE",
            ];
            if next_keyword_token.is_some_and(|nkt| {
                decrease_comment_keywords.contains(&nkt.value.to_uppercase().as_str())
            }) {
                self.indent_stack.pop();
                return;
            }
        }

        let token_value: String = token.value.to_uppercase();
        let decrease_until_match: Vec<&str> = match token_value.as_str() {
            ")" => vec!["("],
            "CLOSE" => vec!["OPEN"],
            "END" => vec!["BEGIN", "CASE", "THEN", "ELSE"],
            "INTO" => vec!["SELECT", "INSERT"],
            "SET" => vec!["UPDATE"],
            "ELSE" => vec!["THEN", "CASE"],
            "VALUE" | "VALUES" => vec!["INTO"],
            "SELECT" | "INSERT" | "UPDATE" | "DELETE" | "UNION" | "CALL" | "EXECUTE" | "EXEC"
            | "DECLARE" | "IF" | "PIVOT" | "OPEN" => {
                vec![
                    "SELECT", "INSERT", "UPDATE", "DELETE", "FROM", "WHERE", "GROUP", "HAVING",
                    "UNION", "WITH", "WHILE", "SET", "PIVOT",
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
                let top: Option<String> = self.indent_stack.pop();
                if top.is_none() {
                    break;
                }
                let top: String = top.unwrap();

                let required_to_decrease: Vec<&str> = match top.as_str() {
                    "(" => vec![")"],
                    "OPEN" => vec!["CLOSE"],
                    "BEGIN" => vec!["END"],
                    "DO" => vec!["END"],
                    "THEN" => vec!["END", "ELSE"],
                    "ELSE" => vec!["END"],
                    _ => vec![],
                };

                if !required_to_decrease.is_empty()
                    && !required_to_decrease.contains(&token_value.as_str())
                {
                    self.indent_stack.push(top);
                    break;
                }

                if decrease_until_match.contains(&top.as_str()) {
                    break;
                }
            }
        }
    }

    fn get_result(&self, config: &Configuration) -> String {
        let mut result: String = String::new();
        for token in &self.tokens {
            let mut token_value: String = token.value.clone();

            match token.category {
                Some(TokenCategory::Keyword)
                | Some(TokenCategory::DataType)
                | Some(TokenCategory::Method) => match config.case {
                    ConfigCase::Uppercase => token_value = token_value.to_uppercase(),
                    ConfigCase::Lowercase => token_value = token_value.to_lowercase(),
                    ConfigCase::Unchanged => (),
                },
                _ => (),
            }

            result.push_str(token_value.as_str());
        }
        return result.trim().to_string();
    }
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
    (
        SELECT TOP 1
            ID
        FROM TBL1
    ) AS ID"#
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
    (
        SELECT TOP 1
            ID
        FROM TBL1
    ) AS ID,
    C1
FROM TBL1"#
        );
    }

    #[test]
    fn test_get_formatted_sql_delimiter() {
        assert_eq!(
            get_formatted_sql(
                &Configuration::new(),
                String::from("DECLARE C1 = 1;DECLARE C2 = 2;  DECLARE C3 = 3;")
            ),
            r#"DECLARE C1 = 1; DECLARE C2 = 2; DECLARE C3 = 3;"#
        );
    }

    #[test]
    fn test_get_formatted_sql_delimiter_config_newline() {
        let mut config: Configuration = Configuration::new();
        config.newlines = true;
        assert_eq!(
            get_formatted_sql(
                &config,
                String::from("DECLARE C1 = 1;DECLARE C2 = 2;  DECLARE C3 = 3;")
            ),
            r#"DECLARE C1 = 1;
DECLARE C2 = 2;
DECLARE C3 = 3;"#
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
WHERE C1 IN (
        1,
        2,
        3
    )"#
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
                    "#,
                )
            ),
            r#"SELECT C1,
    COUNT(*) AS CNT
FROM TBL1
GROUP BY C1"#
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
                    "#,
                )
            ),
            r#"SELECT
    C1,
    COUNT(*) AS CNT
FROM TBL1
GROUP BY C1"#
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
            r#"SELECT DISTINCT
    T1.C1 AS C1,
    T2.C2 AS C2,
    T3.C3 AS C3
FROM TBL1 AS T1
    INNER JOIN TBL2 AS T2 ON T2.C1 = T1.C1
        AND T2.C2 = T1.C2
    INNER JOIN TBL3 AS T3 ON T3.C2 = T2.C2
WHERE (
        T1.C2 <> T2.C2
        OR T1.C2 <> T3.C2
    )
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
            r#"WITH CTE1 AS (
        SELECT
            C1
        FROM TBL1
    ),
    CTE2 AS (
        SELECT
            C2
        FROM TBL2
    )
SELECT
    *
FROM CTE1
    INNER JOIN CTE2 ON CTE2.C2 = CTE1.C1"#
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
            r#"INSERT INTO TBL1 (ID) VALUES (1)"#
        );
    }

    #[test]
    fn test_get_formatted_sql_insert_simple_config_newline() {
        let mut config: Configuration = Configuration::new();
        config.newlines = true;
        assert_eq!(
            get_formatted_sql(&config, String::from("INSERT INTO TBL1(ID)VALUES(1)")),
            r#"INSERT INTO
    TBL1 (
        ID
    ) VALUES (
        1
    )"#
        );
    }

    #[test]
    fn test_get_formatted_sql_insert_multiple_columns() {
        assert_eq!(
            get_formatted_sql(
                &Configuration::new(),
                String::from("INSERT INTO TBL1 (C1,C2,C3) VALUES (1,2,3)")
            ),
            r#"INSERT INTO TBL1 (C1, C2, C3) VALUES (1, 2, 3)"#
        );
    }

    #[test]
    fn test_get_formatted_sql_insert_multiple_columns_config_newline() {
        let mut config: Configuration = Configuration::new();
        config.newlines = true;
        assert_eq!(
            get_formatted_sql(
                &config,
                String::from("INSERT INTO TBL1 (C1,C2,C3) VALUES (1,2,3)")
            ),
            r#"INSERT INTO
    TBL1 (
        C1,
        C2,
        C3
    ) VALUES (
        1,
        2,
        3
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
            r#"INSERT INTO TBL1 (C1, C2, C3)
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
            r#"INSERT INTO
    TBL1 (
        C1,
        C2,
        C3
    )
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
    fn test_get_formatted_sql_try_catch() {
        assert_eq!(
            get_formatted_sql(
                &Configuration::new(),
                String::from(
                    r#"
                    BEGIN TRY
                        CALL SP1
                    END TRY
                    BEGIN CATCH
                        RETURN 1
                    END CATCH
                    RETURN 0
                    "#
                )
            ),
            r#"BEGIN TRY
    CALL SP1
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
            r#"BEGIN TRY
    CALL SP1;
END TRY
BEGIN CATCH
    RETURN 1
END CATCH
RETURN 0"#
        );
    }

    #[test]
    fn test_get_formatted_sql_create_table_simple() {
        assert_eq!(
            get_formatted_sql(
                &Configuration::new(),
                String::from("CREATE TABLE TBL1 (C1 INT)")
            ),
            r#"CREATE TABLE TBL1 (C1 INT)"#
        );
    }

    #[test]
    fn test_get_formatted_sql_create_table_simple_config_newline() {
        let mut config: Configuration = Configuration::new();
        config.newlines = true;
        assert_eq!(
            get_formatted_sql(&config, String::from("CREATE TABLE TBL1 (C1 INT)")),
            r#"CREATE TABLE TBL1 (
    C1 INT
)"#
        );
    }

    #[test]
    fn test_get_formatted_sql_create_table_varchar() {
        assert_eq!(
            get_formatted_sql(
                &Configuration::new(),
                String::from("CREATE TABLE TBL1 (C1 VARCHAR(10))")
            ),
            r#"CREATE TABLE TBL1 (C1 VARCHAR(10))"#
        );
    }

    #[test]
    fn test_get_formatted_sql_create_table_varchar_config_newline() {
        let mut config: Configuration = Configuration::new();
        config.newlines = true;
        assert_eq!(
            get_formatted_sql(&config, String::from("CREATE TABLE TBL1 (C1 VARCHAR(10))")),
            r#"CREATE TABLE TBL1 (
    C1 VARCHAR(10)
)"#
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
            r#"CREATE TABLE TBL1 (
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
            r#"CREATE TABLE TBL1 (
    ID UUID NOT NULL DEFAULT UUID()
)"#
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
                        PRIMARY KEY (ID)
                    )
                    "#
                )
            ),
            r#"CREATE TABLE IF NOT EXISTS TBL1 (
    ID UUID NOT NULL DEFAULT UUID(),
    C1 VARCHAR(10) NOT NULL,
    D1 DATETIME NULL,
    I1 INT,
    PRIMARY KEY(ID)
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
                        PRIMARY KEY (ID)
                    )
                    "#
                )
            ),
            r#"CREATE TABLE IF NOT EXISTS TBL1 (
    ID UUID NOT NULL DEFAULT UUID(),
    C1 VARCHAR(10) NOT NULL,
    D1 DATETIME NULL,
    I1 INT,
    PRIMARY KEY(ID)
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
AFTER INSERT ON TBL1
    FOR EACH ROW
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
    ) AS SourceTable PIVOT (
    AVG(StandardCost)
    FOR DaysToManufacture IN (
        [0],
        [1],
        [2],
        [3],
        [4]
    )
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
