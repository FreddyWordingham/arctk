//! Loadable light surface structure.

use crate::{
    err::Error,
    fmt_report,
    fs::{Load, Redirect},
    geom::EmitterLoader,
    math::ProbabilityBuilder,
    ord::Name,
    phys::LightLinkerBuilder,
};
use arctk_attr::file;
use std::{
    fmt::{Display, Formatter},
    path::Path,
};

/// Loadable light structure.
#[file]
pub struct LightLinkerBuilderLoader {
    /// Power [J/s].
    power: f64,
    /// Object path link.
    emit: EmitterLoader,
    /// Emission form.
    spec: Redirect<ProbabilityBuilder>,
    /// Emitting material.
    mat: Name,
}

impl Load for LightLinkerBuilderLoader {
    type Inst = LightLinkerBuilder;

    #[inline]
    fn load(self, in_dir: &Path) -> Result<Self::Inst, Error> {
        let power = self.power;
        let emit = self.emit.load(in_dir)?;
        let spec = self.spec.load(in_dir)?;
        let mat = self.mat;

        Ok(Self::Inst::new(power, emit, spec, mat))
    }
}

impl Display for LightLinkerBuilderLoader {
    #[inline]
    fn fmt(&self, fmt: &mut Formatter) -> Result<(), std::fmt::Error> {
        writeln!(fmt, "...")?;
        fmt_report!(fmt, self.power, "power (J/s)");
        fmt_report!(fmt, self.emit, "emitter");
        fmt_report!(fmt, self.spec, "emission spectrum");
        fmt_report!(fmt, self.mat, "emission material");
        Ok(())
    }
}
