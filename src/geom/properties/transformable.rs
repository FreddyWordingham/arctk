//! Transformable trait.

use crate::math::Trans3;

/// Types implementing this trait may be transformed.
pub trait Transformable {
    /// Apply the given transformation.
    fn transform(&mut self, trans: &Trans3);
}
