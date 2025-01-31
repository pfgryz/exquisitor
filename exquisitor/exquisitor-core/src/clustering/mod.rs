//! Module for clustering related functionalities
pub mod cluster;
pub mod dissimilarity;
pub mod neural;
pub mod traits;

/// Nucleotide alphabet for DNA sequences
pub const ALPHABET: &'static [char] = &['A', 'C', 'T', 'G'];
