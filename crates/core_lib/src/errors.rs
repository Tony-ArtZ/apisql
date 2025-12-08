use std::fmt;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ParseError {
    #[error("syntax error at line {line}, column {column}: {message}")]
    Syntax {
        line: usize,
        column: usize,
        message: ErrorCodes,
    },
    #[error("unexpected end of input")]
    Eof,
}

#[derive(Debug)]
pub enum ErrorCodes {
    UnexpectedToken(String),
    InvalidIdentifier(String),
    UnknownError,
    InvalidLimitValue(String),
}

impl fmt::Display for ErrorCodes {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ErrorCodes::UnexpectedToken(tok) => write!(f, "unexpected token: {}", tok),
            ErrorCodes::InvalidIdentifier(id) => write!(f, "invalid identifier: {}", id),
            ErrorCodes::UnknownError => write!(f, "unknown error"),
            ErrorCodes::InvalidLimitValue(val) => write!(f, "invalid limit value: {}", val),
        }
    }
}

#[derive(Debug, Error)]
pub enum QueryError {
    #[error("expected array at path `{path}`, found {found}")]
    ExpectedArray { path: String, found: String },

    #[error("field `{field}` not found in object")]
    MissingField { field: String },

    #[error("type error: {message}")]
    TypeError { message: String },
}
