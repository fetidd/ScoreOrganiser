use crate::constant::*;
use crate::database::{Dao, Symbol, Value, Where};
use crate::errors::{Error, Result};
use crate::models::{Score, Student};
use crate::useful::date_to_str;
use std::collections::HashMap;
use std::sync::Arc;

pub struct SqliteStudentService {
    dao: Arc<dyn Dao>,
}

impl SqliteStudentService {
    pub fn new(dao: Arc<dyn Dao>) -> Result<Self> {
        log::debug!("created new SqliteStudentService");
        Ok(Self { dao })
    }

    pub fn init(&self) -> Result<()> {
        log::debug!("initialising...");
        let sqls = [ENABLE_FOREIGN_KEYS, STUDENT_SCHEMA, SCORE_SCHEMA];
        for sql in sqls {
            log::debug!("executing {sql}");
            self.dao.execute(sql)?;
        }
        Ok(())
    }

    fn select_students(&self, wheres: &Vec<Where>) -> Result<Vec<Student>> {
        // get all the scores first, in a hashmap of id -> vec of scores with that id
        // unless the wheres contain a Where that looks for an id, in which case we might as well only look for those scores
        // get the relevant students based on the wheres
        let students = self.dao.select(&student_fields(), "student", wheres);
        let mut student_vec = Vec::new();
        for student_record in students? {
            let student = match Student::try_from(student_record) {
                Ok(student) => student,
                Err(e) => return Err(e),
            };
            student_vec.push(student);
        }
        log::debug!("got {:#?}", &student_vec);
        Ok(student_vec)
    }

    pub fn all(&self) -> Result<Vec<Student>> {
        log::debug!("getting all students");
        self.select_students(&vec![])
    }

    pub fn get(&self, id: &str) -> Result<Student> {
        log::debug!("getting student with id {id}");
        let wheres = vec![Where::new("id", Symbol::EQ, Value::from(id))];
        let mut result = self.select_students(&wheres)?;
        match result.len() {
            1 => Ok(result.remove(0)),
            _ => Err(Error::NoStudent),
        }
    }

    pub fn get_id_for_name(&self, first: &str, last: &str) -> Result<String> {
        log::debug!("getting id for student with name {first} {last}");
        let wheres = vec![
            Where::new("first_names", Symbol::EQ, Value::from(first)),
            Where::new("last_name", Symbol::EQ, Value::from(last)),
        ];
        let mut result = self
            .dao
            .select(&vec!["id".to_owned()], "student", &wheres)?;
        let id_hm = match result.len() {
            1 => result.remove(0),
            _ => return Err(Error::NoStudent),
        };
        match id_hm.get("id") {
            Some(val) => match val {
                Value::Text(t) => Ok(t.clone()),
                _ => Err(Error::ValueError(
                    "something went very wrong here...".into(),
                )),
            },
            None => Err(Error::ValueError(
                "something went very wrong here...".into(),
            )),
        }
    }

    pub fn add_student(&self, student: &Student) -> Result<usize> {
        log::debug!("adding student {student:?}");
        self.dao.insert(
            &student_fields(),
            "student",
            vec![
                student.id.clone().into(),
                student.first_names.clone().into(),
                student.last_name.clone().into(),
                student.date_of_birth.clone().into(),
            ],
        )
    }

    pub fn add_students(&self, students: &Vec<Student>) -> Result<usize> {
        log::debug!("adding students {students:#?}");
        let mut args: Vec<Value> = vec![];
        for student in students {
            args.push(student.id.to_owned().into());
            args.push(student.first_names.clone().into());
            args.push(student.last_name.clone().into());
            args.push(student.date_of_birth.to_owned().into());
        }
        let added = self.dao.insert(&student_fields(), "student", args)?;
        Ok(added)
    }

    pub fn add_score(&self, score: &Score) -> Result<usize> {
        log::debug!("adding score {score:?}");
        let added = self.dao.insert(
            &score_fields(),
            "score",
            vec![
                score.id.to_owned().into(),
                score.correct.into(),
                score.incorrect.into(),
                score.date.to_owned().into(),
            ],
        )?;
        Ok(added)
    }

    pub fn add_scores(&self, scores: &Vec<Score>) -> Result<usize> {
        log::debug!("adding scores {scores:?}");
        // get vec of all the args to pass to insert
        let mut args = Vec::new();
        // get the latest score for each id
        let mut latest: HashMap<String, Score> = HashMap::new();
        for score in scores {
            args.push(score.id.to_owned().into());
            args.push(score.correct.into());
            args.push(score.incorrect.into());
            args.push(score.date.to_owned().into());
            match latest.get(&score.id.to_owned()) {
                Some(s) => {
                    if s.date < score.date {
                        latest.insert(score.id.to_owned(), score.clone());
                    }
                }
                None => {
                    latest.insert(score.id.to_owned(), score.clone());
                }
            };
        }
        let added = self.dao.insert(&score_fields(), "score", args)?;
        Ok(added)
    }

