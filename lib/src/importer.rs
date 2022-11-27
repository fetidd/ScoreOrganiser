use std::sync::Arc;

use crate::errors::{Error, Result};
use crate::models::{SafmedScore, Student};
use crate::services::{StudentService, SafmedScoreService};
use crate::useful::*;
use csv::{Reader, StringRecord};

pub struct Importer {
    student_service: Arc<StudentService>,
    score_service: Arc<SafmedScoreService>
}
use log::*;

impl Importer {
    pub fn new(student_service: Arc<StudentService>, score_service: Arc<SafmedScoreService>) -> Importer {
        log::debug!("created new Importer");
        Importer { student_service, score_service }
    }

    fn parse_scores(record: Vec<&str>, id: &str, dates: Vec<&str>) -> Result<Vec<SafmedScore>> {
        let parsed_scores: Result<Vec<Option<(i32, i32, String)>>> = record
            .into_iter()
            .zip(dates)
            .map(Self::parse_score)
            .collect();
        let parsed_scores = match parsed_scores {
            Err(err) => return Err(err),
            Ok(parsed_scores) => parsed_scores
        };
        let mut scores = vec![];
        for score in parsed_scores {
            match score {
                Some(sc) => {
                    let new_score = SafmedScore::new(id, sc.0, sc.1, &sc.2)?;
                    scores.push(new_score);
                },
                None => continue
            };
        }
        Ok(scores)
    }

    fn parse_score(score_record: (&str, &str)) -> Result<Option<(i32, i32, String)>> {
        let scores: Vec<&str> = score_record
            .0
            .split("/")
            .map(|s: &str| {
                s.trim()
            })
            .collect();
        match scores.as_slice() {
            [correct, incorrect] => {
                let parsed: std::result::Result<Vec<i32>, std::num::ParseIntError> = vec![correct, incorrect]
                    .iter()
                    .map(|s| s.parse::<i32>())
                    .collect();
                let mut parsed_scores = match parsed {
                    Ok(parsed_scores) => parsed_scores,
                    Err(error) => return Err(Error::from(error))
                };
                let date = score_record.1.trim().to_owned();
                if !validate_date(&date) {
                    return Err(Error::ImporterError(format!(
                        "{} is not a valid date",
                        &date
                    )));
                }
                Ok(Some((parsed_scores.remove(0), parsed_scores.remove(0), date.to_string())))
            },
            [x] if x.is_empty() => {
                Ok(None)
            },
            _ => Err(Error::ImporterError(
                "must provide 2 scores per date".into(),
            ))
        }
    }

    fn extract_data(record: &StringRecord) -> Result<(String, String, String)> {
        let first_names = match record.get(0) {
            Some(s) => {
                let name = s.trim().to_owned();
                if name.is_empty() {
                    return Err(Error::ImporterError("blank first names".into()));
                }
                name
            }
            None => return Err(Error::ImporterError("no first names found".into())),
        };
        let last_name = match record.get(1) {
            Some(s) => {
                let name = s.trim().to_owned();
                if name.is_empty() {
                    return Err(Error::ImporterError("blank last name".into()));
                }
                name
            }
            None => return Err(Error::ImporterError("no last name found".into())),
        };
        let dob = match record.get(2) {
            Some(s) => {
                let name = s.trim().to_owned();
                if name.is_empty() {
                    return Err(Error::ImporterError("blank dob".into()));
                }
                name
            }
            None => return Err(Error::ImporterError("no dob found".into())),
        };
        Ok((first_names, last_name, dob))
    }

    fn get_id(
        &self,
        first_names: &str,
        last_name: &str,
        dob: &str,
    ) -> Result<(String, Option<Student>)> {
        match self.student_service.get_id_for_name(&first_names, &last_name) {
            // if there is one, just get it's id
            Ok(id) => Ok((id, None)),
            // add the student to students_to_add and get its id if not
            Err(_) => {
                let new_student = Student::new(&first_names, &last_name, &dob)?;
                let new_id = new_student.id.clone();
                Ok((new_id, Some(new_student)))
            }
        }
    }

