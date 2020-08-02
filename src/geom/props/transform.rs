//! Transform trait.

use crate::Trans3;

/// Types implementing this trait may be transformed.
pub trait Transform {
    /// Apply the given transformation.
    fn transform(&mut self, trans: &Trans3);
}
