use crate::io::sequence::Sequence;
use std::{fmt, io};

pub trait Record: fmt::Display {
    fn id(&self) -> &str;
    fn description(&self) -> Option<&str>;
    fn sequence(&self) -> &Sequence;
    fn sequence_mut(&mut self) -> &mut Sequence;
    fn is_empty(&self) -> bool;
}

pub trait Reader {
    type Record: Record;
    type Iterator: Iterator;

    fn read(&mut self) -> io::Result<Self::Record>;

    fn iter(self) -> Self::Iterator;
}

pub trait Writer {
    type Record: Record;

    fn write(&mut self, record: &Self::Record) -> io::Result<()>;
}
