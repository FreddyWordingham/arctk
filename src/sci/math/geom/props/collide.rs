//! Collide trait.

use crate::Aabb;

/// Collide trait implementation.
/// Types implementing this trait can be tested for collision with an axis-aligned bounding box.
pub trait Collide {
    /// Check for an overlapping collision.
    fn overlap(&self, aabb: &Aabb) -> bool;
}
