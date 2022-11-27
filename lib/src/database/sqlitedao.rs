use std::{path::PathBuf, sync::Mutex};

use crate::constant::ENABLE_FOREIGN_KEYS;
use crate::database::{Dao, Record, Value, Where};
use crate::errors::{Error, Result};

use dirs::data_dir;
use log::*;
use rusqlite::Connection;

pub struct SqliteDao {
    pub conn: Mutex<Connection>,
}

// Panics if can't create sqlx Pool as without that the program is useless
impl SqliteDao {
    pub fn new() -> SqliteDao {
        let dir = data_dir().expect("failed to get data directory");
        let mut db_path = PathBuf::from(dir);
        db_path.push("scorg");
        match std::fs::create_dir(&db_path) {
            Ok(_) => debug!("created new data directory"),
            Err(err) => error!("failed to create data directory: {err}"),
        };
        db_path.push(crate::constant::DB_FILE);
        SqliteDao::using_file(db_path.as_os_str().to_str().unwrap())
    }

    pub fn using_file(file_path: &str) -> SqliteDao {
        let conn = Connection::open(file_path);
        match conn {
            Ok(conn) => {
                log::debug!("created new SqliteDao connecting to {file_path}");
                SqliteDao {
                    conn: Mutex::new(conn),
                }
            }
            Err(e) => {
                log::error!("failed to connect to db: {e}");
                panic!();
            }
        }
    }
}

impl Dao for SqliteDao {
    fn init(&self) -> Result<()> {
        log::debug!("initialising...");
        let sqls = [ENABLE_FOREIGN_KEYS];
        for sql in sqls {
            log::debug!("executing {sql}");
            self.execute(sql)?;
        }
        Ok(())
    }

    fn select(
        &self,
        fields: &Vec<String>,
        table: &str,
        wheres: &Vec<Where>,
    ) -> Result<Vec<Record>> {
        let conn = self.conn.lock().expect("Failed to get lock on connection");
        let mut sql_string = select_string(fields, table);
        let (where_str, args) = process_wheres(wheres, 0);
        sql_string.push_str(&where_str);
        log::debug!("{} [{:?}]", &sql_string, &args);
        let mut stmt = conn.prepare(&sql_string)?;
        let records: Vec<Record> = stmt
            .query_map(rusqlite::params_from_iter(&mut args.iter()), |row| {
                let mut record = Record::new();
                for field in fields {
                    record.insert(field.clone(), row.get(field.as_str())?);
                }
                Ok(record)
            })?
            .into_iter()
            .map(|rec| rec.unwrap())
            .collect();
        log::debug!("found: {:?}", &records);
        Ok(records)
    }

    fn insert(
        &self,
        fields: &Vec<String>,
        table: &str,
        args: Vec<Value>,
        replace: bool,
    ) -> Result<usize> {
        let conn = self.conn.lock().expect("Failed to get lock on connection");
        let sql_string = insert_string(fields, table, args.len(), replace)?;
        log::debug!("{} [{:?}]", &sql_string, &args);
        match conn.execute(&sql_string, rusqlite::params_from_iter(&mut args.iter())) {
            Ok(n) => Ok(n),
            Err(e) => {
                log::error!("{}", &e);
                Err(Error::DbError(e.to_string()))
            }
        }
    }

    fn update(
        &self,
        fields: &Vec<String>,
        table: &str,
        args: Vec<Value>,
        wheres: &Vec<Where>,
    ) -> Result<usize> {
        let conn = self.conn.lock().expect("Failed to get lock on connection");
        let mut sql_string = update_string(fields, table, args.len());
        let (where_str, w_args) = process_wheres(wheres, args.len());
        sql_string.push_str(&where_str);
        let mut full_args = Vec::new();
        full_args.extend(args);
        full_args.extend(w_args);
        log::debug!("{} [{:?}]", &sql_string, &full_args);
        match conn.execute(
            &sql_string,
            rusqlite::params_from_iter(&mut full_args.iter()),
        ) {
            Ok(n) => Ok(n),
            Err(e) => Err(Error::DbError(e.to_string())),
        }
    }

