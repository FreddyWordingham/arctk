//! Trace trait.

use crate::{Ray, Side};

/// Trace trait implementation.
/// Types implementing this trait can be traced using 'Ray's.
pub trait Trace {
    /// Determine if a ray hit occurs.
    fn hit(&self, ray: &Ray) -> bool;

    /// Distance to the surface along the ray's line of travel.
    fn dist(&self, ray: &Ray) -> Option<f64>;

    /// Distance to the surface along the ray's line of travel and side of collision.
    fn dist_side(&self, ray: &Ray) -> Option<(f64, Side)>;
}
