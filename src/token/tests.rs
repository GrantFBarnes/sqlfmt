use super::*;

#[test]
fn test_decode_encode_raw_bytes_round_trip() {
    let bytes: Vec<u8> = b"INSERT INTO T (B) VALUES ('\x89PNG\r\n\xff\xfe')".to_vec();
    let decoded: String = decode_input(&bytes);
    assert_eq!(decoded.chars().filter(|c| is_raw_byte(*c)).count(), 3);
    assert_eq!(encode_output(&decoded), bytes);
}

#[test]
fn test_decode_encode_raw_bytes_and_private_use_unicode() {
    let unicode: String = (RAW_BYTE_BASE..=RAW_BYTE_BASE + 0xFF)
        .map(|code_point| char::from_u32(code_point).unwrap())
        .collect();
    assert_eq!(
        encode_output(&decode_input(unicode.as_bytes())),
        unicode.as_bytes()
    );

    let mut bytes: Vec<u8> = unicode.as_bytes().to_vec();
    bytes.extend(0..=u8::MAX);
    bytes.extend_from_slice(unicode.as_bytes());
    bytes.extend_from_slice(b"\xf0\x9f");
    assert_eq!(encode_output(&decode_input(&bytes)), bytes);
}

#[test]
fn test_decode_valid_utf8_unchanged() {
    let sql: &str = "SELECT 'héllo' FROM TBL1";
    assert_eq!(decode_input(sql.as_bytes()), sql);
    assert_eq!(encode_output(sql), sql.as_bytes());
}

#[test]
fn test_get_sql_tokens_raw_bytes_in_quote() {
    let sql: String = decode_input(b"SELECT '\r\x89PNG\r\n\xff\x00\r'");
    let tokens: Vec<Token> = get_sql_tokens(&Configuration::new(), sql.clone());
    assert_eq!(tokens.len(), 3);
    assert_eq!(tokens[2].category, TokenCategory::Quote);
    assert_eq!(
        encode_output(&tokens[2].value),
        b"'\r\x89PNG\r\n\xff\x00\r'"
    );
}

#[test]
fn test_get_sql_tokens_raw_bytes_outside_quote() {
    let sql: String = decode_input(b"SELECT \xff\xfe,1");
    let tokens: Vec<Token> = get_sql_tokens(&Configuration::new(), sql.clone());
    assert_eq!(tokens.len(), 5);
    assert_eq!(tokens[2].category, TokenCategory::Identifier);
    assert_eq!(encode_output(&tokens[2].value), b"\xff\xfe");
    assert_eq!(tokens[3], Token::new_test(",", TokenCategory::Comma));
    assert_eq!(tokens[4], Token::new_test("1", TokenCategory::Identifier));
}

#[test]
fn test_get_sql_tokens_carriage_returns_in_quotes() {
    for quoted in [
        "'\rfirst\r\nlast\r'",
        "N'\rfirst\r\nlast\r'",
        "'first''\rlast'",
        "\"\rfirst\r\nlast\r\"",
        "`\rfirst\r\nlast\r`",
        "[\rfirst\r\nlast\r]",
    ] {
        let sql: String = format!("SELECT\r\n{quoted}\r\nFROM\r\nt;");
        assert_eq!(
            get_sql_tokens(&Configuration::new(), sql),
            vec![
                Token::new_test("SELECT", TokenCategory::Keyword),
                Token::new_test("\n", TokenCategory::NewLine),
                Token::new_test(quoted, TokenCategory::Quote),
                Token::new_test("\n", TokenCategory::NewLine),
                Token::new_test("FROM", TokenCategory::Keyword),
                Token::new_test("\n", TokenCategory::NewLine),
                Token::new_test("t", TokenCategory::Identifier),
                Token::new_test(";", TokenCategory::Delimiter),
            ],
            "{quoted:?}"
        );
    }
}

#[test]
fn test_get_sql_tokens_crlf_outside_quotes() {
    let config: Configuration = Configuration::new();
    for sql in [
        "SELECT -- comment\r\nvalue FROM t;\r\n",
        "SELECT /* first\r\nlast */\r\nvalue FROM t;\r\n",
        "SELECT %s, {column}\r\nFROM t;\r\n",
        "DELIMITER $$\r\nSELECT 1$$\r\n",
    ] {
        assert_eq!(
            get_sql_tokens(&config, sql.to_string()),
            get_sql_tokens(&config, sql.replace("\r\n", "\n")),
            "{sql:?}"
        );
    }
}

#[test]
fn test_get_sql_tokens_basic() {
    assert_eq!(
        get_sql_tokens(&Configuration::new(), String::from("SELECT * FROM TBL1")),
        vec![
            Token::new_test("SELECT", TokenCategory::Keyword),
            Token::new_test(" ", TokenCategory::WhiteSpace),
            Token::new_test("*", TokenCategory::Operator),
            Token::new_test(" ", TokenCategory::WhiteSpace),
            Token::new_test("FROM", TokenCategory::Keyword),
            Token::new_test(" ", TokenCategory::WhiteSpace),
            Token::new_test("TBL1", TokenCategory::Identifier),
        ]
    );
}

#[test]
fn test_get_sql_tokens_count() {
    assert_eq!(
        get_sql_tokens(
            &Configuration::new(),
            String::from("SELECT COUNT(*) FROM TBL1")
        ),
        vec![
            Token::new_test("SELECT", TokenCategory::Keyword),
            Token::new_test(" ", TokenCategory::WhiteSpace),
            Token::new_test("COUNT", TokenCategory::Method),
            Token::new_test("(", TokenCategory::ParenOpen),
            Token::new_test("*", TokenCategory::Operator),
            Token::new_test(")", TokenCategory::ParenClose),
            Token::new_test(" ", TokenCategory::WhiteSpace),
            Token::new_test("FROM", TokenCategory::Keyword),
            Token::new_test(" ", TokenCategory::WhiteSpace),
            Token::new_test("TBL1", TokenCategory::Identifier),
        ]
    );
}

#[test]
fn test_get_sql_tokens_alias() {
    assert_eq!(
        get_sql_tokens(
            &Configuration::new(),
            String::from("SELECT T.* FROM TBL1 T")
        ),
        vec![
            Token::new_test("SELECT", TokenCategory::Keyword),
            Token::new_test(" ", TokenCategory::WhiteSpace),
            Token::new_test("T", TokenCategory::Identifier),
            Token::new_test(".", TokenCategory::FullStop),
            Token::new_test("*", TokenCategory::Operator),
            Token::new_test(" ", TokenCategory::WhiteSpace),
            Token::new_test("FROM", TokenCategory::Keyword),
            Token::new_test(" ", TokenCategory::WhiteSpace),
            Token::new_test("TBL1", TokenCategory::Identifier),
            Token::new_test(" ", TokenCategory::WhiteSpace),
            Token::new_test("T", TokenCategory::Identifier),
        ]
    );
}

#[test]
fn test_get_sql_tokens_comment_single_inline() {
    assert_eq!(
        get_sql_tokens(
            &Configuration::new(),
            String::from("SELECT 1 --comment inline")
        ),
        vec![
            Token::new_test("SELECT", TokenCategory::Keyword),
            Token::new_test(" ", TokenCategory::WhiteSpace),
            Token::new_test("1", TokenCategory::Identifier),
            Token::new_test(" ", TokenCategory::WhiteSpace),
            Token::new_test("--comment inline", TokenCategory::Comment),
        ]
    );
}

#[test]
fn test_get_sql_tokens_comment_single_newline() {
    assert_eq!(
        get_sql_tokens(
            &Configuration::new(),
            String::from(
                r#"SELECT *
                -- comment newline
                FROM TBL1"#
            )
        ),
        vec![
            Token::new_test("SELECT", TokenCategory::Keyword),
            Token::new_test(" ", TokenCategory::WhiteSpace),
            Token::new_test("*", TokenCategory::Operator),
            Token::new_test("\n", TokenCategory::NewLine),
            Token::new_test("                ", TokenCategory::WhiteSpace),
            Token::new_test("-- comment newline", TokenCategory::Comment),
            Token::new_test("\n", TokenCategory::NewLine),
            Token::new_test("                ", TokenCategory::WhiteSpace),
            Token::new_test("FROM", TokenCategory::Keyword),
            Token::new_test(" ", TokenCategory::WhiteSpace),
            Token::new_test("TBL1", TokenCategory::Identifier),
        ]
    );
}

