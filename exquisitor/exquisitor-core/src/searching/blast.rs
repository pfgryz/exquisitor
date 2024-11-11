use crate::io::fasta::record::FastaRecord;
use crate::io::fasta::writer::FastaWriter;
use crate::io::sequence::Sequence;
use crate::io::traits::Writer;
use crate::searching::organism::OrganismMatch;
use crate::searching::traits::DatabaseSearch;
use std::fs::File;
use std::io;
use std::io::{BufRead, ErrorKind, Seek, SeekFrom};
use std::path::Path;
use std::process::{Command, Stdio};
use tempfile::NamedTempFile;

pub struct Blast {
    program_path: String,
    database_path: String,
}

impl Blast {
    pub fn new(program_path: &str, database_path: &str) -> Self {
        Self {
            program_path: program_path.into(),
            database_path: database_path.into(),
        }
    }

    pub(crate) fn save_sequences_to_file(
        &self,
        sequences: &mut Vec<Sequence>,
        input_file: &File,
    ) -> std::io::Result<()> {
        let records = sequences
            .iter_mut()
            .enumerate()
            .map(|(idx, s)| FastaRecord::new(idx.to_string().as_str(), None, s.to_owned()));

        let mut writer = FastaWriter::new(input_file, None);

        for record in records {
            writer.write(&record)?
        }

        Ok(())
    }

    pub fn run(&self, input_filepath: &Path, output_filepath: &Path) -> std::io::Result<()> {
        let mut child = Command::new(&self.program_path)
            .env("BLASTDB", &self.database_path)
            .arg("-db")
            .arg("nt")
            .arg("-query")
            .arg(input_filepath)
            .arg("-out")
            .arg(output_filepath)
            .arg("-outfmt")
            .arg("6 qseqid sscinames pident")
            .stdout(Stdio::piped())
            .spawn()?;

        child.wait()?;

        Ok(())
    }

    pub(crate) fn parse_results_file(&self, path: &Path) -> io::Result<Vec<OrganismMatch>> {
        let file = File::open(path)?;
        let reader = io::BufReader::new(file);
        let mut organisms = vec![];

        for line in reader.lines() {
            let line_content = line?;
            let row = line_content.trim().split('\t').collect::<Vec<&str>>();

            if row.len() != 3 {
                return Err(io::Error::new(
                    ErrorKind::InvalidData,
                    "Result line should consist of cluster id, scientific name and quality index",
                ));
            }

            let sequence_id = row[0]
                .parse::<usize>()
                .map_err(|err| io::Error::new(ErrorKind::InvalidData, err))?;

            let confidence_score = row[2]
                .parse::<f64>()
                .map_err(|err| io::Error::new(ErrorKind::InvalidData, err))?;

            organisms.push(OrganismMatch::new(
                sequence_id,
                row[1].into(),
                confidence_score,
            ));
        }

        Ok(organisms)
    }
}

impl DatabaseSearch for Blast {
    fn search(&self, mut sequences: Vec<Sequence>) -> std::io::Result<Vec<OrganismMatch>> {
        let mut input_file = NamedTempFile::new()?;
        let output_file = NamedTempFile::new()?;

        // Save sequences in temporary file
        self.save_sequences_to_file(&mut sequences, input_file.as_file())?;

        // Run Blast
        input_file.seek(SeekFrom::Start(0))?;
        self.run(input_file.path(), output_file.path())?;

        // Get results
        Ok(self.parse_results_file(output_file.path())?)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::{Read, Seek, SeekFrom};

    // region save_sequences_to_file()

    #[test]
    fn test_save_sequences_to_file() {
        let mut file = NamedTempFile::new().unwrap();
        let mut sequences = vec![Sequence::new("AACT"), Sequence::new("TTGC")];

        // Write sequences
        let blast = Blast::new("/blast/blastn".into(), "/blast/db".into());
        blast
            .save_sequences_to_file(&mut sequences, file.as_file())
            .unwrap();

        // Seek file
        file.seek(SeekFrom::Start(0)).unwrap();

        // Read and compare file
        let result = &mut "".to_string();
        file.read_to_string(result).unwrap();

        assert_eq!(result, ">0\nAACT\n>1\nTTGC\n");
    }

    // endregion

    // region search

    #[test]
    fn test_search() {
        let sequences = vec![Sequence::new(
            "GGCTTTTTTTATGAAAAGTCTTGTGTGAGCCATGGCGACTTTTAAAGACGCTTGTTATCA
CTATAAAAAGTTGAATAAGTTAAACAGCTTAGTACTCAAACTAGGAGCAAATGATGAATG",
        )];

        let blast = Blast::new("/blast/blastn".into(), "/blast/db".into());
        let result = blast.search(sequences).unwrap();

        assert!(result.len() > 0);
    }

    // endregion
}
