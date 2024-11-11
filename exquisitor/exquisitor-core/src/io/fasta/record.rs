/// Implementation of FASTA format record.
use crate::io::sequence::Sequence;
use crate::io::traits::Record;
use std::fmt::{Display, Formatter};

/// Represents FASTA format record.
#[derive(Clone, Eq, PartialEq, Hash, Debug)]
pub struct FastaRecord {
    /// Identifier of the record.
    id: String,

    /// An optional description of the record.
    description: Option<String>,

    /// DNA Sequence.
    sequence: Sequence,
}

impl FastaRecord {
    /// Creates empty FASTA record.
    pub fn default() -> Self {
        Self {
            id: String::new(),
            description: None,
            sequence: Sequence::default(),
        }
    }

    /// Creates new FASTA record.
    pub fn new(id: &str, description: Option<String>, sequence: Sequence) -> Self {
        Self {
            id: id.to_string(),
            description,
            sequence,
        }
    }
}

impl Record for FastaRecord {
    fn id(&self) -> &str {
        &self.id
    }

    fn description(&self) -> Option<&str> {
        match self.description.as_ref() {
            Some(d) => Some(d),
            None => None,
        }
    }

    fn sequence(&self) -> &Sequence {
        &self.sequence
    }

    fn sequence_mut(&mut self) -> &mut Sequence {
        &mut self.sequence
    }

    fn is_empty(&self) -> bool {
        self.id.is_empty() && self.description.is_none() && self.sequence.length() == 0
    }

    fn is_valid(&self) -> bool {
        if let Some(d) = &self.description {
            if !d.is_ascii() {
                return false;
            }
        }

        !self.is_empty() && self.id.is_ascii()
    }
}

impl Display for FastaRecord {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let description = match self.description() {
            Some(d) => format!("{}", d),
            None => String::from(""),
        };

        write!(
            f,
            "FastaRecord(id={}, description={}, sequence={})",
            self.id(),
            description,
            self.sequence()
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fasta_record_new() {
        let record = FastaRecord::new("X3", Some(String::from("Desc")), Sequence::new("ACTG"));

        assert_eq!(record.id(), "X3");
        assert!(record.description().is_some());
        assert_eq!(record.description().unwrap(), "Desc");
        assert_eq!(record.sequence().content(), "ACTG");
        assert!(!record.is_empty());
        assert!(record.is_valid());
    }

    #[test]
    fn test_fasta_record_empty() {
        let record = FastaRecord::default();

        assert!(record.is_empty());
    }

    #[test]
    fn test_fasta_record_invalid() {
        let record = FastaRecord::new("Ą", None, Sequence::new("ACTG"));

        assert!(!record.is_valid());
    }

    #[test]
    fn test_fasta_record_display() {
        let record = FastaRecord::new("X3", Some(String::from("Desc")), Sequence::new("ACTG"));
        let formatted = format!("{}", record);

        assert_eq!(
            formatted,
            "FastaRecord(id=X3, description=Desc, sequence=ACTG)"
        );
    }
}
