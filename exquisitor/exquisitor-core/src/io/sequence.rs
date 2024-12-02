use serde::{Deserialize, Serialize};
/// Module containing abstraction over DNA sequence.
use std::fmt;
use std::fmt::Formatter;

/// Represents DNA sequence.
#[derive(Serialize, Deserialize, Clone, Eq, PartialEq, Hash, Debug)]
pub struct Sequence {
    /// Sequence content.
    sequence: String,
}

/// Alignment options for padding and truncation operations.
pub enum Alignment {
    Left,
    Right,
    Center,
    CenterRight,
}

impl Sequence {
    /// Creates empty sequence.
    pub fn default() -> Self {
        Self {
            sequence: String::new(),
        }
    }

    /// Create sequence with content.
    pub fn new(sequence: &str) -> Self {
        Self {
            sequence: sequence.to_string(),
        }
    }

    /// Returns a content of the sequence.
    pub fn content(&self) -> &str {
        &self.sequence
    }

    /// Returns a length of the sequence.
    pub fn length(&self) -> usize {
        self.sequence.len()
    }

    /// Reverses the sequence content.
    pub fn reverse(&mut self) -> &mut Self {
        self.sequence = self.sequence.chars().rev().collect();
        self
    }

    /// Truncates the sequence to the specified length based on the given alignment.
    pub fn truncate(&mut self, length: usize, alignment: Alignment) -> &mut Self {
        if length > self.length() {
            return self;
        }

        let (left, right) = match alignment {
            Alignment::Left => (0, length),
            Alignment::Right => (self.length() - length, self.length()),
            Alignment::Center => {
                let side = (self.length() - length) / 2;
                (side, side + length)
            }
            Alignment::CenterRight => {
                let side = (self.length() - length + 1) / 2;
                (side, side + length)
            }
        };

        self.sequence = self.sequence[left..right].to_string();
        self
    }

    /// Pads the sequence to the specified length based on the given alignment.
    pub fn pad(&mut self, length: usize, character: char, alignment: Alignment) -> &mut Self {
        if length <= self.length() {
            return self;
        }

        let (left, right) = match alignment {
            Alignment::Left => (0, length - self.length()),
            Alignment::Right => (length - self.length(), 0),
            Alignment::Center => {
                let side = (length - self.length()) / 2;
                (side, length - self.length() - side)
            }
            Alignment::CenterRight => {
                let side = (length - self.length() + 1) / 2;
                (side, length - self.length() - side)
            }
        };

        self.sequence = format!(
            "{}{}{}",
            character.to_string().repeat(left),
            self.content(),
            character.to_string().repeat(right)
        );
        self
    }
}

/// Implements displaying sequences
impl fmt::Display for Sequence {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.content())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sequence_default() {
        let seq = Sequence::default();

        assert_eq!(seq.content(), "");
    }

    #[test]
    fn test_sequence_new() {
        let seq = Sequence::new("ACTG");

        assert_eq!(seq.content(), "ACTG");
    }

    #[test]
    fn test_sequence_display() {
        let seq = Sequence::new("ACTG");
        let formatted = format!("{}", seq);

        assert_eq!(formatted, "ACTG");
    }

    #[test]
    fn test_sequence_length() {
        let sequence = Sequence::new("AATTCC");

        assert_eq!(sequence.length(), 6);
    }

    #[test]
    fn test_sequence_reversed() {
        let mut seq = Sequence::new("TGGAA");
        seq.reverse();

        assert_eq!(seq.content(), "AAGGT");
        assert_eq!(seq.length(), 5);
    }

    #[test]
    fn test_truncate_left() {
        let mut sequence = Sequence::new("AAGTCC");

        assert_eq!(sequence.truncate(3, Alignment::Left).content(), "AAG")
    }

    #[test]
    fn test_truncate_right() {
        let mut sequence = Sequence::new("AAGTCC");

        assert_eq!(sequence.truncate(3, Alignment::Right).content(), "TCC")
    }

    #[test]
    fn test_truncate_center() {
        let mut sequence = Sequence::new("AAGTCC");

        assert_eq!(sequence.truncate(4, Alignment::Center).content(), "AGTC")
    }

    #[test]
    fn test_truncate_center_left() {
        let mut sequence = Sequence::new("AAGTCC");

        assert_eq!(sequence.truncate(3, Alignment::Center).content(), "AGT")
    }

    #[test]
    fn test_truncate_center_right() {
        let mut sequence = Sequence::new("AAGTCC");

        assert_eq!(
            sequence.truncate(3, Alignment::CenterRight).content(),
            "GTC"
        )
    }

    #[test]
    fn test_pad_left() {
        let mut sequence = Sequence::new("AAA");

        assert_eq!(sequence.pad(6, 'G', Alignment::Left).content(), "AAAGGG");
    }

    #[test]
    fn test_pad_right() {
        let mut sequence = Sequence::new("AAA");

        assert_eq!(sequence.pad(6, 'G', Alignment::Right).content(), "GGGAAA");
    }

    #[test]
    fn test_pad_center() {
        let mut sequence = Sequence::new("AAA");

        assert_eq!(sequence.pad(5, 'G', Alignment::Center).content(), "GAAAG");
    }

    #[test]
    fn test_pad_center_left() {
        let mut sequence = Sequence::new("AAA");

        assert_eq!(sequence.pad(6, 'G', Alignment::Center).content(), "GAAAGG");
    }

    #[test]
    fn test_pad_center_right() {
        let mut sequence = Sequence::new("AAA");

        assert_eq!(
            sequence.pad(6, 'G', Alignment::CenterRight).content(),
            "GGAAAG"
        );
    }
}
