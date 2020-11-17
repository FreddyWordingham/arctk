//! Colour particle.

use crate::{access, geom::Ray};

/// Colour particle.
#[derive(Clone)]
pub struct Tracer {
    /// Ray of travel.
    ray: Ray,
}

impl Tracer {
    access!(ray, ray_mut, Ray);

    /// Construct a new instance.
    #[inline]
    #[must_use]
    pub const fn new(ray: Ray) -> Self {
        Self { ray }
    }
}
