//! Implementation of FASTA format reader.
use crate::io::fasta::record::FastaRecord;
use crate::io::sequence::Sequence;
use crate::io::traits::{Reader, Record};
use std::io;
use std::io::Result as IoResult;

/// Represents FASTA format reader.
pub struct FastaReader<R> {
    reader: R,
    line: String,
}

impl<R> FastaReader<io::BufReader<R>>
where
    R: io::Read,
{
    /// Creates new FASTA reader.
    pub fn new(reader: R) -> Self {
        Self {
            reader: io::BufReader::new(reader),
            line: String::new(),
        }
    }
}

impl<R> Reader for FastaReader<R>
where
    R: io::BufRead,
{
    type Record = FastaRecord;
    type Iterator = FastaReaderIter<R>;

    fn read(&mut self) -> IoResult<Self::Record> {
        if self.line.is_empty() {
            self.reader.read_line(&mut self.line)?;

            if self.line.is_empty() {
                return Ok(FastaRecord::default());
            }
        }

        if !self.line.starts_with(">") {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "Line must start with >",
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
            if self.line.is_empty() || self.line.starts_with(">") {
                break;
            }

            sequence.push_str(self.line.trim());
            self.line.clear();
        }

        if sequence.is_empty() {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "Record should have non-empty sequence",
            ));
        }

        Ok(FastaRecord::new(&id, description, Sequence::new(&sequence)))
    }

    fn iter(self) -> Self::Iterator {
        Self::Iterator {
            reader: self,
            done: false,
        }
    }
}

/// Represents iterator over FASTA reader.
pub struct FastaReaderIter<R> {
    reader: FastaReader<R>,
    done: bool,
}

impl<R> Iterator for FastaReaderIter<R>
where
    R: io::BufRead,
{
    type Item = IoResult<FastaRecord>;

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

    fn create_fasta_reader(content: &str) -> FastaReader<BufReader<Cursor<&str>>> {
        let data = io::Cursor::new(content);
        FastaReader::new(data)
    }

    #[test]
    fn test_fasta_reader_empty_sequence() {
        let mut reader = create_fasta_reader("");
        let record = reader.read().unwrap();

        assert!(record.is_empty());
    }

    #[test]
    fn test_fasta_reader_missing_start_character() {
        let mut reader = create_fasta_reader("ATGC");

        assert!(
            reader.read().is_err(),
            "read() should fail if start character is missing"
        );
    }

    #[test]
    fn test_fasta_reader_missing_record_id() {
        let mut reader = create_fasta_reader(">\nATGC");

        assert!(
            reader.read().is_err(),
            "read() should fail if header is empty"
        );
    }

    #[test]
    fn test_fasta_reader_non_ascii_record_id() {
        let mut reader = create_fasta_reader(">Ą\nATGC");

        assert!(
            reader.read().is_err(),
            "read() should fail if record id has non-ASCII characters"
        );
    }

    #[test]
    fn test_fasta_reader_missing_description() {
        let mut reader = create_fasta_reader(">S1\nATGC");
        let record = reader.read().unwrap();

        assert_eq!(record.id(), "S1");
        assert_eq!(record.description(), None);
        assert_eq!(record.sequence().content(), "ATGC");
    }

    #[test]
    fn test_fasta_reader_non_ascii_record_description() {
        let mut reader = create_fasta_reader(">X1 Ą\nATGC");

        assert!(
            reader.read().is_err(),
            "read() should fail if record description has non-ASCII characters"
        );
    }

    #[test]
    fn test_fasta_reader_missing_sequence() {
        let mut reader = create_fasta_reader(">X1 Desc\n");

        assert!(
            reader.read().is_err(),
            "read() should fail if sequence is empty"
        )
    }

    #[test]
    fn test_fasta_reader_not_trimmed_sequence() {
        let mut reader = create_fasta_reader(">S1\n  TC  ");
        let record = reader.read().unwrap();

        assert_eq!(record.id(), "S1");
        assert_eq!(record.sequence().content(), "TC");
    }

    #[test]
    fn test_fasta_reader_single_line_sequence() {
        let mut reader = create_fasta_reader(">X3 Desc\nACTG");
        let record = reader.read().unwrap();

        assert_eq!(record.id(), "X3");
        assert_eq!(record.description(), Some("Desc"));
        assert_eq!(record.sequence().content(), "ACTG");
    }

    #[test]
    fn test_fasta_read_multi_line_sequence() {
        let mut reader = create_fasta_reader(">X3 D\nAA\nTT\nGG");
        let record = reader.read().unwrap();

        assert_eq!(record.sequence().content(), "AATTGG");
    }

    #[test]
    fn test_fasta_reader_many_sequences() {
        let mut reader = create_fasta_reader(
            ">X1 D1\n\
        ATC\n\
        >X2\n\
        TCG\n\
        >X3 D3\n\
        GTC",
        );
        let ids = ["X1", "X2", "X3"];
        let descriptions = [Some("D1"), None, Some("D3")];
        let sequences = ["ATC", "TCG", "GTC"];

        for i in 0..3 {
            let record = reader.read().unwrap();

            assert_eq!(record.id(), ids[i]);
            assert_eq!(record.description(), descriptions[i]);
            assert_eq!(record.sequence().content(), sequences[i]);
        }

        assert!(reader.read().unwrap().is_empty())
    }

    #[test]
    fn test_fasta_reader_iter() {
        let reader = create_fasta_reader(">X1 D1\nACT\n>X2 D2\nTCG");

        assert_eq!(reader.iter().count(), 2);
    }

    #[test]
    fn test_fasta_reader_iter_empty() {
        let reader = create_fasta_reader("");
        let mut iter = reader.iter();

        assert!(
            iter.next().is_none(),
            "Iter should return None if all records are read"
        );
    }

    #[test]
    fn test_fasta_reader_iter_error() {
        let reader = create_fasta_reader(">\nTCG\n>X1 D2\nACT");
        let mut iter = reader.iter();

        let first = iter.next();
        let second = iter.next();

        assert!(first.is_some());
        assert!(first.unwrap().is_err());
        assert!(second.is_none());
    }
}
