use std::io;
use crate::io::sequence::Sequence;

pub trait Searching {
    fn search(&self, sequences: Vec<Sequence>) -> io::Result<()>;
}