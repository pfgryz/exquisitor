use crate::clustering::distance::DistanceMatrix;
use crate::clustering::traits::Clustering;
use crate::result::ExquisitorResult;
use kmedoids::ArrayAdapter;
use serde::Deserialize;
use serde::Serialize;
use std::fmt;
use std::fmt::Formatter;
use std::io::{Read, Result as IoResult, Write};

#[derive(Clone, Eq, PartialEq, Hash, Serialize, Deserialize, Debug)]
pub struct Cluster {
    representative_id: usize,
    elements_ids: Vec<usize>,
}

impl Cluster {
    pub fn default() -> Self {
        Self {
            representative_id: 0,
            elements_ids: Vec::new(),
        }
    }

    pub fn new(representative_id: usize, elements_ids: Vec<usize>) -> Self {
        Self {
            representative_id,
            elements_ids,
        }
    }

    pub fn representative(&self) -> usize {
        self.representative_id
    }

    pub fn sequence_ids(&self) -> &Vec<usize> {
        &self.elements_ids
    }
}

impl fmt::Display for Cluster {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Cluster(representative={}, sequences={:?})",
            self.representative_id, self.elements_ids
        )
    }
}

pub struct NaiveClustering {
    max_distance: f64,
}

impl NaiveClustering {
    pub fn new(max_distance: f64) -> Self {
        Self { max_distance }
    }
}

impl Clustering<DistanceMatrix> for NaiveClustering {
    fn cluster(&self, distances: DistanceMatrix) -> ExquisitorResult<Vec<Cluster>> {
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

pub struct KMedoidClustering {
    k: usize,
}

impl KMedoidClustering {
    pub fn new(k: usize) -> Self {
        Self { k }
    }
}

struct PackedDistanceMatrix(DistanceMatrix);

impl ArrayAdapter<f64> for PackedDistanceMatrix {
    fn len(&self) -> usize {
        self.0.len()
    }

    fn is_square(&self) -> bool {
        self.0.iter().all(|v| v.len() == self.0.len())
    }

    fn get(&self, x: usize, y: usize) -> f64 {
        self.0[x][y]
    }
}

impl Clustering<DistanceMatrix> for KMedoidClustering {
    fn cluster(&self, distances: DistanceMatrix) -> ExquisitorResult<Vec<Cluster>> {
        let distances = PackedDistanceMatrix(distances);
        let mut meds =
            kmedoids::random_initialization(distances.len(), self.k, &mut rand::thread_rng());
        let (loss, assi, n_iter, n_swap): (f64, _, _, _) =
            kmedoids::fasterpam(&distances, &mut meds, 100);

        Ok(vec![])
    }
}

// @TODO: Need tests
pub fn save_clustering_data(buffer: &mut dyn Write, clusters: &Vec<Cluster>) -> IoResult<()> {
    let json = serde_json::to_string(&clusters)?;
    buffer.write_all(json.as_bytes())?;
    Ok(())
}

// @TODO: Need tests
pub fn load_clustering_data(buffer: &mut dyn Read) -> IoResult<Vec<Cluster>> {
    let mut data = String::new();
    buffer.read_to_string(&mut data)?;
    let vec: Vec<Cluster> = serde_json::from_str(&data)?;
    Ok(vec)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cluster_new() {
        let cluster = Cluster::new(0, vec![2, 3, 4]);

        assert_eq!(cluster.representative(), 0);
        assert_eq!(cluster.sequence_ids().len(), 3);
    }

    #[test]
    fn test_cluster_display() {
        let cluster = Cluster::new(0, vec![2, 3, 4]);

        assert_eq!(
            format!("{}", cluster),
            "Cluster(representative=0, sequences=[2, 3, 4])"
        );
    }

    #[test]
    fn test_naive_clustering_cluster() {
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
            vec![5f64, 6f64, 2f64, 0f64],
        ];

        let result = clustering.cluster(distances);
        assert!(result.is_ok());

        let clusters = result.unwrap();
        assert_eq!(clusters.len(), 3);

        for i in 0..3 {
            assert_eq!(clusters[i].representative_id, expected[i].representative_id);
            assert_eq!(
                clusters[i].sequence_ids().len(),
                expected[i].sequence_ids().len()
            );

            for j in 0..clusters[i].sequence_ids().len() {
                assert_eq!(clusters[i].sequence_ids()[j], expected[i].sequence_ids()[j]);
            }
        }
    }
}
