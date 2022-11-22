use crate::database::Value;
use crate::errors::Result;
#[cfg(test)]
use mockall::{predicate::*, *};
use std::collections::HashMap;

#[cfg_attr(test, automock)]
pub trait Dao: Send + Sync {
    fn select(&self, fields: &Vec<String>, table: &str, wheres: &Vec<Where>)
        -> Result<Vec<Record>>;
    fn insert(&self, fields: &Vec<String>, table: &str, args: Vec<Value>) -> Result<usize>;
    fn update(
        &self,
        fields: &Vec<String>,
        table: &str,
        args: Vec<Value>,
        wheres: &Vec<Where>,
    ) -> Result<usize>;
    fn delete(&self, table: &str, wheres: &Vec<Where>) -> Result<usize>;
    fn execute(&self, sql: &str) -> Result<()>;
    // fn fetch(&self, sql: &str) -> Result<Vec<Vec<Value>>>;
}

#[derive(Clone, PartialEq)]
pub struct Where {
    pub(crate) field: String,
    pub(crate) symbol: Symbol,
    pub(crate) value: Value,
}

impl Where {
    pub fn new(field: &str, symbol: Symbol, value: Value) -> Self {
        Where {
            field: field.to_string(),
            symbol,
            value,
        }
    }
}

#[derive(Clone, PartialEq)]
pub enum Symbol {
    EQ,
    //NEQ,
    //GT,
    LT,
    //GTE,
    //LTE,
    IN,
}

impl Symbol {
    pub fn to_string(&self) -> String {
        let symbol = match self {
            Symbol::EQ => "=",
            //Symbol::NEQ => "!=",
            //Symbol::GT => ">",
            Symbol::LT => "<",
            //Symbol::GTE => ">=",
            //Symbol::LTE => "<=",
            Symbol::IN => "IN",
        };
        symbol.to_string()
    }
}

pub type Record = HashMap<String, Value>;
