//! Hit implementation.

use crate::rt::Side;

/// Hit collision information.
#[derive(Clone)]
pub struct Hit<'a, T> {
    /// Tag reference.
    pub tag: &'a T,
    /// Distance to the hit.
    pub dist: f64,
    /// Normal of the surface.
    pub side: Side,
}

impl<'a, T> Hit<'a, T> {
    /// Construct a new instance.
    #[inline]
    #[must_use]
    pub fn new(tag: &'a T, dist: f64, side: Side) -> Self {
        debug_assert!(dist > 0.0);

        Self { tag, dist, side }
    }
}
