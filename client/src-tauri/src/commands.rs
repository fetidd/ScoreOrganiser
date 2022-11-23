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

