//! Light surface structure.

use crate::{
    err::Error,
    fs::{Load, Redirect},
    geom::EmitterLoader,
    math::ProbabilityBuilder,
    phys::Light,
};
use arctk_attr::file;
use std::path::Path;

/// Loadable light structure.
#[file]
pub struct LightBuilder {
    /// Power [J/s].
    power: f64,
    /// Object path link.
    emit: EmitterLoader,
    /// Emission form.
    spec: Redirect<ProbabilityBuilder>,
}

impl Load for LightBuilder {
    type Inst = Light;

    #[inline]
    fn load(self, in_dir: &Path) -> Result<Self::Inst, Error> {
        let emit = self.emit.build(in_dir)?;
        let spec = self.spec.build(in_dir)?.build(in_dir)?;

        Ok(Self::Inst::new(self.power, emit, spec))
    }
}
