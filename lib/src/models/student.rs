use super::*;
use crate::database::Record;
use crate::errors::{Error, Result};
use crate::useful::*;
use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Student {
    pub id: String,
    pub first_names: String,
    pub last_name: String,
    pub date_of_birth: NaiveDate,
    pub scores: Vec<Score>,
}

impl Student {
    pub fn new(first: &str, last: &str, dob: &str) -> Result<Self> {
        Ok(Student {
            id: Uuid::new_v4().to_string(),
            first_names: first.to_string(),
            last_name: last.to_string(),
            date_of_birth: date_from_str(dob)?,
            scores: vec![],
        })
    }
}

impl TryFrom<Record> for Student {
    type Error = Error;
    fn try_from(rec: Record) -> Result<Self> {
        let id = match rec.get("id") {
            Some(v) => v.try_into()?,
            None => return Err(Error::ValueError("Missing id".to_string())),
        };
        let first_names = match rec.get("first_names") {
            Some(v) => v.try_into()?,
            None => return Err(Error::ValueError("Missing first_names".to_string())),
        };
        let last_name = match rec.get("last_name") {
            Some(v) => v.try_into()?,
            None => return Err(Error::ValueError("Missing last_name".to_string())),
        };
        let date_of_birth = match rec.get("date_of_birth") {
            Some(v) => v.try_into()?,
            None => return Err(Error::ValueError("Missing date_of_birth".to_string())),
        };
        Ok(Student {
            id,
            first_names,
            last_name,
            date_of_birth,
            scores: vec![],
        })
    }
}

#[cfg(test)]
mod test_student {
    use super::*;

    #[test]
    fn test_new() {
        let s = Student::new("first", "last", "1990-01-23").unwrap();
        assert_eq!(s.first_names, "first".to_owned());
        assert_eq!(s.last_name, "last".to_owned());
        assert_eq!(s.scores, vec![]);
        assert!(!s.id.is_empty()); // randomised id so just make sure its a populated string
    }

    #[test]
    fn test_try_from() {
        let tests: Vec<(Record, Result<Student>)> = vec![
            (
                Record::from([
                    ("id".into(), "st1".into()),
                    ("first_names".into(), "first".into()),
                    ("last_name".into(), "last".into()),
                    ("date_of_birth".into(), "1990-01-23".into()),
                ]),
                Ok(Student {
                    id: "st1".into(),
                    first_names: "first".into(),
                    last_name: "last".into(),
                    date_of_birth: date_from_str("1990-01-23").unwrap(),
                    scores: vec![],
                }),
            ),
            (
                Record::from([
                    ("first_names".into(), "first".into()),
                    ("last_name".into(), "last".into()),
                ]),
                Err(Error::ValueError("Missing id".to_owned())),
            ),
        ];
        for (rec, exp) in tests {
            let act = Student::try_from(rec);
            match exp.is_err() {
                true => assert_eq!(exp.unwrap_err(), act.unwrap_err()),
                false => assert_eq!(exp.unwrap(), act.unwrap()),
            };
        }
    }

    #[test]
    fn test_serde() {
        let student = Student {
            id: "st1".into(),
            first_names: "Ben".into(),
            last_name: "Jones".into(),
            date_of_birth: date_from_str("1990-01-23").unwrap(),
            scores: vec![],
        };
        let json_student = serde_json::to_string(&student).expect("failed to serialize");
        assert_eq!(json_student, "{\"id\":\"st1\",\"first_names\":\"Ben\",\"last_name\":\"Jones\",\"date_of_birth\":\"1990-01-23\",\"scores\":[]}".to_owned());
        assert_eq!(
            student,
            serde_json::from_str(&json_student).expect("failed to deserialize")
        );
    }
}
