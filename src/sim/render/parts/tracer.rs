//! Colour particle.

use crate::{access, geom::Ray};

/// Colour particle.
#[derive(Clone)]
pub struct Tracer {
    /// Ray of travel.
    ray: Ray,
    /// Statistical weighting.
    weight: f64,
}

impl Tracer {
    access!(ray, ray_mut, Ray);
    access!(weight, weight_mut, f64);

    /// Construct a new instance.
    #[inline]
    #[must_use]
    pub const fn new(ray: Ray, weight: f64) -> Self {
        debug_assert!(weight > 0.0);
        debug_assert!(weight <= 1.0);

        Self { ray, weight }
    }
}
