// error.rs
use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub enum DaxError {
    ParseError(String),
    EvaluationError(String),
    IoError(std::io::Error),
}

impl fmt::Display for DaxError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            DaxError::ParseError(msg) => write!(f, "Parse error: {}", msg),
            DaxError::EvaluationError(msg) => write!(f, "Evaluation error: {}", msg),
            DaxError::IoError(err) => write!(f, "IO error: {}", err),
        }
    }
}

impl Error for DaxError {}

impl From<std::io::Error> for DaxError {
    fn from(err: std::io::Error) -> Self {
        DaxError::IoError(err)
    }
}
