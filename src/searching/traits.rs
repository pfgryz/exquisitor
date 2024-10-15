use std::io;
use crate::io::sequence::Sequence;
use crate::searching::organism::OrganismMatch;

pub trait Searching {
    fn search(&self, sequences: Vec<Sequence>) -> io::Result<Vec<OrganismMatch>>;
}