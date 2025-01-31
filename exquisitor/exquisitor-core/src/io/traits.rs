//! Module containing I/O traits
use crate::io::sequence::Sequence;
use std::fmt;
use std::io::Result as IoResult;

/// Represents DNA record
pub trait Record: fmt::Display {
    /// Returns the identifier of the record.
    fn id(&self) -> &str;

    /// Returns an optional description of the record.
    fn description(&self) -> Option<&str>;

    /// Returns an immutable reference to DNA sequence of the record.
    fn sequence(&self) -> &Sequence;

    /// Returns a mutable reference to DNA sequence of the record.
    fn sequence_mut(&mut self) -> &mut Sequence;

    /// Checks if the record contains an empty sequence.
    fn is_empty(&self) -> bool;

    /// Checks if the record is valid.
    fn is_valid(&self) -> bool;
}

/// Trait for Record reader.
pub trait Reader {
    type Record: Record;
    type Iterator: Iterator;

    /// Reads a record.
    fn read(&mut self) -> IoResult<Self::Record>;

    /// Returns an iterator over reader.
    fn iter(self) -> Self::Iterator;
}

/// Trait for Record writer.
pub trait Writer {
    type Record: Record;

    /// Writes record.
    fn write(&mut self, record: &Self::Record) -> IoResult<()>;
}
