//! Light surface structure.

use crate::{access, display_field, display_field_ln, mcrt::Photon, Emit, Mesh, Probability};
use rand::rngs::ThreadRng;
use std::fmt::{Display, Formatter, Result};

/// Photon emission structure.
pub struct Light {
    /// Emission surface.
    surf: Mesh,
    /// Emission spectrum.
    spec: Probability,
}

impl Light {
    access!(surf, Mesh);
    access!(spec, Probability);

    /// Emit a new photon.
    #[inline]
    #[must_use]
    pub fn emit(&self, mut rng: &mut ThreadRng) -> Photon {
        let ray = self.surf.cast(&mut rng);
        let wavelength = self.spec.sample(&mut rng);
        Photon::new(ray, wavelength)
    }
}

impl Display for Light {
    #[allow(clippy::result_expect_used)]
    #[inline]
    fn fmt(&self, fmt: &mut Formatter) -> Result {
        display_field_ln!(fmt, "surface", &self.surf)?;
        display_field!(fmt, "spectrum", &self.spec)
    }
}
