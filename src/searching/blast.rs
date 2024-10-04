use std::fs::File;
use std::path::Path;
use tempfile::{NamedTempFile};
use crate::io::fasta::record::FastaRecord;
use crate::io::fasta::writer::FastaWriter;
use crate::io::sequence::Sequence;
use crate::io::traits::Writer;
use crate::searching::traits::Searching;

pub struct Blast;

impl Blast {
    pub fn run(&self, path: &Path) -> std::io::Result<()> {
        todo!()
    }

    pub(crate) fn save_sequences_to_file(&self, sequences: &mut Vec<Sequence>, input_file: &File) -> std::io::Result<()> {
        let records = sequences
            .iter_mut()
            .enumerate()
            .map(|(idx, s)| FastaRecord::new(idx.to_string(), None, s.to_owned()));

        let mut writer = FastaWriter::new(input_file);

        for record in records {
            writer.write(&record)?
        }

        Ok(())
    }
}

impl Searching for Blast {
    fn search(&self, mut sequences: Vec<Sequence>) -> std::io::Result<()> {
        let input_file = NamedTempFile::new()?;

        // Save sequences in temporary file
        self.save_sequences_to_file(&mut sequences, input_file.as_file())?;

        // Run Blast
        self.run(input_file.path())?;

        // Read results


        todo!()
    }
}

#[cfg(test)]
mod tests {
    use std::io::{Read, Seek, SeekFrom};
    use super::*;

    #[test]
    fn test_save_sequences_to_file() {
        let mut file = NamedTempFile::new().unwrap();
        let mut sequences = vec![
            Sequence::new(String::from("AACT")),
            Sequence::new(String::from("TTGC"))
        ];

        // Write sequences
        let blast = Blast {};
        blast.save_sequences_to_file(&mut sequences, file.as_file()).unwrap();

        // Seek file
        file.seek(SeekFrom::Start(0)).unwrap();

        // Read and compare file
        let mut result = &mut "".to_string();
        file.read_to_string(result).unwrap();

        assert_eq!(result, ">0\nAACT\n>1\nTTGC\n");
    }
}