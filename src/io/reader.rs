use std::io;
use crate::io::fasta::record::FastaRecord;
use crate::io::fasta::traits::FastaRead;
use crate::io::sequence::Sequence;

#[derive(Clone, Eq, PartialEq, Hash, Debug)]
pub struct Reader<R> {
    reader: R,
    line: String,
}

impl<R> Reader<io::BufReader<R>>
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

impl<R> Reader<R>
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

impl<R> FastaRead for Reader<R>
where
    R: io::BufRead,
{
    fn read(&mut self) -> io::Result<FastaRecord> {
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
        let mut sequence = String::new();

        if id.is_empty() {
            return Err(
                io::Error::new(
                    io::ErrorKind::InvalidData,
                    "Record should have identifier",
                )
            );
        }

        self.line.clear();
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
}


#[cfg(test)]
mod tests {
    use crate::io::traits::Record;
    use super::*;


    #[test]
    fn test_reader_empty_sequence() {
        let mut reader = Reader::new(io::Cursor::new(""));
        let record = reader.read().unwrap();

        assert!(record.is_empty());
    }

    #[test]
    fn test_reader_missing_start_character() {
        let mut reader = Reader::new(io::Cursor::new("! S1 \n TG"));
        assert!(
            reader.read().is_err(),
            "read() should fail if start character is missing"
        );
    }

    #[test]
    fn test_reader_missing_record_id() {
        let mut reader = Reader::new(io::Cursor::new(">\n TG"));
        assert!(
            reader.read().is_err(),
            "read() should fail if header is empty"
        )
    }

    #[test]
    fn test_reader_no_description() {
        let mut reader = Reader::new(io::Cursor::new("> S1 \nTGAC"));
        let record = reader.read().unwrap();

        assert_eq!(record.id(), "S1");
        assert_eq!(record.description(), None);
        assert_eq!(record.sequence().sequence(), "TGAC");
    }

    #[test]
    fn test_reader_missing_sequence() {
        let mut reader = Reader::new(io::Cursor::new(">X1 Desc\n"));
        assert!(
            reader.read().is_err(),
            "read() should fail if sequence is empty"
        )
    }

    #[test]
    fn test_reader_not_trimmed_sequence() {
        let mut reader = Reader::new(io::Cursor::new(">S1\n  TC  "));
        let record = reader.read().unwrap();

        assert_eq!(record.id(), "S1");
        assert_eq!(record.sequence().sequence(), "TC");
    }

    #[test]
    fn test_reader_single_line_sequence() {
        let mut reader = Reader::new(io::Cursor::new(">X3 Desc\nACTG"));
        let record = reader.read().unwrap();

        assert_eq!(record.id(), "X3");
        assert_eq!(record.description(), Some("Desc"));
        assert_eq!(record.sequence().sequence(), "ACTG");
    }

    #[test]
    fn test_read_multi_line_sequence() {
        let fasta = ">X3 D\nAA\nTT\nGG";
        let mut reader = Reader::new(io::Cursor::new(fasta));
        let record = reader.read().unwrap();

        assert_eq!(record.sequence().sequence(), "AATTGG");
    }

    #[test]
    fn test_reader_many_sequences() {
        let fasta = ">X1 D1\n\
        ATC\n\
        >X2\n\
        TCG\n\
        >X3 D3\n\
        GTC";
        let mut reader = Reader::new(io::Cursor::new(fasta));

        let ids = ["X1", "X2", "X3"];
        let descriptions = [Some("D1"), None, Some("D3")];
        let sequences = ["ATC", "TCG", "GTC"];

        for i in 0..3 {
            let record = reader.read().unwrap();

            assert_eq!(record.id(), ids[i]);
            assert_eq!(record.description(), descriptions[i]);
            assert_eq!(record.sequence().sequence(), sequences[i]);
        }

        assert!(reader.read().unwrap().is_empty())
    }
}