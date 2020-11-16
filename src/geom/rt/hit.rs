//! Hit implementation.

use crate::{access, clone, geom::Side};

/// Hit collision information.
#[derive(Clone)]
pub struct Hit<'a, T> {
    /// Tag reference.
    tag: &'a T,
    /// Distance to the hit.
    dist: f64,
    /// Normal of the surface.
    side: Side,
}

impl<'a, T> Hit<'a, T> {
    access!(tag, T);
    clone!(dist, dist_mut, f64);
    access!(side, Side);

    /// Construct a new instance.
    #[inline]
    #[must_use]
    pub fn new(tag: &'a T, dist: f64, side: Side) -> Self {
        debug_assert!(dist > 0.0);

        Self { tag, dist, side }
    }
}
