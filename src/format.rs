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

    fn get_indent(&self) -> String {
        " ".repeat(INDENT_SIZE * self.indent_stack.len())
    }

    fn get_indent_stack(&self, token_value: String) -> Vec<String> {
        let mut new_indent_stack: Vec<String> = self.indent_stack.clone();
        match token_value.as_str() {
            "SELECT" => {
                new_indent_stack.push(token_value);
            }
            "FROM" => loop {
                let top: Option<String> = new_indent_stack.pop();
                if top.is_none() {
                    break;
                }

                let top: String = top.unwrap();
                let top: &str = top.as_str();
                if top == "SELECT" {
                    break;
                }
            },
            _ => (),
        }
        return new_indent_stack;
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

        state.indent_stack = state.get_indent_stack(token.value.clone());

        let pre_space: Option<String> = get_pre_space(&state, token);
        if pre_space.is_some() {
            state.result_builder.push(pre_space.unwrap());
        }

        state.result_builder.push(token.value.clone());
    }

    return state.get_result();
}

fn get_pre_space(state: &FormatState, token: &Token) -> Option<String> {
    match token.category {
        Some(TokenCategory::NewLine)
        | Some(TokenCategory::Delimiter)
        | Some(TokenCategory::Comma) => {
            return None;
        }
        Some(TokenCategory::Operator)
        | Some(TokenCategory::Compare)
        | Some(TokenCategory::Bitwise) => {
            return Some(String::from(" "));
        }
        _ => (),
    }

    if state.is_line_start() {
        return Some(state.get_indent());
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
}