    pub fn delete_student(&self, id: &str) -> Result<usize> {
        log::debug!("deleting student with id {id}");
        let w = vec![Where::new("id", Symbol::EQ, Value::from(id))];
        let deleted = self.dao.delete("student", &w)?;
        self.dao.delete("score", &w)?;
        Ok(deleted)
    }

    pub fn update_student(&self, update: &Student) -> Result<usize> {
        log::debug!("updating student with id {}", update.id);
        let wheres = vec![Where::new("id", Symbol::EQ, Value::from(update.id.clone()))];
        let args = vec![update.id.clone().into(), update.first_names.clone().into(), update.last_name.clone().into(), date_to_str(update.date_of_birth).into()];
        let updated = self.dao.update(&student_fields(), "student", args, &wheres)?;
        Ok(updated)
    }

    pub fn get_safmed_scores(&self, id: &str) -> Result<Vec<Score>> {
        let records = self.dao.select(&score_fields(), "score", &vec![Where::new("id", Symbol::EQ, id.into())]);
        let mut score_vec = Vec::new();
        for score_record in records? {
            let score = match Score::try_from(score_record) {
                Ok(score) => score,
                Err(e) => return Err(e),
            };
            score_vec.push(score);
        };
        Ok(score_vec)
    }
}

fn student_fields() -> Vec<String> {
    STUDENT_FIELDS
        .to_vec()
        .iter()
        .map(|x| x.to_string())
        .collect()
}

fn score_fields() -> Vec<String> {
    SCORE_FIELDS
        .to_vec()
        .iter()
        .map(|x| x.to_string())
        .collect()
}

// #################
// ##### TESTS #####
// #################

#[cfg(test)]
mod tests {
    use super::*;
    use crate::database::{dao::MockDao, Record, Symbol, Value, Where};
    use crate::models::{Score, Student};
    use crate::useful::*;

    type ServiceTest = Vec<(Vec<Record>, Vec<Student>)>;

    #[test]
    fn test_all() {
        let tests: ServiceTest = vec![(
            vec![
                [
                    ("id".into(), "st1".into()),
                    ("first_names".into(), "Ben".into()),
                    ("last_name".into(), "Jones".into()),
                    ("date_of_birth".into(), "1990-01-23".into()),
                ]
                .into(),
                [
                    ("id".into(), "st2".into()),
                    ("first_names".into(), "Gemma Victoria".into()),
                    ("last_name".into(), "Mercer-Forbes".into()),
                    ("date_of_birth".into(), "1988-09-30".into()),
                ]
                .into(),
            ],
            vec![
                Student {
                    id: "st1".into(),
                    first_names: "Ben".into(),
                    last_name: "Jones".into(),
                    date_of_birth: date_from_str("1990-01-23").unwrap(),
                },
                Student {
                    id: "st2".into(),
                    first_names: "Gemma Victoria".into(),
                    last_name: "Mercer-Forbes".into(),
                    date_of_birth: date_from_str("1988-09-30").unwrap(),
                },
            ],
        )];
        for (students, expected) in tests {
            let mut dao = MockDao::new();
            dao.expect_select()
                .withf(|f, t, w| *f == student_fields() && t == "student" && *w == vec![])
                .times(1)
                .returning(move |_, _, _| Ok(students.clone()));
            let dao: Arc<dyn Dao> = Arc::new(dao);
            let ss = SqliteStudentService::new(Arc::clone(&dao)).unwrap();
            let actual = ss.all().unwrap();
            assert_eq!(expected, actual);
        }
    }

    #[test]
    fn test_get() {
        let tests: ServiceTest = vec![(
            vec![[
                ("id".into(), "st1".into()),
                ("first_names".into(), "Ben".into()),
                ("last_name".into(), "Jones".into()),
                ("date_of_birth".into(), "1990-01-23".into()),
            ]
            .into()],
            vec![Student {
                id: "st1".into(),
                first_names: "Ben".into(),
                last_name: "Jones".into(),
                date_of_birth: date_from_str("1990-01-23").unwrap(),
            }],
        )];
        for (students, expected) in tests {
            let mut dao = MockDao::new();
            dao.expect_select()
                .withf(|f, t, w| {
                    *f == student_fields()
                        && t == "student"
                        && *w == vec![Where::new("id", Symbol::EQ, Value::from("st1"))]
                })
                .times(1)
                .returning(move |_, _, _| Ok(students.clone()));
            let ss = SqliteStudentService::new(Arc::new(dao)).unwrap();
            let actual = ss.get("st1").unwrap();
            assert_eq!(expected[0], actual);
        }
    }