    fn delete(&self, table: &str, wheres: &Vec<Where>) -> Result<usize> {
        let conn = self.conn.lock().expect("Failed to get lock on connection");
        let mut sql_string = delete_string(table);
        let (where_str, args) = process_wheres(wheres, 0);
        sql_string.push_str(&where_str);
        log::debug!("{}", &sql_string);
        match conn.execute(&sql_string, rusqlite::params_from_iter(&mut args.iter())) {
            Ok(n) => Ok(n),
            Err(e) => Err(Error::DbError(e.to_string())),
        }
    }

    fn execute(&self, sql: &str) -> Result<()> {
        let conn = self.conn.lock().expect("Failed to get lock on connection");
        match conn.execute(sql, []) {
            Ok(_) => Ok(()),
            Err(e) => Err(Error::DbError(e.to_string())),
        }
    }

    // fn fetch(&self, sql: &str) -> Result<Vec<Vec<Value>>> {
    //     let mut stmt = self.conn.prepare(sql)?;
    //     let records: Vec<Vec<Value>> = stmt.query_map([], |row| {
    //         let mut record = Vec::new();
    //         row.
    //         Ok(record)
    //     })?.into_iter().map(|rec| rec.unwrap()).collect();
    //     Ok(records)
    // }
}

fn process_wheres(wheres: &Vec<Where>, start: usize) -> (String, Vec<Value>) {
    let mut args: Vec<Value> = Vec::new();
    let mut where_string = String::new();
    if wheres.len() >= 1 {
        where_string.push_str(" WHERE ");
        let mut arg_count = start;
        where_string.push_str(
            &wheres
                .iter()
                .map(|w| {
                    arg_count += 1;
                    args.push(w.value.clone());
                    format!("{}{}?{}", w.field, w.symbol.to_string(), arg_count)
                })
                .collect::<Vec<String>>()
                .join(" AND "),
        );
    }
    (where_string, args)
}

fn select_string(fields: &Vec<String>, table: &str) -> String {
    let field_string = fields.clone().join(",");
    format!("SELECT {field_string} FROM {table}")
}

fn insert_string(fields: &Vec<String>, table: &str, args: usize, replace: bool) -> Result<String> {
    let field_string = fields.clone().join(",");
    let num_fields = fields.len();
    let value_string: String = match num_fields {
        n if n == args => {
            let mut args_list = Vec::new();
            let mut count = 1;
            for _ in 0..args {
                args_list.push(format!("${count}"));
                count += 1;
            }
            format!("({})", args_list.join(","))
        }
        n if args % n == 0 => {
            let num_recs = args / n;
            let mut count = 1;
            let mut temp = Vec::new();
            for _ in 0..num_recs {
                let mut args_list: Vec<String> = Vec::new();
                for _ in 0..num_fields {
                    args_list.push(format!("${count}"));
                    count += 1;
                }
                temp.push(format!("({})", args_list.join(",")));
            }
            format!("{}", temp.join(","))
        }
        _ => {
            return Err(Error::DbError(
                "number of args doesn't work with number of records".into(),
            ))
        }
    };
    let replace = match replace {
        true => "OR REPLACE ",
        false => "",
    };
    Ok(format!(
        "INSERT {replace}INTO {table} ({field_string}) VALUES {value_string}"
    ))
}

fn update_string(fields: &Vec<String>, table: &str, args: usize) -> String {
    let mut updates_list = Vec::new();
    let mut count = 1;
    for i in 0..args {
        updates_list.push(format!("{}=${count}", fields[i]));
        count += 1;
    }
    format!("UPDATE {table} SET {}", updates_list.join(","))
}

fn delete_string(table: &str) -> String {
    format!("DELETE FROM {table}")
}

// #################
// ##### TESTS #####
// #################

#[cfg(test)]
mod tests {
    use super::*;
    use crate::database::{Symbol, Value};
    use matches::assert_matches;
    use std::collections::HashMap;

    fn mock() -> SqliteDao {
        let conn = Connection::open_in_memory().unwrap();
        conn.execute(
            "CREATE TABLE test (field1 INTEGER, field2 TEXT, field3 TEXT)",
            [],
        )
        .expect("failed to create mock table test");
        conn.execute(
            "INSERT INTO test VALUES (123, 'hello', '2021-01-01'),(124, 'hello', '')",
            [],
        )
        .expect("failed to insert test data");
        SqliteDao {
            conn: Mutex::new(conn),
        }
    }

