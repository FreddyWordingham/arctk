//! Photon structure.

use crate::{access, clone, Ray};

/// Light quanta.
pub struct Photon {
    /// Internal ray.
    ray: Ray,
    /// Wavelength [m].
    wavelength: f64,
    /// Statistical weight.
    weight: f64,
}

impl Photon {
    access!(ray, mut_ray, Ray);
    clone!(wavelength, f64);
    clone!(weight, f64);

    /// Construct a new instance.
    #[inline]
    #[must_use]
    pub fn new(ray: Ray, wavelength: f64) -> Self {
        debug_assert!(wavelength > 0.0);

        Self {
            ray,
            wavelength,
            weight: 1.0,
        }
    }
}