#[test]
fn test_get_sql_tokens_comment_multi_inline() {
    assert_eq!(
        get_sql_tokens(
            &Configuration::new(),
            String::from("SELECT * /*multi inline*/ FROM TBL1")
        ),
        vec![
            Token::new_test("SELECT", TokenCategory::Keyword),
            Token::new_test(" ", TokenCategory::WhiteSpace),
            Token::new_test("*", TokenCategory::Operator),
            Token::new_test(" ", TokenCategory::WhiteSpace),
            Token::new_test("/*multi inline*/", TokenCategory::Comment),
            Token::new_test(" ", TokenCategory::WhiteSpace),
            Token::new_test("FROM", TokenCategory::Keyword),
            Token::new_test(" ", TokenCategory::WhiteSpace),
            Token::new_test("TBL1", TokenCategory::Identifier),
        ]
    );
}

#[test]
fn test_get_sql_tokens_comment_multi_odd() {
    assert_eq!(
        get_sql_tokens(&Configuration::new(), String::from("*/*multi odd*/*")),
        vec![
            Token::new_test("*", TokenCategory::Operator),
            Token::new_test("/*multi odd*/", TokenCategory::Comment),
            Token::new_test("*", TokenCategory::Operator),
        ]
    );
}

#[test]
fn test_get_sql_tokens_comment_multi_slash_after_open() {
    assert_eq!(
        get_sql_tokens(&Configuration::new(), String::from("/*/ multi */ A")),
        vec![
            Token::new_test("/*/ multi */", TokenCategory::Comment),
            Token::new_test(" ", TokenCategory::WhiteSpace),
            Token::new_test("A", TokenCategory::Identifier),
        ]
    );
}

#[test]
fn test_get_sql_tokens_comment_multi_empty() {
    assert_eq!(
        get_sql_tokens(&Configuration::new(), String::from("/**/ A")),
        vec![
            Token::new_test("/**/", TokenCategory::Comment),
            Token::new_test(" ", TokenCategory::WhiteSpace),
            Token::new_test("A", TokenCategory::Identifier),
        ]
    );
}

#[test]
fn test_get_sql_tokens_comment_multi_double() {
    assert_eq!(
        get_sql_tokens(
            &Configuration::new(),
            String::from("/*multi double*//*multi double*/")
        ),
        vec![Token::new_test(
            "/*multi double*//*multi double*/",
            TokenCategory::Comment
        )],
    );
}

#[test]
fn test_get_sql_tokens_comment_multi_newline() {
    assert_eq!(
        get_sql_tokens(
            &Configuration::new(),
            String::from(
                r#"SELECT *
                /*
                    multi line
                    comment
                */
                FROM TBL1"#
            )
        ),
        vec![
            Token::new_test("SELECT", TokenCategory::Keyword),
            Token::new_test(" ", TokenCategory::WhiteSpace),
            Token::new_test("*", TokenCategory::Operator),
            Token::new_test("\n", TokenCategory::NewLine),
            Token::new_test("                ", TokenCategory::WhiteSpace),
            Token::new_test(
                r#"/*
                    multi line
                    comment
                */"#,
                TokenCategory::Comment
            ),
            Token::new_test("\n", TokenCategory::NewLine),
            Token::new_test("                ", TokenCategory::WhiteSpace),
            Token::new_test("FROM", TokenCategory::Keyword),
            Token::new_test(" ", TokenCategory::WhiteSpace),
            Token::new_test("TBL1", TokenCategory::Identifier),
        ]
    );
}

#[test]
fn test_get_sql_tokens_quote_backtick() {
    assert_eq!(
        get_sql_tokens(&Configuration::new(), String::from("SELECT `Column 1`")),
        vec![
            Token::new_test("SELECT", TokenCategory::Keyword),
            Token::new_test(" ", TokenCategory::WhiteSpace),
            Token::new_test("`Column 1`", TokenCategory::Quote),
        ]
    );
}

#[test]
fn test_get_sql_tokens_quote_single() {
    assert_eq!(
        get_sql_tokens(&Configuration::new(), String::from("SELECT 'Column 1'")),
        vec![
            Token::new_test("SELECT", TokenCategory::Keyword),
            Token::new_test(" ", TokenCategory::WhiteSpace),
            Token::new_test("'Column 1'", TokenCategory::Quote),
        ]
    );
}

#[test]
fn test_get_sql_tokens_quote_double() {
    assert_eq!(
        get_sql_tokens(&Configuration::new(), String::from("SELECT \"Column 1\"")),
        vec![
            Token::new_test("SELECT", TokenCategory::Keyword),
            Token::new_test(" ", TokenCategory::WhiteSpace),
            Token::new_test("\"Column 1\"", TokenCategory::Quote),
        ]
    );
}

#[test]
fn test_get_sql_tokens_quote_bracket() {
    assert_eq!(
        get_sql_tokens(&Configuration::new(), String::from("SELECT [Column 1]")),
        vec![
            Token::new_test("SELECT", TokenCategory::Keyword),
            Token::new_test(" ", TokenCategory::WhiteSpace),
            Token::new_test("[Column 1]", TokenCategory::Quote),
        ]
    );
}

#[test]
fn test_get_sql_tokens_quote_bracket_datatype() {
    assert_eq!(
        get_sql_tokens(&Configuration::new(), String::from("[NVARCHAR]")),
        vec![Token::new_test("[NVARCHAR]", TokenCategory::DataType)],
    );
    assert_eq!(
        get_sql_tokens(&Configuration::new(), String::from("[nvarchar]")),
        vec![Token::new_test("[nvarchar]", TokenCategory::DataType)],
    );
}

#[test]
fn test_get_sql_tokens_quote_bracket_schema() {
    assert_eq!(
        get_sql_tokens(
            &Configuration::new(),
            String::from("SELECT * FROM [S].[TBL1]")
        ),
        vec![
            Token::new_test("SELECT", TokenCategory::Keyword),
            Token::new_test(" ", TokenCategory::WhiteSpace),
            Token::new_test("*", TokenCategory::Operator),
            Token::new_test(" ", TokenCategory::WhiteSpace),
            Token::new_test("FROM", TokenCategory::Keyword),
            Token::new_test(" ", TokenCategory::WhiteSpace),
            Token::new_test("[S]", TokenCategory::Quote),
            Token::new_test(".", TokenCategory::FullStop),
            Token::new_test("[TBL1]", TokenCategory::Quote),
        ]
    );
}

#[test]
fn test_get_sql_tokens_quote_bracket_column() {
    assert_eq!(
        get_sql_tokens(&Configuration::new(), String::from("SELECT TBL1.[C1]")),
        vec![
            Token::new_test("SELECT", TokenCategory::Keyword),
            Token::new_test(" ", TokenCategory::WhiteSpace),
            Token::new_test("TBL1", TokenCategory::Identifier),
            Token::new_test(".", TokenCategory::FullStop),
            Token::new_test("[C1]", TokenCategory::Quote),
        ]
    );
}

#[test]
fn test_get_sql_tokens_interpolation_bracket() {
    assert_eq!(
        get_sql_tokens(&Configuration::new(), String::from("{value}")),
        vec![Token::new_test("{value}", TokenCategory::Interpolation)],
    );
}

#[test]
fn test_get_sql_tokens_interpolation_percent() {
    assert_eq!(
        get_sql_tokens(&Configuration::new(), String::from("%T")),
        vec![Token::new_test("%T", TokenCategory::Interpolation)],
    );
}

