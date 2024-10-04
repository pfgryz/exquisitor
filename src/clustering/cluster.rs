use std::fmt;
use std::fmt::Formatter;
use crate::clustering::distance::DistanceMatrix;
use crate::clustering::errors::ClusterResult;
use crate::clustering::traits::{Clustering};


// region Cluster

#[derive(Clone, Eq, PartialEq, Hash, Debug)]
pub struct Cluster {
    representative_id: usize,
    sequence_ids: Vec<usize>,
}

impl Cluster {
    pub fn default() -> Self {
        Self {
            representative_id: 0,
            sequence_ids: vec![],
        }
    }

    pub fn new(representative_id: usize, sequence_ids: Vec<usize>) -> Self {
        Self {
            representative_id,
            sequence_ids,
        }
    }

    pub fn representative(&self) -> usize {
        self.representative_id
    }

    pub fn sequence_ids(&self) -> &Vec<usize> {
        &self.sequence_ids
    }
}

impl fmt::Display for Cluster {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "Cluster(representative={}, sequences={:?})", self.representative_id, self.sequence_ids)
    }
}

// endregion

// region Naive Clustering

pub struct NaiveClustering {
    max_distance: f64,
}

impl NaiveClustering {
    pub fn new(max_distance: f64) -> Self {
        Self {
            max_distance
        }
    }
}

impl Clustering<DistanceMatrix> for NaiveClustering {
    fn cluster(&self, distances: DistanceMatrix) -> ClusterResult<Vec<Cluster>> {
        let mut result = vec![];
        let mut used = vec![false; distances.len()];

        for i in 0..distances.len() {
            if used[i] {
                continue;
            }

            used[i] = true;
            let mut ids = vec![];
            for j in (i + 1)..distances.len() {
                if used[j] {
                    continue;
                }

                if distances[i][j] < self.max_distance {
                    ids.push(j);
                    used[j] = true;
                }
            }

            result.push(Cluster::new(i, ids));
        }


        Ok(result)
    }
}

// endregion


#[cfg(test)]
mod tests {
    use super::*;

    // region Cluster

    #[test]
    fn test_new() {
        let cluster = Cluster::new(0, vec![2, 3, 4]);

        assert_eq!(cluster.representative(), 0);
        assert_eq!(cluster.sequence_ids().len(), 3);
    }

    #[test]
    fn test_display() {
        let cluster = Cluster::new(0, vec![2, 3, 4]);

        assert_eq!(format!("{}", cluster), "Cluster(representative=0, sequences=[2, 3, 4])");
    }

    // endregion

    // region Naive Clustering

    #[test]
    fn test_naive_clustering() {
        let clustering = NaiveClustering::new(3.0f64);
        let expected = vec![
            Cluster::new(0, vec![2]),
            Cluster::new(1, vec![]),
            Cluster::new(3, vec![]),
        ];

        let distances = vec![
            vec![0f64, 4f64, 2f64, 5f64],
            vec![4f64, 0f64, 1f64, 6f64],
            vec![2f64, 1f64, 0f64, 2f64],
            vec![5f64, 6f64, 2f64, 0f64]
        ];

        let result = clustering.cluster(distances);
        assert!(result.is_ok());

        let clusters = result.unwrap();
        assert_eq!(clusters.len(), 3);

        for i in 0..3 {
            assert_eq!(clusters[i].representative_id, expected[i].representative_id);
            assert_eq!(clusters[i].sequence_ids().len(), expected[i].sequence_ids().len());

            for j in 0..clusters[i].sequence_ids().len() {
                assert_eq!(clusters[i].sequence_ids()[j], expected[i].sequence_ids()[j]);
            }
        }
    }

    // endregion
}