    #[test]
    fn test_select() {
        let dao = mock();
        let exp = vec![
            HashMap::from([
                ("field1".to_string(), Value::from(123)),
                ("field2".to_string(), Value::from("hello")),
            ]),
            HashMap::from([
                ("field1".to_string(), Value::from(124)),
                ("field2".to_string(), Value::from("hello")),
            ]),
        ];
        let act = dao
            .select(&vec!["field1".into(), "field2".into()], "test", &vec![])
            .expect("Select failed");
        assert_eq!(exp, act);
    }

    #[test]
    fn test_select_where() {
        let dao = mock();
        let wheres = vec![Where::new("field1", Symbol::EQ, Value::from(123))];
        let exp = vec![HashMap::from([
            ("field1".to_string(), Value::from(123)),
            ("field2".to_string(), Value::from("hello")),
        ])];
        let act = dao
            .select(&vec!["field1".into(), "field2".into()], "test", &wheres)
            .expect("select failed");
        assert_eq!(exp, act);
    }

    #[test]
    fn test_select_bad_field() {
        let dao = mock();
        let err = dao.select(&vec!["bad".into()], "test", &vec![]);
        assert_matches!(&err, Err(Error::DbError(_)));
        match err {
            Err(Error::DbError(ms)) => assert_eq!(
                ms,
                "no such column: bad in SELECT bad FROM test at offset 7"
            ), // TODO offset 7?
            _ => panic!("this should be an error"),
        };
    }

    #[test]
    fn test_select_dates() {
        let dao = mock();
        let w = vec![Where::new("field3", Symbol::LT, Value::from("2022-01-01"))];
        let exp = vec![
            HashMap::from([("field1".to_string(), Value::from(123))]),
            HashMap::from([("field1".to_string(), Value::from(124))]),
        ];
        let act = dao.select(&vec!["field1".into()], "test", &w).unwrap();
        assert_eq!(exp, act);
    }

    #[test]
    fn test_insert() {
        let dao = mock();
        let fields = vec!["field1".into(), "field2".into()];
        let args = vec![Value::from(99), Value::from("inserted")];
        let res = dao.insert(&fields, "test", args, false);
        assert_matches!(res, Ok(_));
        let w = vec![Where::new("field1", Symbol::EQ, Value::from(99))];
        let db_check = dao.select(&vec!["field1".into(), "field2".into()], "test", &w);
        assert_eq!(
            db_check.unwrap()[0],
            Record::from([
                ("field1".to_string(), Value::from(99)),
                ("field2".to_string(), Value::from("inserted"))
            ])
        );
    }

    #[test]
    fn test_insert_error() {
        let dao = mock();
        let fields = vec!["field1".into(), "bad".into()];
        let args = vec![Value::from(99), Value::from("inserted")];
        let err = dao.insert(&fields, "test", args, false);
        assert_matches!(&err, Err(Error::DbError(_)));
        match err {
            Err(Error::DbError(ms)) => assert_eq!(ms, "table test has no column named bad"),
            _ => panic!("this should be an error"),
        };
    }

    #[test]
    fn test_update() {
        let dao = mock();
        let fields = vec!["field2".into()];
        let args = vec![Value::from("updated")];
        let res = dao.update(&fields, "test", args, &vec![]);
        assert_matches!(res, Ok(_));
        let db_check = dao.select(&vec!["field1".into(), "field2".into()], "test", &vec![]);
        assert_eq!(
            db_check.unwrap(),
            vec![
                Record::from([
                    ("field1".to_string(), Value::from(123)),
                    ("field2".to_string(), Value::from("updated"))
                ]),
                Record::from([
                    ("field1".to_string(), Value::from(124)),
                    ("field2".to_string(), Value::from("updated"))
                ]),
            ]
        );
    }

    #[test]
    fn test_update_where() {
        let dao = mock();
        let fields = vec!["field2".into()];
        let args = vec![Value::from("updated")];
        let w = vec![Where::new("field1", Symbol::EQ, Value::from(123))];
        let res = dao.update(&fields, "test", args, &w);
        assert_matches!(res, Ok(_));
        let db_check = dao.select(&vec!["field1".into(), "field2".into()], "test", &vec![]);
        assert_eq!(
            db_check.unwrap(),
            vec![
                Record::from([
                    ("field1".to_string(), Value::from(123)),
                    ("field2".to_string(), Value::from("updated"))
                ]),
                Record::from([
                    ("field1".to_string(), Value::from(124)),
                    ("field2".to_string(), Value::from("hello"))
                ]),
            ]
        );
    }

