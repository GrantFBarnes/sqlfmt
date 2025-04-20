use crate::token::*;

pub fn get_formatted_sql(sql: String) -> String {
    let mut result_builder: Vec<String> = vec![];

    let tokens: Vec<Token> = get_sql_tokens(sql);
    for i in 0..tokens.len() {
        let pre_space: Option<String> = get_pre_space(&tokens, i);
        if pre_space.is_some() {
            result_builder.push(pre_space.unwrap());
        }
        result_builder.push(tokens[i].value.clone());
    }

    return result_builder.concat().trim().to_string();
}

fn get_pre_space(tokens: &Vec<Token>, curr_idx: usize) -> Option<String> {
    let token: &Token = &tokens[curr_idx];

    if curr_idx == 0 || token.category == Some(TokenCategory::NewLine) {
        return None;
    }

    let prev_token: &Token = &tokens[curr_idx - 1];

    if prev_token.category == Some(TokenCategory::NewLine) {
        return None;
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
}
