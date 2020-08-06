//! Light surface structure.

use crate::{
    access, clone, display_field, display_field_ln, mcrt::Photon, Emit, Mesh, Probability,
};
use rand::rngs::ThreadRng;
use std::fmt::{Display, Formatter, Result};

/// Photon emission structure.
pub struct Light {
    /// Power [J/s].
    power: f64,
    /// Emission surface.
    surf: Mesh,
    /// Emission spectrum.
    spec: Probability,
}

impl Light {
    clone!(power, f64);
    access!(surf, Mesh);
    access!(spec, Probability);

    /// Construct a new instance.
    #[inline]
    #[must_use]
    pub fn new(power: f64, surf: Mesh, spec: Probability) -> Self {
        debug_assert!(power > 0.0);

        Self { power, surf, spec }
    }

    /// Emit a new photon.
    #[inline]
    #[must_use]
    pub fn emit(&self, mut rng: &mut ThreadRng, power: f64) -> Photon {
        debug_assert!(power > 0.0);

        let ray = self.surf.cast(&mut rng);
        let wavelength = self.spec.sample(&mut rng);

        Photon::new(ray, wavelength, power)
    }
}

impl Display for Light {
    #[allow(clippy::result_expect_used)]
    #[inline]
    fn fmt(&self, fmt: &mut Formatter) -> Result {
        display_field_ln!(fmt, "power", self.power, "J/s")?;
        display_field_ln!(fmt, "surface", &self.surf)?;
        display_field!(fmt, "spectrum", &self.spec)
    }
}
