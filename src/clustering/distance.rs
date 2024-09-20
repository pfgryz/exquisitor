use crate::clustering::errors::{ClusterResult, ClusteringError, ClusteringErrorKind};
use crate::clustering::traits::DistanceMetric;
use crate::io::traits::Record;

// region HammingDistance

pub struct HammingDistance;

impl DistanceMetric<dyn Record> for HammingDistance {
    fn distance(&self, a: &dyn Record, b: &dyn Record) -> ClusterResult<f64> {
        if a.sequence().length() != b.sequence().length() {
            return Err(ClusteringError::new(
                ClusteringErrorKind::UnequalSequenceLengths,
                format!("{} vs {}", a.sequence().length(), b.sequence().length()),
            ));
        }

        let mut distance = 0;
        for (left, right) in a.sequence().content().chars().zip(b.sequence().content().chars()) {
            if left != right {
                distance += 1;
            }
        }

        Ok(distance as f64)
    }
}

// endregion

#[cfg(test)]
mod tests {
    use float_cmp::approx_eq;
    use crate::io::fasta::record::FastaRecord;
    use crate::io::sequence::Sequence;
    use super::*;

    // region HammingDistance

    fn record_from_sequence(id: &str, sequence: &str) -> FastaRecord {
        FastaRecord::new(String::from(id), None, Sequence::new(String::from(sequence)))
    }

    #[test]
    fn test_hamming_distance() {
        let a = record_from_sequence("X1", "ACTG");
        let b = record_from_sequence("X2", "ATTG");

        let distance = HammingDistance.distance(&a, &b);
        assert!(distance.is_ok());
        approx_eq!(f64, distance.unwrap(), 1f64);
    }

    #[test]
    fn test_hamming_empty() {
        let a = FastaRecord::default();
        let b = FastaRecord::default();

        let distance = HammingDistance.distance(&a, &b);
        assert!(distance.is_ok());
        approx_eq!(f64, distance.unwrap(), 0f64);
    }

    #[test]
    fn test_hamming_unequal_sequences_lengths() {
        let a = record_from_sequence("U1", "ACTG");
        let b = record_from_sequence("U2", "ACT");

        let distance = HammingDistance.distance(&a, &b);
        assert!(distance.is_err());
        assert_eq!(distance.unwrap_err().kind(), &ClusteringErrorKind::UnequalSequenceLengths);
    }

    // endregion
}