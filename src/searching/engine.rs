use std::collections::HashMap;
use std::io;
use crate::clustering::distance::{distance_matrix, DistanceMatrix};
use crate::clustering::traits::{Clustering, DistanceMetric};
use crate::io::traits::Record;
use crate::searching::organism::OrganismFound;
use crate::searching::traits::DatabaseSearch;


pub struct OrganismIdentificationEngine<R> {
    distance_metric: Box<dyn DistanceMetric<R>>,
    clustering_method: Box<dyn Clustering<DistanceMatrix>>,
    database: Box<dyn DatabaseSearch>,
}

impl<R> OrganismIdentificationEngine<R>
where
    R: Record + Clone,
{
    pub fn new(
        distance_metric: Box<dyn DistanceMetric<R>>,
        clustering_method: Box<dyn Clustering<DistanceMatrix>>,
        database: Box<dyn DatabaseSearch>,
    ) -> Self {
        Self {
            distance_metric,
            clustering_method,
            database,
        }
    }

    pub fn search(&self, records: Vec<R>) -> io::Result<Vec<OrganismFound>>
    {
        // Create distance matrix
        let distance_matrix = distance_matrix(&records, &*self.distance_metric);

        // Cluster sequences
        let clusters = self.clustering_method.cluster(distance_matrix).unwrap();

        // Extract representatives
        let representatives = clusters
            .iter()
            .filter_map(|c| records.get(c.representative()))
            .map(|r| r.sequence())
            .cloned()
            .collect();

        // Search in database
        let organism_matches = self.database.search(representatives)?;

        // Create final organism list
        let mut found = HashMap::<String, f64>::new();
        for organism_match in organism_matches {
            println!("{} {}", organism_match.sequence_id(), organism_match.name());
            let cluster = clusters
                .get(organism_match.sequence_id()).unwrap();

            let match_score = organism_match.confidence_score()
                * cluster.sequence_ids().len() as f64
                / records.len() as f64;

            match found.get_mut(organism_match.name()) {
                Some(score) => {
                    *score *= match_score;
                }
                None => {
                    found.insert(organism_match.name().into(), match_score);
                }
            }
        }

        Ok(found.iter().map(|(k, v)| OrganismFound::new(k.into(), *v)).collect::<Vec<_>>())
    }
}

#[cfg(test)]
mod tests {
    use crate::clustering::cluster::NaiveClustering;
    use crate::clustering::distance::HammingDistance;
    use crate::io::fasta::record::FastaRecord;
    use crate::io::sequence::{Alignment, Sequence};
    use crate::searching::blast::Blast;
    use super::*;

    #[test]
    fn test_search()
    {
        let rotavirus = FastaRecord::new(
            "Rotavirus".into(),
            None,
            Sequence::new(String::from("GGCTTTTTTTATGAAAAGTCTTGTGTGAGCCATGGCGACTTTTAAAGACGCTTGTTA\
            TCACTATAAAAAGTTGAATAAGTTAAACAGCTTAGTACTCAAACTAGGAGCAAATGATGAATG")),
        );
        let mut rotavirus_2 = FastaRecord::new(
            "Rotavirus Short".into(),
            None,
            Sequence::new(String::from("GGCTTTTTTTATGAAAAGTGTTGAATAAGTTAAACAGCTTAGTACTCAAACTAGGAG\
            CAAATGATGCTTGTGTGAGCCATGGCGACTTTTAAAGACGCTTGTTATCA")),
        );
        rotavirus_2.sequence_mut().pad(120, '.', Alignment::Center);

        let engine = OrganismIdentificationEngine::new(
            Box::new(HammingDistance {}),
            Box::new(NaiveClustering::new(1.0)),
            Box::new(Blast::new("/blast/blastn".into(), "/blast/db".into())),
        );

        let found = engine.search(vec![rotavirus, rotavirus_2]).unwrap();
        assert_eq!(found.len(), 7);
    }
}