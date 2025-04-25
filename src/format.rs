use crate::configuration::{ConfigCase, ConfigTab, Configuration};
use crate::token::*;

struct FormatState {
    tokens: Vec<Token>,
    indent_stack: Vec<String>,
}

impl FormatState {
    fn new() -> FormatState {
        FormatState {
            tokens: vec![],
            indent_stack: vec![],
        }
    }

    fn get_indent(&self, config: &Configuration) -> String {
        match config.tabs {
            ConfigTab::Tab => "\t".repeat(self.indent_stack.len()),
            ConfigTab::Space(c) => " ".repeat(c as usize * self.indent_stack.len()),
        }
    }

    fn push(&mut self, token: Token) {
        self.tokens.push(token);
    }

    fn add_pre_space(&mut self, token: &Token, config: &Configuration) {
        if token.category == Some(TokenCategory::NewLine)
            || token.category == Some(TokenCategory::Delimiter)
            || token.category == Some(TokenCategory::Comma)
        {
            return;
        }

        let prev_token_category: Option<TokenCategory> = if self.tokens.len() > 0 {
            let prev_token: &Token = self.tokens.last().unwrap();
            prev_token.category.clone()
        } else {
            None
        };

        if prev_token_category == Some(TokenCategory::NewLine) {
            self.push(Token::new_space(self.get_indent(config)));
            return;
        }

        if token.category == Some(TokenCategory::ParenClose)
            || prev_token_category == Some(TokenCategory::ParenOpen)
        {
            return;
        }

        match token.category {
            Some(TokenCategory::Compare) | Some(TokenCategory::Bitwise) => {
                self.push(Token::new_space(String::from(" ")));
                return;
            }
            Some(TokenCategory::ParenOpen) => {
                if prev_token_category == Some(TokenCategory::DataType)
                    || prev_token_category == Some(TokenCategory::Method)
                {
                    return;
                }
            }
            _ => (),
        }

        self.push(Token::new_space(String::from(" ")));
    }

    fn increase_indent_stack(&mut self, token: &Token) {
        let token_value: String = token.value.to_uppercase();
        match token_value.as_str() {
            "SELECT" | "INSERT" | "DELETE" | "UPDATE" | "FROM" | "WHERE" | "ORDER" | "GROUP"
            | "HAVING" | "CASE" | "BEGIN" | "INTO" | "SET" | "VALUE" | "VALUES" | "WHILE"
            | "WITH" | "DO" | "(" => {
                self.indent_stack.push(token_value);
            }
            "THEN" => {
                if self.indent_stack.last() != Some(&String::from("CASE")) {
                    self.indent_stack.push(token_value);
                }
            }
            _ => (),
        }
    }

    fn decrease_indent_stack(&mut self, token: &Token) {
        let token_value: String = token.value.to_uppercase();
        match token_value.as_str() {
            ")" => self.decrease_indent_stack_until(token_value, vec!["("]),
            "END" => self.decrease_indent_stack_until(token_value, vec!["BEGIN", "CASE", "THEN"]),
            "INTO" => self.decrease_indent_stack_until(token_value, vec!["SELECT", "INSERT"]),
            "SET" => self.decrease_indent_stack_until(token_value, vec!["UPDATE"]),
            "VALUE" | "VALUES" => self.decrease_indent_stack_until(token_value, vec!["INTO"]),
            "SELECT" | "INSERT" | "UPDATE" | "DELETE" | "CALL" => self.decrease_indent_stack_until(
                token_value,
                vec![
                    "SELECT", "INSERT", "UPDATE", "DELETE", "FROM", "WHERE", "GROUP", "HAVING",
                    "WITH", "WHILE", "SET",
                ],
            ),
            "FROM" => self.decrease_indent_stack_until(
                token_value,
                vec!["SELECT", "DELETE", "UPDATE", "INTO"],
            ),
            "WHERE" | "ORDER" | "GROUP" | "HAVING" | "LIMIT" | "WHILE" => {
                self.decrease_indent_stack_until(token_value, vec!["FROM"])
            }
            _ => (),
        }
    }

    fn decrease_indent_stack_until(&mut self, token_value: String, find_values: Vec<&str>) {
        loop {
            let top: Option<String> = self.indent_stack.pop();
            if top.is_none() {
                break;
            }

            let top: String = top.unwrap();

            if top == "(" && token_value != ")" {
                self.indent_stack.push(top);
                break;
            }

            if (top == "BEGIN" || top == "DO" || top == "THEN") && token_value != "END" {
                self.indent_stack.push(top);
                break;
            }

            if find_values.contains(&top.as_str()) {
                break;
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

pub fn get_formatted_sql(config: &Configuration, sql: String) -> String {
    let mut state: FormatState = FormatState::new();

    let tokens: Vec<Token> = get_sql_tokens(sql);
    for i in 0..tokens.len() {
        let token: &Token = &tokens[i];
        state.decrease_indent_stack(token);
        state.add_pre_space(token, config);
        state.push(token.clone());
        state.increase_indent_stack(token);
    }

    return state.get_result(config);
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
    fn test_get_formatted_sql_multiple_columns_newlines() {
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
    fn test_get_formatted_sql_sub_query_inline() {
        assert_eq!(
            get_formatted_sql(
                &Configuration::new(),
                String::from(
                    r#"
                    SELECT ( SELECT TOP 1 ID FROM TBL1 ) AS ID
                "#
                )
            ),
            r#"SELECT (SELECT TOP 1 ID FROM TBL1) AS ID"#
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
                    INNER JOIN TBL2 AS T2 ON T2.C1 = T1.C1
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
    fn test_get_formatted_sql_single_comments() {
        assert_eq!(
            get_formatted_sql(
                &Configuration::new(),
                String::from(
                    r#"
                    -- top comment
                    SELECT C1--inline comment
                    -- after comment
                    FROM TBL1
                "#,
                )
            ),
            r#"-- top comment
SELECT C1 --inline comment
    -- after comment
FROM TBL1"#
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
    PRIMARY KEY (ID)
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
        CALL SP1 (NEW.ID);
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
}
