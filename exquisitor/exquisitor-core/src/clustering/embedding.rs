use crate::io::sequence::Sequence;
use num_traits::pow;
use std::cmp::min;
use std::collections::HashMap;

pub type KMerEmbedding = HashMap<String, usize>;

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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::io::sequence::Sequence;
    use std::collections::HashSet;

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
}
