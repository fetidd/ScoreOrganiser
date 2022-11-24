pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, PartialEq)]
pub enum Error {
    DbError(String),
    NoStudent,
    ValueError(String),
    FieldArgMismatch,
    BadDateConversion(String),
    ImporterError(String),
    ParseIntError(String),
    NoScoresToPlot
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::DbError(s) => write!(f, "DbError: {s}"),
            Error::NoStudent => write!(f, "No Student found in DB"),
            Error::ValueError(s) => write!(f, "ValueError: {s}"),
            Error::FieldArgMismatch => write!(f, "FieldArgMismatch"),
            Error::BadDateConversion(s) => write!(f, "BadDateConversion: {s}"),
            Error::ImporterError(s) => write!(f, "ImporterError: {s}"),
            Error::ParseIntError(s) => write!(f, "ParseIntError: {s}"),
            Error::NoScoresToPlot => write!(f, "No scores found to plot"),
        }
    }
}

impl From<rusqlite::Error> for Error {
    fn from(re: rusqlite::Error) -> Error {
        let e_string = re.to_string();
        Error::DbError(e_string.to_owned())
    }
}

impl From<chrono::ParseError> for Error {
    fn from(pe: chrono::ParseError) -> Error {
        let e_string = pe.to_string();
        Error::BadDateConversion(e_string.to_owned())
    }
}

impl From<std::num::ParseIntError> for Error {
    fn from(e: std::num::ParseIntError) -> Error {
        let e_string = e.to_string();
        Error::ParseIntError(e_string.to_owned())
    }
}

impl From<csv::Error> for Error {
    fn from(e: csv::Error) -> Error {
        let e_string = e.to_string();
        Error::ParseIntError(e_string.to_owned())
    }
}
