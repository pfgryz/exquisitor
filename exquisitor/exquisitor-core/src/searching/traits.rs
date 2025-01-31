use crate::io::sequence::Sequence;
use crate::searching::organism::OrganismMatch;
use std::io;

pub trait DatabaseSearch {
    /// Searches given sequences in database
    fn search(&self, sequences: Vec<Sequence>) -> io::Result<Vec<OrganismMatch>>;
}
