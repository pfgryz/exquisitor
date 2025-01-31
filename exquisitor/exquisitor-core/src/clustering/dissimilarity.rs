//! Module implementing dissimilarity

use crate::clustering::traits::DissimilarityMeasure;
use crate::clustering::ALPHABET;
use crate::io::sequence::Sequence;
use crate::result::{ExquisitorError, ExquisitorErrorKind, ExquisitorResult};
use num_traits::{pow, One};
use std::cmp::min;
use std::collections::{HashMap, HashSet};
use std::iter::Sum;
use std::ops::{Mul, Sub};

/// Represents distance matrix
pub type DissimilarityMatrix = Vec<Vec<f64>>;

/// Calculates dissimilarity matrix between elements using given measure
pub fn dissimilarity_matrix<Element>(
    elements: &Vec<Element>,
    metric: &dyn DissimilarityMeasure<Element>,
) -> ExquisitorResult<DissimilarityMatrix> {
    let size = elements.len();
    let mut matrix = vec![vec![0.0; size]; size];

    for i in 0..size {
        for j in 0..size {
            let dissimilarity = metric.dissimilarity(&elements[i], &elements[j])?;
            matrix[i][j] = dissimilarity;
            matrix[j][i] = dissimilarity;
        }
    }

    Ok(matrix)
}

/// Calculates Euclidean distance between elements
pub struct EuclideanDistance;

impl<T> DissimilarityMeasure<Vec<T>> for EuclideanDistance
where
    T: Clone + One + Mul<T, Output = T>,
    for<'a> &'a T: Sub<&'a T, Output = T>,
    f64: Sum<T>,
{
    fn dissimilarity(&self, a: &Vec<T>, b: &Vec<T>) -> ExquisitorResult<f64> {
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
pub type SimilarityMatrix = HashMap<(char, char), f64>;

/// Needleman-Wunsch algorithm
pub struct NeedlemanWunsch {
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

    pub fn build_matrix(&self, a: &Sequence, b: &Sequence) -> DissimilarityMatrix {
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
                matrix[row][column] = f64::min(diagonal, f64::min(up, left));
            }
        }

        matrix
    }

    pub fn create_default_similarity_matrix() -> SimilarityMatrix {
        let mut similarity_matrix = HashMap::new();

        for &x in ALPHABET {
            for &y in ALPHABET {
                let value = if x == y { 0f64 } else { 1f64 };
                similarity_matrix.insert((x, y), value);
            }
        }

        similarity_matrix
    }
}

impl DissimilarityMeasure<Sequence> for NeedlemanWunsch {
    fn dissimilarity(&self, a: &Sequence, b: &Sequence) -> ExquisitorResult<f64> {
        if a.length() < 1 || b.length() < 1 {
            return Ok(0f64);
        }

        let matrix = self.build_matrix(a, b);
        Ok(matrix[b.length()][a.length()])
    }
}

/// K-Mer embedding type
pub type KMerEmbedding = HashMap<String, usize>;

/// K-mer embedding method
pub struct KMer {
    k: usize,
}

impl KMer {
    pub fn new(k: usize) -> Self {
        Self { k }
    }

    pub fn embed(&self, sequence: &Sequence) -> KMerEmbedding {
        let mut embedding: KMerEmbedding = HashMap::new();
        embedding.reserve(min(sequence.length() - self.k, pow(4usize, self.k)));

        for i in 0..sequence.length() - self.k + 1 {
            let mer = &sequence.content()[i..i + self.k];

            if embedding.contains_key(mer) {
                *embedding.get_mut(mer).unwrap() += 1;
            } else {
                embedding.insert(mer.into(), 1);
            }
        }

        embedding
    }
}