    pub fn import(&self, data: &str) -> Result<(usize, usize)> {
        let mut reader = Reader::from_reader(data.as_bytes());
        let headers = reader.headers()?.clone();
        let dates: Vec<&str> = headers.into_iter().skip(3).collect();
        let records = reader.records();
        let mut students_to_add: Vec<Student> = vec![];
        let mut scores_to_add: Vec<SafmedScore> = vec![];
        for record in records {
            let r = record?;
            let (first_names, last_name, dob) = Self::extract_data(&r)?;
            debug!("found {} {}", &first_names, &last_name);
            let (id, new_student) = self.get_id(&first_names, &last_name, &dob)?;
            if new_student.is_some() {
                debug!("will add {} {} as new student", &first_names, &last_name);
                students_to_add.push(new_student.unwrap())
            };
            let scores_in_record = r.into_iter().skip(3).collect();
            let scores = Self::parse_scores(scores_in_record, &id, dates.clone())?;
            scores_to_add.extend(scores);
        }
        debug!("adding {} new students", &students_to_add.len());
        let students_added = self.student_service.add_students(&students_to_add)?;
        debug!("adding {} scores", &scores_to_add.len());
        let scores_added = self.score_service.add_scores(&scores_to_add)?;
        debug!(
            "added {} students and {} scores",
            students_added,
            scores_added
        );
        Ok((students_added, scores_added))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::database::{dao::MockDao, Value, Dao};
    use std::collections::HashMap;

    #[test]
    fn test_parse_score() {
        let tests: Vec<((&str, &str), Result<Option<(i32, i32, String)>>)> = vec![
            (("89/12", "2021-01-01"), Ok(Some((89, 12, "2021-01-01".into())))),
            (
                (" 89/12 ", "2021-01-01 "),
                Ok(Some((89, 12, "2021-01-01".into()))),
            ),
            (
                ("89/", "2021-01-01"),
                Err(Error::ParseIntError(
                    "cannot parse integer from empty string".into(),
                )),
            ),
            (
                ("/23", "2021-01-01"),
                Err(Error::ParseIntError(
                    "cannot parse integer from empty string".into(),
                )),
            ),
            (
                ("23", "2021-01-01"),
                Err(Error::ImporterError(
                    "must provide 2 scores per date".into(),
                )),
            ),
            (
                ("89/1", "2021-01 "),
                Err(Error::ImporterError("2021-01 is not a valid date".into())),
            ),
        ];
        for (input, expected) in tests {
            let actual = Importer::parse_score(input);
            assert_eq!(expected, actual);
        }
    }

    #[test]
    fn test_parse_scores() {
        let tests: Vec<(Vec<&str>, &str, Vec<&str>, Result<Vec<SafmedScore>>)> = vec![(
            vec!["67/23", "89/34"],
            "st1",
            vec!["2021-01-01", "2021-01-02"],
            Ok(vec![
                SafmedScore::new("st1", 67, 23, "2021-01-01").unwrap(),
                SafmedScore::new("st1", 89, 34, "2021-01-02").unwrap(),
            ]),
        )];
        for (rec, id, dates, exp) in tests {
            let actual = Importer::parse_scores(rec, id, dates);
            assert_eq!(exp, actual);
        }
    }

    #[test]
    fn test_extract_data() {
        let tests: Vec<(StringRecord, Result<(String, String, String)>)> = vec![
            (
                StringRecord::from(vec!["Ben", "Jones", "1990-01-23"]),
                Ok(("Ben".into(), "Jones".into(), "1990-01-23".into())),
            ),
            (
                StringRecord::from(vec!["", "Jones", ""]),
                Err(Error::ImporterError("blank first names".into())),
            ),
            (
                StringRecord::from(vec!["Ben", "", ""]),
                Err(Error::ImporterError("blank last name".into())),
            ),
        ];
        for (sr, exp) in tests {
            assert_eq!(exp, Importer::extract_data(&sr));
        }
    }

    #[test]
    fn test_get_id_existing_student() {
        let mut dao = MockDao::new();
        dao.expect_select().returning(move |_, _, _| {
            Ok(vec![HashMap::from([(
                "id".to_owned(),
                Value::from("existing_id"),
            )])])
        });
        let arc_dao: Arc<dyn Dao> = Arc::new(dao);
        let student_service = Arc::new(StudentService::new(Arc::clone(&arc_dao)));
        let score_service = Arc::new(SafmedScoreService::new(Arc::clone(&arc_dao)));
        let importer = Importer::new(student_service, score_service);
        let actual = importer.get_id("Ben", "Jones", "1999-01-23").unwrap();
        assert_eq!(actual.0, "existing_id".to_owned());
        assert!(actual.1.is_none());
    }

    #[test]
    fn test_get_id_new_student() {
        let mut dao = MockDao::new();
        dao.expect_select()
            .returning(move |_, _, _| Err(Error::NoStudent));
        let arc_dao: Arc<dyn Dao> = Arc::new(dao);
        let student_service = Arc::new(StudentService::new(Arc::clone(&arc_dao)));
        let score_service = Arc::new(SafmedScoreService::new(Arc::clone(&arc_dao)));
        let importer = Importer::new(student_service, score_service);
        let actual = importer.get_id("Ben", "Jones", "1990-01-23").unwrap();
        assert!(!actual.0.is_empty());
        assert_eq!(actual.0, actual.1.unwrap().id);
    }
}
