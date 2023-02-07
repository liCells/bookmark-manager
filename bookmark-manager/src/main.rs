#[macro_use]
extern crate rocket;

use std::fs::File;
use std::path::Path;
use rocket::serde::json::Json;

use rusqlite::{Connection};
use serde::{Deserialize, Serialize};

#[post("/search", format = "application/json", data = "<search_params>")]
fn search(search_params: Json<SearchParams>) -> Json<Vec<Bookmark>> {
    let conditions = split_by_symbol(&search_params.params, ' ', '\\');

    let where_sql = parse_conditions_to_sql(conditions);

    let mut sql;
    if !where_sql.is_empty() {
        sql = String::from("SELECT id, title, url, tags, write_time FROM bookmark where ");
        sql += &where_sql;
    } else {
        sql = String::from("SELECT id, title, url, tags, write_time FROM bookmark");
    }
    let conn = Connection::open("index/bookmarks.db").unwrap();
    let mut stmt = conn.prepare(sql.as_str()).unwrap();
    let result = stmt.query_map([], |row| {
        Ok(Bookmark {
            id: row.get(0)?,
            title: row.get(1)?,
            url: row.get(2)?,
            tags: row.get(3)?,
            write_time: row.get(4)?,
        })
    }).unwrap();

    let mut bookmarks: Vec<Bookmark> = Vec::new();
    for res in result {
        bookmarks.push(res.unwrap())
    }

    Json(bookmarks)
}

#[post("/init")]
fn init() -> &'static str {
    let conn = Connection::open("index/bookmarks.db").unwrap();
    conn.execute(
        "CREATE TABLE bookmark (
            id    INTEGER PRIMARY KEY,
            title TEXT NOT NULL,
            url   TEXT NOT NULL,
            tags  TEXT NOT NULL,
            write_time DATE NOT NULL
        )",
        (),
    ).unwrap();

    let path = "index/bookmarks.json";
    let file = Path::new(path);
    // 验证路径存在且是文件
    if file.exists() && file.is_file() {
        let data = parse_data(path.to_string());
        let mut batch_sql = String::from("");
        // 循环data
        for item in data {
            batch_sql = format!(
                "{} INSERT INTO bookmark (title, url, tags, write_time) VALUES ('{}', '{}', '{}', '{}');",
                batch_sql, item.title, item.url, item.tags, item.write_time
            ).to_string();
        }
        conn.execute_batch(
            format!(
                "BEGIN; {} COMMIT;",
                batch_sql
            ).as_str()
        ).unwrap();
    }

    "Initialization Data Succeed."
}

#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    let _ = rocket::build()
        .mount("/", routes![search, init])
        .launch()
        .await?;

    Ok(())
}

fn split_by_symbol(s: &str, symbol: char, prefix: char) -> Vec<String> {
    let mut result = Vec::new();
    let mut current_string = String::new();
    let mut last_char_is_backslash = false;

    for c in s.chars() {
        if c == symbol && !last_char_is_backslash {
            result.push(current_string);
            current_string = String::new();
        } else if c == prefix {
            last_char_is_backslash = true;
        } else {
            last_char_is_backslash = false;
            current_string.push(c);
        }
    }
    result.push(current_string);
    result
}

// 转换条件为SQL
fn parse_conditions_to_sql(conditions: Vec<String>) -> String {
    let mut sql_condition = Vec::new();
    let mut or_conditions_sql = Vec::new();
    for condition in conditions {
        or_conditions_sql.clear();

        if condition.chars().nth(0).unwrap() == '#' {
            let cond = condition.replace("#", "");
            // tags like
            let or_conditions: Vec<String> = split_by_symbol(cond.as_str(), '|', '\\');

            for or_condition in or_conditions {
                or_conditions_sql.push(format!("tags like '%{}%'", or_condition));
            }
            sql_condition.push(format!("({})", or_conditions_sql.join(" or ")));
        } else if condition.chars().nth(0).unwrap() == '!' {
            let cond = condition.replace("!", "");
            // title and url not like
            sql_condition.push(format!("(title not like '%{}%' and url not like '%{}%')", cond, cond));
        } else {
            // title and url like
            let or_conditions: Vec<String> = split_by_symbol(condition.as_str(), '|', '\\');

            for or_condition in or_conditions {
                or_conditions_sql.push(format!("title like '%{}%' or url like '%{}%'", or_condition, or_condition));
            }
            sql_condition.push(format!("({})", or_conditions_sql.join(" or ")));
        }
    }
    sql_condition.join(" or ")
}

// 转换
fn parse_data(path: String) -> Vec<Bookmark> {
    let data_file = File::open(path).unwrap();
    serde_json::from_reader(data_file).unwrap()
}

#[derive(Serialize, Deserialize, Debug)]
struct Bookmark {
    id: Option<u16>,
    title: String,
    url: String,
    tags: String,
    write_time: String,
}

#[derive(Deserialize)]
struct SearchParams {
    params: String,
}
