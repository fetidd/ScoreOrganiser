#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use log::*;
use scorg_lib::{
    constant::DB_FILE,
    errors::Error,
    database::{Dao, SqliteDao},
    importer::Importer,
    models::{SafmedScore, Student},
    plotter::{Plotter, SafmedPlotter},
    services::{SafmedScoreService, StudentService},
};
use simplelog::{ColorChoice, Config, LevelFilter, TermLogger, TerminalMode};
use std::sync::Arc;
use tauri::State;

fn main() {
    TermLogger::init(
        LevelFilter::Debug,
        Config::default(),
        TerminalMode::Mixed,
        ColorChoice::Auto,
    )
    .expect("failed to init logging");
    info!("setting up backend services...");
    debug!("creating and initialising DAO");
    let dao: Arc<dyn Dao> = Arc::new(SqliteDao::new());
    dao.init().expect("failed to init sqlite database");
    debug!("creating and initialising STUDENT SERVICE");
    let students = Arc::new(StudentService::new(Arc::clone(&dao)));
    students.init().expect("failed to init student service");
    debug!("creating and initialising SCORE SERVICE");
    let scores = Arc::new(SafmedScoreService::new(Arc::clone(&dao)));
    scores.init().expect("failed to init score service");
    debug!("creating and initialising IMPORTER");
    let importer = Importer::new(Arc::clone(&students), Arc::clone(&scores));
    info!("Starting...");
    tauri::Builder::default()
        .manage(students)
        .manage(scores)
        .manage(importer)
        .invoke_handler(tauri::generate_handler![
            all_students,
            add_student,
            delete_student,
            edit_student,
            add_safmeds_score,
            get_safmeds_scores,
            import_csv
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

// STUDENT COMMANDS
#[tauri::command]
fn all_students(service: State<Arc<StudentService>>) -> Result<Vec<Student>, Error> {
    debug!("received request for all students");
    match service.all() {
        Ok(students) => Ok(students),
        Err(error) => Err(error),
    }
}

#[tauri::command]
fn add_student(
    first_names: String,
    last_name: String,
    date_of_birth: String,
    service: State<Arc<StudentService>>,
) -> Result<String, Error> {
    debug!(
        "recieved student to add: {} {} {}",
        &first_names, &last_name, &date_of_birth
    );
    let new_student = match Student::new(&first_names, &last_name, &date_of_birth) {
        Ok(student) => student,
        Err(error) => return Err(error),
    };
    service.add_student(&new_student).and_then(|_| Ok(new_student.id.to_owned()))
}

#[tauri::command]
fn delete_student(
    id: String,
    students: State<Arc<StudentService>>,
    scores: State<Arc<SafmedScoreService>>,
) -> Result<(), Error> {
    match scores.delete_scores(&id) {
        Ok(num) => debug!("deleted {num} scores"),
        Err(error) => {
            error!("{}", error);
            return Err(error);
        }
    };
    match students.delete_student(&id) {
        Ok(_) => Ok(()),
        Err(error) => {
            error!("{}", error);
            Err(error)
        }
    }
}

#[tauri::command]
fn edit_student(update: Student, service: State<Arc<StudentService>>) -> Result<usize, Error> {
    service.update_student(&update)
}

// SCORE COMMANDS
#[tauri::command]
fn add_safmeds_score(
    id: String,
    date: String,
    correct: i32,
    incorrect: i32,
    service: State<Arc<SafmedScoreService>>,
) -> Result<(), String> {
    let new_score = match SafmedScore::new(&id, correct, incorrect, &date) {
        Ok(score) => score,
        Err(error) => return Err(error.to_string()),
    };
    match service.add_score(&new_score) {
        Ok(_) => Ok(()),
        Err(error) => Err(error.to_string())
    }
}

#[tauri::command]
fn get_safmeds_scores(
    student_id: &str,
    service: State<Arc<SafmedScoreService>>,
) -> Result<Vec<SafmedScore>, String> {
    service.get_safmed_scores(&student_id).or_else(|e| Err(e.to_string()))
}

#[tauri::command]
fn import_csv(file: &str, importer: State<Importer>) -> Result<(), String> {
    debug!("importing {file}");
    match importer.import(file) {
        Ok(_) => Ok(()),
        Err(error) => {
            error!("{error}");
            Err(error.to_string())
        }
    }
}
