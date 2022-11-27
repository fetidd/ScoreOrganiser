use clap::Parser;
use scorg_lib::{
    database::{Dao, SqliteDao},
    services::{StudentService, SafmedScoreService},
    models::{Student, SafmedScore},
    importer::Importer,
    constant::*,
    useful::*,
};
use std::sync::Arc;
use simplelog::{TermLogger, LevelFilter, Config, TerminalMode, ColorChoice};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    action: String,
    #[arg(short, long)]
    first_names: Option<String>,
    #[arg(short,long)]
    last_name: Option<String>,
    #[arg(short,long)]
    date_of_birth: Option<String>,
    filepath: Option<String>
}

type CliResult = Result<(), String>;

fn main() -> CliResult {
    TermLogger::init(
        LevelFilter::Debug,
        Config::default(),
        TerminalMode::Mixed,
        ColorChoice::Auto,
    ).expect("failed to init logger");
    let args = Args::parse();
    let dao: Arc<dyn Dao> = Arc::new(SqliteDao::new());
    let students = Arc::new(StudentService::new(Arc::clone(&dao)));
    let scores = Arc::new(SafmedScoreService::new(Arc::clone(&dao)));
    students.init().unwrap();
    scores.init().unwrap();
    match args.action.as_str() {
        "add" => handle_add(&args, &students, &scores),
        "all" => handle_all(&args, &students, &scores),
        "import" => {
            let importer = Importer::new(Arc::clone(&students), Arc::clone(&scores));
            handle_import(&args, &students, &scores, &importer)
        },
        _ => Err("no command".to_owned())
    }
}

fn handle_add(args: &Args, students: &StudentService, scores: &SafmedScoreService) -> CliResult {
    let first_names = match &args.first_names.as_ref() {
        Some(first_names) => first_names.clone(),
        None => return Err("Need to provide first names with -f, --first_names".to_owned())
    };
    let last_name = &args.last_name.as_ref().unwrap();
    let date_of_birth = &args.date_of_birth.as_ref().unwrap();
    let student = Student::new(first_names, last_name, date_of_birth).unwrap();
    students.add_student(&student).unwrap();
    Ok(())
}

fn handle_all(_args: &Args, students: &StudentService, scores: &SafmedScoreService) -> CliResult {
    let res = students.all().unwrap();
    println!("{res:#?}");
    Ok(())
}

fn handle_import(args: &Args, students: &StudentService, scores: &SafmedScoreService, importer: &Importer) -> CliResult {
    let path = args.filepath.as_ref().unwrap();
    let csv_data = match std::fs::read(path) {
        Ok(data) => data,
        Err(error) => return Err(error.to_string())
    };
    match String::from_utf8(csv_data) {
        Ok(data_str) => {
            if let Err(err) = importer.import(&data_str) {
                return Err(err.to_string());
            }
            Ok(())
        },
        Err(err) => Err(err.to_string())
    }
}
