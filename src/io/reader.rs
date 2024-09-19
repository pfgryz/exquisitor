use std::io;
use crate::io::fasta::record::FastaRecord;
use crate::io::fasta::traits::FastaRead;
use crate::io::sequence::Sequence;

#[derive(Clone, Eq, PartialEq, Hash, Debug)]
pub struct Reader<R> {
    reader: R,
}

impl<R> Reader<io::BufReader<R>>
where
    R: io::Read,
{
    pub fn new(reader: R) -> Self {
        Self {
            reader: io::BufReader::new(reader)
        }
    }
}

impl<R> Reader<R>
where
    R: io::BufRead,
{
    pub fn from_bufread(reader: R) -> Self {
        Self {
            reader
        }
    }
}

impl<R> FastaRead for Reader<R>
where
    R: io::BufRead,
{
    fn read(&mut self) -> io::Result<FastaRecord> {
        let mut line = String::new();

        self.reader.read_line(&mut line)?;

        if !line.starts_with(">") {
            return Err(
                io::Error::new(
                    io::ErrorKind::InvalidData,
                    "Record should start with '>'",
                )
            );
        }

        let mut header = line[1..].trim().splitn(2, char::is_whitespace);
        let id = header.next().map(|s| s.to_owned()).unwrap();
        let description = header.next().map(|s| s.to_owned());
        let mut sequence = String::new();

        line.clear();
        while self.reader.read_line(&mut line)? > 0 {
            if line.is_empty() {
                break;
            }

            sequence.push_str(&line);
            line.clear();
        }

        Ok(FastaRecord::new(id, description, Sequence::new(sequence)))
    }
}


#[cfg(test)]
mod tests {
    use crate::io::traits::Record;
    use super::*;

    #[test]
    fn test_read_single_sequence() {
        let mut reader = Reader::new(io::Cursor::new(">X3 Desc\nACTG"));
        let record = reader.read().expect("Expected record");;

        assert_eq!(record.id(), "X3");
        assert_eq!(record.description(), Some("Desc"));
        assert_eq!(record.sequence().sequence(), "ACTG");
    }
}