    #[test]
    fn test_update_error() {
        let dao = mock();
        let fields = vec!["field1".into(), "bad".into()];
        let args = vec![Value::from(99), Value::from("inserted")];
        let err = dao.update(&fields, "test", args, &vec![]);
        assert_matches!(&err, Err(Error::DbError(_)));
        match err {
            Err(Error::DbError(ms)) => assert_eq!(ms, "no such column: bad"),
            _ => panic!("this should be an error"),
        };
    }

    #[test]
    fn test_delete() {
        let dao = mock();
        let res = dao.delete("test", &vec![]);
        assert_matches!(res, Ok(_));
        let db_check = dao.select(&vec!["field1".into(), "field2".into()], "test", &vec![]);
        assert_eq!(db_check.unwrap(), vec![]);
    }

    #[test]
    fn test_delete_where() {
        let dao = mock();
        let w = vec![Where::new("field1", Symbol::EQ, Value::from(123))];
        let res = dao.delete("test", &w);
        assert_matches!(res, Ok(_));
        let db_check = dao.select(&vec!["field1".into(), "field2".into()], "test", &vec![]);
        assert_eq!(
            db_check.unwrap(),
            vec![Record::from([
                ("field1".to_string(), Value::from(124)),
                ("field2".to_string(), Value::from("hello"))
            ]),]
        );
    }

    // #[test]
    // fn test_fetch() {
    //     let dao = mock();
    //     let sql = "SELECT field2 FROM test";
    //     let exp = vec![
    //         Vec::from([(Value::from("hello"))]),
    //         Vec::from([(Value::from("hello"))]),
    //     ];
    //     let act = dao.fetch(sql).unwrap();
    //     assert_eq!(exp, act);
    // }

    #[test]
    fn test_process_wheres() {
        let exp_where_str = " WHERE x=?1 AND y=?2".to_string();
        let exp_args = vec![Value::from(1), Value::from("derp")];
        let wheres = vec![
            Where::new("x", Symbol::EQ, Value::from(1)),
            Where::new("y", Symbol::EQ, Value::from("derp")),
        ];
        let act = process_wheres(&wheres, 0);
        assert_eq!(exp_where_str, act.0);
        assert_eq!(exp_args, act.1);
    }

    #[test]
    fn test_select_string() {
        let fields = vec!["one".to_owned(), "two".to_owned(), "three".to_owned()];
        let table = "table";
        let exp = "SELECT one,two,three FROM table".to_string();
        let act = select_string(&fields, table);
        assert_eq!(exp, act);
    }

    #[test]
    fn test_insert_string() {
        let tests = vec![
            (
                vec!["field1".into(), "field2".into()],
                2,
                false,
                Ok("INSERT INTO test (field1,field2) VALUES ($1,$2)".into()),
            ),
            (
                vec![
                    "field1".into(),
                    "field2".into(),
                    "field3".into(),
                    "field4".into(),
                ],
                4,
                false,
                Ok("INSERT INTO test (field1,field2,field3,field4) VALUES ($1,$2,$3,$4)".into()),
            ),
            (
                vec!["field1".into(), "field2".into()],
                4,
                false,
                Ok("INSERT INTO test (field1,field2) VALUES ($1,$2),($3,$4)".into()),
            ),
            (
                vec!["field1".into(), "field2".into()],
                7,
                false,
                Err(Error::DbError(
                    "number of args doesn't work with number of records".into(),
                )),
            ),
            (
                vec!["field1".into()],
                1,
                true,
                Ok("INSERT OR REPLACE INTO test (field1) VALUES ($1)".into()),
            ),
        ];
        for (fields, num_args, replace, exp) in tests {
            let act = insert_string(&fields, "test", num_args, replace);
            assert_eq!(exp, act);
        }
    }

    #[test]
    fn test_update_string() {
        let exp = String::from("UPDATE test SET field1=$1,field2=$2");
        let act = update_string(
            &vec!["field1".to_owned(), "field2".to_owned()],
            "test",
            2_usize,
        );
        assert_eq!(exp, act);
    }

    #[test]
    fn test_delete_string() {
        let exp = String::from("DELETE FROM test");
        let act = delete_string("test");
        assert_eq!(exp, act);
    }
}
