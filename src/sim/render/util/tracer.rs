//! Pixel colouring structure implementation.

use crate::{access, clone, display_field, display_field_ln, Dir3, Pos3, Ray};
use std::fmt::{Display, Formatter, Result};

/// Light quanta.
pub struct Tracer {
    /// Internal ray.
    ray: Ray,
    /// Generation.
    gen: i32,
    /// Weighting power.
    weight: f64,
    /// Cumulative distance travelled.
    dist_travelled: f64,
}

impl Tracer {
    access!(ray, Ray);
    clone!(gen, i32);
    clone!(dist_travelled, f64);
    clone!(weight, f64);

    /// Construct a new instance.
    #[inline]
    #[must_use]
    pub fn new(ray: Ray, gen: i32) -> Self {
        debug_assert!(gen >= 0);

        Self {
            ray,
            gen,
            weight: 1.0,
            dist_travelled: 0.0,
        }
    }

    /// Access the position.
    #[inline]
    #[must_use]
    pub fn pos(&self) -> &Pos3 {
        self.ray.pos()
    }

    /// Access the direction.
    #[inline]
    #[must_use]
    pub fn dir(&self) -> &Dir3 {
        self.ray.dir()
    }

    /// Move along the direction of travel a given distance.
    #[inline]
    pub fn travel(&mut self, dist: f64) {
        debug_assert!(dist > 0.0);

        self.ray.travel(dist);
        self.dist_travelled += dist;
    }
}

impl Display for Tracer {
    #[allow(clippy::result_expect_used)]
    #[inline]
    fn fmt(&self, fmt: &mut Formatter) -> Result {
        display_field_ln!(fmt, "ray", &self.ray)?;
        display_field_ln!(fmt, "generation", self.gen)?;
        display_field_ln!(fmt, "weighting", self.weight)?;
        display_field!(fmt, "distance travelled", self.dist_travelled, "m")
    }
}
