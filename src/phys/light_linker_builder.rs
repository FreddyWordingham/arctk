//! Light surface structure.

use crate::{
    fmt_report,
    geom::Emitter,
    math::ProbabilityBuilder,
    ord::{Build, Name},
    phys::LightLinker,
};
use std::fmt::{Display, Error, Formatter};

/// Buildable light structure.
pub struct LightLinkerBuilder {
    /// Power [J/s].
    power: f64,
    /// Object path link.
    emit: Emitter,
    /// Emission form.
    spec: ProbabilityBuilder,
    /// Emitting material.
    mat: Name,
}

impl LightLinkerBuilder {
    /// Construct a new instance.
    #[inline]
    #[must_use]
    pub fn new(power: f64, emit: Emitter, spec: ProbabilityBuilder, mat: Name) -> Self {
        Self {
            power,
            emit,
            spec,
            mat,
        }
    }
}

impl Build for LightLinkerBuilder {
    type Inst = LightLinker;

    #[inline]
    fn build(self) -> Self::Inst {
        let power = self.power;
        let emit = self.emit;
        let spec = self.spec.build();
        let mat = self.mat;

        Self::Inst::new(power, emit, spec, mat)
    }
}

impl Display for LightLinkerBuilder {
    #[inline]
    fn fmt(&self, fmt: &mut Formatter) -> Result<(), Error> {
        writeln!(fmt, "...")?;
        fmt_report!(fmt, self.power, "power (J/s)");
        fmt_report!(fmt, self.emit, "emitter");
        fmt_report!(fmt, self.spec, "emission spectrum");
        fmt_report!(fmt, self.mat, "emission material");
        Ok(())
    }
}