    #[test]
    fn test_get_id_for_name() {
        let tests = [(
            vec![Record::from([
                ("id".to_string(), Value::from("st1")),
                ("first_names".to_string(), Value::from("Ben")),
                ("last_name".to_string(), Value::from("Jones")),
            ])],
            String::from("st1"),
        )];
        for (students, expected) in tests {
            let mut dao = MockDao::new();
            dao.expect_select()
                .withf(|f, t, w| {
                    *f == vec!["id".to_owned()]
                        && t == "student"
                        && *w
                            == vec![
                                Where::new("first_names", Symbol::EQ, Value::from("Ben")),
                                Where::new("last_name", Symbol::EQ, Value::from("Jones")),
                            ]
                })
                .times(1)
                .returning(move |_, _, _| Ok(students.clone()));
            let ss = SqliteStudentService::new(Arc::new(dao)).unwrap();
            let actual = ss.get_id_for_name("Ben", "Jones").unwrap();
            assert_eq!(expected, actual);
        }
    }

    #[test]
    fn test_add_student() {
        let mut dao = MockDao::new();
        let student = Student {
            id: "st1".into(),
            first_names: "Ben".into(),
            last_name: "Jones".into(),
            date_of_birth: date_from_str("1990-01-23").unwrap(),
        };
        dao.expect_insert()
            .withf(move |f, t, args| {
                *f == student_fields()
                    && t == "student"
                    && *args
                        == vec![
                            "st1".into(),
                            "Ben".into(),
                            "Jones".into(),
                            "1990-01-23".into(),
                        ]
            })
            .times(1)
            .returning(move |_, _, _| Ok(1));
        let ss = SqliteStudentService::new(Arc::new(dao)).unwrap();
        assert_eq!(Ok(1), ss.add_student(&student));
    }

    #[test]
    fn test_add_students() {
        let mut dao = MockDao::new();
        let students = vec![
            Student {
                id: "st1".into(),
                first_names: "Ben".into(),
                last_name: "Jones".into(),
                date_of_birth: date_from_str("1990-01-23").unwrap(),
            },
            Student {
                id: "st2".into(),
                first_names: "Gemma".into(),
                last_name: "Forbes".into(),
                date_of_birth: date_from_str("1988-09-30").unwrap(),
            },
        ];
        dao.expect_insert()
            .withf(move |f, t, args| {
                *f == student_fields()
                    && t == "student"
                    && *args
                        == vec![
                            "st1".into(),
                            "Ben".into(),
                            "Jones".into(),
                            "1990-01-23".into(),
                            "st2".into(),
                            "Gemma".into(),
                            "Forbes".into(),
                            "1988-09-30".into(),
                        ]
            })
            .times(1)
            .returning(move |_, _, _| Ok(2));
        let ss = SqliteStudentService::new(Arc::new(dao)).unwrap();
        assert_eq!(Ok(2), ss.add_students(&students));
    }

    #[test]
    fn test_add_score() {
        let mut dao = MockDao::new();
        let score = Score::new("st1".into(), 99, 11, "2022-01-01".into()).unwrap();
        dao.expect_insert()
            .withf(move |f, t, args| {
                *f == score_fields()
                    && t == "score"
                    && *args == vec!["st1".into(), 99.into(), 11.into(), "2022-01-01".into()]
            })
            .times(1)
            .returning(move |_, _, _| Ok(1));
        let ss = SqliteStudentService::new(Arc::new(dao)).unwrap();
        assert_eq!(Ok(1), ss.add_score(&score));
    }

    #[test]
    fn test_add_scores() {
        let mut dao = MockDao::new();
        let scores = vec![
            Score::new("st1".into(), 99, 11, "2022-01-01".into()).unwrap(),
            Score::new("st1".into(), 87, 8, "2022-01-02".into()).unwrap(),
        ];
        dao.expect_insert()
            .withf(move |f, t, args| {
                *f == score_fields()
                    && t == "score"
                    && *args
                        == vec![
                            "st1".into(),
                            99.into(),
                            11.into(),
                            "2022-01-01".into(),
                            "st1".into(),
                            87.into(),
                            8.into(),
                            "2022-01-02".into(),
                        ]
            })
            .times(1)
            .returning(move |_, _, _| Ok(2));
        let ss = SqliteStudentService::new(Arc::new(dao)).unwrap();
        assert_eq!(Ok(2), ss.add_scores(&scores));
    }