#[test]
fn test_get_sql_tokens_interpolation_bracket_in_quote() {
    assert_eq!(
        get_sql_tokens(&Configuration::new(), String::from("'{value}'")),
        vec![Token::new_test("'{value}'", TokenCategory::Quote)],
    );
}

#[test]
fn test_get_sql_tokens_interpolation_percent_in_quote() {
    assert_eq!(
        get_sql_tokens(&Configuration::new(), String::from("'%%'")),
        vec![Token::new_test("'%%'", TokenCategory::Quote)],
    );
}

#[test]
fn test_get_sql_tokens_interpolation_complex_in_quote() {
    assert_eq!(
        get_sql_tokens(
            &Configuration::new(),
            String::from(r#"'{string.Join("', '", value)}'"#)
        ),
        vec![
            Token::new_test(r#"'{string.Join("'"#, TokenCategory::Quote),
            Token::new_test(",", TokenCategory::Comma),
            Token::new_test(" ", TokenCategory::WhiteSpace),
            Token::new_test(r#"'", value)}'"#, TokenCategory::Quote)
        ],
    );
}

#[test]
fn test_get_sql_tokens_interpolation_table_name_bracket() {
    assert_eq!(
        get_sql_tokens(
            &Configuration::new(),
            String::from("SELECT C1 FROM {tableNames[i]} AS T")
        ),
        vec![
            Token::new_test("SELECT", TokenCategory::Keyword),
            Token::new_test(" ", TokenCategory::WhiteSpace),
            Token::new_test("C1", TokenCategory::Identifier),
            Token::new_test(" ", TokenCategory::WhiteSpace),
            Token::new_test("FROM", TokenCategory::Keyword),
            Token::new_test(" ", TokenCategory::WhiteSpace),
            Token::new_test("{tableNames[i]}", TokenCategory::Interpolation),
            Token::new_test(" ", TokenCategory::WhiteSpace),
            Token::new_test("AS", TokenCategory::Keyword),
            Token::new_test(" ", TokenCategory::WhiteSpace),
            Token::new_test("T", TokenCategory::Identifier),
        ]
    );
}

#[test]
fn test_get_sql_tokens_interpolation_table_name_percent() {
    assert_eq!(
        get_sql_tokens(
            &Configuration::new(),
            String::from("SELECT C1 FROM %v AS T")
        ),
        vec![
            Token::new_test("SELECT", TokenCategory::Keyword),
            Token::new_test(" ", TokenCategory::WhiteSpace),
            Token::new_test("C1", TokenCategory::Identifier),
            Token::new_test(" ", TokenCategory::WhiteSpace),
            Token::new_test("FROM", TokenCategory::Keyword),
            Token::new_test(" ", TokenCategory::WhiteSpace),
            Token::new_test("%v", TokenCategory::Interpolation),
            Token::new_test(" ", TokenCategory::WhiteSpace),
            Token::new_test("AS", TokenCategory::Keyword),
            Token::new_test(" ", TokenCategory::WhiteSpace),
            Token::new_test("T", TokenCategory::Identifier),
        ]
    );
}

#[test]
fn test_get_sql_tokens_interpolation_table_name_part_bracket() {
    assert_eq!(
        get_sql_tokens(
            &Configuration::new(),
            String::from("SELECT C1 FROM B{tableNames[i]}E AS T")
        ),
        vec![
            Token::new_test("SELECT", TokenCategory::Keyword),
            Token::new_test(" ", TokenCategory::WhiteSpace),
            Token::new_test("C1", TokenCategory::Identifier),
            Token::new_test(" ", TokenCategory::WhiteSpace),
            Token::new_test("FROM", TokenCategory::Keyword),
            Token::new_test(" ", TokenCategory::WhiteSpace),
            Token::new_test("B", TokenCategory::Identifier),
            Token::new_test("{tableNames[i]}", TokenCategory::Interpolation),
            Token::new_test("E", TokenCategory::Identifier),
            Token::new_test(" ", TokenCategory::WhiteSpace),
            Token::new_test("AS", TokenCategory::Keyword),
            Token::new_test(" ", TokenCategory::WhiteSpace),
            Token::new_test("T", TokenCategory::Identifier),
        ]
    );
}

#[test]
fn test_get_sql_tokens_interpolation_table_name_part_percent() {
    assert_eq!(
        get_sql_tokens(
            &Configuration::new(),
            String::from("SELECT C1 FROM B%vE AS T")
        ),
        vec![
            Token::new_test("SELECT", TokenCategory::Keyword),
            Token::new_test(" ", TokenCategory::WhiteSpace),
            Token::new_test("C1", TokenCategory::Identifier),
            Token::new_test(" ", TokenCategory::WhiteSpace),
            Token::new_test("FROM", TokenCategory::Keyword),
            Token::new_test(" ", TokenCategory::WhiteSpace),
            Token::new_test("B", TokenCategory::Identifier),
            Token::new_test("%v", TokenCategory::Interpolation),
            Token::new_test("E", TokenCategory::Identifier),
            Token::new_test(" ", TokenCategory::WhiteSpace),
            Token::new_test("AS", TokenCategory::Keyword),
            Token::new_test(" ", TokenCategory::WhiteSpace),
            Token::new_test("T", TokenCategory::Identifier),
        ]
    );
}

#[test]
fn test_get_sql_tokens_interpolation_procedure_name_bracket() {
    assert_eq!(
        get_sql_tokens(
            &Configuration::new(),
            String::from("CALL SCH.{procedureName}();")
        ),
        vec![
            Token::new_test("CALL", TokenCategory::Keyword),
            Token::new_test(" ", TokenCategory::WhiteSpace),
            Token::new_test("SCH", TokenCategory::Identifier),
            Token::new_test(".", TokenCategory::FullStop),
            Token::new_test("{procedureName}", TokenCategory::Interpolation),
            Token::new_test("(", TokenCategory::ParenOpen),
            Token::new_test(")", TokenCategory::ParenClose),
            Token::new_test(";", TokenCategory::Delimiter),
        ]
    );
}

#[test]
fn test_get_sql_tokens_interpolation_procedure_name_percent() {
    assert_eq!(
        get_sql_tokens(&Configuration::new(), String::from("CALL SCH.%s();")),
        vec![
            Token::new_test("CALL", TokenCategory::Keyword),
            Token::new_test(" ", TokenCategory::WhiteSpace),
            Token::new_test("SCH", TokenCategory::Identifier),
            Token::new_test(".", TokenCategory::FullStop),
            Token::new_test("%s", TokenCategory::Interpolation),
            Token::new_test("(", TokenCategory::ParenOpen),
            Token::new_test(")", TokenCategory::ParenClose),
            Token::new_test(";", TokenCategory::Delimiter),
        ]
    );
}

#[test]
fn test_get_sql_tokens_quote_empty() {
    assert_eq!(
        get_sql_tokens(&Configuration::new(), String::from("DECLARE V1 = '';")),
        vec![
            Token::new_test("DECLARE", TokenCategory::Keyword),
            Token::new_test(" ", TokenCategory::WhiteSpace),
            Token::new_test("V1", TokenCategory::Identifier),
            Token::new_test(" ", TokenCategory::WhiteSpace),
            Token::new_test("=", TokenCategory::Compare),
            Token::new_test(" ", TokenCategory::WhiteSpace),
            Token::new_test("''", TokenCategory::Quote),
            Token::new_test(";", TokenCategory::Delimiter),
        ]
    );
}

#[test]
fn test_get_sql_tokens_quote_of_empty() {
    assert_eq!(
        get_sql_tokens(&Configuration::new(), String::from("DECLARE V1 = '''';")),
        vec![
            Token::new_test("DECLARE", TokenCategory::Keyword),
            Token::new_test(" ", TokenCategory::WhiteSpace),
            Token::new_test("V1", TokenCategory::Identifier),
            Token::new_test(" ", TokenCategory::WhiteSpace),
            Token::new_test("=", TokenCategory::Compare),
            Token::new_test(" ", TokenCategory::WhiteSpace),
            Token::new_test("''''", TokenCategory::Quote),
            Token::new_test(";", TokenCategory::Delimiter),
        ]
    );
}

#[test]
fn test_get_sql_tokens_quote_single_n() {
    assert_eq!(
        get_sql_tokens(&Configuration::new(), String::from("SELECT N'Column Name'")),
        vec![
            Token::new_test("SELECT", TokenCategory::Keyword),
            Token::new_test(" ", TokenCategory::WhiteSpace),
            Token::new_test("N'Column Name'", TokenCategory::Quote),
        ]
    );
}

#[test]
fn test_get_sql_tokens_quote_single_prefixes() {
    assert_eq!(
        get_sql_tokens(
            &Configuration::new(),
            String::from("n'a',X'0A',x'ff',B'1',b'0',e'c'")
        ),
        vec![
            Token::new_test("n'a'", TokenCategory::Quote),
            Token::new_test(",", TokenCategory::Comma),
            Token::new_test("X'0A'", TokenCategory::Quote),
            Token::new_test(",", TokenCategory::Comma),
            Token::new_test("x'ff'", TokenCategory::Quote),
            Token::new_test(",", TokenCategory::Comma),
            Token::new_test("B'1'", TokenCategory::Quote),
            Token::new_test(",", TokenCategory::Comma),
            Token::new_test("b'0'", TokenCategory::Quote),
            Token::new_test(",", TokenCategory::Comma),
            Token::new_test("e'c'", TokenCategory::Quote),
        ]
    );
}

#[test]
fn test_get_sql_tokens_quote_single_prefix_letter_ends_word() {
    assert_eq!(
        get_sql_tokens(
            &Configuration::new(),
            String::from("WHEN'a' THEN'b' ELSE'c' LIKE'd'")
        ),
        vec![
            Token::new_test("WHEN", TokenCategory::Keyword),
            Token::new_test("'a'", TokenCategory::Quote),
            Token::new_test(" ", TokenCategory::WhiteSpace),
            Token::new_test("THEN", TokenCategory::Keyword),
            Token::new_test("'b'", TokenCategory::Quote),
            Token::new_test(" ", TokenCategory::WhiteSpace),
            Token::new_test("ELSE", TokenCategory::Keyword),
            Token::new_test("'c'", TokenCategory::Quote),
            Token::new_test(" ", TokenCategory::WhiteSpace),
            Token::new_test("LIKE", TokenCategory::Keyword),
            Token::new_test("'d'", TokenCategory::Quote),
        ]
    );
}

#[test]
fn test_get_sql_tokens_quote_single_e_prefix_backslash_escape() {
    assert_eq!(
        get_sql_tokens(&Configuration::new(), String::from(r"E'a\'b' 'c\'d'")),
        vec![
            Token::new_test(r"E'a\'b'", TokenCategory::Quote),
            Token::new_test(" ", TokenCategory::WhiteSpace),
            Token::new_test(r"'c\'", TokenCategory::Quote),
            Token::new_test("d", TokenCategory::Identifier),
            Token::new_test("'", TokenCategory::Quote),
        ]
    );
}

#[test]
fn test_get_sql_tokens_quote_single_escape() {
    assert_eq!(
        get_sql_tokens(
            &Configuration::new(),
            String::from("SELECT 'Column''s Name'")
        ),
        vec![
            Token::new_test("SELECT", TokenCategory::Keyword),
            Token::new_test(" ", TokenCategory::WhiteSpace),
            Token::new_test("'Column''s Name'", TokenCategory::Quote),
        ]
    );
}

#[test]
fn test_get_sql_tokens_quote_double_escape() {
    assert_eq!(
        get_sql_tokens(
            &Configuration::new(),
            String::from(r#"SELECT "Column""s Name" FROM t"#)
        ),
        vec![
            Token::new_test("SELECT", TokenCategory::Keyword),
            Token::new_test(" ", TokenCategory::WhiteSpace),
            Token::new_test(r#""Column""s Name""#, TokenCategory::Quote),
            Token::new_test(" ", TokenCategory::WhiteSpace),
            Token::new_test("FROM", TokenCategory::Keyword),
            Token::new_test(" ", TokenCategory::WhiteSpace),
            Token::new_test("t", TokenCategory::Identifier),
        ]
    );
}

#[test]
fn test_get_sql_tokens_quote_double_escape_at_end() {
    assert_eq!(
        get_sql_tokens(
            &Configuration::new(),
            String::from(r#"SELECT "Name""" FROM t"#)
        ),
        vec![
            Token::new_test("SELECT", TokenCategory::Keyword),
            Token::new_test(" ", TokenCategory::WhiteSpace),
            Token::new_test(r#""Name""""#, TokenCategory::Quote),
            Token::new_test(" ", TokenCategory::WhiteSpace),
            Token::new_test("FROM", TokenCategory::Keyword),
            Token::new_test(" ", TokenCategory::WhiteSpace),
            Token::new_test("t", TokenCategory::Identifier),
        ]
    );
}

#[test]
fn test_get_sql_tokens_quote_backtick_escape() {
    assert_eq!(
        get_sql_tokens(
            &Configuration::new(),
            String::from(r#"SELECT `Column``s Name` FROM t"#)
        ),
        vec![
            Token::new_test("SELECT", TokenCategory::Keyword),
            Token::new_test(" ", TokenCategory::WhiteSpace),
            Token::new_test(r#"`Column``s Name`"#, TokenCategory::Quote),
            Token::new_test(" ", TokenCategory::WhiteSpace),
            Token::new_test("FROM", TokenCategory::Keyword),
            Token::new_test(" ", TokenCategory::WhiteSpace),
            Token::new_test("t", TokenCategory::Identifier),
        ]
    );
}

#[test]
fn test_get_sql_tokens_quote_backtick_escape_at_end() {
    assert_eq!(
        get_sql_tokens(
            &Configuration::new(),
            String::from(r#"SELECT `Name``` FROM t"#)
        ),
        vec![
            Token::new_test("SELECT", TokenCategory::Keyword),
            Token::new_test(" ", TokenCategory::WhiteSpace),
            Token::new_test(r#"`Name```"#, TokenCategory::Quote),
            Token::new_test(" ", TokenCategory::WhiteSpace),
            Token::new_test("FROM", TokenCategory::Keyword),
            Token::new_test(" ", TokenCategory::WhiteSpace),
            Token::new_test("t", TokenCategory::Identifier),
        ]
    );
}

#[test]
fn test_get_sql_tokens_quote_bracket_escape() {
    assert_eq!(
        get_sql_tokens(
            &Configuration::new(),
            String::from(r#"SELECT [Column]]s Name] FROM t"#)
        ),
        vec![
            Token::new_test("SELECT", TokenCategory::Keyword),
            Token::new_test(" ", TokenCategory::WhiteSpace),
            Token::new_test(r#"[Column]]s Name]"#, TokenCategory::Quote),
            Token::new_test(" ", TokenCategory::WhiteSpace),
            Token::new_test("FROM", TokenCategory::Keyword),
            Token::new_test(" ", TokenCategory::WhiteSpace),
            Token::new_test("t", TokenCategory::Identifier),
        ]
    );
}

#[test]
fn test_get_sql_tokens_quote_bracket_escape_at_end() {
    assert_eq!(
        get_sql_tokens(
            &Configuration::new(),
            String::from(r#"SELECT [Name]]] FROM t"#)
        ),
        vec![
            Token::new_test("SELECT", TokenCategory::Keyword),
            Token::new_test(" ", TokenCategory::WhiteSpace),
            Token::new_test(r#"[Name]]]"#, TokenCategory::Quote),
            Token::new_test(" ", TokenCategory::WhiteSpace),
            Token::new_test("FROM", TokenCategory::Keyword),
            Token::new_test(" ", TokenCategory::WhiteSpace),
            Token::new_test("t", TokenCategory::Identifier),
        ]
    );
}

#[test]
fn test_get_sql_tokens_quote_adjacent_close() {
    assert_eq!(
        get_sql_tokens(&Configuration::new(), String::from(r#"SELECT "a",[b],`c`"#)),
        vec![
            Token::new_test("SELECT", TokenCategory::Keyword),
            Token::new_test(" ", TokenCategory::WhiteSpace),
            Token::new_test(r#""a""#, TokenCategory::Quote),
            Token::new_test(",", TokenCategory::Comma),
            Token::new_test("[b]", TokenCategory::Quote),
            Token::new_test(",", TokenCategory::Comma),
            Token::new_test("`c`", TokenCategory::Quote),
        ]
    );
}

#[test]
fn test_get_sql_tokens_quote_single_backslash_escape() {
    let mut config: Configuration = Configuration::new();
    config.backslash_escapes = true;
    assert_eq!(
        get_sql_tokens(&config, String::from(r#"SELECT 'it\'s time'"#)),
        vec![
            Token::new_test("SELECT", TokenCategory::Keyword),
            Token::new_test(" ", TokenCategory::WhiteSpace),
            Token::new_test(r#"'it\'s time'"#, TokenCategory::Quote),
        ]
    );
}

#[test]
fn test_get_sql_tokens_quote_single_backslash_escape_raw_bytes() {
    let mut config: Configuration = Configuration::new();
    config.backslash_escapes = true;
    let sql: String = decode_input(b"SELECT 'x\xff\\'y\xfe\\'z'");
    let tokens: Vec<Token> = get_sql_tokens(&config, sql);
    assert_eq!(tokens.len(), 3);
    assert_eq!(tokens[2].category, TokenCategory::Quote);
    assert_eq!(encode_output(&tokens[2].value), b"'x\xff\\'y\xfe\\'z'");
}

#[test]
fn test_get_sql_tokens_quote_single_backslash_literal_by_default() {
    assert_eq!(
        get_sql_tokens(&Configuration::new(), String::from(r#"SELECT 'C:\' AS p"#)),
        vec![
            Token::new_test("SELECT", TokenCategory::Keyword),
            Token::new_test(" ", TokenCategory::WhiteSpace),
            Token::new_test(r#"'C:\'"#, TokenCategory::Quote),
            Token::new_test(" ", TokenCategory::WhiteSpace),
            Token::new_test("AS", TokenCategory::Keyword),
            Token::new_test(" ", TokenCategory::WhiteSpace),
            Token::new_test("p", TokenCategory::Identifier),
        ]
    );
}

#[test]
fn test_get_sql_tokens_quote_single_backslash_doubled_quote_by_default() {
    assert_eq!(
        get_sql_tokens(&Configuration::new(), String::from(r#"SELECT 'it\''s'"#)),
        vec![
            Token::new_test("SELECT", TokenCategory::Keyword),
            Token::new_test(" ", TokenCategory::WhiteSpace),
            Token::new_test(r#"'it\''s'"#, TokenCategory::Quote),
        ]
    );
}

#[test]
fn test_get_sql_tokens_quote_single_multiline() {
    assert_eq!(
        get_sql_tokens(
            &Configuration::new(),
            String::from(
                r#"SELECT 'Column
Name'"#
            )
        ),
        vec![
            Token::new_test("SELECT", TokenCategory::Keyword),
            Token::new_test(" ", TokenCategory::WhiteSpace),
            Token::new_test(
                r#"'Column
Name'"#,
                TokenCategory::Quote
            ),
        ]
    );
}

#[test]
fn test_get_sql_tokens_quote_single_abrupt_end() {
    assert_eq!(
        get_sql_tokens(&Configuration::new(), String::from("SELECT 'Column")),
        vec![
            Token::new_test("SELECT", TokenCategory::Keyword),
            Token::new_test(" ", TokenCategory::WhiteSpace),
            Token::new_test("'Column", TokenCategory::Quote),
        ]
    );
}

#[test]
fn test_get_sql_tokens_delimiter_basic() {
    assert_eq!(
        get_sql_tokens(&Configuration::new(), String::from("SELECT 1;")),
        vec![
            Token::new_test("SELECT", TokenCategory::Keyword),
            Token::new_test(" ", TokenCategory::WhiteSpace),
            Token::new_test("1", TokenCategory::Identifier),
            Token::new_test(";", TokenCategory::Delimiter),
        ]
    );
}

#[test]
fn test_get_sql_tokens_delimiter_two() {
    assert_eq!(
        get_sql_tokens(&Configuration::new(), String::from("SELECT 1; SELECT 1;")),
        vec![
            Token::new_test("SELECT", TokenCategory::Keyword),
            Token::new_test(" ", TokenCategory::WhiteSpace),
            Token::new_test("1", TokenCategory::Identifier),
            Token::new_test(";", TokenCategory::Delimiter),
            Token::new_test(" ", TokenCategory::WhiteSpace),
            Token::new_test("SELECT", TokenCategory::Keyword),
            Token::new_test(" ", TokenCategory::WhiteSpace),
            Token::new_test("1", TokenCategory::Identifier),
            Token::new_test(";", TokenCategory::Delimiter),
        ]
    );
}

#[test]
fn test_get_sql_tokens_delimiter_change() {
    assert_eq!(
        get_sql_tokens(
            &Configuration::new(),
            String::from("SELECT 1; DELIMITER $$ SELECT 1; DELIMITER \\")
        ),
        vec![
            Token::new_test("SELECT", TokenCategory::Keyword),
            Token::new_test(" ", TokenCategory::WhiteSpace),
            Token::new_test("1", TokenCategory::Identifier),
            Token::new_test(";", TokenCategory::Delimiter),
            Token::new_test(" ", TokenCategory::WhiteSpace),
            Token::new_test("DELIMITER", TokenCategory::Keyword),
            Token::new_test(" ", TokenCategory::WhiteSpace),
            Token::new_test("$$", TokenCategory::Delimiter),
            Token::new_test(" ", TokenCategory::WhiteSpace),
            Token::new_test("SELECT", TokenCategory::Keyword),
            Token::new_test(" ", TokenCategory::WhiteSpace),
            Token::new_test("1;", TokenCategory::Identifier),
            Token::new_test(" ", TokenCategory::WhiteSpace),
            Token::new_test("DELIMITER", TokenCategory::Keyword),
            Token::new_test(" ", TokenCategory::WhiteSpace),
            Token::new_test("\\", TokenCategory::Delimiter),
        ]
    );
}

#[test]
fn test_get_sql_tokens_delimiter_change_operator_chars() {
    assert_eq!(
        get_sql_tokens(
            &Configuration::new(),
            String::from("DELIMITER // SELECT 1// SELECT 2 //")
        ),
        vec![
            Token::new_test("DELIMITER", TokenCategory::Keyword),
            Token::new_test(" ", TokenCategory::WhiteSpace),
            Token::new_test("//", TokenCategory::Delimiter),
            Token::new_test(" ", TokenCategory::WhiteSpace),
            Token::new_test("SELECT", TokenCategory::Keyword),
            Token::new_test(" ", TokenCategory::WhiteSpace),
            Token::new_test("1", TokenCategory::Identifier),
            Token::new_test("//", TokenCategory::Delimiter),
            Token::new_test(" ", TokenCategory::WhiteSpace),
            Token::new_test("SELECT", TokenCategory::Keyword),
            Token::new_test(" ", TokenCategory::WhiteSpace),
            Token::new_test("2", TokenCategory::Identifier),
            Token::new_test(" ", TokenCategory::WhiteSpace),
            Token::new_test("//", TokenCategory::Delimiter),
        ]
    );
}

#[test]
fn test_get_sql_tokens_delimiter_change_in_quote_and_comment() {
    assert_eq!(
        get_sql_tokens(
            &Configuration::new(),
            String::from("DELIMITER // SELECT '//' /* // */ //")
        ),
        vec![
            Token::new_test("DELIMITER", TokenCategory::Keyword),
            Token::new_test(" ", TokenCategory::WhiteSpace),
            Token::new_test("//", TokenCategory::Delimiter),
            Token::new_test(" ", TokenCategory::WhiteSpace),
            Token::new_test("SELECT", TokenCategory::Keyword),
            Token::new_test(" ", TokenCategory::WhiteSpace),
            Token::new_test("'//'", TokenCategory::Quote),
            Token::new_test(" ", TokenCategory::WhiteSpace),
            Token::new_test("/* // */", TokenCategory::Comment),
            Token::new_test(" ", TokenCategory::WhiteSpace),
            Token::new_test("//", TokenCategory::Delimiter),
        ]
    );
}

#[test]
fn test_get_sql_tokens_delimiter_change_back() {
    assert_eq!(
        get_sql_tokens(
            &Configuration::new(),
            String::from("DELIMITER // SELECT 1;// DELIMITER ; SELECT 2;")
        ),
        vec![
            Token::new_test("DELIMITER", TokenCategory::Keyword),
            Token::new_test(" ", TokenCategory::WhiteSpace),
            Token::new_test("//", TokenCategory::Delimiter),
            Token::new_test(" ", TokenCategory::WhiteSpace),
            Token::new_test("SELECT", TokenCategory::Keyword),
            Token::new_test(" ", TokenCategory::WhiteSpace),
            Token::new_test("1;", TokenCategory::Identifier),
            Token::new_test("//", TokenCategory::Delimiter),
            Token::new_test(" ", TokenCategory::WhiteSpace),
            Token::new_test("DELIMITER", TokenCategory::Keyword),
            Token::new_test(" ", TokenCategory::WhiteSpace),
            Token::new_test(";", TokenCategory::Delimiter),
            Token::new_test(" ", TokenCategory::WhiteSpace),
            Token::new_test("SELECT", TokenCategory::Keyword),
            Token::new_test(" ", TokenCategory::WhiteSpace),
            Token::new_test("2", TokenCategory::Identifier),
            Token::new_test(";", TokenCategory::Delimiter),
        ]
    );
}

#[test]
fn test_get_sql_tokens_comma() {
    assert_eq!(
        get_sql_tokens(&Configuration::new(), String::from("SELECT 1,2, 3")),
        vec![
            Token::new_test("SELECT", TokenCategory::Keyword),
            Token::new_test(" ", TokenCategory::WhiteSpace),
            Token::new_test("1", TokenCategory::Identifier),
            Token::new_test(",", TokenCategory::Comma),
            Token::new_test("2", TokenCategory::Identifier),
            Token::new_test(",", TokenCategory::Comma),
            Token::new_test(" ", TokenCategory::WhiteSpace),
            Token::new_test("3", TokenCategory::Identifier),
        ]
    );
}

#[test]
fn test_get_sql_tokens_negative_number() {
    assert_eq!(
        get_sql_tokens(&Configuration::new(), String::from("SELECT -1")),
        vec![
            Token::new_test("SELECT", TokenCategory::Keyword),
            Token::new_test(" ", TokenCategory::WhiteSpace),
            Token::new_test("-1", TokenCategory::Identifier),
        ]
    );
}

#[test]
fn test_get_sql_tokens_paren_empty() {
    assert_eq!(
        get_sql_tokens(&Configuration::new(), String::from("SELECT MIN()")),
        vec![
            Token::new_test("SELECT", TokenCategory::Keyword),
            Token::new_test(" ", TokenCategory::WhiteSpace),
            Token::new_test("MIN", TokenCategory::Method),
            Token::new_test("(", TokenCategory::ParenOpen),
            Token::new_test(")", TokenCategory::ParenClose),
        ]
    );
}

#[test]
fn test_get_sql_tokens_paren_content() {
    assert_eq!(
        get_sql_tokens(&Configuration::new(), String::from("SELECT (SELECT 1)")),
        vec![
            Token::new_test("SELECT", TokenCategory::Keyword),
            Token::new_test(" ", TokenCategory::WhiteSpace),
            Token::new_test("(", TokenCategory::ParenOpen),
            Token::new_test("SELECT", TokenCategory::Keyword),
            Token::new_test(" ", TokenCategory::WhiteSpace),
            Token::new_test("1", TokenCategory::Identifier),
            Token::new_test(")", TokenCategory::ParenClose),
        ]
    );
}

#[test]
fn test_get_sql_tokens_operator_add() {
    assert_eq!(
        get_sql_tokens(&Configuration::new(), String::from("1+2 + 3")),
        vec![
            Token::new_test("1", TokenCategory::Identifier),
            Token::new_test("+", TokenCategory::Operator),
            Token::new_test("2", TokenCategory::Identifier),
            Token::new_test(" ", TokenCategory::WhiteSpace),
            Token::new_test("+", TokenCategory::Operator),
            Token::new_test(" ", TokenCategory::WhiteSpace),
            Token::new_test("3", TokenCategory::Identifier),
        ]
    );
}

#[test]
fn test_get_sql_tokens_operator_subtract() {
    assert_eq!(
        get_sql_tokens(&Configuration::new(), String::from("1-2 - 3")),
        vec![
            Token::new_test("1", TokenCategory::Identifier),
            Token::new_test("-", TokenCategory::Operator),
            Token::new_test("2", TokenCategory::Identifier),
            Token::new_test(" ", TokenCategory::WhiteSpace),
            Token::new_test("-", TokenCategory::Operator),
            Token::new_test(" ", TokenCategory::WhiteSpace),
            Token::new_test("3", TokenCategory::Identifier),
        ]
    );
}

#[test]
fn test_get_sql_tokens_operator_multiply() {
    assert_eq!(
        get_sql_tokens(&Configuration::new(), String::from("1*2 * 3")),
        vec![
            Token::new_test("1", TokenCategory::Identifier),
            Token::new_test("*", TokenCategory::Operator),
            Token::new_test("2", TokenCategory::Identifier),
            Token::new_test(" ", TokenCategory::WhiteSpace),
            Token::new_test("*", TokenCategory::Operator),
            Token::new_test(" ", TokenCategory::WhiteSpace),
            Token::new_test("3", TokenCategory::Identifier),
        ]
    );
}

#[test]
fn test_get_sql_tokens_operator_divide() {
    assert_eq!(
        get_sql_tokens(&Configuration::new(), String::from("1/2 / 3")),
        vec![
            Token::new_test("1", TokenCategory::Identifier),
            Token::new_test("/", TokenCategory::Operator),
            Token::new_test("2", TokenCategory::Identifier),
            Token::new_test(" ", TokenCategory::WhiteSpace),
            Token::new_test("/", TokenCategory::Operator),
            Token::new_test(" ", TokenCategory::WhiteSpace),
            Token::new_test("3", TokenCategory::Identifier),
        ]
    );
}

#[test]
fn test_get_sql_tokens_operator_modulo() {
    assert_eq!(
        get_sql_tokens(&Configuration::new(), String::from("1%2 % 3")),
        vec![
            Token::new_test("1", TokenCategory::Identifier),
            Token::new_test("%", TokenCategory::Operator),
            Token::new_test("2", TokenCategory::Identifier),
            Token::new_test(" ", TokenCategory::WhiteSpace),
            Token::new_test("%", TokenCategory::Operator),
            Token::new_test(" ", TokenCategory::WhiteSpace),
            Token::new_test("3", TokenCategory::Identifier),
        ]
    );
}

#[test]
fn test_get_sql_tokens_operator_add_equal() {
    assert_eq!(
        get_sql_tokens(&Configuration::new(), String::from("V+=1")),
        vec![
            Token::new_test("V", TokenCategory::Identifier),
            Token::new_test("+=", TokenCategory::Operator),
            Token::new_test("1", TokenCategory::Identifier),
        ]
    );
}

#[test]
fn test_get_sql_tokens_operator_minus_equal() {
    assert_eq!(
        get_sql_tokens(&Configuration::new(), String::from("V-=1")),
        vec![
            Token::new_test("V", TokenCategory::Identifier),
            Token::new_test("-=", TokenCategory::Operator),
            Token::new_test("1", TokenCategory::Identifier),
        ]
    );
}

#[test]
fn test_get_sql_tokens_operator_multiply_equal() {
    assert_eq!(
        get_sql_tokens(&Configuration::new(), String::from("V*=1")),
        vec![
            Token::new_test("V", TokenCategory::Identifier),
            Token::new_test("*=", TokenCategory::Operator),
            Token::new_test("1", TokenCategory::Identifier),
        ]
    );
}

#[test]
fn test_get_sql_tokens_operator_divide_equal() {
    assert_eq!(
        get_sql_tokens(&Configuration::new(), String::from("V/=1")),
        vec![
            Token::new_test("V", TokenCategory::Identifier),
            Token::new_test("/=", TokenCategory::Operator),
            Token::new_test("1", TokenCategory::Identifier),
        ]
    );
}

#[test]
fn test_get_sql_tokens_operator_modulo_equal() {
    assert_eq!(
        get_sql_tokens(&Configuration::new(), String::from("V%=1")),
        vec![
            Token::new_test("V", TokenCategory::Identifier),
            Token::new_test("%=", TokenCategory::Operator),
            Token::new_test("1", TokenCategory::Identifier),
        ]
    );
}

#[test]
fn test_get_sql_tokens_bitwise_and() {
    assert_eq!(
        get_sql_tokens(&Configuration::new(), String::from("V1&V2")),
        vec![
            Token::new_test("V1", TokenCategory::Identifier),
            Token::new_test("&", TokenCategory::Bitwise),
            Token::new_test("V2", TokenCategory::Identifier),
        ]
    );
}

#[test]
fn test_get_sql_tokens_bitwise_or() {
    assert_eq!(
        get_sql_tokens(&Configuration::new(), String::from("V1|V2")),
        vec![
            Token::new_test("V1", TokenCategory::Identifier),
            Token::new_test("|", TokenCategory::Bitwise),
            Token::new_test("V2", TokenCategory::Identifier),
        ]
    );
}

#[test]
fn test_get_sql_tokens_concatenation() {
    assert_eq!(
        get_sql_tokens(&Configuration::new(), String::from("V1||V2 || V3")),
        vec![
            Token::new_test("V1", TokenCategory::Identifier),
            Token::new_test("||", TokenCategory::Operator),
            Token::new_test("V2", TokenCategory::Identifier),
            Token::new_test(" ", TokenCategory::WhiteSpace),
            Token::new_test("||", TokenCategory::Operator),
            Token::new_test(" ", TokenCategory::WhiteSpace),
            Token::new_test("V3", TokenCategory::Identifier),
        ]
    );
}

#[test]
fn test_get_sql_tokens_json_operators() {
    assert_eq!(
        get_sql_tokens(
            &Configuration::new(),
            String::from("C1->'a'->>'b' C1 @> C2 C1@>C2 C1 #> C2 C1 #>> C2")
        ),
        vec![
            Token::new_test("C1", TokenCategory::Identifier),
            Token::new_test("->", TokenCategory::Compare),
            Token::new_test("'a'", TokenCategory::Quote),
            Token::new_test("->>", TokenCategory::Compare),
            Token::new_test("'b'", TokenCategory::Quote),
            Token::new_test(" ", TokenCategory::WhiteSpace),
            Token::new_test("C1", TokenCategory::Identifier),
            Token::new_test(" ", TokenCategory::WhiteSpace),
            Token::new_test("@>", TokenCategory::Compare),
            Token::new_test(" ", TokenCategory::WhiteSpace),
            Token::new_test("C2", TokenCategory::Identifier),
            Token::new_test(" ", TokenCategory::WhiteSpace),
            Token::new_test("C1", TokenCategory::Identifier),
            Token::new_test("@>", TokenCategory::Compare),
            Token::new_test("C2", TokenCategory::Identifier),
            Token::new_test(" ", TokenCategory::WhiteSpace),
            Token::new_test("C1", TokenCategory::Identifier),
            Token::new_test(" ", TokenCategory::WhiteSpace),
            Token::new_test("#>", TokenCategory::Compare),
            Token::new_test(" ", TokenCategory::WhiteSpace),
            Token::new_test("C2", TokenCategory::Identifier),
            Token::new_test(" ", TokenCategory::WhiteSpace),
            Token::new_test("C1", TokenCategory::Identifier),
            Token::new_test(" ", TokenCategory::WhiteSpace),
            Token::new_test("#>>", TokenCategory::Compare),
            Token::new_test(" ", TokenCategory::WhiteSpace),
            Token::new_test("C2", TokenCategory::Identifier),
        ]
    );
}

#[test]
fn test_get_sql_tokens_bitwise_exclusive_or() {
    assert_eq!(
        get_sql_tokens(&Configuration::new(), String::from("V1^V2")),
        vec![
            Token::new_test("V1", TokenCategory::Identifier),
            Token::new_test("^", TokenCategory::Bitwise),
            Token::new_test("V2", TokenCategory::Identifier),
        ]
    );
}

#[test]
fn test_get_sql_tokens_paren_compare_lt() {
    assert_eq!(
        get_sql_tokens(
            &Configuration::new(),
            String::from("WHERE C1<C2 AND C1 < C2")
        ),
        vec![
            Token::new_test("WHERE", TokenCategory::Keyword),
            Token::new_test(" ", TokenCategory::WhiteSpace),
            Token::new_test("C1", TokenCategory::Identifier),
            Token::new_test("<", TokenCategory::Compare),
            Token::new_test("C2", TokenCategory::Identifier),
            Token::new_test(" ", TokenCategory::WhiteSpace),
            Token::new_test("AND", TokenCategory::Keyword),
            Token::new_test(" ", TokenCategory::WhiteSpace),
            Token::new_test("C1", TokenCategory::Identifier),
            Token::new_test(" ", TokenCategory::WhiteSpace),
            Token::new_test("<", TokenCategory::Compare),
            Token::new_test(" ", TokenCategory::WhiteSpace),
            Token::new_test("C2", TokenCategory::Identifier),
        ]
    );
}

#[test]
fn test_get_sql_tokens_paren_compare_gt() {
    assert_eq!(
        get_sql_tokens(
            &Configuration::new(),
            String::from("WHERE C1>C2 AND C1 > C2")
        ),
        vec![
            Token::new_test("WHERE", TokenCategory::Keyword),
            Token::new_test(" ", TokenCategory::WhiteSpace),
            Token::new_test("C1", TokenCategory::Identifier),
            Token::new_test(">", TokenCategory::Compare),
            Token::new_test("C2", TokenCategory::Identifier),
            Token::new_test(" ", TokenCategory::WhiteSpace),
            Token::new_test("AND", TokenCategory::Keyword),
            Token::new_test(" ", TokenCategory::WhiteSpace),
            Token::new_test("C1", TokenCategory::Identifier),
            Token::new_test(" ", TokenCategory::WhiteSpace),
            Token::new_test(">", TokenCategory::Compare),
            Token::new_test(" ", TokenCategory::WhiteSpace),
            Token::new_test("C2", TokenCategory::Identifier),
        ]
    );
}

#[test]
fn test_get_sql_tokens_paren_compare_eq() {
    assert_eq!(
        get_sql_tokens(
            &Configuration::new(),
            String::from("WHERE C1=C2 AND C1 = C2")
        ),
        vec![
            Token::new_test("WHERE", TokenCategory::Keyword),
            Token::new_test(" ", TokenCategory::WhiteSpace),
            Token::new_test("C1", TokenCategory::Identifier),
            Token::new_test("=", TokenCategory::Compare),
            Token::new_test("C2", TokenCategory::Identifier),
            Token::new_test(" ", TokenCategory::WhiteSpace),
            Token::new_test("AND", TokenCategory::Keyword),
            Token::new_test(" ", TokenCategory::WhiteSpace),
            Token::new_test("C1", TokenCategory::Identifier),
            Token::new_test(" ", TokenCategory::WhiteSpace),
            Token::new_test("=", TokenCategory::Compare),
            Token::new_test(" ", TokenCategory::WhiteSpace),
            Token::new_test("C2", TokenCategory::Identifier),
        ]
    );
}

#[test]
fn test_get_sql_tokens_paren_compare_neq() {
    assert_eq!(
        get_sql_tokens(
            &Configuration::new(),
            String::from("WHERE C1<>C2 AND C1 <> C2")
        ),
        vec![
            Token::new_test("WHERE", TokenCategory::Keyword),
            Token::new_test(" ", TokenCategory::WhiteSpace),
            Token::new_test("C1", TokenCategory::Identifier),
            Token::new_test("<>", TokenCategory::Compare),
            Token::new_test("C2", TokenCategory::Identifier),
            Token::new_test(" ", TokenCategory::WhiteSpace),
            Token::new_test("AND", TokenCategory::Keyword),
            Token::new_test(" ", TokenCategory::WhiteSpace),
            Token::new_test("C1", TokenCategory::Identifier),
            Token::new_test(" ", TokenCategory::WhiteSpace),
            Token::new_test("<>", TokenCategory::Compare),
            Token::new_test(" ", TokenCategory::WhiteSpace),
            Token::new_test("C2", TokenCategory::Identifier),
        ]
    );
}

#[test]
fn test_get_sql_tokens_paren_compare_neq_alt() {
    assert_eq!(
        get_sql_tokens(
            &Configuration::new(),
            String::from("WHERE C1!=C2 AND C1 != C2")
        ),
        vec![
            Token::new_test("WHERE", TokenCategory::Keyword),
            Token::new_test(" ", TokenCategory::WhiteSpace),
            Token::new_test("C1", TokenCategory::Identifier),
            Token::new_test("!=", TokenCategory::Compare),
            Token::new_test("C2", TokenCategory::Identifier),
            Token::new_test(" ", TokenCategory::WhiteSpace),
            Token::new_test("AND", TokenCategory::Keyword),
            Token::new_test(" ", TokenCategory::WhiteSpace),
            Token::new_test("C1", TokenCategory::Identifier),
            Token::new_test(" ", TokenCategory::WhiteSpace),
            Token::new_test("!=", TokenCategory::Compare),
            Token::new_test(" ", TokenCategory::WhiteSpace),
            Token::new_test("C2", TokenCategory::Identifier),
        ]
    );
}

#[test]
fn test_get_sql_tokens_paren_compare_gteq() {
    assert_eq!(
        get_sql_tokens(
            &Configuration::new(),
            String::from("WHERE C1>=C2 AND C1 >= C2")
        ),
        vec![
            Token::new_test("WHERE", TokenCategory::Keyword),
            Token::new_test(" ", TokenCategory::WhiteSpace),
            Token::new_test("C1", TokenCategory::Identifier),
            Token::new_test(">=", TokenCategory::Compare),
            Token::new_test("C2", TokenCategory::Identifier),
            Token::new_test(" ", TokenCategory::WhiteSpace),
            Token::new_test("AND", TokenCategory::Keyword),
            Token::new_test(" ", TokenCategory::WhiteSpace),
            Token::new_test("C1", TokenCategory::Identifier),
            Token::new_test(" ", TokenCategory::WhiteSpace),
            Token::new_test(">=", TokenCategory::Compare),
            Token::new_test(" ", TokenCategory::WhiteSpace),
            Token::new_test("C2", TokenCategory::Identifier),
        ]
    );
}

#[test]
fn test_get_sql_tokens_paren_compare_lteq() {
    assert_eq!(
        get_sql_tokens(
            &Configuration::new(),
            String::from("WHERE C1<=C2 AND C1 <= C2")
        ),
        vec![
            Token::new_test("WHERE", TokenCategory::Keyword),
            Token::new_test(" ", TokenCategory::WhiteSpace),
            Token::new_test("C1", TokenCategory::Identifier),
            Token::new_test("<=", TokenCategory::Compare),
            Token::new_test("C2", TokenCategory::Identifier),
            Token::new_test(" ", TokenCategory::WhiteSpace),
            Token::new_test("AND", TokenCategory::Keyword),
            Token::new_test(" ", TokenCategory::WhiteSpace),
            Token::new_test("C1", TokenCategory::Identifier),
            Token::new_test(" ", TokenCategory::WhiteSpace),
            Token::new_test("<=", TokenCategory::Compare),
            Token::new_test(" ", TokenCategory::WhiteSpace),
            Token::new_test("C2", TokenCategory::Identifier),
        ]
    );
}

#[test]
fn test_get_sql_tokens_paren_compare_all() {
    assert_eq!(
        get_sql_tokens(
            &Configuration::new(),
            String::from("C1=C2 C1>C2 C1<C2 C1>=C2 C1<=C2 C1<>C2 C1!=C2 C1!<C2 C1!>C2 C1<=>C2")
        ),
        vec![
            Token::new_test("C1", TokenCategory::Identifier),
            Token::new_test("=", TokenCategory::Compare),
            Token::new_test("C2", TokenCategory::Identifier),
            Token::new_test(" ", TokenCategory::WhiteSpace),
            Token::new_test("C1", TokenCategory::Identifier),
            Token::new_test(">", TokenCategory::Compare),
            Token::new_test("C2", TokenCategory::Identifier),
            Token::new_test(" ", TokenCategory::WhiteSpace),
            Token::new_test("C1", TokenCategory::Identifier),
            Token::new_test("<", TokenCategory::Compare),
            Token::new_test("C2", TokenCategory::Identifier),
            Token::new_test(" ", TokenCategory::WhiteSpace),
            Token::new_test("C1", TokenCategory::Identifier),
            Token::new_test(">=", TokenCategory::Compare),
            Token::new_test("C2", TokenCategory::Identifier),
            Token::new_test(" ", TokenCategory::WhiteSpace),
            Token::new_test("C1", TokenCategory::Identifier),
            Token::new_test("<=", TokenCategory::Compare),
            Token::new_test("C2", TokenCategory::Identifier),
            Token::new_test(" ", TokenCategory::WhiteSpace),
            Token::new_test("C1", TokenCategory::Identifier),
            Token::new_test("<>", TokenCategory::Compare),
            Token::new_test("C2", TokenCategory::Identifier),
            Token::new_test(" ", TokenCategory::WhiteSpace),
            Token::new_test("C1", TokenCategory::Identifier),
            Token::new_test("!=", TokenCategory::Compare),
            Token::new_test("C2", TokenCategory::Identifier),
            Token::new_test(" ", TokenCategory::WhiteSpace),
            Token::new_test("C1", TokenCategory::Identifier),
            Token::new_test("!<", TokenCategory::Compare),
            Token::new_test("C2", TokenCategory::Identifier),
            Token::new_test(" ", TokenCategory::WhiteSpace),
            Token::new_test("C1", TokenCategory::Identifier),
            Token::new_test("!>", TokenCategory::Compare),
            Token::new_test("C2", TokenCategory::Identifier),
            Token::new_test(" ", TokenCategory::WhiteSpace),
            Token::new_test("C1", TokenCategory::Identifier),
            Token::new_test("<=>", TokenCategory::Compare),
            Token::new_test("C2", TokenCategory::Identifier),
        ]
    );
}

#[test]
fn test_get_sql_tokens_xml_method() {
    assert_eq!(
        get_sql_tokens(
            &Configuration::new(),
            String::from("Instructions.nodes('/root/Location')")
        ),
        vec![
            Token::new_test("Instructions", TokenCategory::Identifier),
            Token::new_test(".", TokenCategory::FullStop),
            Token::new_test("nodes", TokenCategory::XmlMethod),
            Token::new_test("(", TokenCategory::ParenOpen),
            Token::new_test("'/root/Location'", TokenCategory::Quote),
            Token::new_test(")", TokenCategory::ParenClose),
        ]
    );
}
