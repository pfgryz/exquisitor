use crate::result::ExquisitorResult;

pub trait DistanceMetric<R: ?Sized> {
    /// Calculates distance between two elements
    fn distance(&self, a: &R, b: &R) -> ExquisitorResult<f64>;
}
