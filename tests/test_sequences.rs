use std::io;
use exquisitor::io::fasta::reader::FastaReader;
use exquisitor::io::fasta::record::FastaRecord;
use exquisitor::io::sequence::Alignment;
use exquisitor::io::traits::{Reader, Record};

#[test]
fn empty() {
    assert!(true);
}

#[test]
fn preprocess_sequences() {
    let fasta = ">X1 D1\n\
    ACTGACTG\n\
    >X2 D2\n\
    ACTG\n\
    >X3 d3\n\
    ACTGAC";
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
}