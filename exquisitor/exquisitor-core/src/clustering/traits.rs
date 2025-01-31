use crate::clustering::cluster::Cluster;
use crate::result::ExquisitorResult;

pub trait DissimilarityMeasure<R: ?Sized> {
    /// Calculates dissimilarity between two objects
    fn dissimilarity(&self, a: &R, b: &R) -> ExquisitorResult<f64>;
}

pub trait Clustering<T: ?Sized> {
    /// Clusters the objects represents by dissimilarity matrix
    fn cluster(&self, dissimilarities: T) -> ExquisitorResult<Vec<Cluster>>;
}
