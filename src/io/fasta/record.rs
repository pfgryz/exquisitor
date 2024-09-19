use std::fmt;
use crate::io::sequence::Sequence;
use crate::io::traits::Record;

#[derive(Clone, Eq, PartialEq, Hash, Debug)]
pub struct FastaRecord {
    id: String,
    description: Option<String>,
    sequence: Sequence,
}

impl FastaRecord {
    pub fn default() -> Self {
        Self {
            id: String::new(),
            description: None,
            sequence: Sequence::default()
        }
    }

    pub fn new(id: String, description: Option<String>, sequence: Sequence) -> Self {
        Self {
            id,
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
            None => None
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
}

impl fmt::Display for FastaRecord {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let description = match self.description() {
            Some(d) => format!("Some({})", d),
            None => String::from("None")
        };

        write!(f, "FastaRecord(id={}, description={}, sequence={})", self.id(), description, self.sequence())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let record = FastaRecord::new(String::from("X3"), Some(String::from("Desc")), Sequence::new(String::from("ACTG")));

        assert_eq!(record.id(), "X3");
        assert!(record.description().is_some());
        assert_eq!(record.description().unwrap(), "Desc");
        assert_eq!(record.sequence().content(), "ACTG");
    }

    #[test]
    fn test_display() {
        let record = FastaRecord::new(String::from("X3"), Some(String::from("Desc")), Sequence::new(String::from("ACTG")));
        let formatted = format!("{}", record);

        assert_eq!(formatted, "FastaRecord(id=X3, description=Some(Desc), sequence=ACTG)");
    }
}