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

    pub fn content(&self) -> &str {
        &self.sequence
    }

    pub fn length(&self) -> usize {
        self.sequence.len()
    }

    pub fn reverse(&mut self) -> &mut Self {
        self.sequence = self.sequence.chars().rev().collect();
        self
    }

    pub fn truncate(&mut self, size: usize, alignment: Alignment) -> &mut Self {
        if size >= self.length() {
            return self;
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

        self.sequence = self.sequence[left..right].to_string();
        self
    }

    pub fn pad(&mut self, size: usize, character: char, alignment: Alignment) -> &mut Self {
        if size <= self.length() {
            return self;
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


        self.sequence = format!("{}{}{}", character.to_string().repeat(left), self.content(), character.to_string().repeat(right));
        self
    }
}

impl fmt::Display for Sequence {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.content())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let sequence = Sequence::new(String::from("ACTG"));

        assert_eq!(sequence.content(), "ACTG");
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
        let mut reversed = sequence.clone();
        reversed.reverse();

        assert_eq!(reversed.content(), "AAGGT");
        assert_eq!(reversed.length(), 5);
    }

    #[test]
    fn test_truncate_left() {
        let mut sequence = Sequence::new(String::from("AAGTCC"));
        assert_eq!(sequence.truncate(3, Alignment::Left).content(), "AAG")
    }

    #[test]
    fn test_truncate_right() {
        let mut sequence = Sequence::new(String::from("AAGTCC"));
        assert_eq!(sequence.truncate(3, Alignment::Right).content(), "TCC")
    }

    #[test]
    fn test_truncate_center() {
        let mut sequence = Sequence::new(String::from("AAGTCC"));
        assert_eq!(sequence.truncate(4, Alignment::Center).content(), "AGTC")
    }

    #[test]
    fn test_truncate_center_left() {
        let mut sequence = Sequence::new(String::from("AAGTCC"));
        assert_eq!(sequence.truncate(3, Alignment::Center).content(), "AGT")
    }

    #[test]
    fn test_truncate_center_right() {
        let mut sequence = Sequence::new(String::from("AAGTCC"));
        assert_eq!(sequence.truncate(3, Alignment::CenterRight).content(), "GTC")
    }

    #[test]
    fn test_pad_left() {
        let mut sequence = Sequence::new(String::from("AAA"));
        assert_eq!(sequence.pad(6, 'G', Alignment::Left).content(), "AAAGGG");
    }

    #[test]
    fn test_pad_right() {
        let mut sequence = Sequence::new(String::from("AAA"));
        assert_eq!(sequence.pad(6, 'G', Alignment::Right).content(), "GGGAAA");
    }

    #[test]
    fn test_pad_center() {
        let mut sequence = Sequence::new(String::from("AAA"));
        assert_eq!(sequence.pad(5, 'G', Alignment::Center).content(), "GAAAG");
    }

    #[test]
    fn test_pad_center_left() {
        let mut sequence = Sequence::new(String::from("AAA"));
        assert_eq!(sequence.pad(6, 'G', Alignment::Center).content(), "GAAAGG");
    }

    #[test]
    fn test_pad_center_right() {
        let mut sequence = Sequence::new(String::from("AAA"));
        assert_eq!(sequence.pad(6, 'G', Alignment::CenterRight).content(), "GGAAAG");
    }
}