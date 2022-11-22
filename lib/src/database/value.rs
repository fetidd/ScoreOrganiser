use crate::{
    errors::{Error, Result},
    useful::{date_from_str, date_to_str},
};
use chrono::NaiveDate;
use rusqlite::types::Value as RVal;
use rusqlite::{
    types::{FromSql, ToSqlOutput},
    Error as SqlError, ToSql,
};

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum Value {
    Text(String),
    Integer(i32),
    TextList(Vec<String>),
    IntegerList(Vec<i32>),
}

impl Value {
    pub fn contains(&self, value: &Value) -> Result<bool> {
        match self {
            Self::TextList(tl) => match value {
                Self::Text(t) => Ok(tl.contains(&t)),
                _ => Err(Error::ValueError("TextList only holds Strings".to_string())),
            },
            Self::IntegerList(il) => match value {
                Self::Integer(i) => Ok(il.contains(&i)),
                _ => Err(Error::ValueError("IntegerList only holds i32s".to_string())),
            },
            _ => Err(Error::ValueError(
                "Only _List Values can contain values".to_string(),
            )),
        }
    }
}

impl ToSql for Value {
    fn to_sql(&self) -> std::result::Result<ToSqlOutput, SqlError> {
        match self {
            Self::Integer(i) => Ok(ToSqlOutput::Owned(RVal::Integer(*i as i64))),
            Self::Text(t) => Ok(ToSqlOutput::Owned(RVal::Text(t.clone()))),
            _ => Err(SqlError::ToSqlConversionFailure(Box::new(ValueError::new(
                "Unable to convert to sql",
            )))),
        }
    }
}

impl FromSql for Value {
    fn column_result(value: rusqlite::types::ValueRef<'_>) -> rusqlite::types::FromSqlResult<Self> {
        match value {
            rusqlite::types::ValueRef::Null => todo!(),
            rusqlite::types::ValueRef::Integer(i) => Ok(Value::from(i as i32)),
            rusqlite::types::ValueRef::Real(_) => Err(rusqlite::types::FromSqlError::InvalidType),
            rusqlite::types::ValueRef::Text(t) => unsafe {
                // unsafe because unchecked conversion
                Ok(Value::from(String::from_utf8_unchecked(t.to_vec())))
            },
            rusqlite::types::ValueRef::Blob(_) => Err(rusqlite::types::FromSqlError::InvalidType),
        }
    }
}

#[derive(Debug)]
struct ValueError {
    msg: String,
}

impl std::error::Error for ValueError {}

impl std::fmt::Display for ValueError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ValueError: {}", self.msg)
    }
}

impl ValueError {
    fn new(msg: &str) -> Self {
        ValueError {
            msg: msg.to_owned(),
        }
    }
}

impl From<i32> for Value {
    fn from(val: i32) -> Self {
        Value::Integer(val)
    }
}

impl From<&str> for Value {
    fn from(val: &str) -> Self {
        Value::Text(val.to_string())
    }
}

impl From<String> for Value {
    fn from(val: String) -> Self {
        Value::Text(val)
    }
}

impl From<Vec<&str>> for Value {
    fn from(val: Vec<&str>) -> Self {
        Value::TextList(val.into_iter().map(|x| x.to_string()).collect())
    }
}

impl From<NaiveDate> for Value {
    fn from(val: NaiveDate) -> Self {
        Value::Text(date_to_str(val))
    }
}

impl TryInto<NaiveDate> for Value {
    type Error = Error;

    fn try_into(self) -> Result<NaiveDate> {
        match self {
            Value::Text(date_string) => date_from_str(&date_string),
            _ => Err(Error::ValueError("Not a date".to_string())),
        }
    }
}

impl TryInto<NaiveDate> for &Value {
    type Error = Error;

    fn try_into(self) -> Result<NaiveDate> {
        match self {
            Value::Text(date_string) => date_from_str(&date_string),
            _ => Err(Error::ValueError("Not a date".to_string())),
        }
    }
}

impl TryInto<i32> for Value {
    type Error = Error;

    fn try_into(self) -> Result<i32> {
        match self {
            Value::Integer(n) => Ok(n),
            _ => Err(Error::ValueError("Not an i32".to_string())),
        }
    }
}

impl TryInto<String> for Value {
    type Error = Error;

    fn try_into(self) -> Result<String> {
        match self {
            Value::Text(t) => Ok(t),
            _ => Err(Error::ValueError("Not a String".to_string())),
        }
    }
}

impl TryInto<String> for &Value {
    type Error = Error;

    fn try_into(self) -> Result<String> {
        match self {
            Value::Text(t) => Ok(t.clone()),
            _ => Err(Error::ValueError("Not a String".to_string())),
        }
    }
}

impl TryInto<i32> for &Value {
    type Error = Error;

    fn try_into(self) -> Result<i32> {
        match self {
            Value::Integer(n) => Ok(*n),
            _ => Err(Error::ValueError("Not an i32".to_string())),
        }
    }
}

impl TryInto<Vec<String>> for &Value {
    type Error = Error;

    fn try_into(self) -> Result<Vec<String>> {
        match self {
            Value::TextList(tl) => Ok(tl.clone()),
            _ => Err(Error::ValueError("Not a Vec<String>".to_string())),
        }
    }
}
