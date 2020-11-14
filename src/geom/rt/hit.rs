//! Hit implementation.

use crate::{access, clone, geom::Side};

/// Hit collision information.
#[derive(Clone)]
pub struct Hit {
    /// Hit index.
    index: usize,
    /// Distance to the hit.
    dist: f64,
    /// Normal of the surface.
    side: Side,
}

impl Hit {
    access!(index, usize);
    clone!(dist, dist_mut, f64);
    access!(side, Side);

    /// Construct a new instance.
    #[inline]
    #[must_use]
    pub fn new(index: usize, dist: f64, side: Side) -> Self {
        debug_assert!(dist > 0.0);

        Self { index, dist, side }
    }
}
