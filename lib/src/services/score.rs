use std::collections::HashMap;
use std::sync::Arc;

use crate::constant::{SCORE_FIELDS, SCORE_SCHEMA};
use crate::database::{Where, Symbol, Dao};
use crate::errors::Result;
use crate::models::SafmedScore;

pub struct SafmedScoreService {
    dao: Arc<dyn Dao>
}

impl SafmedScoreService {
    pub fn new(dao: Arc<dyn Dao>) -> Self {
        Self {dao}
    }

    pub fn init(&self) -> Result<()> {
        log::debug!("initialising...");
        let sqls = [SCORE_SCHEMA];
        for sql in sqls {
            log::debug!("executing {sql}");
            self.dao.execute(sql)?;
        }
        Ok(())
    }

    pub fn add_score(&self, score: &SafmedScore) -> Result<usize> {
        log::debug!("adding score {score:?}");
        let added = self.dao.insert(
            &score_fields(),
            "safmed",
            vec![
                score.id.to_owned().into(),
                score.correct.into(),
                score.incorrect.into(),
                score.date.to_owned().into(),
            ],
        )?;
        Ok(added)
    }

    pub fn add_scores(&self, scores: &Vec<SafmedScore>) -> Result<usize> {
        log::debug!("adding scores {scores:?}");
        // get vec of all the args to pass to insert
        let mut args = Vec::new();
        // get the latest score for each id
        let mut latest: HashMap<String, SafmedScore> = HashMap::new();
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
        let added = self.dao.insert(&score_fields(), "safmed", args)?;
        Ok(added)
    }

    pub fn get_safmed_scores(&self, id: &str) -> Result<Vec<SafmedScore>> {
        let records = self.dao.select(&score_fields(), "safmed", &vec![Where::new("id", Symbol::EQ, id.into())]);
        let mut score_vec = Vec::new();
        for score_record in records? {
            let score = match SafmedScore::try_from(score_record) {
                Ok(score) => score,
                Err(e) => return Err(e),
            };
            score_vec.push(score);
        };
        Ok(score_vec)
    }
}

fn score_fields() -> Vec<String> {
    SCORE_FIELDS
        .to_vec()
        .iter()
        .map(|x| x.to_string())
        .collect()
}

#[cfg(test)]
mod tests {
    use crate::{database::{dao::MockDao, Record}, useful::date_from_str};

    use super::*;
    
    #[test]
    fn test_add_score() {
        let mut dao = MockDao::new();
        let score = SafmedScore::new("st1".into(), 99, 11, "2022-01-01".into()).unwrap();
        dao.expect_insert()
            .withf(move |f, t, args| {
                *f == score_fields()
                    && t == "safmed"
                    && *args == vec!["st1".into(), 99.into(), 11.into(), "2022-01-01".into()]
            })
            .times(1)
            .returning(move |_, _, _| Ok(1));
        let ss = SafmedScoreService::new(Arc::new(dao));
        assert_eq!(Ok(1), ss.add_score(&score));
    }

    #[test]
    fn test_add_scores() {
        let mut dao = MockDao::new();
        let scores = vec![
            SafmedScore::new("st1".into(), 99, 11, "2022-01-01".into()).unwrap(),
            SafmedScore::new("st1".into(), 87, 8, "2022-01-02".into()).unwrap(),
        ];
        dao.expect_insert()
            .withf(move |f, t, args| {
                *f == score_fields()
                    && t == "safmed"
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
        let ss = SafmedScoreService::new(Arc::new(dao));
        assert_eq!(Ok(2), ss.add_scores(&scores));
    }

    #[test]
    fn test_add_scores_for_multiple_students() {
        let mut dao = MockDao::new();
        let scores = vec![
            SafmedScore::new("st1".into(), 99, 11, "2022-01-01".into()).unwrap(),
            SafmedScore::new("st1".into(), 87, 8, "2022-01-02".into()).unwrap(),
            SafmedScore::new("st2".into(), 99, 11, "2022-01-01".into()).unwrap(),
            SafmedScore::new("st3".into(), 87, 8, "2022-01-02".into()).unwrap(),
        ];
        dao.expect_insert()
            .withf(move |fields, table, args| {
                *fields == score_fields()
                    && table == "safmed"
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
        let ss = SafmedScoreService::new(Arc::new(dao));
        assert_eq!(Ok(4), ss.add_scores(&scores));
    }

    #[test]
    fn test_get_safmed_scores() {
        let mut dao = MockDao::new();
        dao.expect_select().withf(move |_,table,_| table=="safmed").times(1).returning(move |_,_,_| {
            Ok(vec![
                Record::from([("id".into(), "st1".into()), ("correct".into(), 89.into()), ("incorrect".into(), 19.into()), ("date".into(), date_from_str("2021-01-01").unwrap().into())]),
                Record::from([("id".into(), "st1".into()), ("correct".into(), 89.into()), ("incorrect".into(), 19.into()), ("date".into(), date_from_str("2021-01-02").unwrap().into())]),
                Record::from([("id".into(), "st1".into()), ("correct".into(), 89.into()), ("incorrect".into(), 19.into()), ("date".into(), date_from_str("2021-01-03").unwrap().into())]),
            ])
        });
        let ss = SafmedScoreService::new(Arc::new(dao));
        assert_eq!(ss.get_safmed_scores("st1"), Ok(vec![
            SafmedScore{id: "st1".into(), correct: 89.into(), incorrect: 19.into(), date: date_from_str("2021-01-01").unwrap()},
            SafmedScore{id: "st1".into(), correct: 89.into(), incorrect: 19.into(), date: date_from_str("2021-01-02").unwrap()},
            SafmedScore{id: "st1".into(), correct: 89.into(), incorrect: 19.into(), date: date_from_str("2021-01-03").unwrap()},
        ]));
    }
}