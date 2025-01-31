//! Implementation of FASTA format writer.
use crate::io::fasta::record::FastaRecord;
use crate::io::record::validate_record;
use crate::io::traits::{Record, Writer};
use std::io;
use std::io::Result as IoResult;

/// Represents FASTA format writer.
pub struct FastaWriter<R> {
    writer: R,
    max_line_length: usize,
}

impl<R> FastaWriter<io::BufWriter<R>>
where
    R: io::Write,
{
    /// Creates new FASTA writer.
    pub fn new(writer: R, max_line_length: Option<usize>) -> Self {
        Self {
            writer: io::BufWriter::new(writer),
            max_line_length: max_line_length.unwrap_or(60),
        }
    }
}

impl<R> Writer for FastaWriter<R>
where
    R: io::Write,
{
    type Record = FastaRecord;

    fn write(&mut self, record: &Self::Record) -> IoResult<()> {
        validate_record(record)?;

        write!(self.writer, ">{}", record.id())?;

        match record.description() {
            Some(d) => {
                write!(self.writer, " {}", d)?;
            }
            None => {}
        }

        write!(self.writer, "\n")?;

        let lines: Vec<String> = record
            .sequence()
            .content()
            .chars()
            .collect::<Vec<_>>()
            .chunks(self.max_line_length)
            .map(|chunk| chunk.iter().collect())
            .collect();

        for line in lines.iter() {
            write!(self.writer, "{}\n", line)?;
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::io::sequence::Sequence;

    #[test]
    fn test_fasta_writer_missing_record_id() {
        let mut writer = FastaWriter::new(Vec::new(), None);

        let record = FastaRecord::new("", None, Sequence::new("ACT"));
        assert!(
            writer.write(&record).is_err(),
            "write() should fail if id is empty"
        )
    }

    #[test]
    fn test_fasta_writer_missing_record_sequence() {
        let mut writer = FastaWriter::new(Vec::new(), None);

        let record = FastaRecord::new("Id", None, Sequence::default());
        assert!(
            writer.write(&record).is_err(),
            "write() should fail if sequence is empty"
        )
    }

    #[test]
    fn test_fasta_writer_to_string() {
        let mut writer = FastaWriter::new(Vec::new(), None);

        let record = FastaRecord::new("X3", Some(String::from("Desc")), Sequence::new("ACT"));
        writer.write(&record).expect("write() should succeed");

        let content = String::from_utf8(writer.writer.into_inner().unwrap()).unwrap();
        assert_eq!(content, ">X3 Desc\nACT\n");
    }

    #[test]
    fn test_fasta_writer_multi_line() {
        let mut writer = FastaWriter::new(Vec::new(), Some(2));

        let record = FastaRecord::new("X3", Some(String::from("Desc")), Sequence::new("ACTCDC"));
        writer.write(&record).expect("write() should succeed");

        let content = String::from_utf8(writer.writer.into_inner().unwrap()).unwrap();
        assert_eq!(content, ">X3 Desc\nAC\nTC\nDC\n");
    }
}
