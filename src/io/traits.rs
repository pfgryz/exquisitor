use std::{fmt, io};
use crate::io::sequence::Sequence;

pub trait Record: fmt::Display {
    fn id(&self) -> &str;
    fn description(&self) -> Option<&str>;
    fn sequence(&self) -> &Sequence;
    fn sequence_mut(&mut self) -> &mut Sequence;
    fn is_empty(&self) -> bool;
}

pub trait Reader {
    type Record;
    type Iterator: Iterator;

    fn read(&mut self) -> io::Result<Self::Record>;

    fn iter(self) -> Self::Iterator;
}