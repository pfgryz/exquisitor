use std::fmt;
use std::fmt::Formatter;

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


#[cfg(test)]
mod tests {
    use super::*;

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
}