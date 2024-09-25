use crate::clustering::errors::{ClusterResult, ClusteringError, ClusteringErrorKind};
use crate::clustering::traits::DistanceMetric;
use crate::io::fasta::record::FastaRecord;
use crate::io::traits::Record;

pub type DistanceMatrix = Vec<Vec<f64>>;

// region distance_matrix()

pub fn distance_matrix<Element, Metric>(elements: &Vec<Element>, metric: &Metric) -> DistanceMatrix
where
    Metric: DistanceMetric<Element>,
{
    let size = elements.len();
    let mut matrix = vec![vec![0.0; size]; size];

    for i in 0..size {
        for j in 0..size {
            let distance = metric.distance(&elements[i], &elements[j]).unwrap();
            matrix[i][j] = distance;
            matrix[j][i] = distance;
        }
    }

    matrix
}

// endregion

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

impl DistanceMetric<FastaRecord> for HammingDistance {
    fn distance(&self, a: &FastaRecord, b: &FastaRecord) -> ClusterResult<f64> {
        self.distance(a as &dyn Record, b as &dyn Record)
    }
}

// endregion

#[cfg(test)]
mod tests {
    use float_cmp::approx_eq;
    use crate::io::fasta::record::FastaRecord;
    use crate::io::sequence::Sequence;
    use super::*;

    fn record_from_sequence(id: &str, sequence: &str) -> FastaRecord {
        FastaRecord::new(String::from(id), None, Sequence::new(String::from(sequence)))
    }

    // region distance_matrix()

    #[test]
    fn test_distance_matrix() {
        let records = vec![
            record_from_sequence("X1", "ACTG"),
            record_from_sequence("X2", "ATTG"),
            record_from_sequence("X3", "CTGA")
        ];
        let expected = vec![
            vec![0f64, 1f64, 4f64],
            vec![1f64, 0f64, 3f64],
            vec![4f64, 3f64, 0f64]
        ];

        let matrix = distance_matrix(&records, &HammingDistance {});

        assert_eq!(matrix.len(), 3);
        assert_eq!(matrix[0].len(), 3);

        for i in 0..3 {
            for j in 0..3 {
                assert_eq!(matrix[i][j], expected[i][j]);
            }
        }
    }

    // endregion

    // region HammingDistance
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