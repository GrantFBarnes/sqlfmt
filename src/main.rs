use format::*;

mod format;
mod token;

fn main() {
    let sql: String = String::from(
        r#"
            SELECT *
            FROM TBL1
        "#,
    );
    let formatted: String = get_formatted_sql(sql);
    dbg!(formatted);
}
