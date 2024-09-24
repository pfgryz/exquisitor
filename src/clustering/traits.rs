use crate::clustering::cluster::Cluster;
use crate::clustering::errors::ClusterResult;

pub trait DistanceMetric<R: ?Sized> {
    fn distance(&self, a: &R, b: &R) -> ClusterResult<f64>;
}

pub trait Clustering<T: Sized> {
    fn cluster(&self, distances: T) -> ClusterResult<Vec<Cluster>>;
}