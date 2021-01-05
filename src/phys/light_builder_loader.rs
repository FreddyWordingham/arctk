//! Loadable light surface structure.

use crate::{
    err::Error,
    fmt_report,
    fs::{Load, Redirect},
    geom::EmitterLoader,
    math::ProbabilityBuilder,
    phys::LightBuilder,
};
use arctk_attr::file;
use std::{
    fmt::{Display, Formatter},
    path::Path,
};

/// Loadable light structure.
#[file]
pub struct LightBuilderLoader {
    /// Power [J/s].
    power: f64,
    /// Object path link.
    emit: EmitterLoader,
    /// Emission form.
    spec: Redirect<ProbabilityBuilder>,
}

impl Load for LightBuilderLoader {
    type Inst = LightBuilder;

    #[inline]
    fn load(self, in_dir: &Path) -> Result<Self::Inst, Error> {
        let emit = self.emit.load(in_dir)?;
        let spec = self.spec.load(in_dir)?;

        Ok(Self::Inst::new(self.power, emit, spec))
    }
}

impl Display for LightBuilderLoader {
    #[inline]
    fn fmt(&self, fmt: &mut Formatter) -> Result<(), std::fmt::Error> {
        writeln!(fmt, "...")?;
        fmt_report!(fmt, self.power, "power (J/s)");
        fmt_report!(fmt, self.emit, "emitter");
        fmt_report!(fmt, self.spec, "emission spectrum");
        Ok(())
    }
}
