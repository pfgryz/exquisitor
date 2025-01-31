//! Module with clustering implementations

use crate::clustering::dissimilarity::DissimilarityMatrix;
use crate::clustering::traits::Clustering;
use crate::result::ExquisitorResult;
use float_cmp::approx_eq;
use kmedoids::ArrayAdapter;
use serde::Deserialize;
use serde::Serialize;
use std::collections::HashSet;
use std::fmt;
use std::fmt::Formatter;
use std::io::{Read, Result as IoResult, Write};

/// Represents the cluster with representative and all elements (including representative)
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

/// Naive clustering method
pub struct NaiveClustering {
    max_distance: f64,
}

impl NaiveClustering {
    pub fn new(max_distance: f64) -> Self {
        Self { max_distance }
    }
}

impl Clustering<DissimilarityMatrix> for NaiveClustering {
    fn cluster(&self, distances: DissimilarityMatrix) -> ExquisitorResult<Vec<Cluster>> {
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

/// K-Medoid clustering method
///
/// Wraps external algorithm provided by k-medoid crate
pub struct KMedoidClustering {
    k: usize,
}

impl KMedoidClustering {
    pub fn new(k: usize) -> Self {
        Self { k }
    }
}

struct PackedDistanceMatrix(DissimilarityMatrix);

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

impl Clustering<DissimilarityMatrix> for KMedoidClustering {
    fn cluster(&self, distances: DissimilarityMatrix) -> ExquisitorResult<Vec<Cluster>> {
        let distances = PackedDistanceMatrix(distances);
        let mut medoids =
            kmedoids::random_initialization(distances.len(), self.k, &mut rand::thread_rng());
        let (_, assignments, _, _): (f64, _, _, _) =
            kmedoids::fasterpam(&distances, &mut medoids, 100);

        let mut clusters: Vec<Vec<usize>> = vec![Vec::new(); self.k];

        for (idx, assign) in assignments.iter().enumerate() {
            clusters[*assign].push(idx);
        }

        Ok(medoids
            .iter()
            .zip(clusters.iter())
            .map(|(representative, members)| Cluster::new(*representative, members.to_owned()))
            .collect::<Vec<_>>())
    }
}

/// Saves clustering data to file
pub fn save_clustering_data(buffer: &mut dyn Write, clusters: &Vec<Cluster>) -> IoResult<()> {
    let json = serde_json::to_string(&clusters)?;
    buffer.write_all(json.as_bytes())?;
    Ok(())
}

/// Loads clustering data from file
pub fn load_clustering_data(buffer: &mut dyn Read) -> IoResult<Vec<Cluster>> {
    let mut data = String::new();
    buffer.read_to_string(&mut data)?;
    let vec: Vec<Cluster> = serde_json::from_str(&data)?;
    Ok(vec)
}

/// Calculates the FMI score between two clustering result sets
pub fn clusters_fmi_score(reference: &Vec<Cluster>, other: &Vec<Cluster>) -> f64 {
    let all = reference.len() as f64;

    let reference: HashSet<usize> =
        HashSet::from_iter(reference.iter().map(|cluster| cluster.representative_id));
    let other: HashSet<usize> =
        HashSet::from_iter(other.iter().map(|cluster| cluster.representative_id));

    reference.intersection(&other).count() as f64 / all
}

/// Calculates the entropy of the clusters set
fn clusters_entropy(clusters: &Vec<Cluster>) -> f64 {
    let all = clusters
        .iter()
        .map(|c| c.elements_ids.len() as f64)
        .sum::<f64>();

    -clusters
        .iter()
        .map(|cluster| cluster.elements_ids.len() as f64 / all)
        .map(|probability: f64| probability * probability.log(2f64))
        .sum::<f64>()
}

/// Calculates mutual information about two clustering result sets
fn clusters_mutual_information(u: &Vec<Cluster>, v: &Vec<Cluster>) -> f64 {
    let count = |cluster: &Vec<Cluster>| -> f64 {
        cluster
            .iter()
            .map(|cluster| cluster.elements_ids.len() as f64)
            .sum()
    };
    let u_count = count(u);
    let v_count = count(v);
    let all = f64::max(u_count, v_count);

    let probability_in_both = |a: &Cluster, b: &Cluster| -> f64 {
        a.elements_ids
            .iter()
            .map(|x| b.elements_ids.iter().filter(|&n| n == x).count() as f64)
            .sum::<f64>()
            / all
    };

    u.iter()
        .map(|a| {
            v.iter()
                .map(|b| {
                    (
                        probability_in_both(a, b),
                        a.elements_ids.len() as f64 / u_count,
                        b.elements_ids.len() as f64 / v_count,
                    )
                })
                .map(|(p_uv, p_u, p_v)| {
                    if approx_eq!(f64, p_uv, 0f64)
                        || approx_eq!(f64, p_u, 0f64)
                        || approx_eq!(f64, p_v, 0f64)
                    {
                        0f64
                    } else {
                        p_uv * (p_uv / (p_u * p_v)).log(2f64)
                    }
                })
                .sum::<f64>()
        })
        .sum()
}

/// Calculates the NMI score between two clustering result sets
pub fn clusters_nmi_score(reference: &Vec<Cluster>, other: &Vec<Cluster>) -> f64 {
    clusters_mutual_information(&reference, &other)
        / (clusters_entropy(&reference) * clusters_entropy(&other)).sqrt()
}

#[cfg(test)]
mod tests {
    use super::*;
    use float_cmp::assert_approx_eq;

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

    // region FMI & NMI

    #[test]
    fn test_clusters_fmi_score() {
        let first = vec![Cluster::new(2, vec![2, 3, 4]), Cluster::new(1, vec![1, 5])];
        let second = vec![Cluster::new(1, vec![1, 3]), Cluster::new(4, vec![2, 4, 5])];

        let fmi_score = clusters_fmi_score(&first, &second);
        assert_approx_eq!(f64, fmi_score, 0.5f64, epsilon = 1e-3f64);
    }

    #[test]
    fn test_clusters_entropy() {
        let clusters = vec![Cluster::new(0, vec![2, 3, 4]), Cluster::new(1, vec![1, 5])];

        let entropy = clusters_entropy(&clusters);
        assert_approx_eq!(f64, entropy, 0.970f64, epsilon = 1e-3f64);
    }

    #[test]
    fn test_clusters_mutual_information() {
        let first = vec![Cluster::new(0, vec![2, 3, 4]), Cluster::new(1, vec![1, 5])];
        let second = vec![
            Cluster::new(2, vec![1, 3]),
            Cluster::new(3, vec![2, 4]),
            Cluster::new(4, vec![5]),
        ];

        let mutual_information = clusters_mutual_information(&first, &second);
        assert_approx_eq!(f64, mutual_information, 0.570f64, epsilon = 1e-3f64);
    }

    #[test]
    fn test_clusters_nmi_score() {
        let first = vec![Cluster::new(0, vec![2, 3, 4]), Cluster::new(1, vec![1, 5])];
        let second = vec![
            Cluster::new(2, vec![1, 3]),
            Cluster::new(3, vec![2, 4]),
            Cluster::new(4, vec![5]),
        ];

        let nmi_score = clusters_nmi_score(&first, &second);
        assert_approx_eq!(f64, nmi_score, 0.469f64, epsilon = 1e-3f64);
    }

    // endregion
}