impl DissimilarityMeasure<Sequence> for KMer {
    fn dissimilarity(&self, a: &Sequence, b: &Sequence) -> ExquisitorResult<f64> {
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

/// Cosine dissimilarity
pub struct CosineDissimilarity;

impl<T> DissimilarityMeasure<Vec<T>> for CosineDissimilarity
where
    T: Clone + One + Mul<T, Output = T> + Into<f64>,
    for<'a> &'a T: Sub<&'a T, Output = T>,
    for<'a> &'a T: Mul<&'a T, Output = T>,
{
    fn dissimilarity(&self, a: &Vec<T>, b: &Vec<T>) -> ExquisitorResult<f64> {
        if a.len() != b.len() {
            return Err(ExquisitorError::new(
                ExquisitorErrorKind::UnequalSequenceLengths,
                format!("{} vs {}", a.len(), b.len()),
            ));
        }

        let magnitude = |x: &Vec<T>| {
            x.iter()
                .map(|x| x * x)
                .map(|x| x.into())
                .sum::<f64>()
                .sqrt()
        };

        let dot = a
            .iter()
            .zip(b.iter())
            .map(|(x, y)| x * y)
            .map(|x| x.into())
            .sum::<f64>();
        let magnitude_a: f64 = magnitude(a);
        let magnitude_b: f64 = magnitude(b);

        if magnitude_a == 0.0 || magnitude_b == 0.0 {
            return Ok(0.0f64);
        }

        let cosine_similarity = dot / magnitude_a / magnitude_b;

        Ok(1.0 - cosine_similarity)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::clustering::ALPHABET;
    use crate::io::sequence::Sequence;
    use float_cmp::{approx_eq, assert_approx_eq};
    use std::collections::HashMap;

    // region dissimilarity_matrix()

    #[test]
    fn test_dissimilarity_matrix() {
        let elements = vec![vec![0f64, 0f64], vec![3f64, 0f64], vec![0f64, 4f64]];
        let expected = vec![
            vec![0f64, 3f64, 4f64],
            vec![3f64, 0f64, 5f64],
            vec![4f64, 5f64, 0f64],
        ];

        let matrix = dissimilarity_matrix(&elements, &EuclideanDistance {});

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

        let distance = EuclideanDistance.dissimilarity(&a, &b);
        assert!(distance.is_ok());
        approx_eq!(f64, distance.unwrap(), 9f64);
    }

    #[test]
    fn test_euclidean_distance_unequal_sequences_lengths() {
        let a = vec![4f64, 6f64, 12f64];
        let b = vec![1f64, 0f64];

        let distance = EuclideanDistance.dissimilarity(&a, &b);
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
    fn test_needleman_wunsch_dissimilarity_build_matrix() {
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
        assert_approx_eq!(f64, matrix[2][2], -32f64);
        assert_approx_eq!(f64, matrix[4][3], -56f64);
    }

    #[test]
    fn test_needleman_wunsch_dissimilarity() {
        let a = Sequence::new("ACTG");
        let b = Sequence::new("ATTG");

        let similarity_matrix = create_simple_similarity_matrix();
        let metric = NeedlemanWunsch::new(-8f64, similarity_matrix);
        let dissimilarity = metric.dissimilarity(&a, &b);

        assert!(dissimilarity.is_ok());
        assert_approx_eq!(f64, dissimilarity.unwrap(), -64f64);
    }

    // endregion

    // region K-Mer Dissimilarity

    #[test]
    fn test_k_mer_embedding() {
        let a = Sequence::new("ACTAC");
        let expected_keys: HashSet<String> = ["AC", "CT", "TA"]
            .iter()
            .cloned()
            .map(String::from)
            .collect();

        let kmer = KMer::new(2);
        let embedding = kmer.embed(&a);

        assert_eq!(embedding.len(), 3);
        assert_eq!(
            embedding.keys().cloned().collect::<HashSet<_>>(),
            expected_keys
        );
        assert_eq!(embedding.get("AC"), Some(&2usize));
        assert_eq!(embedding.get("CT"), Some(&1usize));
    }

    #[test]
    fn test_k_mer_dissimilarity() {
        let a = Sequence::new("ACTACG");
        let b = Sequence::new("ACTAGG");

        let metric = KMer::new(2);
        let dissimilarity = metric.dissimilarity(&a, &b);

        assert!(dissimilarity.is_ok());
        assert_approx_eq!(f64, dissimilarity.unwrap(), 2f64);
    }

    // endregion

    // region Cosine Dissimilarity

    #[test]
    fn test_cosine_dissimilarity_unequal_lengths() {
        let dissimilarity = CosineDissimilarity.dissimilarity(&vec![0.0f64, 0.0f64], &vec![1.0f64]);

        assert!(dissimilarity.is_err());
        assert_eq!(
            dissimilarity.unwrap_err().kind(),
            &ExquisitorErrorKind::UnequalSequenceLengths
        );
    }

    #[test]
    fn test_cosine_dissimilarity_zero_magnitude() {
        let dissimilarity =
            CosineDissimilarity.dissimilarity(&vec![0.0f64, 0.0f64], &vec![1.0f64, 2.0f64]);

        assert_eq!(dissimilarity, Ok(0.0f64));
    }

    #[test]
    fn test_cosine_dissimilarity() {
        let dissimilarity =
            CosineDissimilarity.dissimilarity(&vec![0.0f64, 1.0f64], &vec![1.0f64, 0.0f64]);

        assert_eq!(dissimilarity, Ok(1.0f64));
    }

    // endregion
}
