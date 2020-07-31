//! Pixel colouring structure implementation.

use crate::{access, clone, display_field, display_field_ln, Ray};
use std::fmt::{Display, Formatter, Result};

/// Light quanta.
pub struct Tracer {
    /// Internal ray.
    ray: Ray,
    /// Generation.
    gen: i32,
    /// Cumulative distance travelled.
    dist_travelled: f64,
}

impl Tracer {
    access!(ray, Ray);
    clone!(gen, i32);
    clone!(dist_travelled, f64);

    /// Construct a new instance.
    #[inline]
    #[must_use]
    pub fn new(ray: Ray, gen: i32) -> Self {
        debug_assert!(gen >= 0);

        Self {
            ray,
            gen,
            dist_travelled: 0.0,
        }
    }
}

impl Display for Tracer {
    #[allow(clippy::result_expect_used)]
    #[inline]
    fn fmt(&self, fmt: &mut Formatter) -> Result {
        display_field_ln!(fmt, "ray", &self.ray)?;
        display_field_ln!(fmt, "generation", self.gen)?;
        display_field!(fmt, "distance travelled", self.dist_travelled, "m")
    }
}
