//! Emit trait.

use crate::{Dir3, Pos3, Ray, Vec3};
use rand::{rngs::ThreadRng, Rng};
use std::f64::consts::PI;

/// Emit trait implementation.
/// Types implementing this trait can cast Rays.
pub trait Emit {
    /// Cast a new ray.
    fn cast(&self, rng: &mut ThreadRng) -> Ray;
}

impl Emit for Pos3 {
    #[inline]
    #[must_use]
    fn cast(&self, rng: &mut ThreadRng) -> Ray {
        let theta = rng.gen_range(0.0, 2.0 * PI);
        let z = rng.gen_range(-1.0, 1.0);

        Ray::new(
            *self,
            Dir3::new_normalize(Vec3::new(
                (1.0_f64 - (z * z)).sqrt() * theta.cos(),
                (1.0_f64 - (z * z)).sqrt() * theta.sin(),
                z,
            )),
        )
    }
}