    #[test]
    fn test_add_scores_for_multiple_students() {
        let mut dao = MockDao::new();
        let scores = vec![
            Score::new("st1".into(), 99, 11, "2022-01-01".into()).unwrap(),
            Score::new("st1".into(), 87, 8, "2022-01-02".into()).unwrap(),
            Score::new("st2".into(), 99, 11, "2022-01-01".into()).unwrap(),
            Score::new("st3".into(), 87, 8, "2022-01-02".into()).unwrap(),
        ];
        dao.expect_insert()
            .withf(move |fields, table, args| {
                *fields == score_fields()
                    && table == "score"
                    && *args
                        == vec![
                            "st1".into(),
                            99.into(),
                            11.into(),
                            "2022-01-01".into(),
                            "st1".into(),
                            87.into(),
                            8.into(),
                            "2022-01-02".into(),
                            "st2".into(),
                            99.into(),
                            11.into(),
                            "2022-01-01".into(),
                            "st3".into(),
                            87.into(),
                            8.into(),
                            "2022-01-02".into(),
                        ]
            })
            .times(1)
            .returning(move |_, _, _| Ok(4));
        let ss = SqliteStudentService::new(Arc::new(dao)).unwrap();
        assert_eq!(Ok(4), ss.add_scores(&scores));
    }

    #[test]
    fn test_delete_student() {
        let mut dao = MockDao::new();
        dao.expect_delete()
            .withf(move |table, wheres| {
                table == "student" && *wheres == vec![Where::new("id", Symbol::EQ, "st1".into())]
            })
            .times(1)
            .returning(move |_, _| Ok(1));
        dao.expect_delete()
            .withf(move |table, wheres| {
                table == "score" && *wheres == vec![Where::new("id", Symbol::EQ, "st1".into())]
            })
            .times(1)
            .returning(move |_, _| Ok(1));
        let ss = SqliteStudentService::new(Arc::new(dao)).unwrap();
        assert_eq!(Ok(1), ss.delete_student("st1"));
    }

    #[test]
    fn test_update_student() {
        let update = Student {
            id: "st1".into(),
            first_names: "Ben".into(),
            last_name: "Jones".into(),
            date_of_birth: date_from_str("1990-01-23").unwrap()
        };
        let mut dao = MockDao::new();
        dao.expect_update()
            .withf(move |fields, table, args, wheres| {
                *fields == student_fields() &&
                table == "student" &&
                *args == vec!["st1".into(), "Ben".into(), "Jones".into(), date_to_str(update.date_of_birth).into()] &&
                *wheres == vec![Where::new("id", Symbol::EQ, "st1".into())]
            })
            .times(1)
            .returning(move |_,_,_,_| {
                Ok(1)
            });
        let ss = SqliteStudentService::new(Arc::new(dao)).unwrap();
        assert_eq!(Ok(1), ss.update_student(&update));
    }

    #[test]
    fn test_get_safmed_scores() {
        let mut dao = MockDao::new();
        dao.expect_select().withf(move |_,table,_| table=="score").times(1).returning(move |_,_,_| {
            Ok(vec![
                Record::from([("id".into(), "st1".into()), ("correct".into(), 89.into()), ("incorrect".into(), 19.into()), ("date".into(), date_from_str("2021-01-01").unwrap().into())]),
                Record::from([("id".into(), "st1".into()), ("correct".into(), 89.into()), ("incorrect".into(), 19.into()), ("date".into(), date_from_str("2021-01-02").unwrap().into())]),
                Record::from([("id".into(), "st1".into()), ("correct".into(), 89.into()), ("incorrect".into(), 19.into()), ("date".into(), date_from_str("2021-01-03").unwrap().into())]),
            ])
        });
        let ss = SqliteStudentService::new(Arc::new(dao)).unwrap();
        assert_eq!(ss.get_safmed_scores("st1"), Ok(vec![
            Score{id: "st1".into(), correct: 89.into(), incorrect: 19.into(), date: date_from_str("2021-01-01").unwrap()},
            Score{id: "st1".into(), correct: 89.into(), incorrect: 19.into(), date: date_from_str("2021-01-02").unwrap()},
            Score{id: "st1".into(), correct: 89.into(), incorrect: 19.into(), date: date_from_str("2021-01-03").unwrap()},
        ]));
    }
}
