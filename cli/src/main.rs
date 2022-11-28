use clap::{Parser, Subcommand};
use scorg_lib::{
    database::{Dao, SqliteDao},
    importer::Importer,
    models::{SafmedScore, Student},
    services::{SafmedScoreService, StudentService},
    useful::*,
};
use simplelog::{ColorChoice, Config, LevelFilter, TermLogger, TerminalMode};
use std::sync::Arc;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[command(subcommand)]
    action: Actions,
}

#[derive(Subcommand, Debug)]
enum Actions {
    AddStudent {
        name: String,
        date_of_birth: String,
    },
    AddSafmed {
        name: String,
        correct: i32,
        incorrect: i32,
        date: Option<String>,
    },
    Delete {
        name: String,
    },
    All,
    Import {
        filepath: String,
    },
}

type CliResult = Result<(), String>;

fn main() -> CliResult {
    TermLogger::init(
        LevelFilter::Debug,
        Config::default(),
        TerminalMode::Mixed,
        ColorChoice::Auto,
    )
    .expect("failed to init logger");

    let dao: Arc<dyn Dao> = Arc::new(SqliteDao::new());
    let students = Arc::new(StudentService::new(Arc::clone(&dao)));
    let scores = Arc::new(SafmedScoreService::new(Arc::clone(&dao)));
    students.init().unwrap();
    scores.init().unwrap();

    let args = Args::parse();
    match args.action {
        Actions::All => students
            .all()
            .and_then(|_| Ok(()))
            .or_else(|e| Err(e.to_string())),
        Actions::AddStudent {
            name,
            date_of_birth,
        } => {
            let (first_names, last_name) = parse_name(&name);
            match Student::new(&first_names, &last_name, &date_of_birth) {
                Ok(new) => students
                    .add_student(&new)
                    .and_then(|_| Ok(()))
                    .or_else(|e| Err(e.to_string())),
                Err(e) => Err(e.to_string()),
            }
        }
        Actions::AddSafmed {
            name,
            correct,
            incorrect,
            date,
        } => {
            let (first_names, last_name) = parse_name(&name);
            let id = match students.get_id_for_name(&first_names, &last_name) {
                Ok(id) => id,
                Err(_) => return Err("student doesn't exist".into()),
            };
            let date = match date {
                Some(date) => date,
                None => curr_date_str(),
            };
            match SafmedScore::new(&id, correct, incorrect, &date) {
                Ok(score) => scores
                    .add_score(&score)
                    .and_then(|_| Ok(()))
                    .or_else(|e| Err(e.to_string())),
                Err(e) => Err(e.to_string()),
            }
        }
        Actions::Delete { name } => {
            let (first_names, last_name) = parse_name(&name);
            let id = match students.get_id_for_name(&first_names, &last_name) {
                Ok(id) => id,
                Err(_) => return Err("student doesn't exist".into()),
            };
            match scores.delete_scores(&id) {
                Ok(_) => students
                    .delete_student(&id)
                    .and_then(|_| Ok(()))
                    .or_else(|e| Err(e.to_string())),
                Err(e) => Err(e.to_string()),
            }
        }
        Actions::Import { filepath } => {
            let importer = Importer::new(Arc::clone(&students), Arc::clone(&scores));
            importer
                .import(&filepath)
                .and_then(|_| Ok(()))
                .or_else(|e| Err(e.to_string()))
        }
    }
}

fn parse_name(name: &str) -> (String, String) {
    let split_name: Vec<&str> = name.split(" ").collect();
    let first_names = split_name[..split_name.len() - 1].join(" ");
    match split_name.last() {
        Some(s) => (first_names, (*s).into()),
        None => (first_names, String::new()),
    }
}
