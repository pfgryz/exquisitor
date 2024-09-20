use crate::clustering::errors::ClusterResult;

pub trait DistanceMetric<R: ?Sized> {
    fn distance(&self, a: &R, b: &R) -> ClusterResult<f64>;
}