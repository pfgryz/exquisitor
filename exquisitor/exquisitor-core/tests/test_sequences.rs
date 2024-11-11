use std::io;
use exquisitor_core::io::fasta::reader::FastaReader;
use exquisitor_core::io::fasta::record::FastaRecord;
use exquisitor_core::io::fasta::writer::FastaWriter;
use exquisitor_core::io::sequence::Alignment;
use exquisitor_core::io::traits::{Reader, Record, Writer};

#[test]
fn empty() {
    assert!(true);
}

#[test]
fn preprocess_sequences() {
    let fasta = ">X1 D1\n\
    ACTGACTG\n\
    >X2  \n\
    ACTG\n\
    >X3 d3\n\
    ACTGAC";
    let expected = ">X1 D1\n\
    ACTGAC\n\
    >X2\n\
    ACTG--\n\
    >X3 d3\n\
    ACTGAC\n";

    let data = io::Cursor::new(fasta);
    let reader = FastaReader::new(data);

    let sequences = ["ACTGAC", "ACTG--", "ACTGAC"];

    let records: Vec<FastaRecord> = reader
        .iter()
        .map(|r| r.unwrap())
        .map(|mut r| {
            r.sequence_mut().truncate(6, Alignment::Left);
            r.sequence_mut().pad(6, '-', Alignment::Left);
            r
        })
        .collect();

    assert_eq!(records.len(), 3);

    for (index, record) in records.iter().enumerate() {
        assert_eq!(record.sequence().length(), 6);
        assert_eq!(record.sequence().content(), sequences[index]);
    }

    let mut buffer = Vec::new();

    {
        let mut writer = FastaWriter::new(&mut buffer, None);

        for record in records.iter() {
            writer.write(record).unwrap();
        }
    }

    let result = String::from_utf8(buffer).unwrap();
    assert_eq!(result, expected);
}
