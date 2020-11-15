//! Hit implementation.

use crate::{access, clone, geom::Side, opt::Attribute};

/// Hit collision information.
#[derive(Clone)]
pub struct Hit<'a> {
    /// Hit index.
    attr: &'a Attribute<'a>,
    /// Distance to the hit.
    dist: f64,
    /// Normal of the surface.
    side: Side,
}

impl<'a> Hit<'a> {
    access!(attr, Attribute);
    clone!(dist, dist_mut, f64);
    access!(side, Side);

    /// Construct a new instance.
    #[inline]
    #[must_use]
    pub fn new(attr: &'a Attribute, dist: f64, side: Side) -> Self {
        debug_assert!(dist > 0.0);

        Self { attr, dist, side }
    }
}
