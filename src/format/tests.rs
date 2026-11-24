use super::*;

// shadows the real formatter so every test also checks that formatting
// already formatted output changes nothing
fn get_formatted_sql(config: &Configuration, input_sql: String) -> String {
    let output: String = super::get_formatted_sql(config, input_sql);
    assert_eq!(
        super::get_formatted_sql(config, output.clone()),
        output,
        "formatting the output again changed it"
    );
    output
}

#[test]
fn test_get_formatted_sql_empty() {
    let mut config: Configuration = Configuration::new();
    let sql: String = String::from(r#""#);

    assert_eq!(get_formatted_sql(&config, sql.clone()), r#""#);

    config.newlines = true;
    assert_eq!(get_formatted_sql(&config, sql.clone()), r#""#);
}

#[test]
fn test_get_formatted_sql_whitespace_only() {
    let mut config: Configuration = Configuration::new();
    let sql: String = String::from(
        r#"


        "#,
    );

    assert_eq!(
        get_formatted_sql(&config, sql.clone()),
        r#"


"#,
    );

    config.newlines = true;
    assert_eq!(get_formatted_sql(&config, sql.clone()), r#""#);
}

#[test]
fn test_get_formatted_sql_select_simple() {
    let mut config: Configuration = Configuration::new();
    let sql: String = String::from(r#"SELECT * FROM TBL1"#);

    config.newlines = false;
    assert_eq!(
        get_formatted_sql(&config, sql.clone()),
        r#"SELECT * FROM TBL1"#
    );
}

#[test]
fn test_get_formatted_sql_newlines() {
    let mut config: Configuration = Configuration::new();
    let sql: String = String::from(r#"SELECT * FROM TBL1"#);

    config.newlines = true;
    assert_eq!(
        get_formatted_sql(&config, sql.clone()),
        r#"SELECT
    *
FROM TBL1"#
    );
}

#[test]
fn test_get_formatted_sql_upper() {
    let mut config: Configuration = Configuration::new();
    let sql: String = String::from(r#"select * from tbl1"#);

    config.case = ConfigCase::Uppercase;
    assert_eq!(
        get_formatted_sql(&config, sql.clone()),
        r#"SELECT * FROM tbl1"#
    );
}

#[test]
fn test_get_formatted_sql_lower() {
    let mut config: Configuration = Configuration::new();
    let sql: String = String::from(r#"SELECT * FROM TBL1"#);

    config.case = ConfigCase::Lowercase;
    assert_eq!(
        get_formatted_sql(&config, sql.clone()),
        r#"select * from TBL1"#
    );
}

#[test]
fn test_get_formatted_sql_select_simple_newlines() {
    let mut config: Configuration = Configuration::new();
    let sql: String = String::from(
        r#"
            SELECT  *
            FROM  TBL1
            "#,
    );

    config.newlines = false;
    assert_eq!(
        get_formatted_sql(&config, sql.clone()),
        r#"
            SELECT *
            FROM TBL1
"#
    );
}

#[test]
fn test_get_formatted_sql_config_tabs() {
    let mut config: Configuration = Configuration::new();
    let sql: String = String::from(
        r#"
            SELECT
                C1,
                C2
            FROM TBL1
            "#,
    );

    config.tabs = ConfigTab::Tab;
    assert_eq!(
        get_formatted_sql(&config, sql.clone()),
        r#"
            SELECT
            	C1,
            	C2
            FROM TBL1
"#
    );
}

#[test]
fn test_get_formatted_sql_prefix_tabs() {
    let mut config: Configuration = Configuration::new();
    let sql: String = String::from(
        r#"
			SELECT
				C1,
				C2
			FROM TBL1
            "#,
    );

    config.newlines = true;
    assert_eq!(
        get_formatted_sql(&config, sql.clone()),
        r#"			SELECT
			    C1,
			    C2
			FROM TBL1"#
    );
}

#[test]
fn test_get_formatted_sql_config_spaces() {
    let mut config: Configuration = Configuration::new();
    let sql: String = String::from(
        r#"
            SELECT
                C1,
                C2
            FROM TBL1
            "#,
    );

    config.tabs = ConfigTab::Space(2);
    assert_eq!(
        get_formatted_sql(&config, sql.clone()),
        r#"
            SELECT
              C1,
              C2
            FROM TBL1
"#
    );
}

#[test]
fn test_get_formatted_sql_config_chars() {
    let mut config: Configuration = Configuration::new();
    let sql: String = String::from(
        r#"
            SELECT
            (SELECT C1, C2 FROM TBL1)
            "#,
    );

    config.newlines = true;

    assert_eq!(
        get_formatted_sql(&config, sql.clone()),
        r#"            SELECT
                (SELECT C1, C2 FROM TBL1)"#
    );

    config.chars = 40;
    assert_eq!(
        get_formatted_sql(&config, sql.clone()),
        r#"            SELECT
                (
                    SELECT
                        C1,
                        C2
                    FROM TBL1
                )"#
    );
}

#[test]
fn test_get_formatted_sql_config_chars_tabs() {
    let mut config: Configuration = Configuration::new();
    let sql: String = String::from(
        r#"
			SELECT
			(SELECT C1, C2 FROM TBL1)
			"#,
    );

    config.tabs = ConfigTab::Tab;
    config.newlines = true;

    assert_eq!(
        get_formatted_sql(&config, sql.clone()),
        r#"			SELECT
				(SELECT C1, C2 FROM TBL1)"#
    );

    config.chars = 40;
    assert_eq!(
        get_formatted_sql(&config, sql.clone()),
        r#"			SELECT
				(
					SELECT
						C1,
						C2
					FROM TBL1
				)"#
    );
}

#[test]
fn test_get_formatted_sql_config_chars_operators() {
    let mut config: Configuration = Configuration::new();
    let sql: String = String::from(
        r#"
            SELECT
            COLUMN1 + COLUMN2 + COLUMN3 + COLUMN4 + COLUMN5 + COLUMN6 + COLUMN7
            FROM TBL1
            "#,
    );

    config.newlines = true;

    assert_eq!(
        get_formatted_sql(&config, sql.clone()),
        r#"            SELECT
                COLUMN1 + COLUMN2 + COLUMN3 + COLUMN4 + COLUMN5 + COLUMN6 +
                COLUMN7
            FROM TBL1"#
    );

    config.chars = 40;
    assert_eq!(
        get_formatted_sql(&config, sql.clone()),
        r#"            SELECT
                COLUMN1 + COLUMN2 +
                COLUMN3 + COLUMN4 +
                COLUMN5 + COLUMN6 +
                COLUMN7
            FROM TBL1"#
    );
}

#[test]
fn test_get_formatted_sql_config_chars_operators_comment() {
    let mut config: Configuration = Configuration::new();
    let sql: String = String::from(
        r#"
            SET V1 = V2 + V3 -- really long comment that goes past the max line length
            SELECT COLUMN1 FROM TBL1
            "#,
    );

    config.newlines = true;

    assert_eq!(
        get_formatted_sql(&config, sql.clone()),
        r#"            SET V1 = V2 + V3 -- really long comment that goes past the max line length
            SELECT
                COLUMN1
            FROM TBL1"#
    );

    config.comment_pre_space = true;
    assert_eq!(
        get_formatted_sql(&config, sql.clone()),
        r#"            SET V1 = V2 + V3
            -- really long comment that goes past the max line length
            SELECT
                COLUMN1
            FROM TBL1"#
    );
}

#[test]
fn test_get_formatted_sql_config_chars_paren_operators() {
    let mut config: Configuration = Configuration::new();
    let sql: String = String::from(
        r#"
            SELECT
            (SELECT COLUMN1, COLUMN2 FROM TBL1) + (SELECT COLUMN1, COLUMN2 FROM TBL1)
            "#,
    );

    config.newlines = true;

    assert_eq!(
        get_formatted_sql(&config, sql.clone()),
        r#"            SELECT
                (SELECT COLUMN1, COLUMN2 FROM TBL1) +
                (SELECT COLUMN1, COLUMN2 FROM TBL1)"#
    );

    config.chars = 40;
    assert_eq!(
        get_formatted_sql(&config, sql.clone()),
        r#"            SELECT
                (
                    SELECT
                        COLUMN1,
                        COLUMN2
                    FROM TBL1
                ) +
                (
                    SELECT
                        COLUMN1,
                        COLUMN2
                    FROM TBL1
                )"#
    );
}

#[test]
fn test_get_formatted_sql_select_multiple_columns_inline() {
    let mut config: Configuration = Configuration::new();
    let sql: String = String::from(r#"SELECT C1,C2, C3 FROM TBL1"#);

    config.newlines = false;
    assert_eq!(
        get_formatted_sql(&config, sql.clone()),
        r#"SELECT C1, C2, C3 FROM TBL1"#
    );
}

#[test]
fn test_get_formatted_sql_select_multiple_columns_newlines() {
    let mut config: Configuration = Configuration::new();
    let sql: String = String::from(
        r#"
            SELECT
            C1 AS 'Column 1',
                C2 AS 'Column 2',
            C3
            FROM TBL1 AS T
            "#,
    );

    config.newlines = false;
    assert_eq!(
        get_formatted_sql(&config, sql.clone()),
        r#"
            SELECT
                C1 AS 'Column 1',
                C2 AS 'Column 2',
                C3
            FROM TBL1 AS T
"#
    );
}

#[test]
fn test_get_formatted_sql_alias() {
    let mut config: Configuration = Configuration::new();
    let sql: String = String::from(r#"SELECT C1 AS 'Column 1' FROM TBL1"#);

    assert_eq!(
        get_formatted_sql(&config, sql.clone()),
        r#"SELECT C1 AS 'Column 1' FROM TBL1"#
    );

    config.newlines = true;
    assert_eq!(
        get_formatted_sql(&config, sql.clone()),
        r#"SELECT
    C1 AS 'Column 1'
FROM TBL1"#
    );
}

#[test]
fn test_get_formatted_sql_alias_keyword() {
    let mut config: Configuration = Configuration::new();
    let sql: String = String::from(r#"SELECT DO.C1,DO.C2,DO.C3 FROM TBL1 AS DO"#);

    assert_eq!(
        get_formatted_sql(&config, sql.clone()),
        r#"SELECT DO.C1, DO.C2, DO.C3 FROM TBL1 AS DO"#
    );

    // formatted result is bad because alias is a keyword with strong behaviors
    // alias should be quoted like in the test case after this one to fix
    config.newlines = true;
    assert_eq!(
        get_formatted_sql(&config, sql.clone()),
        r#"SELECT
DO.C1,
    DO.C2,
        DO.C3
            FROM TBL1 AS
            DO"#
    );
}

#[test]
fn test_get_formatted_sql_alias_keyword_quoted() {
    let mut config: Configuration = Configuration::new();
    let sql: String = String::from(r#"SELECT [DO].C1,[DO].C2,[DO].C3 FROM TBL1 AS [DO]"#);

    assert_eq!(
        get_formatted_sql(&config, sql.clone()),
        r#"SELECT [DO].C1, [DO].C2, [DO].C3 FROM TBL1 AS [DO]"#
    );

    config.newlines = true;
    assert_eq!(
        get_formatted_sql(&config, sql.clone()),
        r#"SELECT
    [DO].C1,
    [DO].C2,
    [DO].C3
FROM TBL1 AS [DO]"#
    );
}

#[test]
fn test_get_formatted_go() {
    let mut config: Configuration = Configuration::new();
    let sql: String = String::from(
        r#"
            SELECT C1 FROM TBL1 GO
            SELECT C1 FROM TBL1 GO
            "#,
    );

    assert_eq!(
        get_formatted_sql(&config, sql.clone()),
        r#"
            SELECT C1 FROM TBL1 GO
            SELECT C1 FROM TBL1 GO
"#,
    );

    config.newlines = true;
    assert_eq!(
        get_formatted_sql(&config, sql.clone()),
        r#"            SELECT
                C1
            FROM TBL1
            GO

            SELECT
                C1
            FROM TBL1
            GO"#,
    );
}

#[test]
fn test_get_formatted_sql_datatype_quote() {
    let mut config: Configuration = Configuration::new();
    let sql: String = String::from(r#"[NVARCHAR](36)"#);

    assert_eq!(get_formatted_sql(&config, sql.clone()), r#"[NVARCHAR](36)"#);

    config.newlines = true;
    assert_eq!(get_formatted_sql(&config, sql.clone()), r#"[NVARCHAR](36)"#);

    let sql: String = String::from(r#"[nvarchar](36)"#);
    assert_eq!(get_formatted_sql(&config, sql.clone()), r#"[nvarchar](36)"#);
}

#[test]
fn test_get_formatted_sql_convert() {
    let mut config: Configuration = Configuration::new();
    let sql: String = String::from(r#"CONVERT(NVARCHAR(36), ID)"#);

    assert_eq!(
        get_formatted_sql(&config, sql.clone()),
        r#"CONVERT(NVARCHAR(36), ID)"#
    );

    config.newlines = true;
    assert_eq!(
        get_formatted_sql(&config, sql.clone()),
        r#"CONVERT(NVARCHAR(36), ID)"#
    );
}

#[test]
fn test_get_formatted_sql_select_joins() {
    let mut config: Configuration = Configuration::new();
    let sql: String = String::from(
        r#"
            SELECT *
            FROM TBL1 AS T1
            INNER JOIN TBL2 AS T2 ON T2.C1 = T1.C1
            LEFT JOIN TBL3 AS T3 ON T3.C1 = T1.C1
            RIGHT JOIN TBL4 AS T4 ON T4.C1 = T1.C1
            OUTER JOIN TBL5 AS T5 ON T5.C1 = T1.C1
            LEFT OUTER JOIN TBL6 AS T6 ON T6.C1 = T1.C1
            RIGHT OUTER JOIN TBL7 AS T7 ON T7.C1 = T1.C1
            FULL OUTER JOIN TBL8 AS T8 ON T8.C1 = T1.C1
            INNER JOIN TBL9 AS T9 ON T9.C1 = T1.C1
            WHERE T1.C2 = 1
            "#,
    );

    assert_eq!(
        get_formatted_sql(&config, sql.clone()),
        r#"
            SELECT *
            FROM TBL1 AS T1
                INNER JOIN TBL2 AS T2 ON T2.C1 = T1.C1
                LEFT JOIN TBL3 AS T3 ON T3.C1 = T1.C1
                RIGHT JOIN TBL4 AS T4 ON T4.C1 = T1.C1
                OUTER JOIN TBL5 AS T5 ON T5.C1 = T1.C1
                LEFT OUTER JOIN TBL6 AS T6 ON T6.C1 = T1.C1
                RIGHT OUTER JOIN TBL7 AS T7 ON T7.C1 = T1.C1
                FULL OUTER JOIN TBL8 AS T8 ON T8.C1 = T1.C1
                INNER JOIN TBL9 AS T9 ON T9.C1 = T1.C1
            WHERE T1.C2 = 1
"#
    );

    config.newlines = true;
    assert_eq!(
        get_formatted_sql(&config, sql.clone()),
        r#"            SELECT
                *
            FROM TBL1 AS T1
                INNER JOIN TBL2 AS T2 ON T2.C1 = T1.C1
                LEFT JOIN TBL3 AS T3 ON T3.C1 = T1.C1
                RIGHT JOIN TBL4 AS T4 ON T4.C1 = T1.C1
                OUTER JOIN TBL5 AS T5 ON T5.C1 = T1.C1
                LEFT OUTER JOIN TBL6 AS T6 ON T6.C1 = T1.C1
                RIGHT OUTER JOIN TBL7 AS T7 ON T7.C1 = T1.C1
                FULL OUTER JOIN TBL8 AS T8 ON T8.C1 = T1.C1
                INNER JOIN TBL9 AS T9 ON T9.C1 = T1.C1
            WHERE T1.C2 = 1"#
    );
}

#[test]
fn test_get_formatted_sql_select_curly_string() {
    let mut config: Configuration = Configuration::new();
    let sql: String = String::from(
        r#"
            SELECT T1.*,  {columnName1},{columnName2} , {columnName3}
            FROM {tableNames[i]} AS T1
            INNER JOIN   {tableNames[i]}   AS T2 ON T2.C1 = T1.C1
            INNER JOIN T{tableNames[i]}3 AS T3 ON T3.C1 = T1.C1
            INNER JOIN   T{tableNames[i]}4   AS T4 ON T4.C1 = T1.C1
            {otherJoin}
            WHERE T1.C2 = 1
            "#,
    );

    assert_eq!(
        get_formatted_sql(&config, sql.clone()),
        r#"
            SELECT T1.*, {columnName1},{columnName2}, {columnName3}
            FROM {tableNames[i]} AS T1
                INNER JOIN {tableNames[i]} AS T2 ON T2.C1 = T1.C1
                INNER JOIN T{tableNames[i]}3 AS T3 ON T3.C1 = T1.C1
                INNER JOIN T{tableNames[i]}4 AS T4 ON T4.C1 = T1.C1
                {otherJoin}
            WHERE T1.C2 = 1
"#
    );

    config.newlines = true;
    assert_eq!(
        get_formatted_sql(&config, sql.clone()),
        r#"            SELECT
                T1.*,
                {columnName1},{columnName2},
                {columnName3}
            FROM {tableNames[i]} AS T1
                INNER JOIN {tableNames[i]} AS T2 ON T2.C1 = T1.C1
                INNER JOIN T{tableNames[i]}3 AS T3 ON T3.C1 = T1.C1
                INNER JOIN T{tableNames[i]}4 AS T4 ON T4.C1 = T1.C1 {otherJoin}
            WHERE T1.C2 = 1"#
    );
}

#[test]
fn test_get_formatted_sql_call_curly_string() {
    let mut config: Configuration = Configuration::new();
    let sql: String = String::from(
        r#"
            CALL SCH.{procedureName}();
            CALL SCH.B{procedureName}E();
            "#,
    );

    assert_eq!(
        get_formatted_sql(&config, sql.clone()),
        r#"
            CALL SCH.{procedureName}();
            CALL SCH.B{procedureName}E();
"#,
    );

    config.newlines = true;
    assert_eq!(
        get_formatted_sql(&config, sql.clone()),
        r#"            CALL SCH.{procedureName}();
            CALL SCH.B{procedureName}E();"#,
    );
}

#[test]
fn test_get_formatted_sql_embedded_conditions() {
    let mut config: Configuration = Configuration::new();
    let sql: String = String::from(
        r#"
            SELECT * FROM TBL1
            WHERE ((C1=0 AND C2=0)OR(C1=1 AND C2=1))
            "#,
    );

    assert_eq!(
        get_formatted_sql(&config, sql.clone()),
        r#"
            SELECT * FROM TBL1
            WHERE ((C1 = 0 AND C2 = 0) OR (C1 = 1 AND C2 = 1))
"#
    );

    config.newlines = true;
    assert_eq!(
        get_formatted_sql(&config, sql.clone()),
        r#"            SELECT
                *
            FROM TBL1
            WHERE ((C1 = 0 AND C2 = 0) OR (C1 = 1 AND C2 = 1))"#
    );
}

#[test]
fn test_get_formatted_sql_top() {
    let mut config: Configuration = Configuration::new();
    let sql: String = String::from(
        r#"
            SELECT TOP 100 ID FROM TBL1;
            SELECT TOP (100) ID FROM TBL1;
            "#,
    );

    assert_eq!(
        get_formatted_sql(&config, sql.clone()),
        r#"
            SELECT TOP 100 ID FROM TBL1;
            SELECT TOP (100) ID FROM TBL1;
"#
    );

    config.newlines = true;
    assert_eq!(
        get_formatted_sql(&config, sql.clone()),
        r#"            SELECT TOP 100
                ID
            FROM TBL1;

            SELECT TOP (100)
                ID
            FROM TBL1;"#
    );
}

#[test]
fn test_get_formatted_sql_sub_query_inline() {
    let mut config: Configuration = Configuration::new();
    let sql: String = String::from(r#"SELECT ( SELECT TOP 1 ID FROM TBL1 ) AS ID"#);

    assert_eq!(
        get_formatted_sql(&config, sql.clone()),
        r#"SELECT (SELECT TOP 1 ID FROM TBL1) AS ID"#
    );

    config.newlines = true;
    assert_eq!(
        get_formatted_sql(&config, sql.clone()),
        r#"SELECT
    (SELECT TOP 1 ID FROM TBL1) AS ID"#
    );
}

#[test]
fn test_get_formatted_sql_union() {
    let mut config: Configuration = Configuration::new();
    let sql: String = String::from(r#"SELECT C1 UNION SELECT C2"#);

    assert_eq!(
        get_formatted_sql(&config, sql.clone()),
        r#"SELECT C1 UNION SELECT C2"#
    );

    config.newlines = true;
    assert_eq!(
        get_formatted_sql(&config, sql.clone()),
        r#"SELECT
    C1
UNION
SELECT
    C2"#
    );
}

#[test]
fn test_get_formatted_sql_union_complex() {
    let mut config: Configuration = Configuration::new();
    let sql: String = String::from(
        r#"
            SELECT C1 FROM TBL1
            UNION ALL SELECT C2 FROM TBL2
            UNION SELECT C3 FROM TBL3
            "#,
    );

    assert_eq!(
        get_formatted_sql(&config, sql.clone()),
        r#"
            SELECT C1 FROM TBL1
            UNION ALL SELECT C2 FROM TBL2
            UNION SELECT C3 FROM TBL3
"#
    );

    config.newlines = true;
    assert_eq!(
        get_formatted_sql(&config, sql.clone()),
        r#"            SELECT
                C1
            FROM TBL1
            UNION ALL
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
fn test_get_formatted_sql_transaction() {
    let mut config: Configuration = Configuration::new();
    let sql: String = String::from(
        r#"
            BEGIN TRY
            BEGIN TRANSACTION;
            CALL SP1();
            COMMIT TRANSACTION;
            END TRY
            BEGIN CATCH
            ROLLBACK TRANSACTION;
            END CATCH
            "#,
    );

    assert_eq!(
        get_formatted_sql(&config, sql.clone()),
        r#"
            BEGIN TRY
                BEGIN TRANSACTION;
                    CALL SP1();
                COMMIT TRANSACTION;
            END TRY
            BEGIN CATCH
            ROLLBACK TRANSACTION;
            END CATCH
"#
    );

    config.newlines = true;
    assert_eq!(
        get_formatted_sql(&config, sql.clone()),
        r#"            BEGIN TRY
                BEGIN TRANSACTION;
                    CALL SP1();
                COMMIT TRANSACTION;
            END TRY
            BEGIN CATCH
            ROLLBACK TRANSACTION;
            END CATCH"#
    );
}

#[test]
fn test_get_formatted_sql_transaction2() {
    let mut config: Configuration = Configuration::new();
    let sql: String = String::from(
        r#"
            BEGIN TRY
            BEGIN TRANSACTION
            SELECT 1
            COMMIT TRANSACTION
            END TRY
            BEGIN CATCH
            ROLLBACK TRANSACTION
            THROW
            END CATCH
            "#,
    );

    assert_eq!(
        get_formatted_sql(&config, sql.clone()),
        r#"
            BEGIN TRY
                BEGIN TRANSACTION
                    SELECT 1
                COMMIT TRANSACTION
            END TRY
            BEGIN CATCH
            ROLLBACK TRANSACTION
            THROW
            END CATCH
"#
    );

    config.newlines = true;
    assert_eq!(
        get_formatted_sql(&config, sql.clone()),
        r#"            BEGIN TRY
                BEGIN TRANSACTION
                    SELECT
                        1
                COMMIT TRANSACTION
            END TRY
            BEGIN CATCH
            ROLLBACK TRANSACTION
            THROW
            END CATCH"#
    );
}

#[test]
fn test_get_formatted_sql_comma_start_multiline() {
    let mut config: Configuration = Configuration::new();
    let sql: String = String::from(
        r#"
            SELECT
            C1
            ,C2
            ,C3
            FROM TBL1
            "#,
    );

    assert_eq!(
        get_formatted_sql(&config, sql.clone()),
        r#"
            SELECT
                C1
                , C2
                , C3
            FROM TBL1
"#
    );

    config.newlines = true;
    assert_eq!(
        get_formatted_sql(&config, sql.clone()),
        r#"            SELECT
                C1,
                C2,
                C3
            FROM TBL1"#
    );
}

#[test]
fn test_get_formatted_sql_sub_query_multiline() {
    let mut config: Configuration = Configuration::new();
    let sql: String = String::from(
        r#"
            SELECT (
            SELECT TOP 1 ID FROM TBL1
            ) AS ID,
            C1
            FROM TBL1
            "#,
    );

    assert_eq!(
        get_formatted_sql(&config, sql.clone()),
        r#"
            SELECT (
                    SELECT TOP 1 ID FROM TBL1
                ) AS ID,
                C1
            FROM TBL1
"#
    );

    config.newlines = true;
    assert_eq!(
        get_formatted_sql(&config, sql.clone()),
        r#"            SELECT
                (SELECT TOP 1 ID FROM TBL1) AS ID,
                C1
            FROM TBL1"#
    );
}

#[test]
fn test_get_formatted_sql_delimiter() {
    let mut config: Configuration = Configuration::new();
    let sql: String = String::from(
        r#"
            SELECT * FROM TBL1;DECLARE C1=1;DECLARE C2= 2;  DECLARE C3 = 3;SELECT * FROM TBL1  DECLARE C4=4;DECLARE C5=5;
            "#,
    );

    assert_eq!(
        get_formatted_sql(&config, sql.clone()),
        r#"
            SELECT * FROM TBL1; DECLARE C1 = 1; DECLARE C2 = 2; DECLARE C3 = 3; SELECT * FROM TBL1 DECLARE C4 = 4; DECLARE C5 = 5;
"#
    );

    config.newlines = true;
    assert_eq!(
        get_formatted_sql(&config, sql.clone()),
        r#"            SELECT
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
fn test_get_formatted_sql_comments_only() {
    let mut config: Configuration = Configuration::new();
    let sql: String = String::from(
        r#"
-- COMMENT
  -- COMMENT
    -- COMMENT
  -- COMMENT
-- COMMENT
-- COMMENT
/*COMMENT*//*COMMENT*/ /*COMMENT*/
    /*COMMENT*/ /*COMMENT*/
    /*COMMENT*/ /*COMMENT*/
                    "#,
    );

    assert_eq!(
        get_formatted_sql(&config, sql.clone()),
        r#"
-- COMMENT
  -- COMMENT
    -- COMMENT
  -- COMMENT
-- COMMENT
-- COMMENT
/*COMMENT*//*COMMENT*/ /*COMMENT*/
    /*COMMENT*/ /*COMMENT*/
    /*COMMENT*/ /*COMMENT*/
"#
    );

    config.newlines = true;
    assert_eq!(
        get_formatted_sql(&config, sql.clone()),
        r#"-- COMMENT
  -- COMMENT
    -- COMMENT
  -- COMMENT
-- COMMENT
-- COMMENT
/*COMMENT*//*COMMENT*/
/*COMMENT*/
    /*COMMENT*/
/*COMMENT*/
    /*COMMENT*/
/*COMMENT*/"#
    );

    config.comment_pre_space = true;
    assert_eq!(
        get_formatted_sql(&config, sql.clone()),
        r#"-- COMMENT
-- COMMENT
-- COMMENT
-- COMMENT
-- COMMENT
-- COMMENT
/*COMMENT*//*COMMENT*/
/*COMMENT*/
/*COMMENT*/
/*COMMENT*/
/*COMMENT*/
/*COMMENT*/"#
    );
}

#[test]
fn test_get_formatted_sql_comments_with_statements() {
    let mut config: Configuration = Configuration::new();
    let sql: String = String::from(
        r#"
    -- COMMENT
        SELECT 1;
    -- COMMENT
    SELECT 1;
-- COMMENT
SELECT 1;
    -- COMMENT
    -- COMMENT
    SELECT 1;
        -- COMMENT
        SELECT 1;
        -- COMMENT
        SELECT 1
    -- COMMENT
    SELECT 1
-- COMMENT
SELECT 1
    -- COMMENT
    SELECT 1
        -- COMMENT
        SELECT 1
            "#,
    );

    assert_eq!(
        get_formatted_sql(&config, sql.clone()),
        r#"
    -- COMMENT
    SELECT 1;
    -- COMMENT
    SELECT 1;
-- COMMENT
    SELECT 1;
    -- COMMENT
    -- COMMENT
    SELECT 1;
        -- COMMENT
    SELECT 1;
        -- COMMENT
    SELECT 1
    -- COMMENT
    SELECT 1
-- COMMENT
    SELECT 1
    -- COMMENT
    SELECT 1
        -- COMMENT
    SELECT 1
"#
    );

    config.newlines = true;
    assert_eq!(
        get_formatted_sql(&config, sql.clone()),
        r#"    -- COMMENT
    SELECT
        1;

    -- COMMENT
    SELECT
        1;

-- COMMENT
    SELECT
        1;

    -- COMMENT
    -- COMMENT
    SELECT
        1;

        -- COMMENT
    SELECT
        1;

        -- COMMENT
    SELECT
        1
    -- COMMENT
    SELECT
        1
-- COMMENT
    SELECT
        1
    -- COMMENT
    SELECT
        1
        -- COMMENT
    SELECT
        1"#
    );

    config.comment_pre_space = true;
    assert_eq!(
        get_formatted_sql(&config, sql.clone()),
        r#"    -- COMMENT
    SELECT
        1;

    -- COMMENT
    SELECT
        1;

    -- COMMENT
    SELECT
        1;

    -- COMMENT
    -- COMMENT
    SELECT
        1;

    -- COMMENT
    SELECT
        1;

    -- COMMENT
    SELECT
        1
    -- COMMENT
    SELECT
        1
    -- COMMENT
    SELECT
        1
    -- COMMENT
    SELECT
        1
    -- COMMENT
    SELECT
        1"#
    );
}

#[test]
fn test_get_formatted_sql_delimiter_comment() {
    let mut config: Configuration = Configuration::new();
    let sql: String = String::from(
        r#"
            -- COMMENT
            DECLARE C1=1;DECLARE C2=2;
            -- COMMENT
            DECLARE C1=1;DECLARE C2=2;
            "#,
    );

    assert_eq!(
        get_formatted_sql(&config, sql.clone()),
        r#"
            -- COMMENT
            DECLARE C1 = 1; DECLARE C2 = 2;
            -- COMMENT
            DECLARE C1 = 1; DECLARE C2 = 2;
"#
    );

    config.newlines = true;
    assert_eq!(
        get_formatted_sql(&config, sql.clone()),
        r#"            -- COMMENT
            DECLARE C1 = 1;
            DECLARE C2 = 2;

            -- COMMENT
            DECLARE C1 = 1;
            DECLARE C2 = 2;"#
    );

    config.comment_pre_space = true;
    assert_eq!(
        get_formatted_sql(&config, sql.clone()),
        r#"            -- COMMENT
            DECLARE C1 = 1;
            DECLARE C2 = 2;

            -- COMMENT
            DECLARE C1 = 1;
            DECLARE C2 = 2;"#
    );
}

#[test]
fn test_get_formatted_sql_delimiter_change() {
    let mut config: Configuration = Configuration::new();
    let sql: String = String::from(
        r#"
            SELECT 1;DELIMITER $$ SELECT 1; DELIMITER ;
            "#,
    );

    assert_eq!(
        get_formatted_sql(&config, sql.clone()),
        r#"
            SELECT 1; DELIMITER $$ SELECT 1; DELIMITER ;
"#
    );

    config.newlines = true;
    assert_eq!(
        get_formatted_sql(&config, sql.clone()),
        r#"            SELECT
                1;

            DELIMITER $$

            SELECT
                1;
            DELIMITER ;"#
    );
}

#[test]
fn test_get_formatted_sql_declare_no_delimiter() {
    let mut config: Configuration = Configuration::new();
    let sql: String = String::from(r#"DECLARE C1 = 1 DECLARE C2 = 2   DECLARE C3 = 3 "#);

    assert_eq!(
        get_formatted_sql(&config, sql.clone()),
        r#"DECLARE C1 = 1 DECLARE C2 = 2 DECLARE C3 = 3"#
    );

    config.newlines = true;
    assert_eq!(
        get_formatted_sql(&config, sql.clone()),
        r#"DECLARE C1 = 1
DECLARE C2 = 2
DECLARE C3 = 3"#
    );
}

#[test]
fn test_get_formatted_sql_multiple_declare() {
    let mut config: Configuration = Configuration::new();
    let sql: String = String::from(r#"DECLARE C1 = 1, C2 = 2, C3 = 3;"#);

    assert_eq!(
        get_formatted_sql(&config, sql.clone()),
        r#"DECLARE C1 = 1, C2 = 2, C3 = 3;"#
    );

    config.newlines = true;
    assert_eq!(
        get_formatted_sql(&config, sql.clone()),
        r#"DECLARE C1 = 1,
    C2 = 2,
    C3 = 3;"#
    );
}

#[test]
fn test_get_formatted_sql_set_no_delimiter() {
    let mut config: Configuration = Configuration::new();
    let sql: String = String::from(r#"SET C1 = 1 SET C2 = 2   SET C3 = 3 "#);

    assert_eq!(
        get_formatted_sql(&config, sql.clone()),
        r#"SET C1 = 1 SET C2 = 2 SET C3 = 3"#
    );

    config.newlines = true;
    assert_eq!(
        get_formatted_sql(&config, sql.clone()),
        r#"SET C1 = 1
SET C2 = 2
SET C3 = 3"#
    );
}

#[test]
fn test_get_formatted_sql_set() {
    let mut config: Configuration = Configuration::new();
    let sql: String = String::from(
        r#"
            SET C1 = 1
            SET C2 = 2
            "#,
    );

    assert_eq!(
        get_formatted_sql(&config, sql.clone()),
        r#"
            SET C1 = 1
            SET C2 = 2
"#
    );

    config.newlines = true;
    assert_eq!(
        get_formatted_sql(&config, sql.clone()),
        r#"            SET C1 = 1
            SET C2 = 2"#
    );
}

#[test]
fn test_get_formatted_sql_update() {
    let mut config: Configuration = Configuration::new();
    let sql: String = String::from(
        r#"
            UPDATE TBL1
            SET
            C1 = 1,
            C2 = 2
            WHERE C3 = 3
            "#,
    );

    assert_eq!(
        get_formatted_sql(&config, sql.clone()),
        r#"
            UPDATE TBL1
            SET
                C1 = 1,
                C2 = 2
            WHERE C3 = 3
"#
    );

    config.newlines = true;
    assert_eq!(
        get_formatted_sql(&config, sql.clone()),
        r#"            UPDATE TBL1
            SET C1 = 1,
                C2 = 2
            WHERE C3 = 3"#
    );
}

#[test]
fn test_get_formatted_sql_select_where_quote() {
    let mut config: Configuration = Configuration::new();
    let sql: String = String::from(
        r#"
            SELECT *
            FROM TBL1
            WHERE C1 = 'some value'
            "#,
    );

    assert_eq!(
        get_formatted_sql(&config, sql.clone()),
        r#"
            SELECT *
            FROM TBL1
            WHERE C1 = 'some value'
"#
    );

    config.newlines = true;
    assert_eq!(
        get_formatted_sql(&config, sql.clone()),
        r#"            SELECT
                *
            FROM TBL1
            WHERE C1 = 'some value'"#
    );
}

#[test]
fn test_get_formatted_sql_select_where_in() {
    let mut config: Configuration = Configuration::new();
    let sql: String = String::from(
        r#"
            SELECT *
            FROM TBL1
            WHERE C1 IN ('VALUE 1','VALUE 2','VALUE 3','VALUE 4','VALUE 5','VALUE 6');
            SELECT *
            FROM TBL1
            WHERE C1 IN (
                'VALUE 1',
                'VALUE 2',
                'VALUE 3',
                'VALUE 4',
                'VALUE 5',
                'VALUE 6'
            );
            "#,
    );

    assert_eq!(
        get_formatted_sql(&config, sql.clone()),
        r#"
            SELECT *
            FROM TBL1
            WHERE C1 IN ('VALUE 1', 'VALUE 2', 'VALUE 3', 'VALUE 4', 'VALUE 5', 'VALUE 6');
            SELECT *
            FROM TBL1
            WHERE C1 IN (
                    'VALUE 1',
                    'VALUE 2',
                    'VALUE 3',
                    'VALUE 4',
                    'VALUE 5',
                    'VALUE 6'
                );
"#
    );

    config.newlines = true;
    assert_eq!(
        get_formatted_sql(&config, sql.clone()),
        r#"            SELECT
                *
            FROM TBL1
            WHERE C1 IN (
                    'VALUE 1',
                    'VALUE 2',
                    'VALUE 3',
                    'VALUE 4',
                    'VALUE 5',
                    'VALUE 6'
                );

            SELECT
                *
            FROM TBL1
            WHERE C1 IN (
                    'VALUE 1',
                    'VALUE 2',
                    'VALUE 3',
                    'VALUE 4',
                    'VALUE 5',
                    'VALUE 6'
                );"#
    );
}

#[test]
fn test_get_formatted_sql_select_where_in_cr() {
    let mut config: Configuration = Configuration::new();
    let sql: String = String::from(
        r#"
            SELECT *
            FROM TBL1
            WHERE C1 IN (
                'VALUE 1',
                'VALUE 2',
                'VALUE 3',
                'VALUE 4',
                'VALUE 5',
                'VALUE 6'
            );
            "#,
    );
    let sql: String = sql.replace('\n', "\r\n");

    assert_eq!(
        get_formatted_sql(&config, sql.clone()),
        r#"
            SELECT *
            FROM TBL1
            WHERE C1 IN (
                    'VALUE 1',
                    'VALUE 2',
                    'VALUE 3',
                    'VALUE 4',
                    'VALUE 5',
                    'VALUE 6'
                );
"#
    );

    config.newlines = true;
    assert_eq!(
        get_formatted_sql(&config, sql.clone()),
        r#"            SELECT
                *
            FROM TBL1
            WHERE C1 IN (
                    'VALUE 1',
                    'VALUE 2',
                    'VALUE 3',
                    'VALUE 4',
                    'VALUE 5',
                    'VALUE 6'
                );"#
    );
}

#[test]
fn test_get_formatted_sql_distinct_from() {
    let mut config: Configuration = Configuration::new();
    let sql: String = String::from(
        r#"
            UPDATE T1
            FROM TBL1 AS T1
            INNER JOIN TBL2 AS T2 ON T2.C1 = T1.C1
            WHERE T1.C2 IS DISTINCT FROM T2.C2
            OR T1.C3 IS DISTINCT FROM T2.C3
            "#,
    );

    assert_eq!(
        get_formatted_sql(&config, sql.clone()),
        r#"
            UPDATE T1
            FROM TBL1 AS T1
                INNER JOIN TBL2 AS T2 ON T2.C1 = T1.C1
            WHERE T1.C2 IS DISTINCT FROM T2.C2
                OR T1.C3 IS DISTINCT FROM T2.C3
"#
    );

    config.newlines = true;
    assert_eq!(
        get_formatted_sql(&config, sql.clone()),
        r#"            UPDATE T1
            FROM TBL1 AS T1
                INNER JOIN TBL2 AS T2 ON T2.C1 = T1.C1
            WHERE T1.C2 IS DISTINCT FROM T2.C2
                OR T1.C3 IS DISTINCT FROM T2.C3"#
    );
}

#[test]
fn test_get_formatted_sql_count_distinct() {
    let mut config: Configuration = Configuration::new();
    let sql: String = String::from(
        r#"
            SELECT COUNT(DISTINCT YEAR(D1))
            FROM TBL1
            "#,
    );

    assert_eq!(
        get_formatted_sql(&config, sql.clone()),
        r#"
            SELECT COUNT(DISTINCT YEAR(D1))
            FROM TBL1
"#
    );

    config.newlines = true;
    assert_eq!(
        get_formatted_sql(&config, sql.clone()),
        r#"            SELECT
                COUNT(DISTINCT YEAR(D1))
            FROM TBL1"#
    );
}

#[test]
fn test_get_formatted_sql_option() {
    let mut config: Configuration = Configuration::new();
    let sql: String = String::from(
        r#"
            SELECT C1 FROM TBL1 OPTION (RECOMPILE)
            "#,
    );

    assert_eq!(
        get_formatted_sql(&config, sql.clone()),
        r#"
            SELECT C1 FROM TBL1 OPTION (RECOMPILE)
"#
    );

    config.newlines = true;
    assert_eq!(
        get_formatted_sql(&config, sql.clone()),
        r#"            SELECT
                C1
            FROM TBL1
            OPTION (RECOMPILE)"#
    );
}

#[test]
fn test_get_formatted_sql_paren_count() {
    let mut config: Configuration = Configuration::new();
    let sql: String = String::from(
        r#"
            (SELECT COUNT(*) FROM TBL1)
            "#,
    );

    assert_eq!(
        get_formatted_sql(&config, sql.clone()),
        r#"
            (SELECT COUNT(*) FROM TBL1)
"#
    );

    config.newlines = true;
    config.chars = 10;
    assert_eq!(
        get_formatted_sql(&config, sql.clone()),
        r#"            (
                SELECT
                    COUNT(*)
                FROM TBL1
            )"#
    );

    config.align_text_groups = true;
    assert_eq!(
        get_formatted_sql(&config, sql.clone()),
        r#"            (
                SELECT
                    COUNT(*)
                FROM TBL1
            )"#
    );
}

#[test]
fn test_get_formatted_sql_non_ascii_line_length() {
    let mut config: Configuration = Configuration::new();
    let sql: String = String::from(r#"SELECT * FROM TBL1 WHERE (C1 = 'ééééééééé' AND C2 = 1)"#);

    config.newlines = true;
    config.chars = 40;
    assert_eq!(
        get_formatted_sql(&config, sql.clone()),
        r#"SELECT
    *
FROM TBL1
WHERE (C1 = 'ééééééééé' AND C2 = 1)"#
    );
}

#[test]
fn test_get_formatted_sql_align_text_groups_non_ascii() {
    let mut config: Configuration = Configuration::new();
    let sql: String = String::from(
        r#"CREATE TABLE TBL1 (
    PRÉNOM VARCHAR(10) NOT NULL,
    NOM INT NULL,
    ÂGÉ INT
)"#,
    );

    config.align_text_groups = true;
    assert_eq!(
        get_formatted_sql(&config, sql.clone()),
        r#"CREATE TABLE TBL1(
    PRÉNOM VARCHAR(10) NOT NULL,
    NOM    INT         NULL,
    ÂGÉ    INT
)"#
    );
}

#[test]
fn test_get_formatted_sql_align_text_groups_leading_comma() {
    let mut config: Configuration = Configuration::new();
    let sql: String = String::from(
        r#"CREATE TABLE TBL1 (C1 INT NOT NULL
, COLUMN2 VARCHAR(10) NULL
, C3 INT
, C4 DATETIME NOT NULL)"#,
    );

    config.align_text_groups = true;
    assert_eq!(
        get_formatted_sql(&config, sql.clone()),
        r#"CREATE TABLE TBL1(C1 INT NOT NULL
    , COLUMN2 VARCHAR(10) NULL
    , C3      INT
    , C4      DATETIME    NOT NULL)"#
    );
}

#[test]
fn test_get_formatted_sql_paren_collapse() {
    let mut config: Configuration = Configuration::new();
    let sql: String = String::from(
        r#"
            BEGIN
            SELECT
            REALLY_REALLY_REALLY_LONG_STORED_PROCEDURE_NAME_NO_PARAMETER(),
            REALLY_REALLY_REALLY_LONG_STORED_PROCEDURE_NAME_ONE_PARAMETER(P1),
            REALLY_REALLY_REALLY_LONG_STORED_PROCEDURE_NAME_TWO_PARAMETER(P1,P2),
            ROUND((LENGTH(LONG_VARIABLE_NAME) - LENGTH(REPLACE(LONG_VARIABLE_NAME, '_____', ''))) / LENGTH('_____')) AS BLANKCOUNT
            END
            "#,
    );

    assert_eq!(
        get_formatted_sql(&config, sql.clone()),
        r#"
            BEGIN
                SELECT
                    REALLY_REALLY_REALLY_LONG_STORED_PROCEDURE_NAME_NO_PARAMETER(),
                    REALLY_REALLY_REALLY_LONG_STORED_PROCEDURE_NAME_ONE_PARAMETER(P1),
                    REALLY_REALLY_REALLY_LONG_STORED_PROCEDURE_NAME_TWO_PARAMETER(P1, P2),
                    ROUND((LENGTH(LONG_VARIABLE_NAME) - LENGTH(REPLACE(LONG_VARIABLE_NAME, '_____', ''))) / LENGTH('_____')) AS BLANKCOUNT
            END
"#
    );

    config.newlines = true;
    assert_eq!(
        get_formatted_sql(&config, sql.clone()),
        r#"            BEGIN
                SELECT
                    REALLY_REALLY_REALLY_LONG_STORED_PROCEDURE_NAME_NO_PARAMETER(),
                    REALLY_REALLY_REALLY_LONG_STORED_PROCEDURE_NAME_ONE_PARAMETER(P1),
                    REALLY_REALLY_REALLY_LONG_STORED_PROCEDURE_NAME_TWO_PARAMETER(
                        P1,
                        P2
                    ),
                    ROUND(
                        (
                            LENGTH(LONG_VARIABLE_NAME) -
                            LENGTH(REPLACE(LONG_VARIABLE_NAME, '_____', ''))
                        ) / LENGTH('_____')
                    ) AS BLANKCOUNT
            END"#
    );
}

#[test]
fn test_get_formatted_sql_select_agg() {
    let mut config: Configuration = Configuration::new();
    let sql: String = String::from(
        r#"
            SELECT STRING_AGG(C1,', ')
            WITHIN GROUP (ORDER BY C2 DESC) AS A1
            FROM TBL1
            "#,
    );

    assert_eq!(
        get_formatted_sql(&config, sql.clone()),
        r#"
            SELECT STRING_AGG(C1, ', ')
                WITHIN GROUP (ORDER BY C2 DESC) AS A1
            FROM TBL1
"#
    );

    config.newlines = true;
    assert_eq!(
        get_formatted_sql(&config, sql.clone()),
        r#"            SELECT
                STRING_AGG(C1, ', ') WITHIN GROUP (ORDER BY C2 DESC) AS A1
            FROM TBL1"#
    );
}

#[test]
fn test_get_formatted_sql_select_group_by() {
    let mut config: Configuration = Configuration::new();
    let sql: String = String::from(
        r#"
            SELECT C1,
            COUNT(*) AS CNT
            FROM TBL1
            WHERE C2 IS NULL
            GROUP BY C1
            HAVING COUNT(*) > 1
            "#,
    );

    assert_eq!(
        get_formatted_sql(&config, sql.clone()),
        r#"
            SELECT C1,
                COUNT(*) AS CNT
            FROM TBL1
            WHERE C2 IS NULL
            GROUP BY C1
            HAVING COUNT(*) > 1
"#
    );

    config.newlines = true;
    assert_eq!(
        get_formatted_sql(&config, sql.clone()),
        r#"            SELECT
                C1,
                COUNT(*) AS CNT
            FROM TBL1
            WHERE C2 IS NULL
            GROUP BY C1
            HAVING COUNT(*) > 1"#
    );
}

#[test]
fn test_get_formatted_sql_join() {
    let mut config: Configuration = Configuration::new();
    let sql: String = String::from(
        r#"
            SELECT T1.C1, T1.C2,
            T2.C2
            FROM TBL1 AS T1
            INNER JOIN TBL2 AS T2 ON T2.C1 = T1.C1
            "#,
    );

    assert_eq!(
        get_formatted_sql(&config, sql.clone()),
        r#"
            SELECT T1.C1, T1.C2,
                T2.C2
            FROM TBL1 AS T1
                INNER JOIN TBL2 AS T2 ON T2.C1 = T1.C1
"#
    );

    config.newlines = true;
    assert_eq!(
        get_formatted_sql(&config, sql.clone()),
        r#"            SELECT
                T1.C1,
                T1.C2,
                T2.C2
            FROM TBL1 AS T1
                INNER JOIN TBL2 AS T2 ON T2.C1 = T1.C1"#
    );
}

#[test]
fn test_get_formatted_sql_select_where() {
    let mut config: Configuration = Configuration::new();
    let sql: String = String::from(
        r#"
            SELECT
            C1,
            C2,
            C3
            FROM TBL1
            WHERE C1>1
            AND C2 IS NOT NULL
            "#,
    );

    assert_eq!(
        get_formatted_sql(&config, sql.clone()),
        r#"
            SELECT
                C1,
                C2,
                C3
            FROM TBL1
            WHERE C1 > 1
                AND C2 IS NOT NULL
"#
    );

    config.newlines = true;
    assert_eq!(
        get_formatted_sql(&config, sql.clone()),
        r#"            SELECT
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
    let mut config: Configuration = Configuration::new();
    let sql: String = String::from(
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
            "#,
    );

    assert_eq!(
        get_formatted_sql(&config, sql.clone()),
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
            WHERE (T1.C2 <> T2.C2 OR T1.C2 <> T3.C2)
            ORDER BY T1.C1
            LIMIT 1
"#
    );

    config.newlines = true;
    assert_eq!(
        get_formatted_sql(&config, sql.clone()),
        r#"            SELECT DISTINCT
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
    let mut config: Configuration = Configuration::new();
    let sql: String = String::from(r#"SELECT * FROM TBL1;SELECT * FROM TBL1;"#);

    assert_eq!(
        get_formatted_sql(&config, sql.clone()),
        String::from("SELECT * FROM TBL1; SELECT * FROM TBL1;")
    );

    config.newlines = true;
    assert_eq!(
        get_formatted_sql(&config, sql.clone()),
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
    let mut config: Configuration = Configuration::new();
    let sql: String = String::from(
        r#"
            -- top comment
            SELECT C1,--inline comment
            -- after comment 1
            -- after comment 2
            C2
            -- after comment 3
            FROM TBL1
            "#,
    );

    assert_eq!(
        get_formatted_sql(&config, sql.clone()),
        r#"
            -- top comment
            SELECT C1, --inline comment
            -- after comment 1
            -- after comment 2
                C2
            -- after comment 3
            FROM TBL1
"#
    );

    config.newlines = true;
    assert_eq!(
        get_formatted_sql(&config, sql.clone()),
        r#"            -- top comment
            SELECT
                C1, --inline comment
            -- after comment 1
            -- after comment 2
                C2
            -- after comment 3
            FROM TBL1"#
    );

    config.comment_pre_space = true;
    assert_eq!(
        get_formatted_sql(&config, sql.clone()),
        r#"            -- top comment
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
    let mut config: Configuration = Configuration::new();
    let sql: String = String::from(
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
    );

    assert_eq!(
        get_formatted_sql(&config, sql.clone()),
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
"#
    );

    config.newlines = true;
    assert_eq!(
        get_formatted_sql(&config, sql.clone()),
        r#"            -- comment
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

    config.comment_pre_space = true;
    assert_eq!(
        get_formatted_sql(&config, sql.clone()),
        r#"            -- comment
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
    let mut config: Configuration = Configuration::new();
    let sql: String = String::from(
        r#"
            SELECT C1
            FROM TBL1
            ORDER BY C1

            -- COMMENT
            SET V1 = 1
            "#,
    );

    assert_eq!(
        get_formatted_sql(&config, sql.clone()),
        r#"
            SELECT C1
            FROM TBL1
            ORDER BY C1

            -- COMMENT
            SET V1 = 1
"#
    );

    config.newlines = true;
    assert_eq!(
        get_formatted_sql(&config, sql.clone()),
        r#"            SELECT
                C1
            FROM TBL1
            ORDER BY C1
            -- COMMENT
            SET V1 = 1"#
    );

    config.comment_pre_space = true;
    assert_eq!(
        get_formatted_sql(&config, sql.clone()),
        r#"            SELECT
                C1
            FROM TBL1
            ORDER BY C1
            -- COMMENT
            SET V1 = 1"#
    );
}

#[test]
fn test_get_formatted_sql_single_comment_inside_paren() {
    let mut config: Configuration = Configuration::new();
    let sql: String = String::from(
        r#"SELECT * FROM TBL1 WHERE C1 IN (1, -- one
2) AND C2 = COUNT(C3 -- note
)"#,
    );

    config.newlines = true;
    assert_eq!(
        get_formatted_sql(&config, sql.clone()),
        r#"SELECT
    *
FROM TBL1
WHERE C1 IN (
        1, -- one
        2
    )
    AND C2 = COUNT(
        C3 -- note
    )"#
    );

    config.comment_pre_space = true;
    assert_eq!(
        get_formatted_sql(&config, sql.clone()),
        r#"SELECT
    *
FROM TBL1
WHERE C1 IN (
        1,
        -- one
        2
    )
    AND C2 = COUNT(
        C3
    -- note
    )"#
    );
}

#[test]
fn test_get_formatted_sql_multiline_comment_inside_paren() {
    let mut config: Configuration = Configuration::new();
    let sql: String = String::from(r#"SELECT (C1 /* note */ + C2) FROM TBL1"#);

    config.newlines = true;
    assert_eq!(
        get_formatted_sql(&config, sql.clone()),
        r#"SELECT
    (C1 /* note */ + C2)
FROM TBL1"#
    );
}

#[test]
fn test_get_formatted_sql_multiline_comments() {
    let mut config: Configuration = Configuration::new();
    let sql: String = String::from(
        r#"
            /* top comment */
            SELECT C1/* inline comment */
            /*

            after

            comment
                indent

            */FROM TBL1
            "#,
    );

    assert_eq!(
        get_formatted_sql(&config, sql.clone()),
        r#"
            /* top comment */
            SELECT C1 /* inline comment */
            /*

            after

            comment
                indent

            */ FROM TBL1
"#
    );

    config.newlines = true;
    assert_eq!(
        get_formatted_sql(&config, sql.clone()),
        r#"            /* top comment */
            SELECT
                C1 /* inline comment */
            /*

            after

            comment
                indent

            */
            FROM TBL1"#
    );

    config.comment_pre_space = true;
    assert_eq!(
        get_formatted_sql(&config, sql.clone()),
        r#"            /* top comment */
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
    let mut config: Configuration = Configuration::new();
    let sql: String = String::from(
        r#"
            SELECT
            C1,
            C2,
            C3
            INTO TBL2
            FROM TBL1
            "#,
    );

    assert_eq!(
        get_formatted_sql(&config, sql.clone()),
        r#"
            SELECT
                C1,
                C2,
                C3
            INTO TBL2
            FROM TBL1
"#
    );

    config.newlines = true;
    assert_eq!(
        get_formatted_sql(&config, sql.clone()),
        r#"            SELECT
                C1,
                C2,
                C3
            INTO
                TBL2
            FROM TBL1"#
    );
}

#[test]
fn test_get_formatted_sql_with_nolock() {
    let mut config: Configuration = Configuration::new();
    let sql: String = String::from(
        r#"
            SELECT C1 FROM TBL1 WITH (NOLOCK)
            SELECT C1 FROM TBL1 WITH (NOLOCK)
            "#,
    );

    assert_eq!(
        get_formatted_sql(&config, sql.clone()),
        r#"
            SELECT C1 FROM TBL1 WITH (NOLOCK)
            SELECT C1 FROM TBL1 WITH (NOLOCK)
"#
    );

    config.newlines = true;
    assert_eq!(
        get_formatted_sql(&config, sql.clone()),
        r#"            SELECT
                C1
            FROM TBL1 WITH (NOLOCK)
            SELECT
                C1
            FROM TBL1 WITH (NOLOCK)"#
    );
}

#[test]
fn test_get_formatted_sql_cte_after_select() {
    let mut config: Configuration = Configuration::new();
    let sql: String = String::from(
        r#"
            SELECT C1 FROM TBL1;
            WITH CTE2 AS
            (SELECT C2 FROM TBL2)
            SELECT * FROM CTE2
            "#,
    );

    assert_eq!(
        get_formatted_sql(&config, sql.clone()),
        r#"
            SELECT C1 FROM TBL1;
            WITH CTE2 AS
                (SELECT C2 FROM TBL2)
            SELECT * FROM CTE2
"#
    );

    config.newlines = true;
    assert_eq!(
        get_formatted_sql(&config, sql.clone()),
        r#"            SELECT
                C1
            FROM TBL1;

            WITH CTE2 AS (SELECT C2 FROM TBL2)
            SELECT
                *
            FROM CTE2"#
    );
}

#[test]
fn test_get_formatted_sql_insert_after_cte() {
    let mut config: Configuration = Configuration::new();
    let sql: String = String::from(
        r#"
            WITH CTE1 AS
            (SELECT C1 FROM TBL1)
            INSERT INTO TBL2 (C1)
            SELECT C1 FROM CTE1
            "#,
    );

    assert_eq!(
        get_formatted_sql(&config, sql.clone()),
        r#"
            WITH CTE1 AS
                (SELECT C1 FROM TBL1)
            INSERT INTO TBL2(C1)
            SELECT C1 FROM CTE1
"#
    );

    config.newlines = true;
    assert_eq!(
        get_formatted_sql(&config, sql.clone()),
        r#"            WITH CTE1 AS (SELECT C1 FROM TBL1)
            INSERT INTO TBL2(C1)
            SELECT
                C1
            FROM CTE1"#
    );
}

#[test]
fn test_get_formatted_sql_insert_after_cte_config_newline_long() {
    let mut config: Configuration = Configuration::new();
    let sql: String = String::from(
        r#"
            WITH CTE1 AS
            (SELECT C00000000000000000000000000000,C00000000000000000000000000001,C00000000000000000000000000002 FROM TBL1)
            INSERT INTO TBL2 (C00000000000000000000000000000,C00000000000000000000000000001,C00000000000000000000000000002)
            SELECT C00000000000000000000000000000,C00000000000000000000000000001,C00000000000000000000000000002 FROM CTE1
            "#,
    );

    config.newlines = true;
    assert_eq!(
        get_formatted_sql(&config, sql.clone()),
        r#"            WITH CTE1 AS (
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
    let mut config: Configuration = Configuration::new();
    let sql: String = String::from(
        r#"
            WITH CTE1 AS
            (SELECT C1 FROM TBL1),
            CTE2 AS
            (SELECT C2 FROM TBL2)
            SELECT * FROM CTE1
            INNER JOIN CTE2 ON CTE2.C2 = CTE1.C1
            "#,
    );

    assert_eq!(
        get_formatted_sql(&config, sql.clone()),
        r#"
            WITH CTE1 AS
                (SELECT C1 FROM TBL1),
                CTE2 AS
                (SELECT C2 FROM TBL2)
            SELECT * FROM CTE1
                INNER JOIN CTE2 ON CTE2.C2 = CTE1.C1
"#
    );

    config.newlines = true;
    assert_eq!(
        get_formatted_sql(&config, sql.clone()),
        r#"            WITH CTE1 AS (SELECT C1 FROM TBL1),
                CTE2 AS (SELECT C2 FROM TBL2)
            SELECT
                *
            FROM CTE1
                INNER JOIN CTE2 ON CTE2.C2 = CTE1.C1"#
    );
}

#[test]
fn test_get_formatted_sql_join_subquery() {
    let mut config: Configuration = Configuration::new();
    let sql: String = String::from(
        r#"
            SELECT * FROM T1
            LEFT JOIN (SELECT C2 FROM T2) AS ST1 ON ST1.C2 = T1.C1
            "#,
    );

    assert_eq!(
        get_formatted_sql(&config, sql.clone()),
        r#"
            SELECT * FROM T1
                LEFT JOIN (SELECT C2 FROM T2) AS ST1 ON ST1.C2 = T1.C1
"#
    );

    config.newlines = true;
    assert_eq!(
        get_formatted_sql(&config, sql.clone()),
        r#"            SELECT
                *
            FROM T1
                LEFT JOIN (SELECT C2 FROM T2) AS ST1 ON ST1.C2 = T1.C1"#
    );
}

#[test]
fn test_get_formatted_sql_join_multi_condition() {
    let mut config: Configuration = Configuration::new();
    let sql: String = String::from(
        r#"
            SELECT * FROM T1
            RIGHT JOIN T2 ON T2.C1 = T1.C1 OR T2.C2 = T1.C2
            "#,
    );

    assert_eq!(
        get_formatted_sql(&config, sql.clone()),
        r#"
            SELECT * FROM T1
                RIGHT JOIN T2 ON T2.C1 = T1.C1 OR T2.C2 = T1.C2
"#
    );

    config.newlines = true;
    assert_eq!(
        get_formatted_sql(&config, sql.clone()),
        r#"            SELECT
                *
            FROM T1
                RIGHT JOIN T2 ON T2.C1 = T1.C1
                OR T2.C2 = T1.C2"#
    );
}

#[test]
fn test_get_formatted_sql_join_multi_condition_embedded() {
    let mut config: Configuration = Configuration::new();
    let sql: String = String::from(
        r#"
            SELECT * FROM T1
            FULL OUTER JOIN T2 ON (T2.C1 = T1.C1 OR T2.C2 = T1.C2)
            AND (T2.C3 = T1.C3 OR T2.C4 = T1.C4)
            "#,
    );

    assert_eq!(
        get_formatted_sql(&config, sql.clone()),
        r#"
            SELECT * FROM T1
                FULL OUTER JOIN T2 ON (T2.C1 = T1.C1 OR T2.C2 = T1.C2)
                AND (T2.C3 = T1.C3 OR T2.C4 = T1.C4)
"#
    );

    config.newlines = true;
    assert_eq!(
        get_formatted_sql(&config, sql.clone()),
        r#"            SELECT
                *
            FROM T1
                FULL OUTER JOIN T2 ON (T2.C1 = T1.C1 OR T2.C2 = T1.C2)
                AND (T2.C3 = T1.C3 OR T2.C4 = T1.C4)"#
    );
}

#[test]
fn test_get_formatted_sql_select_if() {
    let mut config: Configuration = Configuration::new();
    let sql: String = String::from(
        r#"
            SELECT
                C1,
                IIF(C1 > 5, 1, 0) AS 'IIF',
                C2,
                IF(C2 > 5, 1, 0) AS 'IF'
            FROM TBL1
            "#,
    );

    assert_eq!(
        get_formatted_sql(&config, sql.clone()),
        r#"
            SELECT
                C1,
                IIF(C1 > 5, 1, 0) AS 'IIF',
                C2,
            IF(C2 > 5, 1, 0) AS 'IF'
            FROM TBL1
"#,
    );

    config.newlines = true;
    assert_eq!(
        get_formatted_sql(&config, sql.clone()),
        r#"            SELECT
                C1,
                IIF(C1 > 5, 1, 0) AS 'IIF',
                C2,
            IF(C2 > 5, 1, 0) AS 'IF'
            FROM TBL1"#,
    );
}

#[test]
fn test_get_formatted_sql_case() {
    let mut config: Configuration = Configuration::new();
    let sql: String = String::from(
        r#"
            SELECT
            C1,
            CASE WHEN C1<=1 THEN 'small'
            WHEN C1<=3 THEN 'medium'
            ELSE 'large' END AS C2,
            C3
            FROM TBL1
            "#,
    );

    assert_eq!(
        get_formatted_sql(&config, sql.clone()),
        r#"
            SELECT
                C1,
                CASE WHEN C1 <= 1 THEN 'small'
                    WHEN C1 <= 3 THEN 'medium'
                    ELSE 'large' END AS C2,
                C3
            FROM TBL1
"#
    );

    config.newlines = true;
    assert_eq!(
        get_formatted_sql(&config, sql.clone()),
        r#"            SELECT
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
fn test_get_formatted_sql_case_nested() {
    let mut config: Configuration = Configuration::new();
    let sql: String = String::from(
        r#"
            SELECT
            CASE WHEN C1 <= 1 THEN 'small'
            WHEN C1 <= 3 THEN CASE WHEN C2 <= 1 THEN 'medium small' ELSE 'medium large' END
            ELSE 'large' END AS C3
            FROM TBL1
            "#,
    );

    config.newlines = true;
    assert_eq!(
        get_formatted_sql(&config, sql.clone()),
        r#"            SELECT
                CASE
                    WHEN C1 <= 1 THEN 'small'
                    WHEN C1 <= 3 THEN
                    CASE
                        WHEN C2 <= 1 THEN 'medium small'
                        ELSE 'medium large'
                    END
                    ELSE 'large'
                END AS C3
            FROM TBL1"#
    );
}

#[test]
fn test_get_formatted_sql_insert_simple() {
    let mut config: Configuration = Configuration::new();
    let sql: String = String::from(r#"INSERT INTO TBL1(ID)VALUES(1)"#);

    assert_eq!(
        get_formatted_sql(&config, sql.clone()),
        r#"INSERT INTO TBL1(ID) VALUES (1)"#
    );

    config.newlines = true;
    assert_eq!(
        get_formatted_sql(&config, sql.clone()),
        r#"INSERT INTO TBL1(ID)
VALUES (1)"#
    );
}

#[test]
fn test_get_formatted_sql_insert_multiple_columns() {
    let mut config: Configuration = Configuration::new();
    let sql: String = String::from(
        "INSERT INTO TBL1 (C1,C2,C3,C4,C5,C6,C7,C8,C9,C10,C11,C12,C13,C14,C15,C16,C17,C18,C19,C20,C21) VALUES (1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19,20,21)",
    );

    assert_eq!(
        get_formatted_sql(&config, sql.clone()),
        r#"INSERT INTO TBL1(C1, C2, C3, C4, C5, C6, C7, C8, C9, C10, C11, C12, C13, C14, C15, C16, C17, C18, C19, C20, C21) VALUES (1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21)"#
    );

    config.newlines = true;
    assert_eq!(
        get_formatted_sql(&config, sql.clone()),
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
fn test_get_formatted_sql_insert_raw_bytes() {
    let mut config: Configuration = Configuration::new();
    let sql: String =
        decode_input(b"insert into tbl1(id,img)values(1,'\x89PNG\r\n\x1a\n\xff\xfe\x00')");

    config.case = ConfigCase::Uppercase;
    assert_eq!(
        encode_output(&get_formatted_sql(&config, sql.clone())),
        b"INSERT INTO tbl1(id, img) VALUES (1, '\x89PNG\r\n\x1a\n\xff\xfe\x00')"
    );

    config.newlines = true;
    assert_eq!(
        encode_output(&get_formatted_sql(&config, sql.clone())),
        b"INSERT INTO tbl1(id, img)\nVALUES (1, '\x89PNG\r\n\x1a\n\xff\xfe\x00')"
    );
}

#[test]
fn test_get_formatted_sql_raw_bytes_in_identifiers() {
    let mut config: Configuration = Configuration::new();
    config.case = ConfigCase::Uppercase;
    let sql: String = decode_input(b"select caf\xe9, \xe9tat, na\xefve from tabl\xe9;");

    assert_eq!(
        encode_output(&get_formatted_sql(&config, sql)),
        b"SELECT caf\xe9, \xe9tat, na\xefve FROM tabl\xe9;"
    );
}

#[test]
fn test_get_formatted_sql_raw_bytes_placeholder_collision() {
    let sql: &str = "SELECT '\u{10ff00}\u{10ff80}\u{10fffe}\u{10ffff}', \u{10ff80}id FROM t;";

    assert_eq!(
        encode_output(&get_formatted_sql(
            &Configuration::new(),
            decode_input(sql.as_bytes())
        )),
        sql.as_bytes()
    );
}

#[test]
fn test_get_formatted_sql_insert_select() {
    let mut config: Configuration = Configuration::new();
    let sql: String = String::from(
        r#"
            INSERT INTO TBL1 (C1,C2,C3)
            SELECT C1,C2,C3
            FROM TBL1
            "#,
    );

    assert_eq!(
        get_formatted_sql(&config, sql.clone()),
        r#"
            INSERT INTO TBL1(C1, C2, C3)
            SELECT C1, C2, C3
            FROM TBL1
"#
    );

    config.newlines = true;
    assert_eq!(
        get_formatted_sql(&config, sql.clone()),
        r#"            INSERT INTO TBL1(C1, C2, C3)
            SELECT
                C1,
                C2,
                C3
            FROM TBL1"#
    );
}

#[test]
fn test_get_formatted_sql_delete_simple() {
    let mut config: Configuration = Configuration::new();
    let sql: String = String::from(r#"DELETE FROM TBL1 WHERE C<=1"#);

    assert_eq!(
        get_formatted_sql(&config, sql.clone()),
        r#"DELETE FROM TBL1 WHERE C <= 1"#
    );

    config.newlines = true;
    assert_eq!(
        get_formatted_sql(&config, sql.clone()),
        r#"DELETE
FROM TBL1
WHERE C <= 1"#
    );
}

#[test]
fn test_get_formatted_sql_delete_newline() {
    let mut config: Configuration = Configuration::new();
    let sql: String = String::from(
        r#"
            DELETE
            FROM TBL1
            WHERE C<=1
            "#,
    );

    assert_eq!(
        get_formatted_sql(&config, sql.clone()),
        r#"
            DELETE
            FROM TBL1
            WHERE C <= 1
"#
    );

    config.newlines = true;
    assert_eq!(
        get_formatted_sql(&config, sql.clone()),
        r#"            DELETE
            FROM TBL1
            WHERE C <= 1"#
    );
}

#[test]
fn test_get_formatted_sql_truncate_table() {
    let mut config: Configuration = Configuration::new();
    let sql: String = String::from(
        r#"
            TRUNCATE TABLE TBL1 TRUNCATE TABLE TBL2
            TRUNCATE TABLE TBL3
            "#,
    );

    assert_eq!(
        get_formatted_sql(&config, sql.clone()),
        r#"
            TRUNCATE TABLE TBL1 TRUNCATE TABLE TBL2
            TRUNCATE TABLE TBL3
"#
    );

    config.newlines = true;
    assert_eq!(
        get_formatted_sql(&config, sql.clone()),
        r#"            TRUNCATE TABLE TBL1
            TRUNCATE TABLE TBL2
            TRUNCATE TABLE TBL3"#
    );
}

#[test]
fn test_get_formatted_sql_drop_table() {
    let mut config: Configuration = Configuration::new();
    let sql: String = String::from(
        r#"
            DROP TABLE IF EXISTS TBL1 DROP TABLE TBL2
            DROP TABLE TBL3
            "#,
    );

    assert_eq!(
        get_formatted_sql(&config, sql.clone()),
        r#"
            DROP TABLE IF EXISTS TBL1 DROP TABLE TBL2
            DROP TABLE TBL3
"#
    );

    config.newlines = true;
    assert_eq!(
        get_formatted_sql(&config, sql.clone()),
        r#"            DROP TABLE IF EXISTS TBL1
            DROP TABLE TBL2
            DROP TABLE TBL3"#
    );
}

#[test]
fn test_get_formatted_sql_execute() {
    let mut config: Configuration = Configuration::new();
    let sql: String = String::from(
        r#"
            EXEC SP1();EXEC SP1();
            EXEC SP1();
            "#,
    );

    assert_eq!(
        get_formatted_sql(&config, sql.clone()),
        r#"
            EXEC SP1(); EXEC SP1();
            EXEC SP1();
"#
    );

    config.newlines = true;
    assert_eq!(
        get_formatted_sql(&config, sql.clone()),
        r#"            EXEC SP1();
            EXEC SP1();
            EXEC SP1();"#
    );
}

#[test]
fn test_get_formatted_sql_execute_no_delimiter() {
    let mut config: Configuration = Configuration::new();
    let sql: String = String::from(r#"EXEC SP1() EXEC SP1() EXEC SP1()"#);

    assert_eq!(
        get_formatted_sql(&config, sql.clone()),
        r#"EXEC SP1() EXEC SP1() EXEC SP1()"#
    );

    config.newlines = true;
    assert_eq!(
        get_formatted_sql(&config, sql.clone()),
        r#"EXEC SP1()
EXEC SP1()
EXEC SP1()"#
    );
}

#[test]
fn test_get_formatted_sql_execute_parameters() {
    let mut config: Configuration = Configuration::new();
    let sql: String = String::from(r#"EXEC SP1 P1, P2, P3 EXEC SP1 P1, P2, P3"#);

    assert_eq!(
        get_formatted_sql(&config, sql.clone()),
        r#"EXEC SP1 P1, P2, P3 EXEC SP1 P1, P2, P3"#
    );

    config.newlines = true;
    assert_eq!(
        get_formatted_sql(&config, sql.clone()),
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
    let mut config: Configuration = Configuration::new();
    let sql: String = String::from(r#"CALL SP1() CALL SP1() CALL SP1()"#);

    assert_eq!(
        get_formatted_sql(&config, sql.clone()),
        r#"CALL SP1() CALL SP1() CALL SP1()"#
    );

    config.newlines = true;
    assert_eq!(
        get_formatted_sql(&config, sql.clone()),
        r#"CALL SP1()
CALL SP1()
CALL SP1()"#
    );
}

#[test]
fn test_get_formatted_sql_if() {
    let mut config: Configuration = Configuration::new();
    let sql: String = String::from(
        r#"
            IF V1 IS NULL AND V2 IS NULL BEGIN SET V1 = 0; SET V2 = 0; END
            IF V1 IS NULL THEN SET V1 = 0 END IF
            IF V2 IS NULL SET V2 = 0
            "#,
    );

    assert_eq!(
        get_formatted_sql(&config, sql.clone()),
        r#"
            IF V1 IS NULL AND V2 IS NULL BEGIN SET V1 = 0; SET V2 = 0; END
            IF V1 IS NULL THEN SET V1 = 0 END IF
            IF V2 IS NULL SET V2 = 0
"#
    );

    config.newlines = true;
    assert_eq!(
        get_formatted_sql(&config, sql.clone()),
        r#"            IF V1 IS NULL
            AND V2 IS NULL
            BEGIN
                SET V1 = 0;
                SET V2 = 0;
            END
            IF V1 IS NULL THEN
                SET V1 = 0
            END IF
            IF V2 IS NULL
            SET V2 = 0"#
    );
}

#[test]
fn test_get_formatted_sql_end_if() {
    let mut config: Configuration = Configuration::new();
    let sql: String = String::from(
        r#"
            IF V1 IS NULL THEN SET V1 = 0; ELSE SET V1 = 1; END IF;
            IF V2 IS NULL BEGIN SET V2 = 0; END
            IF V3 IS NULL SET V3 = 0;
            "#,
    );

    assert_eq!(
        get_formatted_sql(&config, sql.clone()),
        r#"
            IF V1 IS NULL THEN SET V1 = 0; ELSE SET V1 = 1; END IF;
            IF V2 IS NULL BEGIN SET V2 = 0; END
            IF V3 IS NULL SET V3 = 0;
"#
    );

    config.newlines = true;
    assert_eq!(
        get_formatted_sql(&config, sql.clone()),
        r#"            IF V1 IS NULL THEN
                SET V1 = 0;
            ELSE
                SET V1 = 1;
            END IF;

            IF V2 IS NULL
            BEGIN
                SET V2 = 0;
            END
            IF V3 IS NULL
            SET V3 = 0;"#
    );
}

#[test]
fn test_get_formatted_sql_if_left_method() {
    let mut config: Configuration = Configuration::new();
    let sql: String = String::from(
        r#"
            IF LEFT(@V1, 1) = '#' SET @V1 = RIGHT(@V1, 2)
            "#,
    );

    assert_eq!(
        get_formatted_sql(&config, sql.clone()),
        r#"
            IF LEFT(@V1, 1) = '#' SET @V1 = RIGHT(@V1, 2)
"#
    );

    config.newlines = true;
    assert_eq!(
        get_formatted_sql(&config, sql.clone()),
        r#"            IF LEFT(@V1, 1) = '#'
            SET @V1 = RIGHT(@V1, 2)"#
    );
}

#[test]
fn test_get_formatted_sql_if_else() {
    let mut config: Configuration = Configuration::new();
    let sql: String = String::from(
        r#"
            IF V1 IS NULL THEN SET V1 = 0
            ELSE SET V2 = NULL
            "#,
    );

    assert_eq!(
        get_formatted_sql(&config, sql.clone()),
        r#"
            IF V1 IS NULL THEN SET V1 = 0
            ELSE SET V2 = NULL
"#
    );

    config.newlines = true;
    assert_eq!(
        get_formatted_sql(&config, sql.clone()),
        r#"            IF V1 IS NULL THEN
                SET V1 = 0
            ELSE
                SET V2 = NULL"#
    );
}

#[test]
fn test_get_formatted_sql_if_nested() {
    let mut config: Configuration = Configuration::new();
    let sql: String = String::from(
        r#"
            IF V1 IS NULL THEN IF V2 IS NULL THEN SET V1 = 0; END IF;
            ELSE IF V2 IS NULL THEN SET V1 = 1; ELSE SET V1 = 2; END IF; END IF;
            "#,
    );

    config.newlines = true;
    assert_eq!(
        get_formatted_sql(&config, sql.clone()),
        r#"            IF V1 IS NULL THEN
                IF V2 IS NULL THEN
                    SET V1 = 0;
                END IF;
            ELSE
                IF V2 IS NULL THEN
                    SET V1 = 1;
                ELSE
                    SET V1 = 2;
                END IF;
            END IF;"#
    );
}

#[test]
fn test_get_formatted_sql_if_else_begin_end() {
    let mut config: Configuration = Configuration::new();
    let sql: String = String::from(
        r#"
            IF V1 IS NULL BEGIN
            SET V1 = 0;
            END
            ELSE BEGIN
            SET V2 = NULL;
            END
            "#,
    );

    assert_eq!(
        get_formatted_sql(&config, sql.clone()),
        r#"
            IF V1 IS NULL BEGIN
                SET V1 = 0;
            END
            ELSE BEGIN
                SET V2 = NULL;
            END
"#
    );

    config.newlines = true;
    assert_eq!(
        get_formatted_sql(&config, sql.clone()),
        r#"            IF V1 IS NULL
            BEGIN
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
    let mut config: Configuration = Configuration::new();
    let sql: String = String::from(
        r#"
            SET V1 = 0;
            BEGIN TRY
                CALL SP1;
            END TRY
            BEGIN CATCH
                CALL SP2;
                RETURN 1;
            END CATCH
            RETURN 0
            "#,
    );

    assert_eq!(
        get_formatted_sql(&config, sql.clone()),
        r#"
            SET V1 = 0;
            BEGIN TRY
                CALL SP1;
            END TRY
            BEGIN CATCH
                CALL SP2;
                RETURN 1;
            END CATCH
            RETURN 0
"#
    );

    config.newlines = true;
    assert_eq!(
        get_formatted_sql(&config, sql.clone()),
        r#"            SET V1 = 0;

            BEGIN TRY
                CALL SP1;
            END TRY
            BEGIN CATCH
                CALL SP2;
                RETURN 1;
            END CATCH

            RETURN 0"#
    );
}

#[test]
fn test_get_formatted_sql_try_catch_raiserror() {
    let mut config: Configuration = Configuration::new();
    let sql: String = String::from(
        r#"
            BEGIN TRY
                IF @V1<5
                RAISERROR('Error message: %s',13,1,@ERRORMESSAGE);
            END TRY
            BEGIN CATCH
                RETURN 1
            END CATCH
            RETURN 0
            "#,
    );

    assert_eq!(
        get_formatted_sql(&config, sql.clone()),
        r#"
            BEGIN TRY
                IF @V1 < 5
                RAISERROR('Error message: %s', 13, 1, @ERRORMESSAGE);
            END TRY
            BEGIN CATCH
                RETURN 1
            END CATCH
            RETURN 0
"#
    );

    config.newlines = true;
    config.chars = 40;
    assert_eq!(
        get_formatted_sql(&config, sql.clone()),
        r#"            BEGIN TRY
                IF @V1 < 5
                RAISERROR(
                    'Error message: %s',
                    13,
                    1,
                    @ERRORMESSAGE
                );
            END TRY
            BEGIN CATCH
                RETURN 1
            END CATCH

            RETURN 0"#
    );
}

#[test]
fn test_get_formatted_sql_try_catch_insert() {
    let mut config: Configuration = Configuration::new();
    let sql: String = String::from(
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
            "#,
    );

    assert_eq!(
        get_formatted_sql(&config, sql.clone()),
        r#"
            SET V1 = 0;
            BEGIN TRY
                -- COMMENT
                INSERT INTO TBL1(C1) VALUES (1)
            END TRY
            BEGIN CATCH
                RETURN 1
            END CATCH
            RETURN 0
"#
    );

    config.newlines = true;
    assert_eq!(
        get_formatted_sql(&config, sql.clone()),
        r#"            SET V1 = 0;

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
    let mut config: Configuration = Configuration::new();
    let sql: String = String::from(
        r#"
            BEGIN CATCH END CATCH UPDATE TBL1 SET C1 = 1
            "#,
    );

    assert_eq!(
        get_formatted_sql(&config, sql.clone()),
        r#"
            BEGIN CATCH END CATCH UPDATE TBL1 SET C1 = 1
"#
    );

    config.newlines = true;
    assert_eq!(
        get_formatted_sql(&config, sql.clone()),
        r#"            BEGIN CATCH
            END CATCH

            UPDATE TBL1
            SET C1 = 1"#
    );
}

#[test]
fn test_get_formatted_sql_return() {
    let mut config: Configuration = Configuration::new();
    let sql: String = String::from(
        r#"
            SELECT *
            FROM TBL
            RETURN 0
            "#,
    );

    assert_eq!(
        get_formatted_sql(&config, sql.clone()),
        r#"
            SELECT *
            FROM TBL
            RETURN 0
"#
    );

    config.newlines = true;
    assert_eq!(
        get_formatted_sql(&config, sql.clone()),
        r#"            SELECT
                *
            FROM TBL
            RETURN 0"#
    );
}

#[test]
fn test_get_formatted_sql_declare_select() {
    let mut config: Configuration = Configuration::new();
    let sql: String = String::from(
        r#"
            DECLARE V1 INT = (
            SELECT C1
            FROM TBL
            );
            "#,
    );

    assert_eq!(
        get_formatted_sql(&config, sql.clone()),
        r#"
            DECLARE V1 INT = (
                    SELECT C1
                    FROM TBL
                );
"#
    );

    config.newlines = true;
    assert_eq!(
        get_formatted_sql(&config, sql.clone()),
        r#"            DECLARE V1 INT = (SELECT C1 FROM TBL);"#
    );
}

#[test]
fn test_get_formatted_sql_xml() {
    let mut config: Configuration = Configuration::new();
    let sql: String = String::from(
        r#"
            SELECT C1 AS ID
            FROM TBL1
            FOR XML RAW('ITEM'), TYPE, ELEMENTS, ROOT('VALUES'), BINARY BASE64
            "#,
    );

    assert_eq!(
        get_formatted_sql(&config, sql.clone()),
        r#"
            SELECT C1 AS ID
            FROM TBL1
            FOR XML RAW('ITEM'), TYPE, ELEMENTS, ROOT('VALUES'), BINARY BASE64
"#
    );

    config.newlines = true;
    assert_eq!(
        get_formatted_sql(&config, sql.clone()),
        r#"            SELECT
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
    let mut config: Configuration = Configuration::new();
    let sql: String = String::from(
        r#"
            SELECT T2.Loc.QUERY('.')
            FROM T
            CROSS APPLY Instructions.VALUE('/root/Location') AS T2(Loc)
            "#,
    );

    assert_eq!(
        get_formatted_sql(&config, sql.clone()),
        r#"
            SELECT T2.Loc.query('.')
            FROM T
                CROSS APPLY Instructions.value('/root/Location') AS T2(Loc)
"#
    );

    config.newlines = true;

    config.case = ConfigCase::Uppercase;
    assert_eq!(
        get_formatted_sql(&config, sql.clone()),
        r#"            SELECT
                T2.Loc.query('.')
            FROM T
                CROSS APPLY Instructions.value('/root/Location') AS T2(Loc)"#
    );
}

#[test]
fn test_get_formatted_sql_keyword_column_name() {
    let mut config: Configuration = Configuration::new();
    let sql: String = String::from(
        r#"
            SELECT
            T.VALUE VALUE, T.VALUE AS VALUE, T.[VALUE] AS [VALUE], 'VALUE' AS 'VALUE',
            t.days days, t.days as days, t.[days] as [days], 'days' as 'days'
            FROM TBL1 AS T
            "#,
    );

    assert_eq!(
        get_formatted_sql(&config, sql.clone()),
        r#"
            SELECT
                T.VALUE VALUE, T.VALUE AS VALUE, T.[VALUE] AS [VALUE], 'VALUE' AS 'VALUE',
                t.days days, t.days as days, t.[days] as [days], 'days' as 'days'
            FROM TBL1 AS T
"#
    );

    config.newlines = true;
    config.case = ConfigCase::Uppercase;
    assert_eq!(
        get_formatted_sql(&config, sql.clone()),
        r#"            SELECT
                T.VALUE VALUE,
                T.VALUE AS VALUE,
                T.[VALUE] AS [VALUE],
                'VALUE' AS 'VALUE',
                t.DAYS DAYS,
                t.DAYS AS DAYS,
                t.[days] AS [days],
                'days' AS 'days'
            FROM TBL1 AS T"#
    );
}

#[test]
fn test_get_formatted_sql_stuff_comma_list() {
    let mut config: Configuration = Configuration::new();
    let sql: String = String::from(
        r#"
            STUFF((SELECT ', ' + C1 FROM TBL1 FOR XML PATH('')), 1, 2, '')
            "#,
    );

    assert_eq!(
        get_formatted_sql(&config, sql.clone()),
        r#"
            STUFF((SELECT ', ' + C1 FROM TBL1 FOR XML PATH('')), 1, 2, '')
"#
    );

    config.newlines = true;
    config.chars = 40;
    assert_eq!(
        get_formatted_sql(&config, sql.clone()),
        r#"            STUFF(
                (
                    SELECT
                        ', ' + C1
                    FROM TBL1
                    FOR XML PATH('')
                ),
                1,
                2,
                ''
            )"#
    );
}

#[test]
fn test_get_formatted_sql_table_variable() {
    let mut config: Configuration = Configuration::new();
    let sql: String = String::from(
        r#"
            DECLARE TBL1 TABLE (
                C1 UNIQUEIDENTIFIER, C2 UNIQUEIDENTIFIER,
                C3 UNIQUEIDENTIFIER, C4 UNIQUEIDENTIFIER,
                C5 UNIQUEIDENTIFIER, C6 UNIQUEIDENTIFIER
            );
            "#,
    );

    assert_eq!(
        get_formatted_sql(&config, sql.clone()),
        r#"
            DECLARE TBL1 TABLE(
                    C1 UNIQUEIDENTIFIER, C2 UNIQUEIDENTIFIER,
                    C3 UNIQUEIDENTIFIER, C4 UNIQUEIDENTIFIER,
                    C5 UNIQUEIDENTIFIER, C6 UNIQUEIDENTIFIER
                );
"#,
    );

    config.newlines = true;
    assert_eq!(
        get_formatted_sql(&config, sql.clone()),
        r#"            DECLARE TBL1 TABLE(
                    C1 UNIQUEIDENTIFIER,
                    C2 UNIQUEIDENTIFIER,
                    C3 UNIQUEIDENTIFIER,
                    C4 UNIQUEIDENTIFIER,
                    C5 UNIQUEIDENTIFIER,
                    C6 UNIQUEIDENTIFIER
                );"#,
    );
}

#[test]
fn test_get_formatted_sql_create_table_simple() {
    let mut config: Configuration = Configuration::new();
    let sql: String = String::from(r#"CREATE TABLE TBL1 (C1 INT)"#);

    assert_eq!(
        get_formatted_sql(&config, sql.clone()),
        r#"CREATE TABLE TBL1(C1 INT)"#
    );

    config.newlines = true;
    assert_eq!(
        get_formatted_sql(&config, sql.clone()),
        r#"CREATE TABLE TBL1(C1 INT)"#
    );
}

#[test]
fn test_get_formatted_sql_create_table_varchar() {
    let mut config: Configuration = Configuration::new();
    let sql: String = String::from(r#"CREATE TABLE TBL1 (C1 VARCHAR(10))"#);

    assert_eq!(
        get_formatted_sql(&config, sql.clone()),
        r#"CREATE TABLE TBL1(C1 VARCHAR(10))"#
    );

    config.newlines = true;
    assert_eq!(
        get_formatted_sql(&config, sql.clone()),
        r#"CREATE TABLE TBL1(C1 VARCHAR(10))"#
    );
}

#[test]
fn test_get_formatted_sql_create_table_default() {
    let mut config: Configuration = Configuration::new();
    let sql: String = String::from(
        r#"
            CREATE TABLE TBL1 (
                ID UUID NOT NULL DEFAULT UUID()
            )
            "#,
    );

    assert_eq!(
        get_formatted_sql(&config, sql.clone()),
        r#"
            CREATE TABLE TBL1(
                ID UUID NOT NULL DEFAULT UUID()
            )
"#
    );

    config.newlines = true;
    assert_eq!(
        get_formatted_sql(&config, sql.clone()),
        r#"            CREATE TABLE TBL1(ID UUID NOT NULL DEFAULT UUID())"#
    );
}

#[test]
fn test_get_formatted_sql_create_table_complex() {
    let mut config: Configuration = Configuration::new();
    let sql: String = String::from(
        r#"
            CREATE TABLE IF NOT EXISTS TBL1 (
                ID UUID NOT NULL DEFAULT UUID(),
                C1 VARCHAR(10) NOT NULL,
                D1 DATETIME NULL,
                E1 ENUM('VAL1', 'VAL2') NOT NULL DEFAULT 'VAL1',
                I1 INT,
                I2 INT, PRIMARY KEY (ID), FOREIGN KEY (I1) REFERENCES TBL2 (ID) ON DELETE CASCADE,
                FOREIGN KEY (I2) REFERENCES TBL3 (ID) ON DELETE SET NULL
            )
            "#,
    );

    assert_eq!(
        get_formatted_sql(&config, sql.clone()),
        r#"
            CREATE TABLE IF NOT EXISTS TBL1(
                ID UUID NOT NULL DEFAULT UUID(),
                C1 VARCHAR(10) NOT NULL,
                D1 DATETIME NULL,
                E1 ENUM('VAL1', 'VAL2') NOT NULL DEFAULT 'VAL1',
                I1 INT,
                I2 INT, PRIMARY KEY(ID), FOREIGN KEY(I1) REFERENCES TBL2(ID) ON DELETE CASCADE,
                FOREIGN KEY(I2) REFERENCES TBL3(ID) ON DELETE SET NULL
            )
"#
    );

    config.newlines = true;
    assert_eq!(
        get_formatted_sql(&config, sql.clone()),
        r#"            CREATE TABLE IF NOT EXISTS TBL1(
                ID UUID NOT NULL DEFAULT UUID(),
                C1 VARCHAR(10) NOT NULL,
                D1 DATETIME NULL,
                E1 ENUM('VAL1', 'VAL2') NOT NULL DEFAULT 'VAL1',
                I1 INT,
                I2 INT,
                PRIMARY KEY(ID),
                FOREIGN KEY(I1) REFERENCES TBL2(ID) ON DELETE CASCADE,
                FOREIGN KEY(I2) REFERENCES TBL3(ID) ON DELETE SET NULL
            )"#
    );

    config.align_text_groups = true;
    assert_eq!(
        get_formatted_sql(&config, sql.clone()),
        r#"            CREATE TABLE IF NOT EXISTS TBL1(
                ID      UUID                 NOT NULL   DEFAULT  UUID(),
                C1      VARCHAR(10)          NOT NULL,
                D1      DATETIME             NULL,
                E1      ENUM('VAL1', 'VAL2') NOT NULL   DEFAULT  'VAL1',
                I1      INT,
                I2      INT,
                PRIMARY KEY(ID),
                FOREIGN KEY(I1)              REFERENCES TBL2(ID) ON DELETE CASCADE,
                FOREIGN KEY(I2)              REFERENCES TBL3(ID) ON DELETE SET NULL
            )"#
    );
}

#[test]
fn test_get_formatted_sql_create_table_after_select() {
    let mut config: Configuration = Configuration::new();
    let sql: String = String::from(
        r#"
            SELECT 1;
            CREATE TABLE TBL1 (
                ID UUID NOT NULL DEFAULT UUID()
            );
            "#,
    );

    assert_eq!(
        get_formatted_sql(&config, sql.clone()),
        r#"
            SELECT 1;
            CREATE TABLE TBL1(
                ID UUID NOT NULL DEFAULT UUID()
            );
"#
    );

    config.newlines = true;
    assert_eq!(
        get_formatted_sql(&config, sql.clone()),
        r#"            SELECT
                1;

            CREATE TABLE TBL1(ID UUID NOT NULL DEFAULT UUID());"#
    );
}

#[test]
fn test_get_formatted_sql_alter_table() {
    let mut config: Configuration = Configuration::new();
    let sql: String = String::from(
        r#"
            ALTER TABLE TBL1 ALTER COLUMN C1 NVARCHAR (20) NOT NULL;
            ALTER TABLE TBL1 DROP COLUMN C1;
            "#,
    );

    assert_eq!(
        get_formatted_sql(&config, sql.clone()),
        r#"
            ALTER TABLE TBL1 ALTER COLUMN C1 NVARCHAR(20) NOT NULL;
            ALTER TABLE TBL1 DROP COLUMN C1;
"#,
    );

    config.newlines = true;
    assert_eq!(
        get_formatted_sql(&config, sql.clone()),
        r#"            ALTER TABLE TBL1
            ALTER COLUMN C1 NVARCHAR(20) NOT NULL;

            ALTER TABLE TBL1
            DROP COLUMN C1;"#,
    );
}

#[test]
fn test_get_formatted_sql_trigger() {
    let mut config: Configuration = Configuration::new();
    let sql: String = String::from(
        r#"
            CREATE TRIGGER IF NOT EXISTS TR1
            AFTER INSERT
            ON TBL1
            FOR EACH ROW
            BEGIN
            CALL SP1(NEW.ID);
            END;
            "#,
    );

    assert_eq!(
        get_formatted_sql(&config, sql.clone()),
        r#"
            CREATE TRIGGER IF NOT EXISTS TR1
            AFTER INSERT
                ON TBL1
            FOR EACH ROW
            BEGIN
                CALL SP1(NEW.ID);
            END;
"#
    );

    config.newlines = true;
    assert_eq!(
        get_formatted_sql(&config, sql.clone()),
        r#"            CREATE TRIGGER IF NOT EXISTS TR1
            AFTER INSERT ON TBL1
            FOR EACH ROW
            BEGIN
                CALL SP1(NEW.ID);
            END;"#
    );
}

#[test]
fn test_get_formatted_sql_function() {
    let mut config: Configuration = Configuration::new();
    let sql: String = String::from(
        r#"
            CREATE FUNCTION FUNC1 ( @P1 NVARCHAR(10) )
            RETURNS NVARCHAR(10)
            WITH EXECUTE AS CALLER AS
            BEGIN RETURN ''; END
            "#,
    );

    assert_eq!(
        get_formatted_sql(&config, sql.clone()),
        r#"
            CREATE FUNCTION FUNC1(@P1 NVARCHAR(10))
            RETURNS NVARCHAR(10)
            WITH EXECUTE AS CALLER AS
            BEGIN RETURN ''; END
"#
    );

    config.newlines = true;
    assert_eq!(
        get_formatted_sql(&config, sql.clone()),
        r#"            CREATE FUNCTION FUNC1(@P1 NVARCHAR(10))
            RETURNS NVARCHAR(10) WITH
            EXECUTE AS CALLER AS
            BEGIN
                RETURN '';
            END"#
    );
}

#[test]
fn test_get_formatted_sql_procedure() {
    let mut config: Configuration = Configuration::new();
    let sql: String = String::from(
        r#"
            CREATE PROCEDURE SP1 ( @P1 NVARCHAR(10) = NULL OUTPUT, @P2 INT NOT NULL )
            AS
            BEGIN
            IF @P2 < 0 BEGIN LEAVE; END
            IF @P2 > 5 BEGIN SET @P1 = 'BIG' END
            ELSE BEGIN SET @P1 = 'SMALL' END
            END
            "#,
    );

    assert_eq!(
        get_formatted_sql(&config, sql.clone()),
        r#"
            CREATE PROCEDURE SP1(@P1 NVARCHAR(10) = NULL OUTPUT, @P2 INT NOT NULL)
            AS
            BEGIN
                IF @P2 < 0 BEGIN LEAVE; END
                IF @P2 > 5 BEGIN SET @P1 = 'BIG' END
                ELSE BEGIN SET @P1 = 'SMALL' END
            END
"#
    );

    config.newlines = true;
    assert_eq!(
        get_formatted_sql(&config, sql.clone()),
        r#"            CREATE PROCEDURE SP1(
                @P1 NVARCHAR(10) = NULL OUTPUT,
                @P2 INT NOT NULL
            ) AS
            BEGIN
                IF @P2 < 0
                BEGIN
                    LEAVE;
                END
                IF @P2 > 5
                BEGIN
                    SET @P1 = 'BIG'
                END
                ELSE
                BEGIN
                    SET @P1 = 'SMALL'
                END
            END"#
    );
}

#[test]
fn test_get_formatted_sql_while_loop() {
    let mut config: Configuration = Configuration::new();
    let sql: String = String::from(
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
            "#,
    );

    assert_eq!(
        get_formatted_sql(&config, sql.clone()),
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
    );

    config.newlines = true;
    assert_eq!(
        get_formatted_sql(&config, sql.clone()),
        r#"            DECLARE VAR_COUNT INT;

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
            END
            WHILE;"#
    );
}

#[test]
fn test_get_formatted_sql_pivot() {
    let mut config: Configuration = Configuration::new();
    let sql: String = String::from(
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
            "#,
    );

    assert_eq!(
        get_formatted_sql(&config, sql.clone()),
        r#"
            SELECT 'AverageCost' AS CostSortedByProductionDays,
                [0], [1], [2], [3], [4]
            FROM (
                    SELECT DaysToManufacture, StandardCost
                    FROM Production.Product
                ) AS SourceTable
            PIVOT (
                    AVG(StandardCost) FOR DaysToManufacture IN
                        ([0], [1], [2], [3], [4])
                ) AS PivotTable;
"#
    );

    config.newlines = true;
    assert_eq!(
        get_formatted_sql(&config, sql.clone()),
        r#"            SELECT
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
    let mut config: Configuration = Configuration::new();
    let sql: String = String::from(
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
    );

    assert_eq!(
        get_formatted_sql(&config, sql.clone()),
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
"#
    );

    config.newlines = true;
    assert_eq!(
        get_formatted_sql(&config, sql.clone()),
        r#"            DECLARE @ID INT,
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

#[test]
fn test_get_formatted_merge() {
    let mut config: Configuration = Configuration::new();
    let sql: String = String::from(
        r#"
            select 1;
            merge tbl1 as TargetTable using tbl2 as SourceTable on TargetTable.id = @id and TargetTable.c1 = SourceTable.c1
            when not matched by TargetTable then insert (c1) values (SourceTable.c1)
            when not matched by SourceTable and TargetTable.id = @id then delete;
            "#,
    );

    assert_eq!(
        get_formatted_sql(&config, sql.clone()),
        r#"
            select 1;
            merge tbl1 as TargetTable using tbl2 as SourceTable on TargetTable.id = @id and TargetTable.c1 = SourceTable.c1
            when not matched by TargetTable then insert (c1) values (SourceTable.c1)
            when not matched by SourceTable and TargetTable.id = @id then delete;
"#
    );

    config.case = ConfigCase::Uppercase;
    config.newlines = true;
    assert_eq!(
        get_formatted_sql(&config, sql.clone()),
        r#"            SELECT
                1;

            MERGE tbl1 AS TargetTable
            USING tbl2 AS SourceTable ON TargetTable.id = @id
                AND TargetTable.c1 = SourceTable.c1
            WHEN NOT MATCHED BY TargetTable THEN
            INSERT (c1)
            VALUES (SourceTable.c1)
            WHEN NOT MATCHED BY SourceTable
                AND TargetTable.id = @id THEN
            DELETE;"#
    );
}

#[test]
fn test_get_formatted_merge_output() {
    let mut config: Configuration = Configuration::new();
    let sql: String = String::from(
        r#"
            MERGE INTO TargetTable AS t USING SourceTable AS s ON t.ID = s.ID
            WHEN MATCHED THEN UPDATE SET t.Value = s.Value
            WHEN NOT MATCHED BY TARGET THEN INSERT (ID, Value) VALUES (s.ID, s.Value)
            OUTPUT $action AS ActionType, INSERTED.ID AS NewID, DELETED.Value AS OldValue, INSERTED.Value AS NewValue INTO @LogTable;
            "#,
    );

    assert_eq!(
        get_formatted_sql(&config, sql.clone()),
        r#"
            MERGE INTO TargetTable AS t USING SourceTable AS s ON t.ID = s.ID
            WHEN MATCHED THEN UPDATE SET t.Value = s.Value
            WHEN NOT MATCHED BY TARGET THEN INSERT (ID, Value) VALUES (s.ID, s.Value)
            OUTPUT $action AS ActionType, INSERTED.ID AS NewID, DELETED.Value AS OldValue, INSERTED.Value AS NewValue INTO @LogTable;
"#
    );

    config.newlines = true;
    assert_eq!(
        get_formatted_sql(&config, sql.clone()),
        r#"            MERGE INTO TargetTable AS t
            USING SourceTable AS s ON t.ID = s.ID
            WHEN MATCHED THEN
            UPDATE
            SET t.Value = s.Value
            WHEN NOT MATCHED BY TARGET THEN
            INSERT (ID, Value)
            VALUES (s.ID, s.Value) OUTPUT $action AS ActionType,
            INSERTED.ID AS NewID,
            DELETED.Value AS OldValue,
            INSERTED.Value AS NewValue
            INTO
                @LogTable;"#
    );
}
