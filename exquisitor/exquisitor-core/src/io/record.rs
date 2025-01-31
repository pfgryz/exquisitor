//! Record helper functions
use crate::io::traits::Record;
use std::io;
use std::io::Result as IoResult;

/// Validates record for saving
pub fn validate_record(record: &dyn Record) -> IoResult<()> {
    if record.id().is_empty() {
        return Err(io::Error::new(
            io::ErrorKind::Other,
            "Record should have non-empty identifier",
        ));
    }

    if !record.id().is_ascii() {
        return Err(io::Error::new(
            io::ErrorKind::Other,
            "Record identifier should be ASCII-only",
        ));
    }

    if let Some(d) = record.description() {
        if !d.is_ascii() {
            return Err(io::Error::new(
                io::ErrorKind::Other,
                "Record description should be ASCII-only",
            ));
        }
    }

    if record.sequence().length() == 0 {
        return Err(io::Error::new(
            io::ErrorKind::Other,
            "Record should have non-empty sequence",
        ));
    }

    Ok(())
}
