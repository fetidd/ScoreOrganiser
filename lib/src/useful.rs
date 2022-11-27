use super::constant::*;
use crate::database::Dao;
use crate::errors::{Error, Result};
use chrono::{Duration, Local, NaiveDate};
use regex::Regex;
use std::fs::{create_dir, read_dir, write};
use std::thread::sleep;

pub fn date_to_str(d: NaiveDate) -> String {
    d.format("%Y-%m-%d").to_string()
}

pub fn curr_date() -> NaiveDate {
    Local::now().date_naive()
}

pub fn curr_date_str() -> String {
    date_to_str(curr_date())
}

pub fn date_from_str(s: &str) -> Result<NaiveDate> {
    match NaiveDate::parse_from_str(s, "%Y-%m-%d") {
        Ok(s) => Ok(s),
        Err(_) => Err(Error::BadDateConversion(format!(
            "'{s}' is not a valid date-formatted string"
        ))),
    }
}

pub fn prev_date(days: i64) -> Option<NaiveDate> {
    curr_date().checked_sub_signed(Duration::days(days))
}

pub fn prev_date_str(days: i64) -> String {
    match prev_date(days) {
        Some(date) => date.format("%Y-%m-%d").to_string(),
        None => String::new(),
    }
}

pub fn validate_date(date: &str) -> bool {
    let regex = Regex::new(r"^\d{4}-\d{2}-\d{2}$").unwrap();
    regex.is_match(date)
}

pub fn execute_sqls(sqls: Vec<String>, dao: &dyn Dao) -> Result<()> {
    for sql in sqls {
        dao.execute(&sql)?
    }
    Ok(())
}

pub fn wait() {
    sleep(std::time::Duration::from_secs_f64(0.5));
}

#[macro_export]
macro_rules! function_name {
    () => {{
        fn f() {}
        fn type_name_of<T>(_: T) -> &'static str {
            std::any::type_name::<T>()
        }
        let name = type_name_of(f);

        // Find and cut the rest of the path
        match &name[..name.len() - 3].rfind(':') {
            Some(pos) => &name[pos + 1..name.len() - 3],
            None => &name[..name.len() - 3],
        }
    }};
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_date_to_str() {
        assert_eq!(
            String::from("2021-01-01"),
            date_to_str(NaiveDate::from_ymd_opt(2021, 1, 1).unwrap())
        );
    }

    #[test]
    fn test_date_from_str() {
        assert_eq!(
            NaiveDate::from_ymd_opt(2021, 1, 1).unwrap(),
            date_from_str("2021-01-01".into()).unwrap()
        );
        assert_eq!(
            Error::BadDateConversion(String::from("'' is not a valid date-formatted string")),
            date_from_str("".into()).unwrap_err()
        );
    }

    #[test]
    fn test_validate_date() {
        let tests = vec![
            ("2021-01-01", true),
            ("2021-01-012", false),
            ("2021", false),
        ];
        for (date, exp) in tests {
            assert_eq!(exp, validate_date(date));
        }
    }
}
