use scorg_lib::{
    constant::*, database::SqliteDao, function_name, importer::Importer,
    student_service::SqliteStudentService, useful::*,
};
use std::{
    fs::{remove_file, File},
    io::Write,
    sync::Arc,
};

fn setup(mut sqls: Vec<String>, file_path: &str) -> SqliteStudentService {
    sqls.extend(vec![
        ENABLE_FOREIGN_KEYS.into(),
        STUDENT_SCHEMA.into(),
        SCORE_SCHEMA.into(),
    ]);
    let dao = SqliteDao::new(file_path);
    execute_sqls(sqls, &dao).expect("sqls failed");
    let service = SqliteStudentService::new(Arc::new(dao)).unwrap();
    service
}

#[test]
fn test_import() {
    let csv_path = format!("{}.csv", function_name!());
    let db_path = format!("{}.sqlite", function_name!());
    let mut csv_file = File::create(&csv_path).expect("failed to create test csv file");
    let csv_data = "\
first_names,last_name,date_of_birth,2021-02-01,2021-02-02,2021-02-03
Ben,Jones,1990-01-23,89/23,78/21,90/12
Gemma Victoria,Mercer-Forbes,1988-08-30,98/12,78/12,89/3";
    csv_file
        .write(csv_data.as_bytes())
        .expect("failed to write to csv file");
    let service = Arc::new(setup(vec![], db_path.as_str()));
    let importer = Importer::new(Arc::clone(&service));
    let imported = importer
        .import(&csv_path.as_str())
        .expect("failed to import");
    let all = service.all().expect("failed to get all students");
    drop(importer);
    drop(service);
    remove_file(csv_path.as_str()).expect("failed to remove csv file");
    remove_file(db_path.as_str()).expect("failed to remove csv file");
    assert_eq!(imported, (2, 6));
    assert_eq!(all.len(), 2);
    assert_eq!(all[0].first_names, "Ben".to_owned());
    assert_eq!(all[0].scores.len(), 3);
    assert_eq!(all[1].first_names, "Gemma Victoria".to_owned());
    assert_eq!(all[1].scores.len(), 3);
}

#[test]
fn test_import_bad_csv() {
    let bad_csvs = vec![
        "\
        first_names,last_name,date_of_birth,2021-02-01,2021-02-02,2021-02-03
        Ben,Jones,1990-01-23,89/23,78/21,90/12
        Gemma Victoria,Mercer-Forbes,1990-01-23,98/,78/12,89/3", // missing incorrect score
        "\
        first_names,last_name,date_of_birth,2021-02-01,2021-02-02,
        Ben,Jones,1990-01-23,89/23,78/21,90/12
        Gemma Victoria,Mercer-Forbes,1990-01-23,98/12,78/12,89/3", // missing date
        "\
        first_names,last_name,date_of_birth,2021-02-01,2021-02-02,2021-02-03
        Ben,Jones,1990-01-23,89/23,78/21,90/12
        Gemma Victoria,,,98/12,78/12,89/3", // blank last name
        "\
        first_names,last_name,date_of_birth,2021-02-01,2021-02-02,2021-02-03
        Ben,Jones,1990-01-23,89/23,78/21
        Gemma Victoria,Mercer-Forbes,1990-01-23,98/12,78/12,89/3", // missing a score
        "\
        Ben,Jones,1990-01-23,89/23,78/21,78/21
        Gemma Victoria,Mercer-Forbes,1990-01-23,98/12,78/12,89/3", // had no headers
    ];
    for csv_data in bad_csvs {
        let csv_path = format!("{}.csv", function_name!());
        let db_path = format!("{}.sqlite", function_name!());
        let mut csv_file = File::create(csv_path.as_str()).expect("failed to create test csv file");
        csv_file
            .write(csv_data.as_bytes())
            .expect("failed to write to csv file");
        let service = Arc::new(setup(vec![], db_path.as_str()));
        let importer = Importer::new(service);
        let imported = importer.import(csv_path.as_str());
        drop(importer);
        remove_file(csv_path.as_str()).expect("failed to remove csv file");
        remove_file(db_path.as_str()).expect("failed to remove csv file");
        assert!(imported.is_err(), "{}", csv_data);
    }
}
