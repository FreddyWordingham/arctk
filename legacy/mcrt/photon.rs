//! Photon structure.

use crate::{access, clone, display_field, display_field_ln, Ray};
use std::fmt::{Display, Formatter, Result};

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

impl Display for Photon {
    #[allow(clippy::result_expect_used)]
    #[inline]
    fn fmt(&self, fmt: &mut Formatter) -> Result {
        display_field_ln!(fmt, "ray", &self.ray)?;
        display_field_ln!(fmt, "wavelength", self.wavelength * 1.0e9, "nm")?;
        display_field!(fmt, "weight", self.weight)
    }
}
