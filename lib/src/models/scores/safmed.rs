use crate::database::Record;
use crate::errors::{Error, Result};
use crate::useful::*;
use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SafmedScore {
    pub id: String,
    pub correct: i32,
    pub incorrect: i32,
    pub date: NaiveDate,
}

impl SafmedScore {
    pub fn new(id: &str, correct: i32, incorrect: i32, date: &str) -> Result<Self> {
        Ok(SafmedScore {
            id: String::from(id),
            correct,
            incorrect,
            date: date_from_str(date)?,
        })
    }
}

impl TryFrom<Record> for SafmedScore {
    type Error = Error;
    fn try_from(rec: Record) -> Result<Self> {
        let id = match rec.get("id") {
            Some(v) => v.try_into()?,
            None => return Err(Error::ValueError("Missing id".to_string())),
        };
        let correct = match rec.get("correct") {
            Some(v) => v.try_into()?,
            None => return Err(Error::ValueError("Missing correct".to_string())),
        };
        let incorrect = match rec.get("incorrect") {
            Some(v) => v.try_into()?,
            None => return Err(Error::ValueError("Missing incorrect".to_string())),
        };
        let date = match rec.get("date") {
            Some(v) => v.try_into()?,
            None => return Err(Error::ValueError("Missing date".to_string())),
        };
        Ok(SafmedScore {
            id,
            correct,
            incorrect,
            date,
        })
    }
}

#[cfg(test)]
mod test_score {
    use super::*;

    #[test]
    fn test_new() {
        let s = SafmedScore::new("test_id", 10, 5, "2021-01-01").unwrap();
        assert_eq!(s.id, "test_id".to_owned());
        assert_eq!(s.correct, 10);
        assert_eq!(s.incorrect, 5);
        assert_eq!(s.date, date_from_str("2021-01-01").unwrap());

        let err = SafmedScore::new("test_id", 10, 5, "").unwrap_err();
        assert_eq!(
            err,
            Error::BadDateConversion("'' is not a valid date-formatted string".into())
        )
    }

    #[test]
    fn test_try_from() {
        let tests: Vec<(Record, Result<SafmedScore>)> = vec![
            (
                Record::from([
                    ("id".into(), "st1".into()),
                    ("correct".into(), 10.into()),
                    ("incorrect".into(), 5.into()),
                    ("date".into(), "2021-01-01".into()),
                ]),
                Ok(SafmedScore {
                    id: "st1".into(),
                    correct: 10.into(),
                    incorrect: 5.into(),
                    date: date_from_str("2021-01-01").unwrap(),
                }),
            ),
            (
                Record::from([
                    ("id".into(), "st1".into()),
                    ("correct".into(), 10.into()),
                    ("date".into(), "2021-01-01".into()),
                ]),
                Err(Error::ValueError("Missing incorrect".to_owned())),
            ),
        ];
        for (rec, exp) in tests {
            let act = SafmedScore::try_from(rec);
            match exp.is_err() {
                true => assert_eq!(exp.unwrap_err(), act.unwrap_err()),
                false => assert_eq!(exp.unwrap(), act.unwrap()),
            };
        }
    }
}
