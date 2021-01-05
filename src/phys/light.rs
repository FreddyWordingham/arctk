//! Light surface structure.

use crate::{access, clone, fmt_report, geom::Emitter, math::Probability, phys::Photon};
use rand::Rng;
use std::fmt::{Display, Error, Formatter};

/// Photon emission structure.
pub struct Light {
    /// Power [J/s].
    power: f64,
    /// Emitter.
    emitter: Emitter,
    /// Emission spectrum.
    spec: Probability,
}

impl Light {
    clone!(power, f64);
    access!(spec, Probability);

    /// Construct a new instance.
    #[inline]
    #[must_use]
    pub fn new(power: f64, emitter: Emitter, spec: Probability) -> Self {
        debug_assert!(power > 0.0);

        Self {
            power,
            emitter,
            spec,
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

impl Display for Light {
    #[inline]
    fn fmt(&self, fmt: &mut Formatter) -> Result<(), Error> {
        writeln!(fmt, "...")?;
        fmt_report!(fmt, self.power, "power (J/s)");
        fmt_report!(fmt, self.emitter, "emitter");
        fmt_report!(fmt, self.spec, "emission spectrum");
        Ok(())
    }
}
