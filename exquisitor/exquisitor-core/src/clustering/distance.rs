use crate::clustering::embedding::KMer;
use crate::clustering::traits::DistanceMetric;
use crate::io::sequence::Sequence;
use crate::result::{ExquisitorError, ExquisitorErrorKind, ExquisitorResult};
use num_traits::{pow, One};
use std::collections::{HashMap, HashSet};
use std::iter::Sum;
use std::ops::{Mul, Sub};

/// Represents distance matrix
pub type DistanceMatrix = Vec<Vec<f64>>;

/// Calculates distance matrix between elements using given metric
pub fn distance_matrix<Element>(
    elements: &Vec<Element>,
    metric: &dyn DistanceMetric<Element>,
) -> ExquisitorResult<DistanceMatrix> {
    let size = elements.len();
    let mut matrix = vec![vec![0.0; size]; size];

    for i in 0..size {
        for j in 0..size {
            let distance = metric.distance(&elements[i], &elements[j])?;
            matrix[i][j] = distance;
            matrix[j][i] = distance;
        }
    }

    Ok(matrix)
}

/// Calculates Euclidean distance between elements
struct EuclideanDistance;

impl<T> DistanceMetric<Vec<T>> for EuclideanDistance
where
    T: Clone + One + Mul<T, Output = T>,
    for<'a> &'a T: Sub<&'a T, Output = T>,
    f64: Sum<T>,
{
    fn distance(&self, a: &Vec<T>, b: &Vec<T>) -> ExquisitorResult<f64> {
        if a.len() != b.len() {
            return Err(ExquisitorError::new(
                ExquisitorErrorKind::UnequalSequenceLengths,
                format!("{} vs {}", a.len(), b.len()),
            ));
        }

        Ok(a.iter()
            .zip(b.iter())
            .map(|(x, y)| pow(x - y, 2))
            .sum::<f64>()
            .sqrt())
    }
}

/// Similarity between nucleotides
type SimilarityMatrix = HashMap<(char, char), f64>;

/// Needleman-Wunsch algorithm
struct NeedlemanWunsch {
    gap_penalty: f64,
    similarity_matrix: SimilarityMatrix,
}

impl NeedlemanWunsch {
    pub fn new(gap_penalty: f64, similarity_matrix: SimilarityMatrix) -> NeedlemanWunsch {
        NeedlemanWunsch {
            gap_penalty,
            similarity_matrix,
        }
    }

    fn build_matrix(&self, a: &Sequence, b: &Sequence) -> DistanceMatrix {
        let mut matrix = vec![vec![0f64; a.length() + 1]; b.length() + 1];

        for row in 1..b.length() + 1 {
            matrix[row][0] = self.gap_penalty;
        }

        for column in 1..a.length() + 1 {
            matrix[0][column] = self.gap_penalty;
        }

        for row in 1..b.length() + 1 {
            for column in 1..a.length() + 1 {
                let similarity = self
                    .similarity_matrix
                    .get(&(
                        a.content().chars().nth(row - 1).unwrap(),
                        b.content().chars().nth(column - 1).unwrap(),
                    ))
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

impl DistanceMetric<Sequence> for NeedlemanWunsch {
    fn distance(&self, a: &Sequence, b: &Sequence) -> ExquisitorResult<f64> {
        if a.length() < 1 || b.length() < 1 {
            return Ok(0f64);
        }

        let matrix = self.build_matrix(a, b);
        Ok(matrix[b.length()][a.length()])
    }
}

impl DistanceMetric<Sequence> for KMer {
    fn distance(&self, a: &Sequence, b: &Sequence) -> ExquisitorResult<f64> {
        let a_embedding = &self.embed(a);
        let b_embedding = &self.embed(b);

        let keys: HashSet<String> = a_embedding
            .keys()
            .chain(b_embedding.keys())
            .cloned()
            .collect();

        let mut squared = 0f64;
        for key in keys {
            squared += (*a_embedding.get(&key).unwrap_or(&0usize) as f64
                - *b_embedding.get(&key).unwrap_or(&0usize) as f64)
                .powi(2)
        }

        Ok(squared.sqrt())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::clustering::alphabet::ALPHABET;
    use crate::io::fasta::record::FastaRecord;
    use crate::io::sequence::Sequence;
    use float_cmp::{approx_eq, assert_approx_eq};

    // region distance_matrix()

    #[test]
    fn test_distance_matrix() {
        let elements = vec![vec![0f64, 0f64], vec![3f64, 0f64], vec![0f64, 4f64]];
        let expected = vec![
            vec![0f64, 3f64, 4f64],
            vec![3f64, 0f64, 5f64],
            vec![4f64, 5f64, 0f64],
        ];

        let matrix = distance_matrix(&elements, &EuclideanDistance {});

        assert!(matrix.is_ok());
        let matrix = matrix.unwrap();

        assert_eq!(matrix.len(), 3);
        assert_eq!(matrix[0].len(), 3);

        for i in 0..3 {
            for j in 0..3 {
                assert_eq!(matrix[i][j], expected[i][j]);
            }
        }
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
        assert_eq!(
            distance.unwrap_err().kind(),
            &ExquisitorErrorKind::UnequalSequenceLengths
        );
    }

    // endregion

    // region Needleman-Wunsch

    fn create_simple_similarity_matrix() -> SimilarityMatrix {
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
        let a = Sequence::new("ACTG");
        let b = Sequence::new("ATTG");

        let similarity_matrix = create_simple_similarity_matrix();
        let metric = NeedlemanWunsch::new(-8f64, similarity_matrix);
        let matrix = metric.build_matrix(&a, &b);

        assert_eq!(matrix.len(), 5);
        assert_eq!(matrix[2].len(), 5);
        assert_approx_eq!(f64, matrix[2][2], 0f64);
        assert_approx_eq!(f64, matrix[4][3], -7f64);
    }

    #[test]
    fn test_needleman_wunsch_distance() {
        let a = Sequence::new("ACTG");
        let b = Sequence::new("ATTG");

        let similarity_matrix = create_simple_similarity_matrix();
        let metric = NeedlemanWunsch::new(-8f64, similarity_matrix);
        let distance = metric.distance(&a, &b);

        assert!(distance.is_ok());
        assert_approx_eq!(f64, distance.unwrap(), 2f64);
    }

    // endregion

    // region K-Mer Distance

    #[test]
    fn test_k_mer_distance() {
        let a = Sequence::new("ACTACG");
        let b = Sequence::new("ACTAGG");

        let metric = KMer::new(2);
        let distance = metric.distance(&a, &b);

        assert!(distance.is_ok());
        assert_approx_eq!(f64, distance.unwrap(), 2f64);
    }

    // endregion
}
