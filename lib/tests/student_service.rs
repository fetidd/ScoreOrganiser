use std::{fs::remove_file, sync::Arc};

use scorg_lib::{
    constant::*,
    database::SqliteDao,
    function_name,
    models::{Score, Student},
    student_service::SqliteStudentService,
    useful::*,
};

fn setup(extra_sqls: Vec<String>, file_path: &str) -> SqliteStudentService {
    let mut sqls = vec![
        ENABLE_FOREIGN_KEYS.into(),
        STUDENT_SCHEMA.into(),
        SCORE_SCHEMA.into(),
    ];
    sqls.extend(extra_sqls);
    let dao = SqliteDao::new(file_path);
    execute_sqls(sqls, &dao).expect("sqls failed");
    let service = SqliteStudentService::new(Arc::new(dao)).unwrap();
    service
}

#[test]
fn can_add_a_student() {
    let db_path = format!("{}.sqlite", function_name!());
    let service = setup(vec![], db_path.as_str());
    service
        .add_student(&Student::new("first", "last", "1970-01-01").unwrap())
        .expect("failed to add student");
    let all = service.all().expect("failed to get all students");
    let s = all.get(0).expect("didn't find a student");
    drop(service);
    remove_file(db_path.as_str()).unwrap();
    assert_eq!(s.first_names, "first".to_owned());
    assert_eq!(s.last_name, "last".to_owned());
    assert_eq!(s.scores, vec![]);
    assert!(!s.id.is_empty()); // randomised id so just make sure its a populated string TODO regex?
}

#[test]
fn adding_a_score() {
    let sqls = vec!["INSERT INTO student VALUES ('st1', 'Ben', 'Jones', '1990-01-23')".into()];
    let db_path = format!("{}.sqlite", function_name!());
    let service = setup(sqls, &db_path.as_str());
    let score = Score::new("st1", 33, 11, "2022-01-01").unwrap();
    service.add_score(&score).expect("failed to add score");
    let s = service.get("st1").expect("couldn't find student");
    drop(service);
    remove_file(db_path.as_str()).unwrap();
    assert!((&s.scores).contains(&score));
}

#[test]
fn adding_multiple_score() {
    let sqls: Vec<String> = vec![
        "INSERT INTO student VALUES ('st1', 'Ben', 'Jones', '1990-01-23')".into(),
        "INSERT INTO student VALUES ('st2', 'Gemma', 'Forbes', '1990-01-23')".into(),
    ];
    let db_path = format!("{}.sqlite", function_name!());
    let service = setup(sqls, &db_path.as_str());
    let scores = vec![
        Score::new("st1", 33, 11, "2022-01-01").unwrap(),
        Score::new("st2", 33, 11, "2022-01-01").unwrap(),
    ];
    service.add_scores(&scores).expect("failed to add score");
    let ben = service.get("st1").expect("couldn't find student");
    let gem = service.get("st2").expect("couldn't find student");
    drop(service);
    remove_file(db_path.as_str()).unwrap();
    assert!((&ben.scores).contains(&scores[0]));
    assert!((&gem.scores).contains(&scores[1]));
}

#[test]
fn delete_a_student() {
    let sqls = vec![
        format!(
            "INSERT INTO student VALUES ('st1', 'Ben', 'Jones', '1990-01-23')"
        ),
        format!(
            "INSERT INTO student VALUES ('st2', 'Gemma', 'Forbes', '1990-01-23')",
        ),
    ];
    let db_path = format!("{}.sqlite", function_name!());
    let service = setup(sqls, &db_path.as_str());
    service
        .delete_student("st2")
        .expect("failed to delete student st2");
    let students = service
        .all()
        .expect("failed to get all students after deleting");
    drop(service);
    remove_file(db_path.as_str()).unwrap();
    assert_eq!(
        students,
        vec![Student {
            id: "st1".into(),
            first_names: "Ben".into(),
            last_name: "Jones".into(),
            scores: vec![],
            date_of_birth: date_from_str("1990-01-23").unwrap()
        }]
    )
}
