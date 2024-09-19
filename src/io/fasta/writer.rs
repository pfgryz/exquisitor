use std::io;
use crate::io::fasta::record::FastaRecord;
use crate::io::traits::{Record, Writer};


pub struct FastaWriter<R> {
    writer: R,
    max_line_length: usize,
}

impl<R> FastaWriter<io::BufWriter<R>>
where
    R: io::Write,
{
    pub fn new(writer: R) -> Self {
        Self {
            writer: io::BufWriter::new(writer),
            max_line_length: 60,
        }
    }

    pub fn set_max_line_length(&mut self, size: usize) {
        self.max_line_length = size;
    }
}

impl<R> Writer for FastaWriter<R>
where
    R: io::Write,
{
    type Record = FastaRecord;

    fn write(&mut self, record: &FastaRecord) -> io::Result<()> {
        write!(self.writer, ">{}", record.id())?;

        match record.description() {
            Some(d) => {
                write!(self.writer, " {}", d)?;
            }
            None => {

            }
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
            writeln!(self.writer, "{}", line)?;
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::io::sequence::Sequence;
    use super::*;

    #[test]
    fn test_writer_to_string() {
        let mut writer = FastaWriter::new(Vec::new());

        let record = FastaRecord::new(String::from("X3"), Some(String::from("Desc")), Sequence::new(String::from("ACT")));
        writer.write(&record).expect("Cannot write record");

        let content = String::from_utf8(writer.writer.into_inner().unwrap()).unwrap();
        assert_eq!(content, ">X3 Desc\nACT\n");
    }

    #[test]
    fn test_writer_multi_line() {
        let mut writer = FastaWriter::new(Vec::new());
        writer.set_max_line_length(2);

        let record = FastaRecord::new(String::from("X3"), Some(String::from("Desc")), Sequence::new(String::from("ACTCDC")));
        writer.write(&record).expect("Cannot write record");

        let content = String::from_utf8(writer.writer.into_inner().unwrap()).unwrap();
        assert_eq!(content, ">X3 Desc\nAC\nTC\nDC\n");
    }
}