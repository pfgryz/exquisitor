use std::io;
use crate::io::fasta::record::FastaRecord;
use crate::io::sequence::Sequence;
use crate::io::traits::{Reader, Record};

// region FastaReader

#[derive(Clone, Eq, PartialEq, Hash, Debug)]
pub struct FastaReader<R> {
    reader: R,
    line: String,
}

impl<R> FastaReader<io::BufReader<R>>
where
    R: io::Read,
{
    pub fn new(reader: R) -> Self {
        Self {
            reader: io::BufReader::new(reader),
            line: String::new(),
        }
    }
}

impl<R> FastaReader<R>
where
    R: io::BufRead,
{
    pub fn from_bufread(reader: R) -> Self {
        Self {
            reader,
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

    fn read(&mut self) -> io::Result<Self::Record> {
        if self.line.is_empty() {
            self.reader.read_line(&mut self.line)?;

            if self.line.is_empty() {
                return Ok(FastaRecord::default());
            }
        }

        if !self.line.starts_with(">") {
            return Err(
                io::Error::new(
                    io::ErrorKind::InvalidData,
                    "Record should start with '>'",
                )
            );
        }

        let mut header = self.line[1..].trim().splitn(2, char::is_whitespace);
        let id = header.next().map(|s| s.to_owned()).unwrap();
        let description = header.next().map(|s| s.to_owned());

        if id.is_empty() {
            return Err(
                io::Error::new(
                    io::ErrorKind::InvalidData,
                    "Record should have identifier",
                )
            );
        }

        self.line.clear();
        let mut sequence = String::new();

        while self.reader.read_line(&mut self.line)? > 0 {
            if self.line.is_empty() || self.line.starts_with(">") {
                break;
            }

            sequence.push_str(&self.line.trim());
            self.line.clear();
        }

        if sequence.is_empty() {
            return Err(
                io::Error::new(
                    io::ErrorKind::InvalidData,
                    "Record should contain non-empty sequence",
                )
            );
        }

        Ok(FastaRecord::new(id, description, Sequence::new(sequence)))
    }

    fn iter(self) -> Self::Iterator {
        Self::Iterator {
            reader: self,
            done: false,
        }
    }
}

// endregion

// region FastaReaderIter

pub struct FastaReaderIter<R> {
    reader: FastaReader<R>,
    done: bool,
}

impl<R> Iterator for FastaReaderIter<R>
where
    R: io::BufRead,
{
    type Item = io::Result<FastaRecord>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.done {
            return None;
        }

        let record = self.reader.read();

        match record {
            Ok(r) => {
                match r.is_empty() {
                    true => {
                        self.done = true;
                        None
                    }
                    false => Some(Ok(r))
                }
            }
            Err(err) => {
                self.done = true;
                Some(Err(err))
            }
        }
    }
}

// endregion


#[cfg(test)]
mod tests {
    use crate::io::traits::Record;
    use super::*;


    #[test]
    fn test_reader_empty_sequence() {
        let data = io::Cursor::new("");
        let mut reader = FastaReader::new(data);
        let record = reader.read().unwrap();

        assert!(record.is_empty());
    }

    #[test]
    fn test_reader_missing_start_character() {
        let data = io::Cursor::new("! S1 \n TG");
        let mut reader = FastaReader::new(data);
        assert!(
            reader.read().is_err(),
            "read() should fail if start character is missing"
        );
    }

    #[test]
    fn test_reader_missing_record_id() {
        let data = io::Cursor::new(">\n TG");
        let mut reader = FastaReader::new(data);
        assert!(
            reader.read().is_err(),
            "read() should fail if header is empty"
        )
    }

    #[test]
    fn test_reader_no_description() {
        let data = io::Cursor::new("> S1 \nTGAC");
        let mut reader = FastaReader::new(data);
        let record = reader.read().unwrap();

        assert_eq!(record.id(), "S1");
        assert_eq!(record.description(), None);
        assert_eq!(record.sequence().content(), "TGAC");
    }

    #[test]
    fn test_reader_missing_sequence() {
        let data = io::Cursor::new(">X1 Desc\n");
        let mut reader = FastaReader::new(data);
        assert!(
            reader.read().is_err(),
            "read() should fail if sequence is empty"
        )
    }

    #[test]
    fn test_reader_not_trimmed_sequence() {
        let data = io::Cursor::new(">S1\n  TC  ");
        let mut reader = FastaReader::new(data);
        let record = reader.read().unwrap();

        assert_eq!(record.id(), "S1");
        assert_eq!(record.sequence().content(), "TC");
    }

    #[test]
    fn test_reader_single_line_sequence() {
        let data = io::Cursor::new(">X3 Desc\nACTG");
        let mut reader = FastaReader::new(data);
        let record = reader.read().unwrap();

        assert_eq!(record.id(), "X3");
        assert_eq!(record.description(), Some("Desc"));
        assert_eq!(record.sequence().content(), "ACTG");
    }

    #[test]
    fn test_read_multi_line_sequence() {
        let data = io::Cursor::new(">X3 D\nAA\nTT\nGG");
        let mut reader = FastaReader::new(data);
        let record = reader.read().unwrap();

        assert_eq!(record.sequence().content(), "AATTGG");
    }

    #[test]
    fn test_reader_many_sequences() {
        let fasta = ">X1 D1\n\
        ATC\n\
        >X2\n\
        TCG\n\
        >X3 D3\n\
        GTC";
        let data = io::Cursor::new(fasta);
        let mut reader = FastaReader::new(data);

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
    fn test_reader_iter() {
        let data = io::Cursor::new(">X1 D1\nACT\n>X2 D2\nTCG");
        let reader = FastaReader::new(data);

        assert_eq!(reader.iter().count(), 2);
    }

    #[test]
    fn test_reader_iter_empty() {
        let data = io::Cursor::new("");
        let reader = FastaReader::new(data);
        let mut iter = reader.iter();

        assert!(iter.next().is_none(), "Iter should return None if all records are read");
    }

    #[test]
    fn test_reader_iter_error() {
        let data = io::Cursor::new(">\nTCG\n>X1 D2\nACT");
        let reader = FastaReader::new(data);
        let mut iter = reader.iter();

        let first = iter.next();
        let second = iter.next();

        assert!(first.is_some());
        assert!(first.unwrap().is_err());
        assert!(second.is_none());
    }
}