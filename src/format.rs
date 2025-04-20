use crate::token::*;

const INDENT_SIZE: usize = 4;

struct FormatState {
    result_builder: Vec<String>,
    indent_stack: Vec<String>,
}

impl FormatState {
    fn new() -> FormatState {
        FormatState {
            result_builder: vec![],
            indent_stack: vec![],
        }
    }

    fn is_line_start(&self) -> bool {
        self.result_builder.last() == Some(&String::from("\n"))
    }

    fn is_paren_start(&self) -> bool {
        self.result_builder.last() == Some(&String::from("("))
    }

    fn get_indent(&self) -> String {
        " ".repeat(INDENT_SIZE * self.indent_stack.len())
    }

    fn push(&mut self, token_value: String) {
        self.result_builder.push(token_value);
    }

    fn increase_indent_stack(&mut self, token_value: String) {
        match token_value.as_str() {
            "SELECT" | "FROM" | "WHERE" | "(" => {
                self.indent_stack.push(token_value);
            }
            _ => (),
        }
    }

    fn decrease_indent_stack(&mut self, token_value: String) {
        match token_value.as_str() {
            ")" => self.decrease_indent_stack_until(vec!["("]),
            "FROM" => self.decrease_indent_stack_until(vec!["SELECT"]),
            "WHERE" => self.decrease_indent_stack_until(vec!["FROM"]),
            _ => (),
        }
    }

    fn decrease_indent_stack_until(&mut self, find_values: Vec<&str>) {
        loop {
            let top: Option<String> = self.indent_stack.pop();
            if top.is_none() {
                break;
            }
            let top: String = top.unwrap();
            if find_values.contains(&top.as_str()) {
                break;
            }
        }
    }

    fn get_result(&self) -> String {
        self.result_builder.concat().trim().to_string()
    }
}

pub fn get_formatted_sql(sql: String) -> String {
    let mut state: FormatState = FormatState::new();

    let tokens: Vec<Token> = get_sql_tokens(sql);
    for i in 0..tokens.len() {
        let token: &Token = &tokens[i];

        state.decrease_indent_stack(token.value.clone());

        let pre_space: Option<String> = get_pre_space(&state, token);
        if pre_space.is_some() {
            state.push(pre_space.unwrap());
        }

        state.push(token.value.clone());

        state.increase_indent_stack(token.value.clone());
    }

    return state.get_result();
}

fn get_pre_space(state: &FormatState, token: &Token) -> Option<String> {
    match token.category {
        Some(TokenCategory::NewLine)
        | Some(TokenCategory::Delimiter)
        | Some(TokenCategory::Comma) => return None,
        Some(TokenCategory::Operator)
        | Some(TokenCategory::Compare)
        | Some(TokenCategory::Bitwise) => return Some(String::from(" ")),
        _ => (),
    }

    if state.is_line_start() {
        return Some(state.get_indent());
    }

    if state.is_paren_start() {
        return None;
    }

    match token.category {
        Some(TokenCategory::ParenClose) => return None,
        _ => (),
    }

    return Some(String::from(" "));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_formatted_sql_basic() {
        assert_eq!(
            String::from("SELECT * FROM TBL1"),
            get_formatted_sql(String::from("SELECT * FROM TBL1"))
        );
    }

    #[test]
    fn test_get_formatted_sql_basic_newlines() {
        assert_eq!(
            String::from(
                r#"SELECT *
FROM TBL1"#
            ),
            get_formatted_sql(String::from(
                r#"
                    SELECT  *
                    FROM  TBL1
                "#
            ))
        );
    }

    #[test]
    fn test_get_formatted_sql_multiple_columns() {
        assert_eq!(
            String::from(
                r#"SELECT
    C1 AS 'Column 1',
    C2 AS 'Column 2',
    C3
FROM TBL1 AS T"#
            ),
            get_formatted_sql(String::from(
                r#"
                    SELECT
                    C1 AS 'Column 1',
                    C2 AS 'Column 2',
                    C3
                    FROM TBL1 AS T
                "#
            ))
        );
    }

    #[test]
    fn test_get_formatted_sql_sub_query_inline() {
        assert_eq!(
            String::from(r#"SELECT (SELECT TOP 1 ID FROM TBL1) AS ID"#),
            get_formatted_sql(String::from(
                r#"
                    SELECT ( SELECT TOP 1 ID FROM TBL1 ) AS ID
                "#
            ))
        );
    }

    #[test]
    fn test_get_formatted_sql_sub_query_multiline() {
        assert_eq!(
            String::from(
                r#"SELECT (
        SELECT TOP 1 ID FROM TBL1
    ) AS ID"#
            ),
            get_formatted_sql(String::from(
                r#"
                    SELECT (
                    SELECT TOP 1 ID FROM TBL1
                    ) AS ID
                "#
            ))
        );
    }

    #[test]
    fn test_get_formatted_sql_join() {
        assert_eq!(
            String::from(
                r#"SELECT T1.C1, T1.C2,
    T2.C2
FROM TBL1 AS T1
    INNER JOIN TBL2 AS T2 ON T2.C1 = T1.C1"#
            ),
            get_formatted_sql(String::from(
                r#"
                    SELECT T1.C1, T1.C2,
                    T2.C2
                    FROM TBL1 AS T1
                    INNER JOIN TBL2 AS T2 ON T2.C1 = T1.C1
                "#
            ))
        );
    }

    #[test]
    fn test_get_formatted_sql_select_where() {
        assert_eq!(
            String::from(
                r#"SELECT
    C1,
    C2,
    C3
FROM TBL1
WHERE C1 > 1
    AND C2 IS NOT NULL"#
            ),
            get_formatted_sql(String::from(
                r#"
                    SELECT
                    C1,
                    C2,
                    C3
                    FROM TBL1
                    WHERE C1>1
                    AND C2 IS NOT NULL
                "#
            ))
        );
    }
}
