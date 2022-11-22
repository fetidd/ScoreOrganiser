#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

use tauri::State;
use scorg_lib::{
  database::{Dao, SqliteDao},
  student_service::SqliteStudentService,
  importer::Importer,
  models::Student, constant::DB_FILE
};
use std::sync::Arc;
use simplelog::{TermLogger, LevelFilter, Config, TerminalMode, ColorChoice};
use log::*;

fn main() {
  TermLogger::init(LevelFilter::Debug, Config::default(), TerminalMode::Mixed, ColorChoice::Auto).expect("failed to init logging");
  info!("setting up backend services...");
  let dao: Arc<dyn Dao> = Arc::new(SqliteDao::new(DB_FILE));
  let service = Arc::new(SqliteStudentService::new(Arc::clone(&dao)).expect("failed to start student service"));
  service.init().expect("failed to init student service");
  let importer = Importer::new(Arc::clone(&service));
  info!("Starting...");
  tauri::Builder::default()
    .manage(service)
    .manage(importer)
    .invoke_handler(tauri::generate_handler![
      all_students, add_student, delete_student
    ])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}

#[tauri::command]
fn all_students(service: State<Arc<SqliteStudentService>>) -> Result<Vec<Student>, String> {
  debug!("received request for all students");
  match service.all() {
    Ok(students) => Ok(students),
    Err(error) => Err(error.to_string())
  }
}

#[tauri::command]
fn add_student(first_names: String, last_name: String, date_of_birth: String, service: State<Arc<SqliteStudentService>>) -> Result<String, String> {
  debug!("recieved student to add: {} {} {}", &first_names, &last_name, &date_of_birth);
  let new_student = match Student::new(&first_names, &last_name, &date_of_birth) {
    Ok(student) => student,
    Err(error) => return Err(error.to_string())
  };
  match service.add_student(&new_student) {
    Ok(_num) => Ok(new_student.id.to_owned()),
    Err(error) => Err(error.to_string())
  }
}

#[tauri::command]
fn delete_student(id: String, service: State<Arc<SqliteStudentService>>) -> Result<(), String> {
  match service.delete_student(&id) {
    Ok(_) => Ok(()),
    Err(error) => Err(error.to_string())
  }
}