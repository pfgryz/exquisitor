//! Implementation of FASTQ format reader.

use crate::io::fastq::record::FastqRecord;
use crate::io::sequence::Sequence;
use crate::io::traits::{Reader, Record};
use std::io;
use std::io::Result as IoResult;

/// Represents FASTQ format reader.
pub struct FastqReader<R> {
    reader: R,
    line: String,
}

impl<R> FastqReader<io::BufReader<R>>
where
    R: io::Read,
{
    /// Crates new FASTQ reader.
    pub fn new(reader: R) -> Self {
        Self {
            reader: io::BufReader::new(reader),
            line: String::new(),
        }
    }
}

impl<R> Reader for FastqReader<R>
where
    R: io::BufRead,
{
    type Record = FastqRecord;
    type Iterator = FastqReaderIter<R>;

    fn read(&mut self) -> IoResult<Self::Record> {
        if self.line.is_empty() {
            self.reader.read_line(&mut self.line)?;

            if self.line.is_empty() {
                return Ok(FastqRecord::default());
            }
        }

        if !self.line.starts_with("@") {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "Record must start with '@'",
            ));
        }

        let mut header = self.line[1..].trim().splitn(2, char::is_whitespace);
        let id = header.next().map(|s| s.to_owned()).unwrap();
        let description = header.next().map(|s| s.to_owned());

        if id.is_empty() {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "Record should have identifier",
            ));
        }

        if !id.is_ascii() {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "Record identifier should be ASCII-only",
            ));
        }

        if let Some(ref d) = description {
            if !d.is_ascii() {
                return Err(io::Error::new(
                    io::ErrorKind::InvalidData,
                    "Record description should be ASCII-only",
                ));
            }
        }

        self.line.clear();
        let mut sequence = String::new();

        while self.reader.read_line(&mut self.line)? > 0 {
            if self.line.is_empty() || self.line.starts_with("+") {
                break;
            }

            sequence.push_str(&self.line.trim());
            self.line.clear();
        }

        if sequence.is_empty() {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "Record should have non-empty sequence",
            ));
        }

        if !self.line.starts_with("+") {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "Record quality values should be prefixed by'+' line",
            ));
        }
        self.line.clear();

        let mut quality = String::new();

        while self.reader.read_line(&mut self.line)? > 0 {
            if self.line.is_empty() || quality.len() == sequence.len() {
                break;
            }

            quality.push_str(&self.line.trim());
            self.line.clear();
        }

        if quality.len() != sequence.len() {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "Record sequence and quality lengths should match",
            ));
        }

        Ok(FastqRecord::new(
            &id,
            description,
            Sequence::new(&sequence),
            Sequence::new(&quality),
        ))
    }

    fn iter(self) -> Self::Iterator {
        Self::Iterator {
            reader: self,
            done: false,
        }
    }
}

pub struct FastqReaderIter<R> {
    reader: FastqReader<R>,
    done: bool,
}

impl<R> Iterator for FastqReaderIter<R>
where
    R: io::BufRead,
{
    type Item = IoResult<FastqRecord>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.done {
            return None;
        }

        let record = self.reader.read();

        match record {
            Ok(r) => match r.is_empty() {
                true => {
                    self.done = true;
                    None
                }
                false => Some(Ok(r)),
            },
            Err(e) => {
                self.done = true;
                Some(Err(e))
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::{BufReader, Cursor};

    fn create_fastq_reader(content: &str) -> FastqReader<BufReader<Cursor<&str>>> {
        let data = io::Cursor::new(content);
        FastqReader::new(data)
    }

    #[test]
    fn test_fastq_reader_empty_sequence() {
        let mut reader = create_fastq_reader("");
        let record = reader.read().unwrap();

        assert!(record.is_empty());
    }

    #[test]
    fn test_fastq_reader_missing_start_character() {
        let mut reader = create_fastq_reader("X1\nACTG\n+\n!!!!");

        assert!(
            reader.read().is_err(),
            "read() should fail if start character is missing"
        );
    }

    #[test]
    fn test_fastq_missing_record_id() {
        let mut reader = create_fastq_reader("@\nACTG\n+\n!!!!");

        assert!(
            reader.read().is_err(),
            "read() should fail if header is empty"
        );
    }

    #[test]
    fn test_fastq_reader_non_ascii_record_id() {
        let mut reader = create_fastq_reader("@Ą\nATGC\n+\n!!!!");

        assert!(
            reader.read().is_err(),
            "read() should fail if record id has non-ASCII characters"
        );
    }

    #[test]
    fn test_fastq_reader_missing_sequence() {
        let mut reader = create_fastq_reader("@X1 Desc\n");

        assert!(
            reader.read().is_err(),
            "read() should fail if sequence is empty"
        )
    }

    #[test]
    fn test_fastq_reader_missing_quality_values() {
        let mut reader = create_fastq_reader("@X1 Desc\nACTG\n+\n");

        assert!(
            reader.read().is_err(),
            "read() should fail if quality values are empty"
        )
    }

    #[test]
    fn test_fastq_reader_unequals_length_of_sequence_and_quality_values() {
        let mut reader = create_fastq_reader("@X1 Desc\nACTG\n+\n!!");

        assert!(
            reader.read().is_err(),
            "read() should fail if quality values length is not equal to sequence length"
        )
    }

    #[test]
    fn test_fastq_reader_multi_line_sequence() {
        let mut reader = create_fastq_reader("@X1 Example\nAC\nTG\n+\n!!\n..\n");
        let record = reader.read();

        // assert!(record.is_ok());
        let record = record.unwrap();

        assert_eq!(record.id(), "X1");
        assert!(record.description().is_some());
        assert_eq!(record.description().unwrap(), "Example");
        assert_eq!(record.sequence().content(), "ACTG");
        assert_eq!(record.quality().content(), "!!..");
    }
}
