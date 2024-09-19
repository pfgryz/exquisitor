use std::fmt;
use std::fmt::Formatter;

#[derive(Clone, Eq, PartialEq, Hash, Debug)]
pub struct Sequence {
    sequence: String,
}

pub enum Alignment {
    Left,
    Right,
    Center,
    CenterRight,
}

impl Sequence {
    pub fn default() -> Self {
        Self {
            sequence: String::new()
        }
    }

    pub fn new(sequence: String) -> Self {
        Self {
            sequence
        }
    }

    pub fn sequence(&self) -> &str {
        &self.sequence
    }

    pub fn length(&self) -> usize {
        self.sequence.len()
    }

    pub fn reverse(&self) -> Self {
        Self {
            sequence: self.sequence.chars().rev().collect()
        }
    }

    pub fn truncate(&self, size: usize, alignment: Alignment) -> Self {
        if size >= self.length() {
            return Self {
                sequence: self.sequence.to_string()
            };
        }

        let (left, right) = match alignment {
            Alignment::Left => {
                (0, size)
            }
            Alignment::Right => {
                (self.length() - size, self.length())
            }
            Alignment::Center => {
                let side = (self.length() - size) / 2;
                (side, side + size)
            }
            Alignment::CenterRight => {
                let side = (self.length() - size + 1) / 2;
                (side, side + size)
            }
        };

        Self {
            sequence: self.sequence[left..right].to_string()
        }
    }

    pub fn pad(&self, size: usize, character: char, alignment: Alignment) -> Self {
        if size <= self.length() {
            return Self {
                sequence: self.sequence.chars().collect()
            };
        }

        let (left, right) = match alignment {
            Alignment::Left => {
                (0, size - self.length())
            }
            Alignment::Right => {
                (size - self.length(), 0)
            }
            Alignment::Center => {
                let side = (size - self.length()) / 2;
                (side, size - self.length() - side)
            }
            Alignment::CenterRight => {
                let side = (size - self.length() + 1) / 2;
                (side, size - self.length() - side)
            }
        };

        Self {
            sequence: format!("{}{}{}", character.to_string().repeat(left), self.sequence(), character.to_string().repeat(right))
        }
    }
}

impl fmt::Display for Sequence {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.sequence())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let sequence = Sequence::new(String::from("ACTG"));

        assert_eq!(sequence.sequence(), "ACTG");
    }

    #[test]
    fn test_display() {
        let sequence = Sequence::new(String::from("ACTG"));
        let formatted = format!("{}", sequence);

        assert_eq!(formatted, "ACTG");
    }

    #[test]
    fn test_length() {
        let sequence = Sequence::new(String::from("AATTCC"));

        assert_eq!(sequence.length(), 6);
    }

    #[test]
    fn test_reversed() {
        let sequence = Sequence::new(String::from("TGGAA"));
        let reversed = sequence.reverse();

        assert_eq!(reversed.sequence(), "AAGGT");
        assert_eq!(reversed.length(), 5);
    }

    #[test]
    fn test_truncate_left() {
        let sequence = Sequence::new(String::from("AAGTCC"));
        assert_eq!(sequence.truncate(3, Alignment::Left).sequence(), "AAG")
    }

    #[test]
    fn test_truncate_right() {
        let sequence = Sequence::new(String::from("AAGTCC"));
        assert_eq!(sequence.truncate(3, Alignment::Right).sequence(), "TCC")
    }

    #[test]
    fn test_truncate_center() {
        let sequence = Sequence::new(String::from("AAGTCC"));
        assert_eq!(sequence.truncate(4, Alignment::Center).sequence(), "AGTC")
    }

    #[test]
    fn test_truncate_center_left() {
        let sequence = Sequence::new(String::from("AAGTCC"));
        assert_eq!(sequence.truncate(3, Alignment::Center).sequence(), "AGT")
    }

    #[test]
    fn test_truncate_center_right() {
        let sequence = Sequence::new(String::from("AAGTCC"));
        assert_eq!(sequence.truncate(3, Alignment::CenterRight).sequence(), "GTC")
    }

    #[test]
    fn test_pad_left() {
        let sequence = Sequence::new(String::from("AAA"));
        assert_eq!(sequence.pad(6, 'G', Alignment::Left).sequence(), "AAAGGG");
    }

    #[test]
    fn test_pad_right() {
        let sequence = Sequence::new(String::from("AAA"));
        assert_eq!(sequence.pad(6, 'G', Alignment::Right).sequence(), "GGGAAA");
    }

    #[test]
    fn test_pad_center() {
        let sequence = Sequence::new(String::from("AAA"));
        assert_eq!(sequence.pad(5, 'G', Alignment::Center).sequence(), "GAAAG");
    }

    #[test]
    fn test_pad_center_left() {
        let sequence = Sequence::new(String::from("AAA"));
        assert_eq!(sequence.pad(6, 'G', Alignment::Center).sequence(), "GAAAGG");
    }

    #[test]
    fn test_pad_center_right() {
        let sequence = Sequence::new(String::from("AAA"));
        assert_eq!(sequence.pad(6, 'G', Alignment::CenterRight).sequence(), "GGAAAG");
    }
}