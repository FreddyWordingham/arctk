//! Light surface structure.

use crate::{fmt_report, geom::Emitter, math::ProbabilityBuilder, ord::Build, phys::Light};
use std::fmt::{Display, Error, Formatter};

/// Buildable light structure.
pub struct LightBuilder {
    /// Power [J/s].
    power: f64,
    /// Object path link.
    emit: Emitter,
    /// Emission form.
    spec: ProbabilityBuilder,
}

impl LightBuilder {
    /// Construct a new instance.
    #[inline]
    #[must_use]
    pub fn new(power: f64, emit: Emitter, spec: ProbabilityBuilder) -> Self {
        Self { power, emit, spec }
    }
}

impl Build for LightBuilder {
    type Inst = Light;

    #[inline]
    fn build(self) -> Self::Inst {
        let power = self.power;
        let emit = self.emit;
        let spec = self.spec.build();

        Self::Inst::new(power, emit, spec)
    }
}

impl Display for LightBuilder {
    #[inline]
    fn fmt(&self, fmt: &mut Formatter) -> Result<(), Error> {
        writeln!(fmt, "...")?;
        fmt_report!(fmt, self.power, "power (J/s)");
        fmt_report!(fmt, self.emit, "emitter");
        fmt_report!(fmt, self.spec, "emission spectrum");
        Ok(())
    }
}
