use std::collections::HashMap;
use crate::clustering::errors::{ClusterResult, ClusteringError, ClusteringErrorKind};
use crate::clustering::traits::DistanceMetric;
use crate::io::fasta::record::FastaRecord;
use crate::io::sequence::Sequence;
use crate::io::traits::Record;

pub type DistanceMatrix = Vec<Vec<f64>>;

const ALPHABET: &'static [char] = &['A', 'C', 'T', 'G'];

// region distance_matrix()

pub fn distance_matrix<Element>(elements: &Vec<Element>, metric: &dyn DistanceMetric<Element>) -> DistanceMatrix
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

// region Euclidean Distance

struct EuclideanDistance;

impl DistanceMetric<Vec<f64>> for EuclideanDistance {
    fn distance(&self, a: &Vec<f64>, b: &Vec<f64>) -> ClusterResult<f64> {
        if a.len() != b.len() {
            return Err(ClusteringError::new(
                ClusteringErrorKind::UnequalSequenceLengths,
                format!("{} vs {}", a.len(), b.len()),
            ));
        }

        Ok(
            a.iter()
                .zip(b.iter())
                .map(|(x, y)| (x - y).powi(2))
                .sum::<f64>()
                .sqrt()
        )
    }
}

// endregion

// region Needleman-Wunsch

type SimilarityMatrix = HashMap::<(char, char), f64>;
type NeedlemanWunschMatrix = Vec<Vec<f64>>;

struct NeedlemanWunschDistance {
    gap_penalty: f64,
    similarity_matrix: SimilarityMatrix,
}

impl NeedlemanWunschDistance {
    pub fn new(gap_penalty: f64, similarity_matrix: SimilarityMatrix) -> NeedlemanWunschDistance {
        NeedlemanWunschDistance {
            gap_penalty,
            similarity_matrix,
        }
    }

    fn build_matrix(&self, a: &Sequence, b: &Sequence) -> NeedlemanWunschMatrix {
        let mut matrix = vec![vec![0f64; a.length() + 1]; b.length() + 1];

        for row in 1..b.length() + 1 {
            matrix[row][0] = self.gap_penalty;
        }

        for column in 1..a.length() + 1 {
            matrix[0][column] = self.gap_penalty;
        }

        for row in 1..b.length() + 1 {
            for column in 1..a.length() + 1 {
                let similarity = self.similarity_matrix.get(
                    &(
                        a.content().chars().nth(row - 1).unwrap(),
                        b.content().chars().nth(column - 1).unwrap()
                    )
                )
                    .unwrap_or(&0f64);

                let diagonal = matrix[row - 1][column - 1] + similarity;
                let up = matrix[row - 1][column] + self.gap_penalty;
                let left = matrix[row][column - 1] + self.gap_penalty;
                matrix[row][column] = f64::max(diagonal, f64::max(up, left));
            }
        }

        matrix
    }
}

impl DistanceMetric<Sequence> for NeedlemanWunschDistance {
    fn distance(&self, a: &Sequence, b: &Sequence) -> ClusterResult<f64> {
        if a.length() < 1 || b.length() < 1 {
            return Ok(0f64);
        }

        let matrix = self.build_matrix(a, b);
        Ok(matrix[b.length()][a.length()])
    }
}

// endregion

#[cfg(test)]
mod tests {
    use float_cmp::{approx_eq, assert_approx_eq};
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

    // region Euclidean Distance

    #[test]
    fn test_euclidean_distance() {
        let a = vec![4f64, 6f64, 12f64];
        let b = vec![1f64, 0f64, 3f64];

        let distance = EuclideanDistance.distance(&a, &b);
        assert!(distance.is_ok());
        approx_eq!(f64, distance.unwrap(), 9f64);
    }

    #[test]
    fn test_euclidean_distance_unequal_sequences_lengths() {
        let a = vec![4f64, 6f64, 12f64];
        let b = vec![1f64, 0f64];

        let distance = EuclideanDistance.distance(&a, &b);
        assert!(distance.is_err());
        assert_eq!(distance.unwrap_err().kind(), &ClusteringErrorKind::UnequalSequenceLengths);
    }

    // endregion

    // region Needleman-Wunsch

    fn create_simple_similarity_matrix() -> SimilarityMatrix
    {
        let mut matrix = HashMap::new();
        for &x in ALPHABET {
            for &y in ALPHABET {
                let value = if x == y { 1f64 } else { -1f64 };
                matrix.insert((x, y), value);
            }
        }

        matrix
    }

    #[test]
    fn test_needleman_wunsch_distance_build_matrix() {
        /*
            Expected matrix:
            --------------------------
            \    *    A    C    T    G
            *    0   -8   -8   -8   -8
            A   -8    1   -7   -9   -8
            T   -8   -7    0   -8  -10
            T   -8   -9   -8    1   -7
            G   -8   -9  -10   -7    2
         */
        let a = Sequence::new(String::from("ACTG"));
        let b = Sequence::new(String::from("ATTG"));

        let similarity_matrix = create_simple_similarity_matrix();
        let metric = NeedlemanWunschDistance::new(-8f64, similarity_matrix);
        let matrix = metric.build_matrix(&a, &b);

        assert_eq!(matrix.len(), 5);
        assert_eq!(matrix[2].len(), 5);
        assert_approx_eq!(f64, matrix[2][2], 0f64);
        assert_approx_eq!(f64, matrix[4][3], -7f64);
    }

    #[test]
    fn test_needleman_wunsch_distance() {
        let a = Sequence::new(String::from("ACTG"));
        let b = Sequence::new(String::from("ATTG"));

        let similarity_matrix = create_simple_similarity_matrix();
        let metric = NeedlemanWunschDistance::new(-8f64, similarity_matrix);
        let distance = metric.distance(&a, &b);

        assert!(distance.is_ok());
        assert_approx_eq!(f64, distance.unwrap(), 2f64);
    }

    // endregion
}