//! Error kinds for acir_checker

use std::fmt::Display;

#[derive(Debug)]
pub enum Error {
    EncodingError(String),
    SmtSolvingError(String),
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::EncodingError(msg) => write!(f, "Encoding Error: {}", msg),
            Error::SmtSolvingError(msg) => write!(f, "SMT Solving Error: {}", msg),
        }
    }
}
