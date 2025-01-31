/// Exquisitor internal errors

use std::error::Error;
use std::fmt;
use std::fmt::Formatter;
use std::io::{Error as IoError, ErrorKind};

pub type ExquisitorResult<T> = Result<T, ExquisitorError>;

/// Kind of the internal errors
#[derive(Clone, Eq, PartialEq, Hash, Debug)]
pub enum ExquisitorErrorKind {
    UnequalSequenceLengths,
    EmptySequence,
}

impl fmt::Display for ExquisitorErrorKind {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            ExquisitorErrorKind::UnequalSequenceLengths => {
                write!(f, "UnequalSequenceLengths")
            }
            ExquisitorErrorKind::EmptySequence => {
                write!(f, "EmptySequence")
            }
        }
    }
}

#[derive(Clone, Eq, PartialEq, Hash, Debug)]
pub struct ExquisitorError {
    kind: ExquisitorErrorKind,
    message: String,
}

impl ExquisitorError {
    pub fn new(kind: ExquisitorErrorKind, message: String) -> Self {
        Self { kind, message }
    }

    pub fn kind(&self) -> &ExquisitorErrorKind {
        &self.kind
    }

    pub fn message(&self) -> &String {
        &self.message
    }
}

impl fmt::Display for ExquisitorError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}: {}", self.kind, self.message)
    }
}

impl Error for ExquisitorError {
    fn description(&self) -> &str {
        &self.message
    }
}

impl From<ExquisitorError> for IoError {
    fn from(value: ExquisitorError) -> Self {
        IoError::new(ErrorKind::Other, value.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error() {
        let error = ExquisitorError::new(
            ExquisitorErrorKind::UnequalSequenceLengths,
            String::from("1 != 2"),
        );

        assert_eq!(error.kind(), &ExquisitorErrorKind::UnequalSequenceLengths);
        assert_eq!(format!("{}", error), "UnequalSequenceLengths: 1 != 2");
    }
}
