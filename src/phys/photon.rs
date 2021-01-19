//! Photon particle.

use crate::{access, clone, geom::Ray};

/// Photon.
#[derive(Clone)]
pub struct Photon {
    /// Ray of travel.
    ray: Ray,
    /// Statistical weight.
    weight: f64,
    /// Wavelength (m).
    wavelength: f64,
    /// Power (J/s).
    power: f64,
}

impl Photon {
    access!(ray, ray_mut, Ray);
    clone!(weight, weight_mut, f64);
    clone!(wavelength, wavelength_mut, f64);
    clone!(power, f64);

    /// Construct a new instance.
    #[inline]
    #[must_use]
    pub fn new(ray: Ray, wavelength: f64, power: f64) -> Self {
        debug_assert!(wavelength > 0.0);
        debug_assert!(power > 0.0);

        Self {
            ray,
            weight: 1.0,
            wavelength,
            power,
        }
    }
}
