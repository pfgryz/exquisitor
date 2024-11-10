use crate::io::fastq::record::FastqRecord;
use crate::io::sequence::Sequence;
use crate::io::traits::{Record, Writer};
use std::io;
use std::io::Result as IoResult;

/// Represents FASTQ format writer.
pub struct FastqWriter<R> {
    writer: R,
    max_line_length: usize,
}

impl<R> FastqWriter<io::BufWriter<R>>
where
    R: io::Write,
{
    /// Creates new FASTQ writer
    pub fn new(writer: R, max_line_length: Option<usize>) -> Self {
        Self {
            writer: io::BufWriter::new(writer),
            max_line_length: max_line_length.unwrap_or(60),
        }
    }
}

impl<R> Writer for FastqWriter<R>
where
    R: io::Write,
{
    type Record = FastqRecord;

    fn write(&mut self, record: &Self::Record) -> IoResult<()> {
        if record.id().is_empty() {
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "Record should have identifier",
            ));
        }

        if record.sequence().length() == 0 {
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "Record should have non-empty sequence",
            ));
        }

        if record.quality().length() == 0 {
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "Record should have non-empty quality values",
            ));
        }

        if record.sequence().length() != record.quality().length() {
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "Record sequence length should match quality values length",
            ));
        }

        write!(self.writer, "@{}", record.id())?;

        match record.description() {
            Some(d) => {
                write!(self.writer, " {}", d)?;
            }
            None => {}
        }

        write!(self.writer, "\n")?;

        let split_to_lines = |sequence: &Sequence| {
            sequence
                .content()
                .chars()
                .collect::<Vec<_>>()
                .chunks(self.max_line_length)
                .map(|chunk| chunk.iter().collect())
                .collect()
        };

        let sequence_lines: Vec<String> = split_to_lines(record.sequence());
        let quality_lines: Vec<String> = split_to_lines(record.quality());

        for line in sequence_lines.iter() {
            write!(self.writer, "{}\n", line)?;
        }

        write!(self.writer, "+\n")?;

        for line in quality_lines.iter() {
            write!(self.writer, "{}\n", line)?;
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fastq_writer_missing_record_id() {
        let mut writer = FastqWriter::new(Vec::new(), None);

        let record = FastqRecord::new("", None, Sequence::new("ACT"), Sequence::new("!.."));
        assert!(
            writer.write(&record).is_err(),
            "write() should fail if id is empty"
        )
    }

    #[test]
    fn test_fastq_writer_missing_record_sequence() {
        let mut writer = FastqWriter::new(Vec::new(), None);

        let record = FastqRecord::new("Id", None, Sequence::default(), Sequence::new("!.."));
        assert!(
            writer.write(&record).is_err(),
            "write() should fail if sequence is empty"
        )
    }

    #[test]
    fn test_fastq_writer_missing_quality_values() {
        let mut writer = FastqWriter::new(Vec::new(), None);

        let record = FastqRecord::new("Id", None, Sequence::new("ACT"), Sequence::default());
        assert!(
            writer.write(&record).is_err(),
            "write() should fail if quality values is empty"
        )
    }

    #[test]
    fn test_fastq_writer_unequal_lengths_of_sequence_and_quality() {
        let mut writer = FastqWriter::new(Vec::new(), None);

        let record = FastqRecord::new("Id", None, Sequence::new("ACT"), Sequence::new("!."));
        assert!(
            writer.write(&record).is_err(),
            "write() should fail if sequence length not match quality values length"
        )
    }

    #[test]
    fn test_fastq_writer_to_string() {
        let mut writer = FastqWriter::new(Vec::new(), None);

        let record = FastqRecord::new("Id", None, Sequence::new("ACT"), Sequence::new("!.!"));
        writer.write(&record).expect("write() should succeed");

        let content = String::from_utf8(writer.writer.into_inner().unwrap()).unwrap();
        assert_eq!(content, "@Id\nACT\n+\n!.!\n");
    }

    #[test]
    fn test_fastq_writer_multi_line() {
        let mut writer = FastqWriter::new(Vec::new(), Some(2));

        let record = FastqRecord::new("Id", None, Sequence::new("ACTGTC"), Sequence::new("!!..!!"));
        writer.write(&record).expect("write() should succeed");

        let content = String::from_utf8(writer.writer.into_inner().unwrap()).unwrap();
        assert_eq!(content, "@Id\nAC\nTG\nTC\n+\n!!\n..\n!!\n");
    }

    #[test]
    fn test_fastq_writer_many() {
        let mut writer = FastqWriter::new(Vec::new(), None);

        let records = vec![
            FastqRecord::new("Id", None, Sequence::new("ACTGTC"), Sequence::new("!!..!!")),
            FastqRecord::new("P3", None, Sequence::new("GTACTT"), Sequence::new(".!..!.")),
        ];

        for record in records {
            writer.write(&record).expect("write() should succeed");
        }

        let content = String::from_utf8(writer.writer.into_inner().unwrap()).unwrap();
        assert_eq!(content, "@Id\nACTGTC\n+\n!!..!!\n@P3\nGTACTT\n+\n.!..!.\n");
    }
}
