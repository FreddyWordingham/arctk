//! Scan result enumeration.

use crate::Hit;

/// Hit-scan result enumeration.
pub enum Scan<'a> {
    /// Boundary collision.
    Boundary(f64),
    /// Surface collision.
    Surface(Hit<'a>),
}

impl<'a> Scan<'a> {
    /// Construct a new cell boundary instance.
    #[inline]
    #[must_use]
    pub fn new_boundary(dist: f64) -> Self {
        debug_assert!(dist > 0.0);

        Self::Boundary(dist)
    }

    /// Construct a new surface instance.
    #[inline]
    #[must_use]
    pub fn new_surface(hit: Hit<'a>) -> Self {
        debug_assert!(hit.dist() > 0.0);

        Self::Surface(hit)
    }
}
