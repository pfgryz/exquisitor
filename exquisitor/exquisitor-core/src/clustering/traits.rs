use crate::clustering::cluster::Cluster;
use crate::result::ExquisitorResult;

pub trait DistanceMetric<R: ?Sized> {
    /// Calculates distance between two elements
    fn distance(&self, a: &R, b: &R) -> ExquisitorResult<f64>;
}

pub trait Clustering<T: ?Sized> {
    fn cluster(&self, distances: T) -> ExquisitorResult<Vec<Cluster>>;
}
