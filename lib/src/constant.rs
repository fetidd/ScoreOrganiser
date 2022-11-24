pub const ENABLE_FOREIGN_KEYS: &str = "PRAGMA foreign_keys=on";

pub const STUDENT_SCHEMA: &str = "CREATE TABLE IF NOT EXISTS student (
    id TEXT NOT NULL PRIMARY KEY,
    first_names TEXT NOT NULL,
    last_name TEXT NOT NULL,
    date_of_birth TEXT NOT NULL,
    UNIQUE(first_names,last_name)
)";

pub const STUDENT_FIELDS: [&'static str; 4] = [
    "id",
    "first_names",
    "last_name",
    "date_of_birth",
];

pub const SCORE_SCHEMA: &str = "CREATE TABLE IF NOT EXISTS safmed (
    id TEXT NOT NULL,
    correct INTEGER NOT NULL,
    incorrect INTEGER NOT NULL,
    date TEXT NOT NULL UNIQUE,
    CONSTRAINT student_id FOREIGN KEY (id) REFERENCES student(id)
)";

pub const SCORE_FIELDS: [&'static str; 4] = ["id", "correct", "incorrect", "date"];

pub const CONFIG_PATH: &str = "./safmeds/";
pub const DB_FILE: &str = "safmed_dev.sqlite";
