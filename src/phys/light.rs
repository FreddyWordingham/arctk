//! Light surface structure.

use crate::{
    access, clone, fmt_report,
    geom::Emitter,
    math::Probability,
    phys::{Material, Photon},
};
use rand::Rng;
use std::fmt::{Display, Error, Formatter};

/// Photon emission structure.
pub struct Light<'a> {
    /// Power [J/s].
    power: f64,
    /// Emitter.
    emitter: Emitter,
    /// Emission spectrum.
    spec: Probability,
    /// Emitting material.
    mat: &'a Material,
}

impl<'a> Light<'a> {
    clone!(power, f64);
    access!(spec, Probability);
    access!(mat, Material);

    /// Construct a new instance.
    #[inline]
    #[must_use]
    pub fn new(power: f64, emitter: Emitter, spec: Probability, mat: &'a Material) -> Self {
        debug_assert!(power > 0.0);

        Self {
            power,
            emitter,
            spec,
            mat,
        }
    }

    /// Emit a new photon.
    #[inline]
    #[must_use]
    pub fn emit<R: Rng>(&self, mut rng: &mut R, power: f64) -> Photon {
        debug_assert!(power > 0.0);

        let ray = self.emitter.emit(&mut rng);
        let wavelength = self.spec.sample(&mut rng);

        Photon::new(ray, wavelength, power)
    }
}

impl<'a> Display for Light<'a> {
    #[inline]
    fn fmt(&self, fmt: &mut Formatter) -> Result<(), Error> {
        writeln!(fmt, "...")?;
        fmt_report!(fmt, self.power, "power (J/s)");
        fmt_report!(fmt, self.emitter, "emitter");
        fmt_report!(fmt, self.spec, "emission spectrum");
        fmt_report!(fmt, self.mat, "emission material");
        Ok(())
    }
}
