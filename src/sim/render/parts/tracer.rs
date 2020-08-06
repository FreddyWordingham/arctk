//! Tracer structure.

use crate::{access, clone, display_field, display_field_ln, Dir3, Pos3, Ray};
use std::fmt::{Display, Formatter, Result};

/// Colouring tracer.
#[derive(Clone)]
pub struct Tracer {
    /// Internal ray.
    ray: Ray,
    /// Weighting power.
    weight: f64,
    /// Cumulative distance travelled.
    dist_travelled: f64,
}

impl Tracer {
    access!(ray, Ray);
    clone!(dist_travelled, f64);
    clone!(weight, weight_mut, f64);

    /// Construct a new instance.
    #[inline]
    #[must_use]
    pub fn new(ray: Ray) -> Self {
        Self {
            ray,
            weight: 1.0,
            dist_travelled: 0.0,
        }
    }

    /// Access the position.
    #[inline]
    #[must_use]
    pub const fn pos(&self) -> &Pos3 {
        self.ray.pos()
    }

    /// Access the direction.
    #[inline]
    #[must_use]
    pub const fn dir(&self) -> &Dir3 {
        self.ray.dir()
    }

    /// Set the tracer direction.
    #[inline]
    pub fn set_dir(&mut self, dir: Dir3) {
        *self.ray.dir_mut() = dir;
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
        display_field_ln!(fmt, "weighting", self.weight)?;
        display_field!(fmt, "distance travelled", self.dist_travelled, "m")
    }
}
