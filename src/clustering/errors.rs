use std::fmt;
use std::fmt::Formatter;


pub type ClusterResult<T> = Result<T, ClusteringError>;

// region ClusteringErrorKind

#[derive(Clone, Eq, PartialEq, Hash, Debug)]
pub enum ClusteringErrorKind {
    UnequalSequenceLengths,
    EmptySequence
}

impl fmt::Display for ClusteringErrorKind {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", match self {
            ClusteringErrorKind::UnequalSequenceLengths => "UnequalSequenceLengths",
            ClusteringErrorKind::EmptySequence => "EmptySequence"
        })
    }
}

// endregion

// region ClusteringError

#[derive(Clone, Eq, PartialEq, Hash, Debug)]
pub struct ClusteringError {
    kind: ClusteringErrorKind,
    message: String,
}

impl ClusteringError {
    pub fn new(kind: ClusteringErrorKind, message: String) -> Self {
        Self {
            kind,
            message,
        }
    }

    pub fn kind(&self) -> &ClusteringErrorKind {
        &self.kind
    }
}

impl fmt::Display for ClusteringError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}: {}", self.kind, self.message)
    }
}

// endregion

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error() {
        let error = ClusteringError::new(ClusteringErrorKind::UnequalSequenceLengths, String::from("1 != 2"));

        assert_eq!(error.kind(), &ClusteringErrorKind::UnequalSequenceLengths);
        assert_eq!(format!("{}", error), "UnequalSequenceLengths: 1 != 2");
    }
}