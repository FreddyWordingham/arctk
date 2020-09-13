//! Scan result enumeration.

use crate::geom::Hit;

/// Hit-scan result enumeration.
#[derive(Clone)]
pub enum Scan<T: Clone> {
    /// Boundary collision.
    Boundary(f64),
    /// Surface collision.
    Surface(Hit<T>),
}

impl<T: Clone> Scan<T> {
    /// Construct a new cell boundary detection instance.
    #[inline]
    #[must_use]
    pub fn new_boundary(dist: f64) -> Self {
        debug_assert!(dist > 0.0);

        Self::Boundary(dist)
    }

    /// Construct a new surface detection instance.
    #[inline]
    #[must_use]
    pub fn new_surface(hit: Hit<T>) -> Self {
        debug_assert!(hit.dist() > 0.0);

        Self::Surface(hit)
    }
}
