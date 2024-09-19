use std::io;
use crate::io::fasta::record::FastaRecord;

pub trait FastaRead {
    fn read(&mut self) -> io::Result<FastaRecord>;
}
