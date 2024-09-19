use std::fmt;
use crate::io::sequence::Sequence;

pub trait Record: fmt::Display {
    fn id(&self) -> &str;
    fn description(&self) -> Option<&str>;
    fn sequence(&self) -> &Sequence;
}
