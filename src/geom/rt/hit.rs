//! Hit implementation.

use crate::{access, clone, geom::Side};

/// Hit collision information.
#[derive(Clone)]
pub struct Hit<T> {
    /// Hit tag.
    tag: T,
    /// Distance to the hit.
    dist: f64,
    /// Normal of the surface.
    side: Side,
}

impl<T> Hit<T> {
    clone!(dist, dist_mut, f64);
    access!(side, Side);

    /// Construct a new instance.
    #[inline]
    #[must_use]
    pub fn new(tag: T, dist: f64, side: Side) -> Self {
        debug_assert!(dist > 0.0);

        Self { tag, dist, side }
    }

    /// Flip the contained side.
    #[inline]
    pub fn flip_side(&mut self) {
        let s = self.side.clone().flip();
        self.side = s;
    }
}
