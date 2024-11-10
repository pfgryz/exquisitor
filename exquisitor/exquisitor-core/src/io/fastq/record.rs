use crate::io::sequence::Sequence;
use crate::io::traits::Record;
use std::fmt::{Display, Formatter};

/// Represents FASTQ format record.
#[derive(Clone, Eq, PartialEq, Hash, Debug)]
pub struct FastqRecord {
    /// Identifier of the record.
    id: String,

    /// An optional description of the record.
    description: Option<String>,

    /// DNA Sequence.
    sequence: Sequence,

    /// Quality values for the sequence.
    quality: Sequence,
}

impl FastqRecord {
    /// Creates empty FASTQ record.
    pub fn default() -> Self {
        Self {
            id: String::default(),
            description: None,
            sequence: Sequence::default(),
            quality: Sequence::default(),
        }
    }

    /// Creates new FASTQ record.
    pub fn new(
        id: &str,
        description: Option<String>,
        sequence: Sequence,
        quality: Sequence,
    ) -> Self {
        Self {
            id: id.to_string(),
            description,
            sequence,
            quality,
        }
    }
}

impl Record for FastqRecord {
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
        self.id.is_empty()
            && self.description.is_none()
            && self.sequence.length() == 0
            && self.quality.length() == 0
    }

    fn is_valid(&self) -> bool {
        if let Some(d) = &self.description {
            if !d.is_ascii() {
                return false;
            }
        }

        !self.is_empty() && self.id.is_ascii() && self.sequence.length() == self.quality.length()
    }
}

impl Display for FastqRecord {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let description = match self.description() {
            Some(d) => format!("{}", d),
            None => String::from(""),
        };

        write!(
            f,
            "FastqRecord(id={}, description={}, sequence={}, quality={})",
            self.id(),
            description,
            self.sequence(),
            self.quality
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fastq_record_new() {
        let record = FastqRecord::new(
            "YZ",
            Some(String::from("Something")),
            Sequence::new("ACTG"),
            Sequence::new("!!!!"),
        );

        assert_eq!(record.id(), "YZ");
        assert!(record.description().is_some());
        assert_eq!(record.description().unwrap(), "Something");
        assert_eq!(record.sequence().content(), "ACTG");
        assert!(!record.is_empty());
        assert!(record.is_valid());
    }

    #[test]
    fn test_fastq_record_unequal_lengths() {
        let record = FastqRecord::new("YZ", None, Sequence::new("ACTG"), Sequence::new("!."));

        assert!(!record.is_valid());
    }

    #[test]
    fn test_fastq_record_display() {
        let record = FastqRecord::new(
            "XZ",
            Some(String::from("XYZ")),
            Sequence::new("ACTG"),
            Sequence::new("!!.."),
        );
        let formatted = format!("{}", record);

        assert_eq!(
            formatted,
            "FastqRecord(id=XZ, description=XYZ, sequence=ACTG, quality=!!..)"
        );
    }
}
